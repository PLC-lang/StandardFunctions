// Import common functionality into the integration tests
mod common;
use std::string::FromUtf16Error;

use crate::common::compile_and_run_no_params;
use common::add_std;

// helper function to convert null-terminated utf8 byte array to string slice
fn str_from_u8_utf8(src: &[u8]) -> Result<&str, std::str::Utf8Error> {
    let null_pos = src
        .iter()
        .position(|&c| c == 0)
        .unwrap_or_else(|| panic!("No null-terminating character found!"));
    std::str::from_utf8(&src[0..null_pos])
}

// helper function to convert null-terminated utf16 byte array to string slice
fn string_from_utf16(src: &[u16]) -> Result<String, FromUtf16Error> {
    let null_pos = src
        .iter()
        .position(|&c| c == 0)
        .unwrap_or_else(|| panic!("No null-terminating character found!"));
    String::from_utf16(&src[0..null_pos])
}

// utf8 tests
#[test]
fn len_string() {
    let src = r#"
	FUNCTION main : DINT
    VAR
        variable: STRING;
    END_VAR
        variable := '     this is   a  very   long          sentence   with plenty  of    characters.';
		main := LEN(variable);
    END_FUNCTION
        "#;
    let sources = add_std!(src, "string_functions.st");
    let res: i32 = compile_and_run_no_params(sources);
    assert_eq!(80, res);
}

#[test]
fn len_string_long_string() {
    let src = r#"
	FUNCTION main : DINT
		main := LEN('     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.');
    END_FUNCTION
        "#;
    let sources = add_std!(src, "string_functions.st");
    let res: i32 = compile_and_run_no_params(sources);
    assert_eq!(100, res);
}

