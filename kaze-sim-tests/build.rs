use kaze::*;

use std::env;
use std::fs::File;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("modules.rs");
    let mut file = File::create(&dest_path).unwrap();

    let c = Context::new();

    sim::generate(input_masking(&c), &mut file)?;
    sim::generate(widest_input(&c), &mut file)?;
    sim::generate(add_test_module(&c), &mut file)?;
    sim::generate(sub_test_module(&c), &mut file)?;
    sim::generate(mul_test_module(&c), &mut file)?;
    sim::generate(mul_signed_test_module(&c), &mut file)?;
    sim::generate(shl_test_module(&c), &mut file)?;
    sim::generate(shr_test_module(&c), &mut file)?;
    sim::generate(shr_arithmetic_test_module(&c), &mut file)?;
    sim::generate(bit_and_test_module(&c), &mut file)?;
    sim::generate(bit_or_test_module(&c), &mut file)?;
    sim::generate(bit_xor_test_module(&c), &mut file)?;
    sim::generate(not_test_module(&c), &mut file)?;
    sim::generate(reg_test_module(&c), &mut file)?;
    sim::generate(simple_reg_delay(&c), &mut file)?;
    sim::generate(bit_test_module_0(&c), &mut file)?;
    sim::generate(bit_test_module_1(&c), &mut file)?;
    sim::generate(bits_test_module_0(&c), &mut file)?;
    sim::generate(bits_test_module_1(&c), &mut file)?;
    sim::generate(repeat_test_module(&c), &mut file)?;
    sim::generate(concat_test_module(&c), &mut file)?;
    sim::generate(eq_test_module(&c), &mut file)?;
    sim::generate(ne_test_module(&c), &mut file)?;
    sim::generate(lt_test_module(&c), &mut file)?;
    sim::generate(le_test_module(&c), &mut file)?;
    sim::generate(gt_test_module(&c), &mut file)?;
    sim::generate(ge_test_module(&c), &mut file)?;
    sim::generate(lt_signed_test_module(&c), &mut file)?;
    sim::generate(le_signed_test_module(&c), &mut file)?;
    sim::generate(gt_signed_test_module(&c), &mut file)?;
    sim::generate(ge_signed_test_module(&c), &mut file)?;
    sim::generate(mux_test_module(&c), &mut file)?;
    sim::generate(instantiation_test_module_comb(&c), &mut file)?;
    sim::generate(instantiation_test_module_reg(&c), &mut file)?;
    sim::generate(nested_instantiation_test_module(&c), &mut file)?;
    sim::generate(mem_test_module_0(&c), &mut file)?;
    sim::generate(mem_test_module_1(&c), &mut file)?;
    sim::generate(mem_test_module_2(&c), &mut file)?;

    Ok(())
}

fn input_masking<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("InputMasking");

    m.output("o", m.input("i", 27));

    m
}

fn widest_input<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("WidestInput");

    m.output("o", m.input("i", 128));

    m
}

fn add_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("AddTestModule");

    let i1 = m.input("i1", 1);
    let i2 = m.input("i2", 1);
    m.output("o1", i1 + i2);

    let i3 = m.input("i3", 16);
    let i4 = m.input("i4", 16);
    m.output("o2", i3 + i4);

    let i5 = m.input("i5", 32);
    let i6 = m.input("i6", 32);
    m.output("o3", i5 + i6);

    let i7 = m.input("i7", 64);
    let i8_ = m.input("i8", 64);
    m.output("o4", i7 + i8_);

    let i9 = m.input("i9", 128);
    let i10 = m.input("i10", 128);
    m.output("o5", i9 + i10);

    let i11 = m.input("i11", 7);
    let i12 = m.input("i12", 7);
    m.output("o6", i11 + i12);

    m
}

fn sub_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("SubTestModule");

    let i1 = m.input("i1", 1);
    let i2 = m.input("i2", 1);
    m.output("o1", i1 - i2);

    let i3 = m.input("i3", 16);
    let i4 = m.input("i4", 16);
    m.output("o2", i3 - i4);

    let i5 = m.input("i5", 32);
    let i6 = m.input("i6", 32);
    m.output("o3", i5 - i6);

    let i7 = m.input("i7", 64);
    let i8_ = m.input("i8", 64);
    m.output("o4", i7 - i8_);

    let i9 = m.input("i9", 128);
    let i10 = m.input("i10", 128);
    m.output("o5", i9 - i10);

    let i11 = m.input("i11", 7);
    let i12 = m.input("i12", 7);
    m.output("o6", i11 - i12);

    m
}

