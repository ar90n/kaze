use super::ir::*;
use super::module_context::*;

use crate::graph;

use typed_arena::Arena;

use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct CompiledRegister<'a> {
    pub data: &'a graph::RegisterData<'a>,
    pub value_name: String,
    pub next_name: String,
}

pub(crate) struct Compiler<'graph, 'arena> {
    context_arena: &'arena Arena<ModuleContext<'graph, 'arena>>,

    pub regs: HashMap<
        (
            &'arena ModuleContext<'graph, 'arena>,
            &'graph graph::Signal<'graph>,
        ),
        CompiledRegister<'graph>,
    >,
    signal_exprs: HashMap<
        (
            &'arena ModuleContext<'graph, 'arena>,
            &'graph graph::Signal<'graph>,
        ),
        Expr,
    >,

    pub prop_assignments: Vec<Assignment>,

    local_count: u32,
}

impl<'graph, 'arena> Compiler<'graph, 'arena> {
    pub fn new(
        context_arena: &'arena Arena<ModuleContext<'graph, 'arena>>,
    ) -> Compiler<'graph, 'arena> {
        Compiler {
            context_arena,

            regs: HashMap::new(),
            signal_exprs: HashMap::new(),

            prop_assignments: Vec::new(),

            local_count: 0,
        }
    }

    pub fn gather_regs(
        &mut self,
        signal: &'graph graph::Signal<'graph>,
        context: &'arena ModuleContext<'graph, 'arena>,
    ) {
        match signal.data {
            graph::SignalData::Lit { .. } => (),

            graph::SignalData::Input { ref name, .. } => {
                if let Some((instance, parent)) = context.instance_and_parent {
                    self.gather_regs(instance.driven_inputs.borrow()[name], parent);
                }
            }

            graph::SignalData::Reg { data } => {
                let key = (context, signal);
                if self.regs.contains_key(&key) {
                    return;
                }
                let value_name = format!("__reg_{}_{}", data.name, self.regs.len());
                let next_name = format!("{}_next", value_name);
                self.regs.insert(
                    key,
                    CompiledRegister {
                        data,
                        value_name,
                        next_name,
                    },
                );
                self.gather_regs(data.next.borrow().unwrap(), context);
            }

            graph::SignalData::UnOp { source, .. } => {
                self.gather_regs(source, context);
            }
            graph::SignalData::SimpleBinOp { lhs, rhs, .. } => {
                self.gather_regs(lhs, context);
                self.gather_regs(rhs, context);
            }
            graph::SignalData::AdditiveBinOp { lhs, rhs, .. } => {
                self.gather_regs(lhs, context);
                self.gather_regs(rhs, context);
            }
            graph::SignalData::ComparisonBinOp { lhs, rhs, .. } => {
                self.gather_regs(lhs, context);
                self.gather_regs(rhs, context);
            }
            graph::SignalData::ShiftBinOp { lhs, rhs, .. } => {
                self.gather_regs(lhs, context);
                self.gather_regs(rhs, context);
            }

            graph::SignalData::Bits { source, .. } => {
                self.gather_regs(source, context);
            }

            graph::SignalData::Repeat { source, .. } => {
                self.gather_regs(source, context);
            }
            graph::SignalData::Concat { lhs, rhs } => {
                self.gather_regs(lhs, context);
                self.gather_regs(rhs, context);
            }

            graph::SignalData::Mux {
                cond,
                when_true,
                when_false,
            } => {
                self.gather_regs(cond, context);
                self.gather_regs(when_true, context);
                self.gather_regs(when_false, context);
            }

            graph::SignalData::InstanceOutput { instance, ref name } => {
                let output = instance.instantiated_module.outputs.borrow()[name];
                let context = context.get_child(instance, self.context_arena);
                self.gather_regs(output, context);
            }
        }
    }