#[test]
fn len_string_no_variable() {
    let src = r#"
	FUNCTION main : DINT
        main := LEN(STRING#'hello'); 
    END_FUNCTION
        "#;
    let sources = add_std!(src, "string_functions.st");
    let res: i32 = compile_and_run_no_params(sources);
    assert_eq!(5, res);
}

#[test]
fn len_string_empty() {
    let src = r#"
	FUNCTION main : DINT
    VAR_TEMP
        in : STRING[1024];
    END_VAR
        in := '';
		main := LEN(in);
    END_FUNCTION
        "#;
    let sources = add_std!(src, "string_functions.st");
    let res: i32 = compile_and_run_no_params(sources);
    assert_eq!(0, res);
}

#[test]
fn left_string() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in : STRING;
    END_VAR
        in := 'hello';
		main := LEFT(in, DINT#3);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "hel");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn left_string_long_string() {
    let src = r#"
	FUNCTION main : STRING[100]
    VAR_TEMP
        in : STRING[100];
    END_VAR
        in := '     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.';
		main := LEFT(in, DINT#85);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 101] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(
            res,
            "     this is   a  very   long           sentence   with plenty  of    characters and "
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
        in : STRING;
        l : LINT;
    END_VAR
        in := 'lets see if long int is handled correctly';
        l := 31;
		main := LEFT(in, l);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "lets see if long int is handled");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn left_ext_string() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in : STRING;
        out : STRING;
        l : UDINT;
    END_VAR
        in := 'extended';
        l := 6;
        LEFT_EXT(in, l, out);
		main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "extend");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn right_string_usint() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in : STRING;
    END_VAR
        in := 'sample text';
		main := RIGHT(in, USINT#7);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "le text");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
#[should_panic]
fn right_string_substring_too_long() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in : STRING;
    END_VAR
        in := 'sample text';
		main := RIGHT(in, 12);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let _: [u8; 81] = compile_and_run_no_params(sources);
}

#[test]
fn right_ext_string() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in : STRING;
        out : STRING;
    END_VAR
        in := 'extended';
        RIGHT_EXT(in, 3, out);
		main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "ded");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn right_string_long_string() {
    let src = r#"
	FUNCTION main : STRING[100]
    VAR_TEMP
        in : STRING[100];
        l : DINT;
    END_VAR
        in := '     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.';
        l := 85;
        main := RIGHT(in, l);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 101] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(
            res,
            "a  very   long           sentence   with plenty  of    characters and weird  spacing."
        );
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn right_ext_string_long_string() {
    let src = r#"
	FUNCTION main : STRING[128]
    VAR_TEMP
        in : STRING[128];
        out : STRING[128];
        l : DINT;
    END_VAR 
        in := '7gAN5pmmSXqHJ3zZCXnBwika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQcBFgZhMXGhpBW';
        l := 99;
        RIGHT_EXT(in, l, out);
		main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(
            res,
            "ika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQcBFgZhMXGhpBW"
        );
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn mid_string() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in : STRING;
        l : DINT;
        p : DINT;
    END_VAR
        in := 'sample text';
        l := 7;
        p := 2;
		main := MID(in, l, p);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "ample t");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn mid_ext_string() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in : STRING;
        out : STRING;
        l : DINT;
        p : DINT;
    END_VAR
        in := 'sample text';
        l := 7;
        p := 2;
        MID_EXT(in, l, p, out);
		main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "ample t");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn mid_string_long_string() {
    let src = r#"
	FUNCTION main : STRING[128]
    VAR_TEMP
        in : STRING[128];
        l : DINT;
        p : DINT;
    END_VAR
        in := '7gAN5pmmSXqHJ3zZCXnBwika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQcBFgZhMXGhpBW';
        l := 99;
        p := 10;
		main := MID(in, l, p);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(
            res,
            "XqHJ3zZCXnBwika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQc"
        );
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn mid_ext_string_long_string() {
    let src = r#"
	FUNCTION main : STRING[128]
    VAR_TEMP
        in : STRING[128];
        out : STRING[128];
        l : DINT;
        p : DINT;
    END_VAR 
        in := '7gAN5pmmSXqHJ3zZCXnBwika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQcBFgZhMXGhpBW';
        l := 99;
        p := 10;
        MID_EXT(in, l, p, out);
		main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(
            res,
            "XqHJ3zZCXnBwika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQc"
        );
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn insert_string() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in1 : STRING;
        in2 : STRING;
        p : SINT;
    END_VAR
        in1 := 'stuck with you';
        in2 := 'in the middle ';
        p := 6;
		main := INSERT(in1, in2, p);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "stuck in the middle with you");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn insert_ext_string_at_start_and_end() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in1 : STRING;
        in2 : STRING;
        out : STRING;
    END_VAR
        in1 := '2';
        in2 := '1';
		INSERT_EXT(in1, in2, 0, out);
        in1 := out;
        in2 := '3';
		INSERT_EXT(in1, in2, 2, out);
        main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "123");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn delete_string_with_escape_sequence() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in : STRING;
        l : UINT;
        p : ULINT;
    END_VAR
        in := 'the$$e are escape sequences $'𝄞$'';
        l := 21;
        p := 6;
		main := DELETE(in, l, p);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 20] = compile_and_run_no_params(sources);
    let res = std::str::from_utf8(&res)
        .unwrap()
        .trim_end_matches('\0')
        .as_bytes();
    assert_eq!(
        format!("{:?}", "the$e '𝄞'".as_bytes()),
        format!("{:?}", res)
    );
}

#[test]
fn delete_ext_string() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in : STRING;
        out : STRING;
        l : LINT;
        p : USINT;
    END_VAR
        in := '𝄞typoasdf';
        l := 4;
        p := 6;
		DELETE_EXT(in, l, p, out);
        main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "𝄞typo");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn delete_ext_string_with_escape_sequence() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in : STRING;
        out : STRING;
        l : LINT;
        p : USINT;
    END_VAR
        in := 'the$$e are escape sequences $'𝄞$'';
        l := 21;
        p := 6;
		DELETE_EXT(in, l, p, out);
        main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 20] = compile_and_run_no_params(sources);
    let res = std::str::from_utf8(&res)
        .unwrap()
        .trim_end_matches('\0')
        .as_bytes();
    assert_eq!(
        format!("{:?}", "the$e '𝄞'".as_bytes()),
        format!("{:?}", res)
    );
}