fn mul_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("MulTestModule");

    let i1 = m.input("i1", 1);
    let i2 = m.input("i2", 1);
    m.output("o1", i1 * i2);

    let i3 = m.input("i3", 3);
    let i4 = m.input("i4", 4);
    m.output("o2", i3 * i4);

    let i5 = m.input("i5", 32);
    let i6 = m.input("i6", 1);
    m.output("o3", i5 * i6);

    let i7 = m.input("i7", 32);
    let i8_ = m.input("i8", 32);
    m.output("o4", i7 * i8_);

    let i9 = m.input("i9", 64);
    let i10 = m.input("i10", 1);
    m.output("o5", i9 * i10);

    let i11 = m.input("i11", 64);
    let i12 = m.input("i12", 64);
    m.output("o6", i11 * i12);

    let i13 = m.input("i13", 127);
    let i14 = m.input("i14", 1);
    m.output("o7", i13 * i14);

    m
}

fn mul_signed_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("MulSignedTestModule");

    let i1 = m.input("i1", 1);
    let i2 = m.input("i2", 1);
    m.output("o1", i1.mul_signed(i2));

    let i3 = m.input("i3", 3);
    let i4 = m.input("i4", 4);
    m.output("o2", i3.mul_signed(i4));

    let i5 = m.input("i5", 32);
    let i6 = m.input("i6", 1);
    m.output("o3", i5.mul_signed(i6));

    let i7 = m.input("i7", 32);
    let i8_ = m.input("i8", 32);
    m.output("o4", i7.mul_signed(i8_));

    let i9 = m.input("i9", 64);
    let i10 = m.input("i10", 1);
    m.output("o5", i9.mul_signed(i10));

    let i11 = m.input("i11", 64);
    let i12 = m.input("i12", 64);
    m.output("o6", i11.mul_signed(i12));

    let i13 = m.input("i13", 127);
    let i14 = m.input("i14", 1);
    m.output("o7", i13.mul_signed(i14));

    m
}

fn shl_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("ShlTestModule");

    let i1 = m.input("i1", 1);
    let i2 = m.input("i2", 1);
    m.output("o1", i1 << i2);

    let i3 = m.input("i3", 16);
    let i4 = m.input("i4", 6);
    m.output("o2", i3 << i4);

    let i5 = m.input("i5", 32);
    let i6 = m.input("i6", 32);
    m.output("o3", i5 << i6);

    let i7 = m.input("i7", 64);
    let i8_ = m.input("i8", 64);
    m.output("o4", i7 << i8_);

    let i9 = m.input("i9", 128);
    let i10 = m.input("i10", 128);
    m.output("o5", i9 << i10);

    let i11 = m.input("i11", 7);
    let i12 = m.input("i12", 7);
    m.output("o6", i11 << i12);

    let i13 = m.input("i13", 32);
    let i14 = m.input("i14", 1);
    m.output("o7", i13 << i14);

    let i15 = m.input("i15", 64);
    let i16_ = m.input("i16", 1);
    m.output("o8", i15 << i16_);

    let i17 = m.input("i17", 128);
    let i18 = m.input("i18", 1);
    m.output("o9", i17 << i18);

    m
}

fn shr_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("ShrTestModule");

    let i1 = m.input("i1", 1);
    let i2 = m.input("i2", 1);
    m.output("o1", i1 >> i2);

    let i3 = m.input("i3", 16);
    let i4 = m.input("i4", 6);
    m.output("o2", i3 >> i4);

    let i5 = m.input("i5", 32);
    let i6 = m.input("i6", 32);
    m.output("o3", i5 >> i6);

    let i7 = m.input("i7", 64);
    let i8_ = m.input("i8", 64);
    m.output("o4", i7 >> i8_);

    let i9 = m.input("i9", 128);
    let i10 = m.input("i10", 128);
    m.output("o5", i9 >> i10);

    let i11 = m.input("i11", 7);
    let i12 = m.input("i12", 7);
    m.output("o6", i11 >> i12);

    let i13 = m.input("i13", 32);
    let i14 = m.input("i14", 1);
    m.output("o7", i13 >> i14);

    let i15 = m.input("i15", 64);
    let i16_ = m.input("i16", 1);
    m.output("o8", i15 >> i16_);

    let i17 = m.input("i17", 128);
    let i18 = m.input("i18", 1);
    m.output("o9", i17 >> i18);

    m
}

