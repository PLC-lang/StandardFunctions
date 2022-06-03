use common::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[allow(dead_code)]
#[derive(Default)]
#[repr(C)]
struct MainType {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

#[test]
fn add_time() {
    let src = "
	PROGRAM main
	VAR
		a : TIME;
		b : TIME;
		c : LTIME;
		d : LTIME;
	END_VAR
		a := ADD(TIME#5h,TIME#30s);
		b := ADD_TIME(TIME#10s,TIME#10s);

		c := ADD(LTIME#10s,LTIME#10s);
		d := ADD_LTIME(LTIME#10s,LTIME#10s);
	END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 18030000000000);
    assert_eq!(maintype.b, 20000000000);
    assert_eq!(maintype.c, 20000000000);
    assert_eq!(maintype.d, 20000000000);
}

#[test]
fn add_tod_time() {
    // ADD(TOD, TIME) -> fehlt (2a-Tabelle)
    let src = "
	PROGRAM main
	VAR
		a : TOD;
		b : LTOD;
	END_VAR
		a := ADD_TOD_TIME(TOD#20:00:00, TIME#1s);
		b := ADD_LTOD_LTIME(LTOD#12:00:00, LTIME#12m12s);
	END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 72001000);
    assert_eq!(maintype.b, 43932000);
}

#[test]
fn add_dt_time() {
    // ADD(DT, TIME) -> fehlt (3a-Tabelle)
    let src = "
	PROGRAM main
	VAR
		a : DT;
		b : LDT;
	END_VAR
		a := ADD_DT_TIME(DT#2000-01-01-12:00:00, TIME#1d12m12s123ms);
		b := ADD_LDT_LTIME(LDT#2000-01-01-12:00:00, LTIME#1d12m12s123ms);
	END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 946815132123);
    assert_eq!(maintype.b, 946815132123);
}