    pub fn compile_signal(
        &mut self,
        signal: &'graph graph::Signal<'graph>,
        context: &'arena ModuleContext<'graph, 'arena>,
    ) -> Expr {
        let key = (context, signal);
        if !self.signal_exprs.contains_key(&key) {
            let expr = match signal.data {
                graph::SignalData::Lit {
                    ref value,
                    bit_width,
                } => {
                    let value = match value {
                        graph::Constant::Bool(value) => *value as u128,
                        graph::Constant::U32(value) => *value as u128,
                        graph::Constant::U64(value) => *value as u128,
                        graph::Constant::U128(value) => *value,
                    };

                    let target_type = ValueType::from_bit_width(bit_width);
                    Expr::Constant {
                        value: match target_type {
                            ValueType::Bool => Constant::Bool(value != 0),
                            ValueType::I32 | ValueType::I64 | ValueType::I128 => unreachable!(),
                            ValueType::U32 => Constant::U32(value as _),
                            ValueType::U64 => Constant::U64(value as _),
                            ValueType::U128 => Constant::U128(value),
                        },
                    }
                }

                graph::SignalData::Input {
                    ref name,
                    bit_width,
                } => {
                    if let Some((instance, parent)) = context.instance_and_parent {
                        self.compile_signal(instance.driven_inputs.borrow()[name], parent)
                    } else {
                        let target_type = ValueType::from_bit_width(bit_width);
                        let expr = Expr::Ref {
                            name: name.clone(),
                            scope: RefScope::Member,
                        };
                        self.gen_mask(expr, bit_width, target_type)
                    }
                }

                graph::SignalData::Reg { .. } => Expr::Ref {
                    name: self.regs[&key].value_name.clone(),
                    scope: RefScope::Member,
                },

                graph::SignalData::UnOp { source, op } => {
                    let expr = self.compile_signal(source, context);
                    let expr = self.gen_temp(Expr::UnOp {
                        source: Box::new(expr),
                        op: match op {
                            graph::UnOp::Not => UnOp::Not,
                        },
                    });

                    let bit_width = source.bit_width();
                    let target_type = ValueType::from_bit_width(bit_width);
                    self.gen_mask(expr, bit_width, target_type)
                }
                graph::SignalData::SimpleBinOp { lhs, rhs, op } => {
                    let lhs = self.compile_signal(lhs, context);
                    let rhs = self.compile_signal(rhs, context);
                    self.gen_temp(Expr::InfixBinOp {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op: match op {
                            graph::SimpleBinOp::BitAnd => InfixBinOp::BitAnd,
                            graph::SimpleBinOp::BitOr => InfixBinOp::BitOr,
                            graph::SimpleBinOp::BitXor => InfixBinOp::BitXor,
                        },
                    })
                }
                graph::SignalData::AdditiveBinOp { lhs, rhs, op } => {
                    let source_bit_width = lhs.bit_width();
                    let source_type = ValueType::from_bit_width(source_bit_width);
                    let lhs = self.compile_signal(lhs, context);
                    let rhs = self.compile_signal(rhs, context);
                    let op_input_type = match source_type {
                        ValueType::Bool => ValueType::U32,
                        _ => source_type,
                    };
                    let lhs = self.gen_cast(lhs, source_type, op_input_type);
                    let rhs = self.gen_cast(rhs, source_type, op_input_type);
                    let expr = self.gen_temp(Expr::UnaryMemberCall {
                        target: Box::new(lhs),
                        name: match op {
                            graph::AdditiveBinOp::Add => "wrapping_add".into(),
                            graph::AdditiveBinOp::Sub => "wrapping_sub".into(),
                        },
                        arg: Box::new(rhs),
                    });
                    let op_output_type = op_input_type;
                    let target_bit_width = signal.bit_width();
                    let target_type = ValueType::from_bit_width(target_bit_width);
                    let expr = self.gen_cast(expr, op_output_type, target_type);
                    self.gen_mask(expr, target_bit_width, target_type)
                }
                graph::SignalData::ComparisonBinOp { lhs, rhs, op } => {
                    let source_bit_width = lhs.bit_width();
                    let source_type = ValueType::from_bit_width(source_bit_width);
                    let mut lhs = self.compile_signal(lhs, context);
                    let mut rhs = self.compile_signal(rhs, context);
                    match op {
                        graph::ComparisonBinOp::GreaterThanEqualSigned
                        | graph::ComparisonBinOp::GreaterThanSigned
                        | graph::ComparisonBinOp::LessThanEqualSigned
                        | graph::ComparisonBinOp::LessThanSigned => {
                            let source_type_signed = source_type.to_signed();
                            lhs = self.gen_cast(lhs, source_type, source_type_signed);
                            rhs = self.gen_cast(rhs, source_type, source_type_signed);
                            lhs = self.gen_sign_extend_shifts(
                                lhs,
                                source_bit_width,
                                source_type_signed,
                            );
                            rhs = self.gen_sign_extend_shifts(
                                rhs,
                                source_bit_width,
                                source_type_signed,
                            );
                        }
                        _ => (),
                    }
                    self.gen_temp(Expr::InfixBinOp {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op: match op {
                            graph::ComparisonBinOp::Equal => InfixBinOp::Equal,
                            graph::ComparisonBinOp::NotEqual => InfixBinOp::NotEqual,
                            graph::ComparisonBinOp::LessThan
                            | graph::ComparisonBinOp::LessThanSigned => InfixBinOp::LessThan,
                            graph::ComparisonBinOp::LessThanEqual
                            | graph::ComparisonBinOp::LessThanEqualSigned => {
                                InfixBinOp::LessThanEqual
                            }
                            graph::ComparisonBinOp::GreaterThan
                            | graph::ComparisonBinOp::GreaterThanSigned => InfixBinOp::GreaterThan,
                            graph::ComparisonBinOp::GreaterThanEqual
                            | graph::ComparisonBinOp::GreaterThanEqualSigned => {
                                InfixBinOp::GreaterThanEqual
                            }
                        },
                    })
                }
                graph::SignalData::ShiftBinOp { lhs, rhs, op } => {
                    let lhs_source_bit_width = lhs.bit_width();
                    let lhs_source_type = ValueType::from_bit_width(lhs_source_bit_width);
                    let rhs_source_bit_width = rhs.bit_width();
                    let rhs_source_type = ValueType::from_bit_width(rhs_source_bit_width);
                    let lhs = self.compile_signal(lhs, context);
                    let rhs = self.compile_signal(rhs, context);
                    let lhs_op_input_type = match lhs_source_type {
                        ValueType::Bool => ValueType::U32,
                        _ => lhs_source_type,
                    };
                    let lhs = self.gen_cast(lhs, lhs_source_type, lhs_op_input_type);
                    let rhs_op_input_type = match rhs_source_type {
                        ValueType::Bool => ValueType::U32,
                        _ => rhs_source_type,
                    };
                    let rhs = self.gen_cast(rhs, rhs_source_type, rhs_op_input_type);
                    let rhs = Expr::BinaryFunctionCall {
                        name: "std::cmp::min".into(),
                        lhs: Box::new(rhs),
                        rhs: Box::new(Expr::Constant {
                            value: match rhs_op_input_type {
                                ValueType::Bool
                                | ValueType::I32
                                | ValueType::I64
                                | ValueType::I128 => unreachable!(),
                                ValueType::U32 => Constant::U32(std::u32::MAX),
                                ValueType::U64 => Constant::U64(std::u32::MAX as _),
                                ValueType::U128 => Constant::U128(std::u32::MAX as _),
                            },
                        }),
                    };
                    let rhs = self.gen_cast(rhs, lhs_op_input_type, ValueType::U32);
                    let expr = Expr::UnaryMemberCall {
                        target: Box::new(lhs),
                        name: match op {
                            graph::ShiftBinOp::Shl => "checked_shl".into(),
                            graph::ShiftBinOp::Shr => "checked_shr".into(),
                        },
                        arg: Box::new(rhs),
                    };
                    let expr = self.gen_temp(Expr::UnaryMemberCall {
                        target: Box::new(expr),
                        name: "unwrap_or".into(),
                        arg: Box::new(Expr::Constant {
                            value: match lhs_op_input_type {
                                ValueType::Bool
                                | ValueType::I32
                                | ValueType::I64
                                | ValueType::I128 => unreachable!(),
                                ValueType::U32 => Constant::U32(0),
                                ValueType::U64 => Constant::U64(0),
                                ValueType::U128 => Constant::U128(0),
                            },
                        }),
                    });
                    let op_output_type = lhs_op_input_type;
                    let target_bit_width = signal.bit_width();
                    let target_type = ValueType::from_bit_width(target_bit_width);
                    let expr = self.gen_cast(expr, op_output_type, target_type);
                    self.gen_mask(expr, target_bit_width, target_type)
                }

                graph::SignalData::Bits {
                    source, range_low, ..
                } => {
                    let expr = self.compile_signal(source, context);
                    let expr = self.gen_shift_right(expr, range_low);
                    let target_bit_width = signal.bit_width();
                    let target_type = ValueType::from_bit_width(target_bit_width);
                    let expr = self.gen_cast(
                        expr,
                        ValueType::from_bit_width(source.bit_width()),
                        target_type,
                    );
                    self.gen_mask(expr, target_bit_width, target_type)
                }

                graph::SignalData::Repeat { source, count } => {
                    let expr = self.compile_signal(source, context);
                    let mut expr = self.gen_cast(
                        expr,
                        ValueType::from_bit_width(source.bit_width()),
                        ValueType::from_bit_width(signal.bit_width()),
                    );

                    if count > 1 {
                        let source_expr = expr.clone();

                        for i in 1..count {
                            let rhs =
                                self.gen_shift_left(source_expr.clone(), i * source.bit_width());
                            expr = self.gen_temp(Expr::InfixBinOp {
                                lhs: Box::new(expr),
                                rhs: Box::new(rhs),
                                op: InfixBinOp::BitOr,
                            });
                        }
                    }

                    expr
                }
                graph::SignalData::Concat { lhs, rhs } => {
                    let lhs_type = ValueType::from_bit_width(lhs.bit_width());
                    let rhs_bit_width = rhs.bit_width();
                    let rhs_type = ValueType::from_bit_width(rhs_bit_width);
                    let lhs = self.compile_signal(lhs, context);
                    let rhs = self.compile_signal(rhs, context);
                    let target_type = ValueType::from_bit_width(signal.bit_width());
                    let lhs = self.gen_cast(lhs, lhs_type, target_type);
                    let rhs = self.gen_cast(rhs, rhs_type, target_type);
                    let lhs = self.gen_shift_left(lhs, rhs_bit_width);
                    self.gen_temp(Expr::InfixBinOp {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op: InfixBinOp::BitOr,
                    })
                }

                graph::SignalData::Mux {
                    cond,
                    when_true,
                    when_false,
                } => {
                    let cond = self.compile_signal(cond, context);
                    let when_true = self.compile_signal(when_true, context);
                    let when_false = self.compile_signal(when_false, context);
                    self.gen_temp(Expr::Ternary {
                        cond: Box::new(cond),
                        when_true: Box::new(when_true),
                        when_false: Box::new(when_false),
                    })
                }

                graph::SignalData::InstanceOutput { instance, ref name } => {
                    let output = instance.instantiated_module.outputs.borrow()[name];
                    self.compile_signal(output, context.get_child(instance, self.context_arena))
                }
            };
            self.signal_exprs.insert(key.clone(), expr);
        }

        self.signal_exprs[&key].clone()
    }

