use common::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[allow(dead_code)]
#[repr(C)]
struct SingleType {
    a: [usize; 1000],
}

impl Default for SingleType {
    fn default() -> Self {
        SingleType { a: [0; 1000] }
    }
}

#[test]
fn concat_date_tod() {
    let src = "
	FUNCTION main : DT
		main := CONCAT_DATE_TOD(D#2010-03-12, TOD#12:30:15.123);
	END_FUNCTION";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = SingleType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 1268397015122);
}

#[test]
fn concat_date_ltod() {
    let src = "
	FUNCTION main : DT
		main := CONCAT_DATE_LTOD(D#2010-03-12, LTOD#12:30:15.123);
	END_FUNCTION";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = SingleType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 1268397015122);
}

#[derive(Default)]
struct MainType<T> {
    a: T,
    b: T,
    c: T,
    d: T,
}

#[test]
fn concat_date_signed_ints() {
    let src = "
	PROGRAM main
	VAR
		a : DATE;
		b : DATE;
		c : DATE;
	END_VAR
		a := CONCAT_DATE(INT#2000,SINT#01,SINT#01);
		b := CONCAT_DATE(DINT#2000,DINT#01,DINT#01);
		c := CONCAT_DATE(LINT#2000,LINT#01,LINT#01);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 946684800000);
    assert_eq!(maintype.b, 946684800000);
    assert_eq!(maintype.c, 946684800000);
}

#[test]
fn concat_date_unsigned_ints() {
    let src = "
	PROGRAM main
	VAR
		a : DATE;
		b : DATE;
		c : DATE;
	END_VAR
		a := CONCAT_DATE(UINT#2000,USINT#01,USINT#01);
		b := CONCAT_DATE(UDINT#2000,UDINT#01,UDINT#01);
		c := CONCAT_DATE(ULINT#2000,ULINT#01,ULINT#01);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 946684800000);
    assert_eq!(maintype.b, 946684800000);
    assert_eq!(maintype.c, 946684800000);
}

#[test]
fn concat_tod_signed_ints() {
    let src = "
	PROGRAM main
	VAR
		a : TOD;
		b : TOD;
		c : TOD;
		d : TOD;
	END_VAR
		a := CONCAT_TOD(SINT#20,SINT#15,SINT#12,SINT#34);
		b := CONCAT_TOD(INT#20,INT#15,INT#12,INT#341);
		c := CONCAT_TOD(DINT#20,DINT#15,DINT#12,DINT#341);
		d := CONCAT_TOD(LINT#20,LINT#15,LINT#12,LINT#341);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 72912034);
    assert_eq!(maintype.b, 72912341);
    assert_eq!(maintype.c, 72912341);
    assert_eq!(maintype.d, 72912341);
}

#[test]
fn concat_tod_unsigned_ints() {
    let src = "
	PROGRAM main
	VAR
		a : TOD;
		b : TOD;
		c : TOD;
		d : TOD;
	END_VAR
		a := CONCAT_TOD(USINT#20,USINT#15,USINT#12,USINT#34);
		b := CONCAT_TOD(UINT#20,UINT#15,UINT#12,UINT#341);
		c := CONCAT_TOD(UDINT#20,UDINT#15,UDINT#12,UDINT#341);
		d := CONCAT_TOD(ULINT#20,ULINT#15,ULINT#12,ULINT#341);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 72912034);
    assert_eq!(maintype.b, 72912341);
    assert_eq!(maintype.c, 72912341);
    assert_eq!(maintype.d, 72912341);
}

#[test]
fn concat_ltod_signed_ints() {
    let src = "
	PROGRAM main
	VAR
		a : TOD;
		b : TOD;
		c : TOD;
		d : TOD;
	END_VAR
		a := CONCAT_LTOD(SINT#20,SINT#15,SINT#12,SINT#34);
		b := CONCAT_LTOD(INT#20,INT#15,INT#12,INT#341);
		c := CONCAT_LTOD(DINT#20,DINT#15,DINT#12,DINT#341);
		d := CONCAT_LTOD(LINT#20,LINT#15,LINT#12,LINT#341);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 72912034);
    assert_eq!(maintype.b, 72912341);
    assert_eq!(maintype.c, 72912341);
    assert_eq!(maintype.d, 72912341);
}

#[test]
fn concat_ltod_unsigned_ints() {
    let src = "
	PROGRAM main
	VAR
		a : TOD;
		b : TOD;
		c : TOD;
		d : TOD;
	END_VAR
		a := CONCAT_LTOD(USINT#20,USINT#15,USINT#12,USINT#34);
		b := CONCAT_LTOD(UINT#20,UINT#15,UINT#12,UINT#341);
		c := CONCAT_LTOD(UDINT#20,UDINT#15,UDINT#12,UDINT#341);
		d := CONCAT_LTOD(ULINT#20,ULINT#15,ULINT#12,ULINT#341);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 72912034);
    assert_eq!(maintype.b, 72912341);
    assert_eq!(maintype.c, 72912341);
    assert_eq!(maintype.d, 72912341);
}

#[test]
fn concat_dt_signed_ints() {
    let src = "
	PROGRAM main
	VAR
		a : DT;
		b : DT;
		c : DT;
	END_VAR
		a := CONCAT_DT(INT#2000,SINT#1,SINT#2,SINT#20,SINT#15,SINT#12,SINT#111);
		b := CONCAT_DT(DINT#2000,DINT#1,DINT#2,DINT#20,DINT#15,DINT#12,DINT#111);
		c := CONCAT_DT(LINT#2000,LINT#1,LINT#2,LINT#20,LINT#15,LINT#12,LINT#111);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 946844112111);
    assert_eq!(maintype.b, 946844112111);
    assert_eq!(maintype.c, 946844112111);
}

#[test]
fn concat_dt_unsigned_ints() {
    let src = "
	PROGRAM main
	VAR
		a : DT;
		b : DT;
		c : DT;
	END_VAR
		a := CONCAT_DT(UINT#2000,USINT#1,USINT#2,USINT#20,USINT#15,USINT#12,USINT#111);
		b := CONCAT_DT(UDINT#2000,UDINT#1,UDINT#2,UDINT#20,UDINT#15,UDINT#12,UDINT#111);
		c := CONCAT_DT(ULINT#2000,ULINT#1,ULINT#2,ULINT#20,ULINT#15,ULINT#12,ULINT#111);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 946844112111);
    assert_eq!(maintype.b, 946844112111);
    assert_eq!(maintype.c, 946844112111);
}

#[test]
fn concat_ldt_signed_ints() {
    let src = "
	PROGRAM main
	VAR
		a : LDT;
		b : LDT;
		c : LDT;
	END_VAR
		a := CONCAT_LDT(INT#2000,SINT#1,SINT#2,SINT#20,SINT#15,SINT#12,SINT#111);
		b := CONCAT_LDT(DINT#2000,DINT#1,DINT#2,DINT#20,DINT#15,DINT#12,DINT#111);
		c := CONCAT_LDT(LINT#2000,LINT#1,LINT#2,LINT#20,LINT#15,LINT#12,LINT#111);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 946844112111);
    assert_eq!(maintype.b, 946844112111);
    assert_eq!(maintype.c, 946844112111);
}

#[test]
fn concat_ldt_unsigned_ints() {
    let src = "
	PROGRAM main
	VAR
		a : LDT;
		b : LDT;
		c : LDT;
	END_VAR
		a := CONCAT_LDT(UINT#2000,USINT#1,USINT#2,USINT#20,USINT#15,USINT#12,USINT#111);
		b := CONCAT_LDT(UDINT#2000,UDINT#1,UDINT#2,UDINT#20,UDINT#15,UDINT#12,UDINT#111);
		c := CONCAT_LDT(ULINT#2000,ULINT#1,ULINT#2,ULINT#20,ULINT#15,ULINT#12,ULINT#111);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 946844112111);
    assert_eq!(maintype.b, 946844112111);
    assert_eq!(maintype.c, 946844112111);
}

#[test]
fn split_date_int() {
    let src = "
	PROGRAM main
	VAR
		a : INT; // year
		b : INT; // month
		c : INT; // day
	END_VAR
		SPLIT_DATE(DATE#2000-01-02, a, b, c);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i16>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 2000); // year
    assert_eq!(maintype.b, 1); // month
    assert_eq!(maintype.c, 2); // day
}

#[test]
fn split_date_uint() {
    let src = "
	PROGRAM main
	VAR
		a : UINT; // year
		b : UINT; // month
		c : UINT; // day
	END_VAR
		SPLIT_DATE(DATE#2000-01-02, a, b, c);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<u16>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 2000); // year
    assert_eq!(maintype.b, 1); // month
    assert_eq!(maintype.c, 2); // day
}

#[test]
fn split_date_dint() {
    let src = "
	PROGRAM main
	VAR
		a : DINT; // year
		b : DINT; // month
		c : DINT; // day
	END_VAR
		SPLIT_DATE(DATE#2000-01-02, a, b, c);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i32>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 2000); // year
    assert_eq!(maintype.b, 1); // month
    assert_eq!(maintype.c, 2); // day
}

#[test]
fn split_date_udint() {
    let src = "
	PROGRAM main
	VAR
		a : UDINT; // year
		b : UDINT; // month
		c : UDINT; // day
	END_VAR
		SPLIT_DATE(DATE#2000-01-02, a, b, c);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<u32>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 2000); // year
    assert_eq!(maintype.b, 1); // month
    assert_eq!(maintype.c, 2); // day
}

#[test]
fn split_date_lint() {
    let src = "
	PROGRAM main
	VAR
		a : LINT; // year
		b : LINT; // month
		c : LINT; // day
	END_VAR
		SPLIT_DATE(DATE#2000-01-02, a, b, c);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 2000); // year
    assert_eq!(maintype.b, 1); // month
    assert_eq!(maintype.c, 2); // day
}

#[test]
fn split_date_ulint() {
    let src = "
	PROGRAM main
	VAR
		a : ULINT; // year
		b : ULINT; // month
		c : ULINT; // day
	END_VAR
		SPLIT_DATE(DATE#2000-01-02, a, b, c);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<u64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 2000); // year
    assert_eq!(maintype.b, 1); // month
    assert_eq!(maintype.c, 2); // day
}

#[test]
fn split_tod_int() {
    let src = "
	PROGRAM main
	VAR
		a : INT; // hour
		b : INT; // minute
		c : INT; // second
		d : INT; // millisecond
	END_VAR
		SPLIT_TOD(TOD#14:12:03.123, a, b, c, d);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i16>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 14); // hour
    assert_eq!(maintype.b, 12); // minute
    assert_eq!(maintype.c, 3); // second
    assert_eq!(maintype.d, 123); // millisecond
}

#[test]
fn split_tod_uint() {
    let src = "
	PROGRAM main
	VAR
		a : UINT; // hour
		b : UINT; // minute
		c : UINT; // second
		d : UINT; // millisecond
	END_VAR
		SPLIT_TOD(TOD#14:12:03.123, a, b, c, d);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<u16>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 14); // hour
    assert_eq!(maintype.b, 12); // minute
    assert_eq!(maintype.c, 3); // second
    assert_eq!(maintype.d, 123); // millisecond
}

#[test]
fn split_tod_dint() {
    let src = "
	PROGRAM main
	VAR
		a : DINT; // hour
		b : DINT; // minute
		c : DINT; // second
		d : DINT; // millisecond
	END_VAR
		SPLIT_TOD(TOD#14:12:03.123, a, b, c, d);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i32>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 14); // hour
    assert_eq!(maintype.b, 12); // minute
    assert_eq!(maintype.c, 3); // second
    assert_eq!(maintype.d, 123); // millisecond
}

#[test]
fn split_tod_udint() {
    let src = "
	PROGRAM main
	VAR
		a : UDINT; // hour
		b : UDINT; // minute
		c : UDINT; // second
		d : UDINT; // millisecond
	END_VAR
		SPLIT_TOD(TOD#14:12:03.123, a, b, c, d);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<u32>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 14); // hour
    assert_eq!(maintype.b, 12); // minute
    assert_eq!(maintype.c, 3); // second
    assert_eq!(maintype.d, 123); // millisecond
}

#[test]
fn split_tod_lint() {
    let src = "
	PROGRAM main
	VAR
		a : LINT; // hour
		b : LINT; // minute
		c : LINT; // second
		d : LINT; // millisecond
	END_VAR
		SPLIT_TOD(TOD#14:12:03.123, a, b, c, d);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<i64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 14); // hour
    assert_eq!(maintype.b, 12); // minute
    assert_eq!(maintype.c, 3); // second
    assert_eq!(maintype.d, 123); // millisecond
}

#[test]
fn split_tod_ulint() {
    let src = "
	PROGRAM main
	VAR
		a : ULINT; // hour
		b : ULINT; // minute
		c : ULINT; // second
		d : ULINT; // millisecond
	END_VAR
		SPLIT_TOD(TOD#14:12:03.123, a, b, c, d);
	END_PROGRAM";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::<u64>::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.a, 14); // hour
    assert_eq!(maintype.b, 12); // minute
    assert_eq!(maintype.c, 3); // second
    assert_eq!(maintype.d, 123); // millisecond
}
