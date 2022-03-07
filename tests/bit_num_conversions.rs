use common::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[derive(Default)]
struct F64Type {
    zero: f64,
}

#[derive(Default)]
struct F32Type {
    zero: f32,
}

#[derive(Default)]
struct I64Type {
    zero: i64,
}

#[derive(Default)]
struct I32Type {
    zero: i32,
}

#[derive(Default)]
struct I16Type {
    zero: i16,
}

#[derive(Default)]
struct I8Type {
    zero: i8,
}

#[derive(Default)]
struct U64Type {
    zero: u64,
}

#[derive(Default)]
struct U32Type {
    zero: u32,
}

#[derive(Default)]
struct U16Type {
    zero: u16,
}

#[derive(Default)]
struct U8Type {
    zero: u8,
}

#[test]
fn lword_to_lreal_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LREAL;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_LREAL(LWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = F64Type::default();
    let _res: f64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0.0f64);
}

#[test]
fn lword_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_LINT(LWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
}

#[test]
fn lword_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_DINT(LWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
}

#[test]
fn lword_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : INT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_INT(LWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
}

#[test]
fn lword_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : SINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_SINT(LWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
}

#[test]
fn lword_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : ULINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_ULINT(LWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn lword_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_UDINT(LWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn lword_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_UINT(LWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn lword_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_USINT(LWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn dword_to_real_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : REAL;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_REAL(DWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = F32Type::default();
    let _res: f32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0f32);
}

#[test]
fn dword_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_LINT(DWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
}

#[test]
fn dword_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_DINT(DWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
}

#[test]
fn dword_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : INT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_INT(DWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
}

#[test]
fn dword_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : SINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_SINT(DWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
}

#[test]
fn dword_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : ULINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_ULINT(DWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn dword_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_UDINT(DWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn dword_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_UINT(DWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn dword_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_USINT(DWORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn word_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := WORD_TO_LINT(WORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
}

#[test]
fn word_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := WORD_TO_DINT(WORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
}

#[test]
fn word_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : INT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := WORD_TO_INT(WORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
}

#[test]
fn word_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : SINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := WORD_TO_SINT(WORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
}

#[test]
fn word_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : ULINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := WORD_TO_ULINT(WORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn word_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := WORD_TO_UDINT(WORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn word_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := WORD_TO_UINT(WORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn word_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := WORD_TO_USINT(WORD#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn byte_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BYTE_TO_LINT(BYTE#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
}

#[test]
fn byte_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BYTE_TO_DINT(BYTE#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
}

#[test]
fn byte_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : INT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BYTE_TO_INT(BYTE#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
}

#[test]
fn byte_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : SINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BYTE_TO_SINT(BYTE#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
}

#[test]
fn byte_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : ULINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BYTE_TO_ULINT(BYTE#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn byte_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BYTE_TO_UDINT(BYTE#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn byte_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BYTE_TO_UINT(BYTE#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn byte_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BYTE_TO_USINT(BYTE#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn bool_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_LINT(BOOL#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
}

#[test]
fn bool_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_DINT(BOOL#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
}

#[test]
fn bool_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : INT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_INT(BOOL#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
}

#[test]
fn bool_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : SINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_SINT(BOOL#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
}

#[test]
fn bool_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : ULINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_ULINT(BOOL#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn bool_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_UDINT(BOOL#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn bool_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_UINT(BOOL#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn bool_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_USINT(BOOL#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn lreal_to_lword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LREAL_TO_LWORD(LREAL#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn real_to_dword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := REAL_TO_DWORD(REAL#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn lint_to_lword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_TO_LWORD(LINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn lint_to_dword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_TO_DWORD(LINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn lint_to_word_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_TO_WORD(LINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn lint_to_byte_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_TO_BYTE(LINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn dint_to_lword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DINT_TO_LWORD(DINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn dint_to_dword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DINT_TO_DWORD(DINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn dint_to_word_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DINT_TO_WORD(DINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn dint_to_byte_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DINT_TO_BYTE(DINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn int_to_lword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := INT_TO_LWORD(INT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn int_to_dword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := INT_TO_DWORD(INT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn int_to_word_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := INT_TO_WORD(INT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn int_to_byte_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := INT_TO_BYTE(INT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn sint_to_lword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := SINT_TO_LWORD(SINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn sint_to_dword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := SINT_TO_DWORD(SINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn sint_to_word_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := SINT_TO_WORD(SINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn sint_to_byte_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := SINT_TO_BYTE(SINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn ulint_to_lword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_TO_LWORD(ULINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn ulint_to_dword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_TO_DWORD(ULINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn ulint_to_word_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_TO_WORD(ULINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn ulint_to_byte_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_TO_BYTE(ULINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn udint_to_lword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_TO_LWORD(UDINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn udint_to_dword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_TO_DWORD(UDINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn udint_to_word_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_TO_WORD(UDINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn udint_to_byte_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_TO_BYTE(UDINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn uint_to_lword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_TO_LWORD(UINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn uint_to_dword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_TO_DWORD(UINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn uint_to_word_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_TO_WORD(UINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn uint_to_byte_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_TO_BYTE(UINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}

#[test]
fn usint_to_lword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_TO_LWORD(USINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
}

#[test]
fn usint_to_dword_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_TO_DWORD(USINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
}

#[test]
fn usint_to_word_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_TO_WORD(USINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
}

#[test]
fn usint_to_byte_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_TO_BYTE(USINT#0);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
}