    fn gen_temp(&mut self, expr: Expr) -> Expr {
        let target_name = format!("__temp_{}", self.local_count);
        self.local_count += 1;
        self.prop_assignments.push(Assignment {
            target_scope: TargetScope::Local,
            target_name: target_name.clone(),
            expr,
        });

        Expr::Ref {
            scope: RefScope::Local,
            name: target_name,
        }
    }

    fn gen_mask(&mut self, expr: Expr, bit_width: u32, target_type: ValueType) -> Expr {
        if bit_width == target_type.bit_width() {
            return expr;
        }

        let mask = (1u128 << bit_width) - 1;
        self.gen_temp(Expr::InfixBinOp {
            lhs: Box::new(expr),
            rhs: Box::new(Expr::Constant {
                value: match target_type {
                    ValueType::Bool | ValueType::I32 | ValueType::I64 | ValueType::I128 => {
                        unreachable!()
                    }
                    ValueType::U32 => Constant::U32(mask as _),
                    ValueType::U64 => Constant::U64(mask as _),
                    ValueType::U128 => Constant::U128(mask),
                },
            }),
            op: InfixBinOp::BitAnd,
        })
    }

    fn gen_shift_left(&mut self, expr: Expr, shift: u32) -> Expr {
        if shift == 0 {
            return expr;
        }

        self.gen_temp(Expr::InfixBinOp {
            lhs: Box::new(expr),
            rhs: Box::new(Expr::Constant {
                value: Constant::U32(shift),
            }),
            op: InfixBinOp::Shl,
        })
    }

