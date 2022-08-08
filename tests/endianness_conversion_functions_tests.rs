// Import common functionality into the integration tests
mod common;
use crate::common::compile_and_run_no_params;
use common::add_std;

// what is the wanted behaviour for signed endianess conversion? 
// this test will panic for signed values like ABBA because of overflow
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