fn shr_arithmetic_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("ShrArithmeticTestModule");

    let i1 = m.input("i1", 1);
    let i2 = m.input("i2", 1);
    m.output("o1", i1.shr_arithmetic(i2));

    let i3 = m.input("i3", 16);
    let i4 = m.input("i4", 6);
    m.output("o2", i3.shr_arithmetic(i4));

    let i5 = m.input("i5", 32);
    let i6 = m.input("i6", 32);
    m.output("o3", i5.shr_arithmetic(i6));

    let i7 = m.input("i7", 64);
    let i8_ = m.input("i8", 64);
    m.output("o4", i7.shr_arithmetic(i8_));

    let i9 = m.input("i9", 128);
    let i10 = m.input("i10", 128);
    m.output("o5", i9.shr_arithmetic(i10));

    let i11 = m.input("i11", 7);
    let i12 = m.input("i12", 7);
    m.output("o6", i11.shr_arithmetic(i12));

    let i13 = m.input("i13", 32);
    let i14 = m.input("i14", 1);
    m.output("o7", i13.shr_arithmetic(i14));

    let i15 = m.input("i15", 64);
    let i16_ = m.input("i16", 1);
    m.output("o8", i15.shr_arithmetic(i16_));

    let i17 = m.input("i17", 128);
    let i18 = m.input("i18", 1);
    m.output("o9", i17.shr_arithmetic(i18));

    m
}

fn bit_and_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("BitAndTestModule");

    let i1 = m.input("i1", 1);
    let i2 = m.input("i2", 1);
    m.output("o", i1 & i2);

    m
}

fn bit_or_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("BitOrTestModule");

    let i1 = m.input("i1", 1);
    let i2 = m.input("i2", 1);
    m.output("o", i1 | i2);

    m
}

fn bit_xor_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("BitXorTestModule");

    let i1 = m.input("i1", 1);
    let i2 = m.input("i2", 1);
    m.output("o", i1 ^ i2);

    m
}

fn not_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("NotTestModule");

    let i = m.input("i", 4);
    m.output("o", !i);

    m
}

fn reg_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("RegTestModule");

    let r1 = m.reg("r1", 32);
    r1.default_value(0u32);
    r1.drive_next(m.input("i1", 32));
    m.output("o1", r1.value);

    let r2 = m.reg("r2", 32);
    r2.drive_next(m.input("i2", 32));
    m.output("o2", r2.value);

    m
}

fn simple_reg_delay<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("SimpleRegDelay");

    let r1 = m.reg("r1", 100);
    r1.default_value(0u32);
    r1.drive_next(m.input("i", 100));
    let r2 = m.reg("r2", 100);
    r2.default_value(0u32);
    r2.drive_next(r1.value);
    let r3 = m.reg("r3", 100);
    r3.default_value(0u32);
    r3.drive_next(r2.value);
    m.output("o", r3.value);

    m
}

fn bit_test_module_0<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("BitTestModule0");

    let i = m.input("i", 1);
    m.output("o", i.bit(0));

    m
}

fn bit_test_module_1<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("BitTestModule1");

    let i = m.input("i", 4);
    m.output("o0", i.bit(0));
    m.output("o1", i.bit(1));
    m.output("o2", i.bit(2).bit(0));
    m.output("o3", i.bit(3));

    m
}

fn bits_test_module_0<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("BitsTestModule0");

    let i = m.input("i", 4);

    m.output("o210", i.bits(2, 0));
    m.output("o321", i.bits(3, 1).bits(2, 0));
    m.output("o10", i.bits(1, 0).bits(1, 0));
    m.output("o32", i.bits(3, 2));
    m.output("o2", i.bits(2, 2));

    m
}

fn bits_test_module_1<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("BitsTestModule1");

    let i = m.input("i", 128);

    m.output("o0", i.bits(127, 0));
    m.output("o1", i.bits(126, 0));
    m.output("o2", i.bits(127, 64));
    m.output("o3", i.bits(63, 0));
    m.output("o4", i.bits(127, 96));
    m.output("o5", i.bits(95, 64));
    m.output("o6", i.bits(63, 32).bits(31, 0));
    m.output("o7", i.bits(31, 0));
    m.output("o8", i.bits(123, 60));
    m.output("o9", i.bits(99, 99).bits(0, 0).bits(0, 0));
    m.output("o10", i.bits(63, 48));
    m.output("o11", i.bits(63, 0).bits(31, 0).bits(15, 0).bits(0, 0));

    m
}