    fn gen_shift_right(&mut self, expr: Expr, shift: u32) -> Expr {
        if shift == 0 {
            return expr;
        }

        self.gen_temp(Expr::InfixBinOp {
            lhs: Box::new(expr),
            rhs: Box::new(Expr::Constant {
                value: Constant::U32(shift),
            }),
            op: InfixBinOp::Shr,
        })
    }

    fn gen_cast(&mut self, expr: Expr, source_type: ValueType, target_type: ValueType) -> Expr {
        if source_type == target_type {
            return expr;
        }

        if target_type == ValueType::Bool {
            let expr = self.gen_mask(expr, 1, source_type);
            return self.gen_temp(Expr::InfixBinOp {
                lhs: Box::new(expr),
                rhs: Box::new(Expr::Constant {
                    value: match source_type {
                        ValueType::Bool | ValueType::I32 | ValueType::I64 | ValueType::I128 => {
                            unreachable!()
                        }
                        ValueType::U32 => Constant::U32(0),
                        ValueType::U64 => Constant::U64(0),
                        ValueType::U128 => Constant::U128(0),
                    },
                }),
                op: InfixBinOp::NotEqual,
            });
        }

        self.gen_temp(Expr::Cast {
            source: Box::new(expr),
            target_type,
        })
    }

    fn gen_sign_extend_shifts(
        &mut self,
        expr: Expr,
        source_bit_width: u32,
        target_type: ValueType,
    ) -> Expr {
        let shift = target_type.bit_width() - source_bit_width;
        let expr = self.gen_shift_left(expr, shift);
        self.gen_shift_right(expr, shift)
    }
}
