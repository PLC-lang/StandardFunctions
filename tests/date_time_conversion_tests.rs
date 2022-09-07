use chrono::TimeZone;
use common::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[allow(dead_code)]
#[repr(C)]
struct MainType {
    a: [usize; 1000],
}

impl Default for MainType {
    fn default() -> Self {
        MainType { a: [0; 1000] }
    }
}

#[test]
fn ltime_to_time_conversion() {
    let src = "
	FUNCTION main : TIME
		main := LTIME_TO_TIME(LTIME#10s);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 10000000000);
}

#[test]
fn time_to_ltime_conversion() {
    let src = "
	FUNCTION main : LTIME
		main := TIME_TO_LTIME(TIME#10s);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 10000000000);
}

#[test]
fn ldt_to_dt_conversion() {
    let src = "
	FUNCTION main : DT
		main := LDT_TO_DT(LDT#2021-04-20-22:33:14);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        res,
        chrono::Utc
            .ymd(2021, 4, 20)
            .and_hms(22, 33, 14)
            .timestamp_nanos()
    );
}

#[test]
fn ldt_to_date_conversion() {
    let src = "
	FUNCTION main : DATE
		main := LDT_TO_DATE(LDT#2000-01-01-20:15:11);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        res,
        chrono::Utc
            .ymd(2000, 1, 1)
            .and_hms(0, 0, 0)
            .timestamp_nanos()
    );
}

#[test]
fn ldt_to_ltod_conversion() {
    let src = "
	FUNCTION main : LTOD
		main := LDT_TO_LTOD(LDT#2000-01-01-15:36:30.123);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        res,
        chrono::Utc
            .ymd(1970, 1, 1)
            .and_hms_milli(15, 36, 30, 123)
            .timestamp_nanos()
    );
}

#[test]
fn ldt_to_tod_conversion() {
    let src = "
	FUNCTION main : TOD
		main := LDT_TO_TOD(LDT#2120-02-12-20:15:11.543);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        res,
        chrono::Utc
            .ymd(1970, 1, 1)
            .and_hms_milli(20, 15, 11, 543)
            .timestamp_nanos()
    );
}

#[test]
fn dt_to_ldt_conversion() {
    let src = "
	FUNCTION main : LDT
		main := DT_TO_LDT(DT#2021-04-20-22:33:14);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        res,
        chrono::Utc
            .ymd(2021, 4, 20)
            .and_hms(22, 33, 14)
            .timestamp_nanos()
    );
}

#[test]
fn dt_to_date_conversion() {
    let src = "
	FUNCTION main : DATE
		main := DT_TO_DATE(DT#2000-01-01-20:15:11);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        res,
        chrono::Utc
            .ymd(2000, 1, 1)
            .and_hms(0, 0, 0)
            .timestamp_nanos()
    );
}

#[test]
fn dt_to_ltod_conversion() {
    let src = "
	FUNCTION main : LTOD
		main := DT_TO_LTOD(DT#2000-01-01-15:36:30.123);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        res,
        chrono::Utc
            .ymd(1970, 1, 1)
            .and_hms_milli(15, 36, 30, 123)
            .timestamp_nanos()
    );
}

#[test]
fn dt_to_tod_conversion() {
    let src = "
	FUNCTION main : TOD
		main := DT_TO_TOD(DT#2120-02-12-20:15:11.543);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        res,
        chrono::Utc
            .ymd(1970, 1, 1)
            .and_hms_milli(20, 15, 11, 543)
            .timestamp_nanos()
    );
}

#[test]
fn ltod_to_tod_conversion() {
    let src = "
	FUNCTION main : TOD
		main := LTOD_TO_TOD(LTOD#10:20:30);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        res,
        chrono::Utc
            .ymd(1970, 1, 1)
            .and_hms(10, 20, 30)
            .timestamp_nanos()
    );
}

#[test]
fn tod_to_ltod_conversion() {
    let src = "
	FUNCTION main : LTOD
		main := TOD_TO_LTOD(TOD#10:20:30);
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        res,
        chrono::Utc
            .ymd(1970, 1, 1)
            .and_hms(10, 20, 30)
            .timestamp_nanos()
    );
}
