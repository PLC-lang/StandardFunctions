use common::compile_with_native;
use iec61131std::counters::CTDParams;
use iec61131std::counters::CTUParams;
use inkwell::context::Context;
use rusty::runner::run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[repr(C)]
#[derive(Default, Debug)]
struct CTUType<T> {
    fb: CTUParams<T>,
    q: bool,
    cv: T,
}

#[test]
fn ctu_int() {
    let prog = r#"
        PROGRAM main
		VAR
			ctu_inst : CTU_INT;
			Q_res : BOOL;
			CV_res : INT;
		END_VAR
			IF Q_res THEN
				ctu_inst(CU:= TRUE, R:= TRUE, PV:= INT#3, Q:= Q_res, CV:= CV_res);
			ELSE
				ctu_inst(CU:= TRUE, R:= FALSE, PV:= INT#3, Q:= Q_res, CV:= CV_res);
				ctu_inst(CU:= FALSE, R:= FALSE, PV:= INT#3, Q:= Q_res, CV:= CV_res); // input CU evaluated by R_EDGE, this call will enable to count up again
			END_IF
        END_PROGRAM
    "#;

    let source = add_std!(prog, "counters.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = CTUType::<i16> {
        ..CTUType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 3);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctu_dint() {
    let prog = r#"
        PROGRAM main
		VAR
			ctu_inst : CTU_DINT;
			Q_res : BOOL;
			CV_res : DINT;
		END_VAR
			IF Q_res THEN
				ctu_inst(CU:= TRUE, R:= TRUE, PV:= DINT#3, Q:= Q_res, CV:= CV_res);
			ELSE
				ctu_inst(CU:= TRUE, R:= FALSE, PV:= DINT#3, Q:= Q_res, CV:= CV_res);
				ctu_inst(CU:= FALSE, R:= FALSE, PV:= DINT#3, Q:= Q_res, CV:= CV_res); // input CU evaluated by R_EDGE, this call will enable to count up again
			END_IF
        END_PROGRAM
    "#;

    let source = add_std!(prog, "counters.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = CTUType::<i32> {
        ..CTUType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 3);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctu_udint() {
    let prog = r#"
        PROGRAM main
		VAR
			ctu_inst : CTU_UDINT;
			Q_res : BOOL;
			CV_res : UDINT;
		END_VAR
			IF Q_res THEN
				ctu_inst(CU:= TRUE, R:= TRUE, PV:= UDINT#3, Q:= Q_res, CV:= CV_res);
			ELSE
				ctu_inst(CU:= TRUE, R:= FALSE, PV:= UDINT#3, Q:= Q_res, CV:= CV_res);
				ctu_inst(CU:= FALSE, R:= FALSE, PV:= UDINT#3, Q:= Q_res, CV:= CV_res); // input CU evaluated by R_EDGE, this call will enable to count up again
			END_IF
        END_PROGRAM
    "#;

    let source = add_std!(prog, "counters.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = CTUType::<u32> {
        ..CTUType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 3);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctu_lint() {
    let prog = r#"
        PROGRAM main
		VAR
			ctu_inst : CTU_LINT;
			Q_res : BOOL;
			CV_res : LINT;
		END_VAR
			IF Q_res THEN
				ctu_inst(CU:= TRUE, R:= TRUE, PV:= LINT#3, Q:= Q_res, CV:= CV_res);
			ELSE
				ctu_inst(CU:= TRUE, R:= FALSE, PV:= LINT#3, Q:= Q_res, CV:= CV_res);
				ctu_inst(CU:= FALSE, R:= FALSE, PV:= LINT#3, Q:= Q_res, CV:= CV_res); // input CU evaluated by R_EDGE, this call will enable to count up again
			END_IF
        END_PROGRAM
    "#;

    let source = add_std!(prog, "counters.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = CTUType::<i64> {
        ..CTUType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 3);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctu_ulint() {
    let prog = r#"
        PROGRAM main
		VAR
			ctu_inst : CTU_ULINT;
			Q_res : BOOL;
			CV_res : ULINT;
		END_VAR
			IF Q_res THEN
				ctu_inst(CU:= TRUE, R:= TRUE, PV:= ULINT#3, Q:= Q_res, CV:= CV_res);
			ELSE
				ctu_inst(CU:= TRUE, R:= FALSE, PV:= ULINT#3, Q:= Q_res, CV:= CV_res);
				ctu_inst(CU:= FALSE, R:= FALSE, PV:= ULINT#3, Q:= Q_res, CV:= CV_res); // input CU evaluated by R_EDGE, this call will enable to count up again
			END_IF
        END_PROGRAM
    "#;

    let source = add_std!(prog, "counters.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = CTUType::<u64> {
        ..CTUType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 3);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[repr(C)]
#[derive(Default, Debug)]
struct CTDType<T> {
    fb: CTDParams<T>,
    load: bool,
    q: bool,
    cv: T,
}

#[test]
fn ctd_int() {
    let prog = r#"
        PROGRAM main
		VAR
			ctd_inst : CTD_INT;
			load : BOOL := TRUE;
			Q_res : BOOL;
			CV_res : INT;
		END_VAR
			// on first call load should be true to set the start value
			IF load THEN
				ctd_inst(CD:= TRUE, LD:= load, PV:= INT#3, Q:= Q_res, CV:= CV_res);
				load := FALSE;
			END_IF
			ctd_inst(CD:= TRUE, LD:= load, PV:= INT#3, Q:= Q_res, CV:= CV_res);
			ctd_inst(CD:= FALSE, LD:= load, PV:= INT#3, Q:= Q_res, CV:= CV_res); // input CD evaluated by R_EDGE, this call will enable to count down again
        END_PROGRAM
    "#;

    let source = add_std!(prog, "counters.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = CTDType::<i16> {
        load: true,
        ..CTDType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, -1);
}

#[test]
fn ctd_dint() {
    let prog = r#"
        PROGRAM main
		VAR
			ctd_inst : CTD_DINT;
			load : BOOL := TRUE;
			Q_res : BOOL;
			CV_res : DINT;
		END_VAR
			// on first call load should be true to set the start value
			IF load THEN
				ctd_inst(CD:= TRUE, LD:= load, PV:= DINT#3, Q:= Q_res, CV:= CV_res);
				load := FALSE;
			END_IF
			ctd_inst(CD:= TRUE, LD:= load, PV:= DINT#3, Q:= Q_res, CV:= CV_res);
			ctd_inst(CD:= FALSE, LD:= load, PV:= DINT#3, Q:= Q_res, CV:= CV_res); // input CD evaluated by R_EDGE, this call will enable to count down again
        END_PROGRAM
    "#;

    let source = add_std!(prog, "counters.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = CTDType::<i32> {
        load: true,
        ..CTDType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, -1);
}

#[test]
fn ctd_udint() {
    let prog = r#"
        PROGRAM main
		VAR
			ctd_inst : CTD_UDINT;
			load : BOOL := TRUE;
			Q_res : BOOL;
			CV_res : UDINT;
		END_VAR
			// on first call load should be true to set the start value
			IF load THEN
				ctd_inst(CD:= TRUE, LD:= load, PV:= UDINT#3, Q:= Q_res, CV:= CV_res);
				load := FALSE;
			END_IF
			ctd_inst(CD:= TRUE, LD:= load, PV:= UDINT#3, Q:= Q_res, CV:= CV_res);
			ctd_inst(CD:= FALSE, LD:= load, PV:= UDINT#3, Q:= Q_res, CV:= CV_res); // input CD evaluated by R_EDGE, this call will enable to count down again
        END_PROGRAM
    "#;

    let source = add_std!(prog, "counters.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = CTDType::<u32> {
        load: true,
        ..CTDType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctd_lint() {
    let prog = r#"
        PROGRAM main
		VAR
			ctd_inst : CTD_LINT;
			load : BOOL := TRUE;
			Q_res : BOOL;
			CV_res : LINT;
		END_VAR
			// on first call load should be true to set the start value
			IF load THEN
				ctd_inst(CD:= TRUE, LD:= load, PV:= LINT#3, Q:= Q_res, CV:= CV_res);
				load := FALSE;
			END_IF
			ctd_inst(CD:= TRUE, LD:= load, PV:= LINT#3, Q:= Q_res, CV:= CV_res);
			ctd_inst(CD:= FALSE, LD:= load, PV:= LINT#3, Q:= Q_res, CV:= CV_res); // input CD evaluated by R_EDGE, this call will enable to count down again
        END_PROGRAM
    "#;

    let source = add_std!(prog, "counters.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = CTDType::<i64> {
        load: true,
        ..CTDType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, -1);
}

#[test]
fn ctd_ulint() {
    let prog = r#"
        PROGRAM main
		VAR
			ctd_inst : CTD_ULINT;
			load : BOOL := TRUE;
			Q_res : BOOL;
			CV_res : ULINT;
		END_VAR
			// on first call load should be true to set the start value
			IF load THEN
				ctd_inst(CD:= TRUE, LD:= load, PV:= ULINT#3, Q:= Q_res, CV:= CV_res);
				load := FALSE;
			END_IF
			ctd_inst(CD:= TRUE, LD:= load, PV:= ULINT#3, Q:= Q_res, CV:= CV_res);
			ctd_inst(CD:= FALSE, LD:= load, PV:= ULINT#3, Q:= Q_res, CV:= CV_res); // input CD evaluated by R_EDGE, this call will enable to count down again
        END_PROGRAM
    "#;

    let source = add_std!(prog, "counters.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = CTDType::<u64> {
        load: true,
        ..CTDType::default()
    };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
}
