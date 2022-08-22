// Import common functionality into the integration tests
mod common;
use common::add_std;
use crate::common::compile_and_run_no_params;

#[test]
fn len_string() {
    let src = r#"

	FUNCTION main : DINT
    VAR_TEMP
        x : STRING(6);
        y : DINT;
    END_VAR
        x := 'hello';
		main := LEN(x);
    END_FUNCTION
        "#;
    let sources = add_std!(src, "string_functions.st");
    let res : i32 = compile_and_run_no_params(sources); 
    assert_eq!(5, res);
}

// helper function to convert null-terminated utf8 byte arr to str
fn str_from_u8_utf8(src: &[u8]) -> Result<&str, std::str::Utf8Error> {
    let null_pos = src
        .iter()
        .position(|&c| c == 0)
        .unwrap_or_else(|| panic!("No null-terminating character found!"));
    ::std::str::from_utf8(&src[0..null_pos])
}

#[test]
fn left_string() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        x : STRING;
    END_VAR
        x := 'hello';
		main := LEFT(x, DINT#3);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st"); 
    let res: [u8; 81] = compile_and_run_no_params(sources);   
    if let Ok(x) = str_from_u8_utf8(&res) {
        assert_eq!(
            x,
            "hel"
        );
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn left_string_lint() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        x : STRING;
    END_VAR
        x := 'lets see if long int is handled correctly';
		main := LEFT(x, LINT#31);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st"); 
    let res: [u8; 81] = compile_and_run_no_params(sources);   
    if let Ok(x) = str_from_u8_utf8(&res) {
        assert_eq!(
            x,
            "lets see if long int is handled"
        );
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn right_string_usint() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        x : STRING;
    END_VAR
        x := 'sample text';
		main := RIGHT(x, USINT#7);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st"); 
    let res: [u8; 81] = compile_and_run_no_params(sources);   
    if let Ok(x) = str_from_u8_utf8(&res) {
        assert_eq!(
            x,
            "le text"
        );
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}