#[test]
fn replace_string() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in1 : STRING;
        in2 : STRING;
        l : LINT;
        p : USINT;
    END_VAR
        in1 := 'replace me';
        in2 := 'gret𝄞';
        l := 8;
        p := 3;
		main := REPLACE(in1, in2, l, p);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "regret𝄞");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn replace_ext_string() {
    let src = r#"
	FUNCTION main : STRING
    VAR_TEMP
        in1 : STRING;
        in2 : STRING;
        out : STRING;
        l : LINT;
        p : USINT;
    END_VAR
        in1 := 'replace me';
        in2 := 'st𝄞red';
        l := 8;
        p := 3;
		REPLACE_EXT(in1, in2, l, p, out);
        main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u8; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = str_from_u8_utf8(&res) {
        assert_eq!(res, "rest𝄞red");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn find_string() {
    let src = r#"
    FUNCTION main : DINT
    VAR_TEMP
        in1: STRING;
        in2: STRING;
    END_VAR
        in1 := 'Where is Waldo?';
        in2 := 'Waldo';
        main := FIND(in1, in2);        
    END_FUNCTION
    "#;

    let sources = add_std!(src, "string_functions.st");
    let res: usize = compile_and_run_no_params(sources);
    assert_eq!(res, 10);
}

#[test]
#[should_panic]
fn test_double_quotes_on_strings() {
    let src = r#"
    FUNCTION main : DINT
    VAR_TEMP
        in1: STRING;
        in2: STRING;
    END_VAR
        in1 := "Where is Waldo?";
        in2 := "Waldo";
        main := FIND(in1, in2);        
    END_FUNCTION
    "#;

    let sources = add_std!(src, "string_functions.st");
    let _: i32 = compile_and_run_no_params(sources);
}

#[test]
fn test_concat_string() {
    let src = r#"
    FUNCTION main : STRING[2048]
    VAR_TEMP
        a : STRING := 'Hello';
        b : STRING := ', ';
        c : STRING := 'World';
        d : STRING := '!';
    END_VAR    
        main := CONCAT(a, b, c, d);
    END_FUNCTION
    "#;

    let source = add_std!(src, "string_functions.st");    
    let res: [u8; 2048] = compile_and_run_no_params(source); 

    if let Ok(result) = str_from_u8_utf8(&res) {
        assert_eq!(result, "Hello, World!");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

#[test]
fn test_concat_ext_string() {
    let src = r#"
    FUNCTION main : STRING[2048]
    VAR_TEMP
        a : STRING := 'Hello';
        b : STRING := ', ';
        c : STRING := 'World';
        d : STRING := '!';
    END_VAR    
        CONCAT_EXT(main, a, b, c, d);
    END_FUNCTION
    "#;

    let source = add_std!(src, "string_functions.st");
    let res: [u8; 2048] = compile_and_run_no_params(source);
    if let Ok(result) = str_from_u8_utf8(&res) {
        assert_eq!(result, "Hello, World!");
    } else {
        panic!("Given string is not UTF8-encoded")
    }
}

// #[test]
// fn test_concat_long_string_literals() {
//     let src = r#"
//     FUNCTION main : STRING[2048]
//         main := CONCAT('     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.', '$N', 'the                same           is   true                    for             this                     string.');
//     END_FUNCTION
//     "#;

//     let source = add_std!(src, "string_functions.st");    
//     let res: [u8; 245] = compile_and_run_no_params(source); 

//     if let Ok(result) = str_from_u8_utf8(&res) {
//         assert_eq!(
//             result, 
//             r"     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.the                same           is   true                    for             this                     string."
//             );
//     } else {
//         panic!("Given string is not UTF8-encoded")
//     }
// }

// utf16 tests
#[test]
fn len_wstring() {
    let src = r#"
	FUNCTION main : DINT
    VAR_TEMP
        in : WSTRING;
    END_VAR
        in := "Hèßlo😀𝄞";
		main := LEN(in);
    END_FUNCTION
        "#;
    let sources = add_std!(src, "string_functions.st");
    let res: i32 = compile_and_run_no_params(sources);
    assert_eq!(7, res);
}

#[test]
fn len_wstring_no_variable() {
    let src = r#"
	FUNCTION main : DINT
        main := LEN(WSTRING#'Hèßlo😀𝄞'); 
    END_FUNCTION
        "#;
    let sources = add_std!(src, "string_functions.st");
    let res: i32 = compile_and_run_no_params(sources);
    assert_eq!(7, res);
}

#[test]
fn len_wstring_empty() {
    let src = r#"
	FUNCTION main : DINT
    VAR_TEMP
        in : WSTRING[1024];
    END_VAR
        in := "";
		main := LEN(in);
    END_FUNCTION
        "#;
    let sources = add_std!(src, "string_functions.st");
    let res: i32 = compile_and_run_no_params(sources);
    assert_eq!(0, res);
}

#[test]
fn left_wstring() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in : WSTRING;
    END_VAR
        in := "𝄞music";
		main := LEFT(in, DINT#2);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 10] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "𝄞m");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn left_wstring_lint() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in : WSTRING;
        l : LINT;
    END_VAR
        in := "lets see 𝄞f long 𝄞nt is handled correctly";
        l := 31;
		main := LEFT(in, l);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "lets see 𝄞f long 𝄞nt is handled");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn left_ext_wstring() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in : WSTRING;
        out : WSTRING;
        l : UDINT;
    END_VAR
        in := "e𝄞tended";
        l := 6;
        LEFT_EXT(in, l, out);
		main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "e𝄞tend");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn right_wstring_usint() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in : WSTRING;
    END_VAR
        in := "samp𝄞e text";
		main := RIGHT(in, USINT#7);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "𝄞e text");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
#[should_panic]
fn right_wstring_substring_too_long() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in : WSTRING;
    END_VAR
        in := "sa𝄞ple text";
		main := RIGHT(in, 12);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let _: [u16; 81] = compile_and_run_no_params(sources);
}

#[test]
fn right_ext_wstring() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in : WSTRING;
        out : WSTRING;
    END_VAR
        in := "exten𝄞ed𝄞";
        RIGHT_EXT(in, 4, out);
		main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "𝄞ed𝄞");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn right_string_long_wstring() {
    let src = r#"
	FUNCTION main : WSTRING[128]
    VAR_TEMP
        in : WSTRING[128];
        l : DINT;
    END_VAR
        in := "7gAN5pmmSXqHJ3zZCXnBwika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQcBFgZhMXGhp𝄞𝄞";
        l := 99;
        main := RIGHT(in, l);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "ika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQcBFgZhMXGhp𝄞𝄞");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn right_ext_string_long_wstring() {
    let src = r#"
	FUNCTION main : WSTRING[128]
    VAR_TEMP
        in : WSTRING[128];
        out : WSTRING[128];
        l : DINT;
    END_VAR 
        in := "7gAN5pmmSXqHJ3zZCXnBwi𝄞𝄞9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQcBFgZhMXGhpBW";
        l := 99;
        RIGHT_EXT(in, l, out);
		main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "i𝄞𝄞9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQcBFgZhMXGhpBW");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn mid_wstring() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in : WSTRING;
        l : DINT;
        p : DINT;
    END_VAR
        in := "sample 𝄞muϗ😀 text";
        l := 7;
        p := 2;
		main := MID(in, l, p);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "ample 𝄞");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn mid_ext_wstring() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in : WSTRING;
        out : WSTRING;
        l : DINT;
        p : DINT;
    END_VAR
        in := "𝄞muϗ😀 sample text";
        l := 7;
        p := 2;
        MID_EXT(in, l, p, out);
		main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "muϗ😀 sa");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn mid_string_long_wstring() {
    let src = r#"
	FUNCTION main : WSTRING[128]
    VAR_TEMP
        in : WSTRING[128];
        l : DINT;
        p : DINT;
    END_VAR
        in := "𝄞muϗ😀pmmSXqHJ3zZCXnBwika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQcBFgZhMXGhpBW";
        l := 99;
        p := 10;
		main := MID(in, l, p);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "XqHJ3zZCXnBwika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQc");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn mid_ext_string_long_wstring() {
    let src = r#"
	FUNCTION main : WSTRING[128]
    VAR_TEMP
        in : WSTRING[128];
        out : WSTRING[128];
        l : DINT;
        p : DINT;
    END_VAR 
        in := "𝄞muϗ😀pmmSXqHJ3zZCXnBwika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQcBFgZhMXGhpBW";
        l := 99;
        p := 10;
        MID_EXT(in, l, p, out);
		main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 128] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "XqHJ3zZCXnBwika9N8RPXpTAdX4LdwHbLjwv9g3mU3dtpCT2MHVPxwtMw6jMQkip3HDy8Ruw42pVi56fiVhYn8faPLUKRghytQc");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn insert_wstring() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in1 : WSTRING;
        in2 : WSTRING;
        p : SINT;
    END_VAR
        in1 := "stuck with you";
        in2 := "in the middle ";
        p := 6;
		main := INSERT(in1, in2, p);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "stuck in the middle with you");
    } else {
        panic!(
            "Given string is not 
        -encoded"
        )
    }
}