fn repeat_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("RepeatTestModule");

    let i = m.input("i", 4);

    m.output("o0", i.repeat(1));
    m.output("o1", i.repeat(2));
    m.output("o2", i.repeat(5));
    m.output("o3", i.repeat(8));
    m.output("o4", i.repeat(16));
    m.output("o5", i.repeat(32));
    m.output("o6", i.bit(0).repeat(3));
    m.output("o7", i.bit(0).repeat(64));
    m.output("o8", i.bit(0).repeat(128));

    m
}

fn concat_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("ConcatTestModule");

    let i1 = m.input("i1", 4);
    let i2 = m.input("i2", 4);
    let i3 = m.input("i3", 32);

    m.output("o0", i1.concat(i2));
    m.output("o1", i2.concat(i1));
    m.output("o2", m.low().concat(i1));
    m.output("o3", m.high().concat(i1));
    m.output("o4", i2.bit(0).concat(i1));
    m.output("o5", i3.concat(i3));
    m.output("o6", i3.concat(i3).concat(i3).concat(i3));

    m
}

fn eq_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("EqTestModule");

    let i1 = m.input("i1", 4);
    let i2 = m.input("i2", 4);
    m.output("o1", i1.eq(i2));
    m.output("o2", i1.bit(0).eq(i2.bit(0)));

    m
}

fn ne_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("NeTestModule");

    let i1 = m.input("i1", 4);
    let i2 = m.input("i2", 4);
    m.output("o1", i1.ne(i2));
    m.output("o2", i1.bit(0).ne(i2.bit(0)));

    m
}

fn lt_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("LtTestModule");

    let i1 = m.input("i1", 4);
    let i2 = m.input("i2", 4);
    m.output("o1", i1.lt(i2));
    m.output("o2", i1.bit(0).lt(i2.bit(0)));

    m
}

fn le_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("LeTestModule");

    let i1 = m.input("i1", 4);
    let i2 = m.input("i2", 4);
    m.output("o1", i1.le(i2));
    m.output("o2", i1.bit(0).le(i2.bit(0)));

    m
}

fn gt_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("GtTestModule");

    let i1 = m.input("i1", 4);
    let i2 = m.input("i2", 4);
    m.output("o1", i1.gt(i2));
    m.output("o2", i1.bit(0).gt(i2.bit(0)));

    m
}

fn ge_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("GeTestModule");

    let i1 = m.input("i1", 4);
    let i2 = m.input("i2", 4);
    m.output("o1", i1.ge(i2));
    m.output("o2", i1.bit(0).ge(i2.bit(0)));

    m
}

fn lt_signed_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("LtSignedTestModule");

    let i1 = m.input("i1", 4);
    let i2 = m.input("i2", 4);
    m.output("o1", i1.lt_signed(i2));
    m.output("o2", i1.bits(1, 0).lt_signed(i2.bits(1, 0)));

    m
}

fn le_signed_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("LeSignedTestModule");

    let i1 = m.input("i1", 4);
    let i2 = m.input("i2", 4);
    m.output("o1", i1.le_signed(i2));
    m.output("o2", i1.bits(1, 0).le_signed(i2.bits(1, 0)));

    m
}

fn gt_signed_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("GtSignedTestModule");

    let i1 = m.input("i1", 4);
    let i2 = m.input("i2", 4);
    m.output("o1", i1.gt_signed(i2));
    m.output("o2", i1.bits(1, 0).gt_signed(i2.bits(1, 0)));

    m
}

fn ge_signed_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("GeSignedTestModule");

    let i1 = m.input("i1", 4);
    let i2 = m.input("i2", 4);
    m.output("o1", i1.ge_signed(i2));
    m.output("o2", i1.bits(1, 0).ge_signed(i2.bits(1, 0)));

    m
}

fn mux_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("MuxTestModule");

    let invert = m.input("invert", 1);

    let i1 = m.input("i1", 1);
    let i2 = m.input("i2", 1);

    let (i1, i2) = if_(m.high(), {
        let i1 = !i1;
        let i1 = i1;
        let i1 = !i1;
        if_(invert, {
            let i2 = i2;
            if_(!m.low(), {
                let i1 = !i1;
                let i1 = i1;
                let i2 = !i2;
                (i1, i2)
            })
            .else_({ (i1, i2) })
        })
        .else_({ (i1, i2) })
    })
    .else_({ (m.low(), m.low()) });

    m.output("o1", i1);
    m.output("o2", i2);

    m
}

