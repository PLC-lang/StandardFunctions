use common::compile_with_native;
use iec61131std::bistable_functionblocks::SetResetParams;
use inkwell::context::Context;
use rusty::runner::run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[repr(C)]
#[derive(Default, Debug)]
struct MainType {
    fb: SetResetParams,
    t_t_t: bool,
    t_t_f: bool,
    t_f_t: bool,
    t_f_f: bool,
    f_t_t: bool,
    f_t_f: bool,
    f_f_t: bool,
    f_f_f: bool,
}

#[test]
fn sr() {
    let prog = r#"
        PROGRAM main
		VAR
			sr_inst : SR;
			t_t_t  : BOOL;
			t_t_f  : BOOL;
			t_f_t  : BOOL;
			t_f_f  : BOOL;
			f_t_t  : BOOL;
			f_t_f  : BOOL;
			f_f_t  : BOOL;
			f_f_f  : BOOL;
		END_VAR
			t_t_t := TRUE;
			t_f_t := TRUE;
			f_t_t := TRUE;
			f_f_t := TRUE;

			sr_inst(SET1 := TRUE, RESET := TRUE, Q1 => t_t_t);
			sr_inst(SET1 := TRUE, RESET := TRUE, Q1 => t_t_f);
			sr_inst(SET1 := TRUE, RESET := FALSE, Q1 => t_f_t);
			sr_inst(SET1 := TRUE, RESET := FALSE, Q1 => t_f_f);
			sr_inst(SET1 := FALSE, RESET := TRUE, Q1 => f_t_t);
			sr_inst(SET1 := FALSE, RESET := TRUE, Q1 => f_t_f);
			sr_inst(SET1 := FALSE, RESET := FALSE, Q1 => f_f_t);
			sr_inst(SET1 := FALSE, RESET := FALSE, Q1 => f_f_f);
        END_PROGRAM
    "#;

    let source = add_std!(prog, "bistable_functionblocks.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = MainType {
        ..MainType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.t_t_t);
    assert!(main_inst.t_t_f);
    assert!(main_inst.t_f_t);
    assert!(main_inst.t_f_f);

    assert!(!main_inst.f_t_t);
    assert!(!main_inst.f_t_f);
    assert!(main_inst.f_f_t);
    assert!(!main_inst.f_f_f);
}

#[test]
fn rs() {
    let prog = r#"
        PROGRAM main
		VAR
			rs_inst : RS;
			t_t_t  : BOOL;
			t_t_f  : BOOL;
			t_f_t  : BOOL;
			t_f_f  : BOOL;
			f_t_t  : BOOL;
			f_t_f  : BOOL;
			f_f_t  : BOOL;
			f_f_f  : BOOL;
		END_VAR
			t_t_t := TRUE;
			t_f_t := TRUE;
			f_t_t := TRUE;
			f_f_t := TRUE;

			rs_inst(SET := TRUE, RESET1 := TRUE, Q1 => t_t_t);
			rs_inst(SET := TRUE, RESET1 := TRUE, Q1 => t_t_f);
			rs_inst(SET := TRUE, RESET1 := FALSE, Q1 => t_f_t);
			rs_inst(SET := TRUE, RESET1 := FALSE, Q1 => t_f_f);
			rs_inst(SET := FALSE, RESET1 := TRUE, Q1 => f_t_t);
			rs_inst(SET := FALSE, RESET1 := TRUE, Q1 => f_t_f);
			rs_inst(SET := FALSE, RESET1 := FALSE, Q1 => f_f_t);
			rs_inst(SET := FALSE, RESET1 := FALSE, Q1 => f_f_f);
        END_PROGRAM
    "#;

    let source = add_std!(prog, "bistable_functionblocks.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = MainType {
        ..MainType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.t_t_t);
    assert!(!main_inst.t_t_f);
    assert!(main_inst.t_f_t);
    assert!(main_inst.t_f_f);

    assert!(!main_inst.f_t_t);
    assert!(!main_inst.f_t_f);
    assert!(main_inst.f_f_t);
    assert!(!main_inst.f_f_f);
}