#[test]
fn insert_ext_wstring_at_start_and_end() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in1 : WSTRING;
        in2 : WSTRING;
        out : WSTRING;
    END_VAR
        in1 := "2";
        in2 := "1";
		INSERT_EXT(in1, in2, 0, out);
        in1 := out;
        in2 := "3";
		INSERT_EXT(in1, in2, 2, out);
        main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "123");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn delete_wstring() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in : WSTRING;
        l : UINT;
        p : ULINT;
    END_VAR
        in := "this will be deleted";
        l := 13;
        p := 1;
		main := DELETE(in, l, p);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "deleted");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn delete_ext_wstring() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in : WSTRING;
        out : WSTRING;
        l : LINT;
        p : USINT;
    END_VAR
        in := "typoasdf";
        l := 4;
        p := 5;
		DELETE_EXT(in, l, p, out);
        main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "typo");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
    assert_eq!(
        String::from_utf16(&res).unwrap().trim_end_matches('\0'),
        "typo".to_owned()
    );
}

#[test]
fn replace_wstring() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in1 : WSTRING;
        in2 : WSTRING;
        l : LINT;
        p : USINT;
    END_VAR
        in1 := "replace me";
        in2 := "gret";
        l := 8;
        p := 3;
		main := REPLACE(in1, in2, l, p);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "regret");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn replace_ext_wstring() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in1 : WSTRING;
        in2 : WSTRING;
        out : WSTRING;
        l : LINT;
        p : USINT;
    END_VAR
        in1 := "replace me";
        in2 := "stored";
        l := 8;
        p := 3;
		REPLACE_EXT(in1, in2, l, p, out);
        main := out;
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "restored");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn find_wstring() {
    let src = r#"
    FUNCTION main : DINT
    VAR_TEMP
        in1: WSTRING;
        in2: WSTRING;
    END_VAR
        in1 := "Where is Waldo?";
        in2 := "Waldo";
        main := FIND(in1, in2);        
    END_FUNCTION
    "#;

    let sources = add_std!(src, "string_functions.st");
    let res: i32 = compile_and_run_no_params(sources);
    assert_eq!(res, 10);
}