fn instantiation_test_module_comb<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("InstantiationTestModuleCombInner");
    let i1 = m.input("i1", 32);
    let i2 = m.input("i2", 32);
    m.output("o", i1 & i2);

    let m = c.module("InstantiationTestModuleComb");
    let i1 = m.instance("inner1", "InstantiationTestModuleCombInner");
    i1.drive_input("i1", m.input("i1", 32));
    i1.drive_input("i2", m.input("i2", 32));
    let i2 = m.instance("inner2", "InstantiationTestModuleCombInner");
    i2.drive_input("i1", m.input("i3", 32));
    i2.drive_input("i2", m.input("i4", 32));
    let i3 = m.instance("inner3", "InstantiationTestModuleCombInner");
    i3.drive_input("i1", i1.output("o"));
    i3.drive_input("i2", i2.output("o"));
    m.output("o", i3.output("o"));

    m
}

fn instantiation_test_module_reg<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("InstantiationTestModuleRegInner");
    let i1 = m.input("i1", 32);
    let i2 = m.input("i2", 32);
    let r = m.reg("r", 32);
    r.default_value(0u32);
    r.drive_next(i1 & i2);
    m.output("o", r.value);

    let m = c.module("InstantiationTestModuleReg");
    let i1 = m.instance("inner1", "InstantiationTestModuleRegInner");
    i1.drive_input("i1", m.input("i1", 32));
    i1.drive_input("i2", m.input("i2", 32));
    let i2 = m.instance("inner2", "InstantiationTestModuleRegInner");
    i2.drive_input("i1", m.input("i3", 32));
    i2.drive_input("i2", m.input("i4", 32));
    let i3 = m.instance("inner3", "InstantiationTestModuleRegInner");
    i3.drive_input("i1", i1.output("o"));
    i3.drive_input("i2", i2.output("o"));
    m.output("o", i3.output("o"));

    m
}

fn nested_instantiation_test_module<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("NestedInstantiationTestModuleInnerInner");
    let i = m.input("i", 32);
    m.output("o", i);

    let m = c.module("NestedInstantiationTestModuleInner");
    let i = m.instance("inner", "NestedInstantiationTestModuleInnerInner");
    let i1 = m.input("i1", 32);
    let i2 = m.input("i2", 32);
    i.drive_input("i", i1 & i2);
    m.output("o", i.output("o"));

    let m = c.module("NestedInstantiationTestModule");
    let i1 = m.instance("inner1", "NestedInstantiationTestModuleInner");
    i1.drive_input("i1", m.input("i1", 32));
    i1.drive_input("i2", m.input("i2", 32));
    let i2 = m.instance("inner2", "NestedInstantiationTestModuleInner");
    i2.drive_input("i1", m.input("i3", 32));
    i2.drive_input("i2", m.input("i4", 32));
    let i3 = m.instance("inner3", "NestedInstantiationTestModuleInner");
    i3.drive_input("i1", i1.output("o"));
    i3.drive_input("i2", i2.output("o"));
    m.output("o", i3.output("o"));

    m
}

fn mem_test_module_0<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("MemTestModule0");

    // No initial contents, single write port, single read port
    let mem = m.mem("mem", 1, 4);
    mem.write_port(
        m.input("write_addr", 1),
        m.input("write_value", 4),
        m.input("write_enable", 1),
    );
    m.output(
        "read_data",
        mem.read_port(m.input("read_addr", 1), m.input("read_enable", 1)),
    );

    m
}

fn mem_test_module_1<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("MemTestModule1");

    // Initial contents, no write ports, single read port
    let mem = m.mem("mem", 2, 32);
    mem.initial_contents(&[0xfadebabeu32, 0xdeadbeefu32, 0xabadcafeu32, 0xabad1deau32]);
    m.output(
        "read_data",
        mem.read_port(m.input("read_addr", 2), m.input("read_enable", 1)),
    );

    m
}

fn mem_test_module_2<'a>(c: &'a Context<'a>) -> &Module<'a> {
    let m = c.module("MemTestModule2");

    // No initial contents, single write port, single read port
    let mem = m.mem("mem", 1, 1);
    mem.write_port(
        m.input("write_addr", 1),
        m.input("write_value", 1),
        m.input("write_enable", 1),
    );
    m.output(
        "read_data",
        mem.read_port(m.input("read_addr", 1), m.input("read_enable", 1)),
    );

    m
}
