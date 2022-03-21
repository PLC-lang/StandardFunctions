use common::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[test]
fn wstring_to_string_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u8; 6],
    }

    let src = r#"
	PROGRAM main
	VAR
		res : STRING;
		ptr : REF_TO STRING;
	END_VAR
		res := WSTRING_TO_STRING(WSTRING#"hello");
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(&maintype.res, "hello\0".as_bytes());
}

#[test]
fn wstring_to_wchar_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u16; 2],
    }

    let src = r#"
	PROGRAM main
	VAR
		res : WCHAR;
	END_VAR
		res := WSTRING_TO_WCHAR(WSTRING#"ABC");
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, [65u16, 0u16]);
}

#[test]
fn string_to_wstring_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u16; 6],
    }

    let src = r#"
	PROGRAM main
	VAR
		res : WSTRING;
	END_VAR
		res := STRING_TO_WSTRING(STRING#'Hello');
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, [72u16, 101u16, 108u16, 108u16, 111u16, 0u16]);
}

#[test]
fn string_to_char_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u8; 2],
    }

    let src = r#"
	PROGRAM main
	VAR
		res : CHAR;
	END_VAR
		res := STRING_TO_CHAR(STRING#'BCD');
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, "B\0".as_bytes());
}

#[test]
fn wchar_to_wstring_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u16; 2],
    }

    let src = r#"
	PROGRAM main
	VAR
		res : WSTRING[1];
	END_VAR
		res := WCHAR_TO_WSTRING(WCHAR#"A");
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(&maintype.res, &[65u16, 0u16]);
}

#[test]
fn wchar_to_char_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u8; 2],
    }

    let src = r#"
	PROGRAM main
	VAR
		res : CHAR;
	END_VAR
		res := WCHAR_TO_CHAR(WCHAR#"A");
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, "A\0".as_bytes());
}

#[test]
fn char_to_string_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u8; 2],
    }

    let src = r#"
	PROGRAM main
	VAR
		res : STRING[1];
	END_VAR
		res := CHAR_TO_STRING(CHAR#'B');
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, "B\0".as_bytes());
}

#[test]
fn char_to_wchar_conversion() {
    #[derive(Default)]
    struct MainType {
        res: [u16; 2],
    }

    let src = r#"
	PROGRAM main
	VAR
		res : WCHAR;
	END_VAR
		res := CHAR_TO_WCHAR(CHAR#'B');
    END_PROGRAM
        "#;
    let sources = add_std!(src, "string_conversion.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.res, [66u16, 0u16]);
}
