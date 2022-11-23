use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Timelike};
mod common;
use common::{compile_and_run, compile_and_run_no_params};
use common::add_std;

const STR_SIZE: usize = 80;
struct MainType<T> {
    s: [T; STR_SIZE],
}

// x to string/wstring
#[test]
fn byte_to_string_conversion() {
    // let mut maintype = MainType {
    //     s: [0_u8; STR_SIZE],
    // };
    struct MainType{
        s: [u8; 80],
    }
    let mut maintype = MainType { s: [0_u8; STR_SIZE]};
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: BYTE := 2#01010101;
    END_VAR
        main := BYTE_TO_STRING(in);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );

    let expected = "01010101";
    let _: i32 = compile_and_run(sources, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn byte_to_wstring_conversion() {
    let mut maintype = MainType {
        s: [0_u16; STR_SIZE],
    };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: BYTE := 2#01010101;
    END_VAR
        main := BYTE_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );

    let expected = "01010101".to_string();
    let _: i32 = compile_and_run(sources, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn lword_to_string_conversion() {
    let mut maintype = MainType {
        s: [0_u8; STR_SIZE],
    };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: LWORD := 16#DEADBEEFDECAFBAD;
    END_VAR
        main := LWORD_TO_STRING(in);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );

    let expected = format!("{:064b}", 0xDEAD_BEEF_DECAF_BAD_u64);
    let _: i32 = compile_and_run(sources, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res.to_string());
}

#[test]
fn lword_to_wstring_conversion() {
    let mut maintype = MainType {
        s: [0_u16; STR_SIZE],
    };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: LWORD := 16#DEADBEEFDECAFBAD;
    END_VAR
        main := LWORD_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );

    let expected = format!("{:064b}", 0xDEAD_BEEF_DECAF_BAD_u64);
    let _: i32 = compile_and_run(sources, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn lint_to_string_conversion() {
    let mut maintype = MainType {
        s: [0_u8; STR_SIZE],
    };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: LINT := 7_600_500_400_300_200_100;
    END_VAR
        main := LINT_TO_STRING(in);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );

    let expected = format!("{}", 7_600_500_400_300_200_100_i64);
    let _: i32 = compile_and_run(sources, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn lint_to_wstring_conversion() {
    let mut maintype = MainType {
        s: [0_u16; STR_SIZE],
    };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: LINT := 7_600_500_400_300_200_100;
    END_VAR
        main := LINT_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );

    let expected = format!("{}", 7_600_500_400_300_200_100_i64);
    let _: i32 = compile_and_run(sources, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn lreal_to_string_conversion() {
    let mut maintype = MainType {
        s: [0_u8; STR_SIZE],
    };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: LREAL := 13234.25;
    END_VAR
        LREAL_TO_STRING_EXT(in, main);
        // main := LREAL_TO_STRING(in);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );

    let expected = format!("{}", 13234.25_f32);
    let _: i32 = compile_and_run(sources, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn lreal_to_wstring_conversion() {
    let mut maintype = MainType {
        s: [0_u16; STR_SIZE],
    };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: LREAL := 13234.25;
    END_VAR
        main := LREAL_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );

    let expected = format!("{}", 13234.25_f32);
    let _: i32 = compile_and_run(sources, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

// x to lword
#[test]
fn ltod_to_lword_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: LTOD := LTOD#01:59:59.2567;
    END_VAR
        main := LTOD_TO_LWORD(t);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );
    let res: u64 = compile_and_run_no_params(sources);

    let time = NaiveTime::from_hms_nano_opt(1, 59, 59, 2567e5 as u32).unwrap();
    let expected = time.num_seconds_from_midnight() as u64 * 1e9 as u64 + time.nanosecond() as u64;
    assert_eq!(expected, res)
}

#[test]
fn ldate_to_lword_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: LDATE := LDATE#1999-12-31;
    END_VAR
        main := LDATE_TO_LWORD(t);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );
    let res: u64 = compile_and_run_no_params(sources);

    let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let expected = NaiveDateTime::new(date, time).timestamp_nanos() as u64;
    assert_eq!(expected, res)
}

#[test]
fn ltime_to_lword_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: LTIME := LTIME#12h1m20s391ms10ns;
    END_VAR
        main := LTOD_TO_LWORD(t);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );
    let res: u64 = compile_and_run_no_params(sources);

    let time = NaiveTime::from_hms_nano_opt(12, 1, 20, 391e6 as u32 + 10).unwrap();
    let expected = time.num_seconds_from_midnight() as u64 * 1e9 as u64 + time.nanosecond() as u64;
    assert_eq!(expected, res)
}

// x to lint
#[test]
fn string_to_lint_conversion() {
    todo!();
}

#[test]
fn wstring_to_lint_conversion() {
    todo!();
}

#[test]
fn ltime_to_lint_conversion() {
    let src = r#"
    FUNCTION main : LINT
    VAR
        t: LTIME := LTIME#12h1m20s391ms10ns;
    END_VAR
        main := LTOD_TO_LINT(t);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );

    let res: i64 = compile_and_run_no_params(sources);

    let time = NaiveTime::from_hms_nano_opt(12, 1, 20, 391e6 as u32 + 10).unwrap();
    let expected = time.num_seconds_from_midnight() as i64 * 1e9 as i64 + time.nanosecond() as i64;
    assert_eq!(expected, res)
}

#[test]
fn ltod_to_lint_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: LTOD := LTOD#01:59:59.2567;
    END_VAR
        main := LTOD_TO_LINT(t);
    END_FUNCTION
    "#;

    let sources = add_std!(
        src,
        "extra_functions.st",
        "string_conversion.st",
        "string_functions.st"
    );
    let res: i64 = compile_and_run_no_params(sources);

    let time = NaiveTime::from_hms_nano_opt(1, 59, 59, 2567e5 as u32).unwrap();
    let expected = time.num_seconds_from_midnight() as i64 * 1e9 as i64 + time.nanosecond() as i64;
    assert_eq!(expected, res)
}