#[test]
fn delete_wstring_with_escape_sequence() {
    let src = r#"
	FUNCTION main : WSTRING
    VAR_TEMP
        in : WSTRING;
        l : UINT;
        p : ULINT;
    END_VAR
        in := "the$$e are escape sequences $"𝄞$"";
        l := 21;
        p := 6;
		main := DELETE(in, l, p);
    END_FUNCTION
        "#;

    let sources = add_std!(src, "string_functions.st");
    let res: [u16; 81] = compile_and_run_no_params(sources);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "the$e \"𝄞\"");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn test_concat_wstring() {
    let src = r#"
    FUNCTION main : WSTRING[2048]
    VAR_TEMP
        a : WSTRING := "Hello";
        b : WSTRING := ", ";
        c : WSTRING := "World";
        d : WSTRING := "!";
    END_VAR    
        main := CONCAT(a, b, c, d);
    END_FUNCTION
    "#;

    let source = add_std!(src, "string_functions.st");    
    let res: [u16; 2048] = compile_and_run_no_params(source);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "Hello, World!");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}

#[test]
fn test_concat_ext_wstring() {
    let src = r#"
    FUNCTION main : WSTRING[2048]
    VAR_TEMP
        a : WSTRING := "Hello";
        b : WSTRING := ", ";
        c : WSTRING := "World";
        d : WSTRING := "!";
    END_VAR    
        CONCAT_EXT(main, a, b, c, d);
    END_FUNCTION
    "#;

    let source = add_std!(src, "string_functions.st");
    let res: [u16; 2048] = compile_and_run_no_params(source);
    if let Ok(res) = string_from_utf16(&res) {
        assert_eq!(res, "Hello, World!");
    } else {
        panic!("Given string is not UTF16-encoded")
    }
}
