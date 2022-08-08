// Import common functionality into the integration tests
mod common;
use crate::common::compile_and_run_no_params;
use common::add_std;

#[test]
fn test_to_big_endian_int() {
    let src = r#"FUNCTION main : INT
        main := TO_BIG_ENDIAN(INT#16#1001);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0110)
}
#[test]
fn test_to_big_endian_dint() {
    let src = r#"FUNCTION main : DINT
        main := TO_BIG_ENDIAN(DINT#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A0110)
}

#[test]
fn test_to_big_endian_lint() {
    let src = r#"FUNCTION main : LINT
        main := TO_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A01100B0A0110)
}

#[test]
fn test_to_big_endian_uint() {
    let src = r#"FUNCTION main : UINT
        main := TO_BIG_ENDIAN(UINT#16#ABBA);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0xBAAB)
}

#[test]
fn test_to_big_endian_f32() {
    let src = r#"FUNCTION main : REAL
        main := TO_BIG_ENDIAN(REAL#12.5);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: f32 = compile_and_run_no_params(src);
    assert_eq!(res, f32::from_be_bytes(12.5f32.to_be_bytes()))
}

#[test]
fn test_to_big_endian_wchar() {
    let src = r#"FUNCTION main : WCHAR
    main := TO_BIG_ENDIAN(WCHAR#'C');
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x4300)
}

#[test]
fn test_to_big_endian_date() {
    let src = r#"FUNCTION main : DATE
    main := TO_BIG_ENDIAN(DATE#1984-06-25);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, 0)
}