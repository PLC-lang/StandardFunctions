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
    let src = "
	PROGRAM main
	VAR
		a : TOD;
		b : TOD;
		c : LTOD;
		d : LTOD;
	END_VAR
		a := ADD_TOD_TIME(TOD#20:00:00, TIME#1s);
		b := ADD(TOD#20:00:00, TIME#1s);
		c := ADD_LTOD_LTIME(LTOD#12:00:00, LTIME#12m12s);
		d := ADD(LTOD#12:00:00, LTIME#12m12s);
	END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 72001000);
    assert_eq!(maintype.b, 72001000);
    assert_eq!(maintype.c, 43932000);
    assert_eq!(maintype.d, 43932000);
}

#[test]
fn add_dt_time() {
    let src = "
	PROGRAM main
	VAR
		a : DT;
		b : DT;
		c : LDT;
		d : LDT;
	END_VAR
		a := ADD_DT_TIME(DT#2000-01-01-12:00:00, TIME#1d12m12s123ms);
		b := ADD(DT#2000-01-01-12:00:00, TIME#1d12m12s123ms);
		c := ADD_LDT_LTIME(LDT#2000-01-01-12:00:00, LTIME#1d12m12s123ms);
		d := ADD(LDT#2000-01-01-12:00:00, LTIME#1d12m12s123ms);
	END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 946815132123);
    assert_eq!(maintype.b, 946815132123);
    assert_eq!(maintype.c, 946815132123);
    assert_eq!(maintype.d, 946815132123);
}

#[test]
fn sub_time() {
    let src = "
	PROGRAM main
	VAR
		a : TIME;
		b : TIME;
		c : LTIME;
		d : LTIME;
	END_VAR
		a := SUB(TIME#10h50m, TIME#6h20m);
		b := SUB_TIME(TIME#5h35m20s, TIME#1h5m20s);

		c := SUB(LTIME#10h50m, LTIME#6h20m);
		d := SUB_LTIME(LTIME#5h35m20s, LTIME#1h5m20s);
	END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 16200000000000);
    assert_eq!(maintype.b, 16200000000000);
    assert_eq!(maintype.c, 16200000000000);
    assert_eq!(maintype.d, 16200000000000);
}

#[test]
fn sub_date() {
    let src = "
	PROGRAM main
	VAR
		a : TIME;
		b : TIME;
		c : LTIME;
		d : LTIME;
	END_VAR
		a := SUB(DATE#2000-12-31, DATE#2000-01-01);
		b := SUB_DATE_DATE(DATE#2000-05-21, DATE#2000-05-01);
		
		c := SUB(LDATE#2000-12-31, LDATE#2000-01-01);
		d := SUB_LDATE_LDATE(LDATE#2000-05-21, LDATE#2000-05-01);
	END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 31536000000000000);
    assert_eq!(maintype.b, 1728000000000000);
    assert_eq!(maintype.c, 31536000000000000);
    assert_eq!(maintype.d, 1728000000000000);
}

#[test]
fn sub_tod_time() {
    let src = "
	PROGRAM main
	VAR
		a : TOD;
		b : TOD;
		c : LTOD;
		d : LTOD;
	END_VAR
		a := SUB_TOD_TIME(TOD#23:10:05.123, TIME#3h10m5s123ms);
		b := SUB(TOD#23:10:05.123, TIME#3h10m5s123ms);
		c := SUB_LTOD_LTIME(LTOD#23:10:05.123, LTIME#3h10m5s123ms);
		d := SUB(LTOD#23:10:05.123, LTIME#3h10m5s123ms);
	END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 72000000);
    assert_eq!(maintype.b, 72000000);
    assert_eq!(maintype.c, 72000000);
    assert_eq!(maintype.d, 72000000);
}

#[test]
fn sub_tod() {
    let src = "
	PROGRAM main
	VAR
		a : TIME;
		b : TIME;
		c : LTIME;
		d : LTIME;
	END_VAR
		a := SUB(TOD#23:10:05.123, TOD#3:10:05.123);
		b := SUB_TOD_TOD(TOD#23:10:05.123, TOD#3:10:05.123);
		c := SUB(LTOD#23:10:05.123, LTOD#3:10:05.123);
		d := SUB_LTOD_LTOD(LTOD#23:10:05.123, LTOD#3:10:05.123);
	END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 72000000000000);
    assert_eq!(maintype.a, 72000000000000);
    assert_eq!(maintype.a, 72000000000000);
    assert_eq!(maintype.b, 72000000000000);
}

#[test]
fn sub_dt_time() {
    let src = "
	PROGRAM main
	VAR
		a : DT;
		b : DT;
		c : LDT;
		d : LDT;
	END_VAR
		a := SUB(DT#2000-01-02-21:15:12.345, TIME#1d1h15m12s345ms);
		b := SUB_DT_TIME(DT#2000-01-02-21:15:12.345, TIME#1d1h15m12s345ms);
		c := SUB(LDT#2000-01-02-21:15:12.345, LTIME#1d1h15m12s345ms);
		d := SUB_LDT_LTIME(LDT#2000-01-02-21:15:12.345, LTIME#1d1h15m12s345ms);
	END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 946756800000);
    assert_eq!(maintype.b, 946756800000);
    assert_eq!(maintype.c, 946756800000);
    assert_eq!(maintype.d, 946756800000);
}

#[test]
fn sub_dt() {
    let src = "
	PROGRAM main
	VAR
		a : TIME;
		b : TIME;
		c : LTIME;
		d : LTIME;
	END_VAR
		a := SUB(DT#2000-01-02-21:22:33.444, DT#2000-01-01-10:00:00.000);
		b := SUB_DT_DT(DT#2000-01-02-21:22:33.444, DT#2000-01-01-10:00:00.000);
		c := SUB(LDT#2000-01-02-21:22:33.444, LDT#2000-01-01-10:00:00.000);
		d := SUB_LDT_LDT(LDT#2000-01-02-21:22:33.444, LDT#2000-01-01-10:00:00.000);
	END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 127353444000000);
    assert_eq!(maintype.b, 127353444000000);
    assert_eq!(maintype.c, 127353444000000);
    assert_eq!(maintype.d, 127353444000000);
}
