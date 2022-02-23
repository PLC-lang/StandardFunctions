use common::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[derive(Default)]
struct U64Type {
    zero: u64,
    positive: u64,
    negative: u64,
}

#[derive(Default)]
struct U32Type {
    zero: u32,
    positive: u32,
    negative: u32,
    max_overflow: u32,
    min_overflow: u32,
}

#[derive(Default)]
struct U16Type {
    zero: u16,
    positive: u16,
    negative: u16,
    max_overflow: u16,
    min_overflow: u16,
}

#[derive(Default)]
struct U8Type {
    zero: u8,
    positive: u8,
    negative: u8,
    max_overflow: u8,
    min_overflow: u8,
}

#[derive(Default)]
struct BoolType {
    true_: bool,
    false_: bool,
    max_overflow: bool,
    min_overflow: bool,
}

#[test]
fn lword_to_dword() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD; positive : DWORD; negative : DWORD;
		max_overflow: DWORD; min_overflow: DWORD;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		max : LWORD := 4294967295;
		min : LWORD := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_DWORD(LWORD#0);
		ret.positive := LWORD_TO_DWORD(LWORD#100);
		ret.negative := LWORD_TO_DWORD(-1);
		ret.max_overflow := LWORD_TO_DWORD(max+1);
		ret.min_overflow := LWORD_TO_DWORD(min-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.positive, 100u32);
    assert_eq!(maintype.negative, 4294967295u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 4294967295u32);
}

#[test]
fn lword_to_word() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD; positive : WORD; negative : WORD;
		max_overflow: WORD; min_overflow: WORD;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		max : LWORD := 65535;
		min : LWORD := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_WORD(LWORD#0);
		ret.positive := LWORD_TO_WORD(LWORD#100);
		ret.negative := LWORD_TO_WORD(-1);
		ret.max_overflow := LWORD_TO_WORD(max+1);
		ret.min_overflow := LWORD_TO_WORD(min-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.positive, 100u16);
    assert_eq!(maintype.negative, 65535u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 65535u16);
}

#[test]
fn lword_to_byte() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE; positive : BYTE; negative : BYTE;
		max_overflow: BYTE; min_overflow: BYTE;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		max : LWORD := 255;
		min : LWORD := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LWORD_TO_BYTE(LWORD#0);
		ret.positive := LWORD_TO_BYTE(LWORD#100);
		ret.negative := LWORD_TO_BYTE(-1);
		ret.max_overflow := LWORD_TO_BYTE(max+1);
		ret.min_overflow := LWORD_TO_BYTE(min-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.positive, 100u8);
    assert_eq!(maintype.negative, 255u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 255u8);
}

#[test]
fn lword_to_bool() {
    let src = r"
	TYPE myType : STRUCT
		true_ : BOOL; false_ : BOOL;
		max_overflow : BOOL; min_overflow : BOOL;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.true_ := LWORD_TO_BOOL(LWORD#1);
		ret.false_ := LWORD_TO_BOOL(LWORD#0);
		ret.max_overflow := LWORD_TO_BOOL(LWORD#2);
		ret.min_overflow := LWORD_TO_BOOL(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = BoolType::default();
    let _res: bool = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.true_, true);
    assert_eq!(maintype.false_, false);
    assert_eq!(maintype.max_overflow, false);
    assert_eq!(maintype.min_overflow, true);
}

#[test]
fn dword_to_lword() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD; positive : LWORD; negative : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_LWORD(DWORD#0);
		ret.positive := DWORD_TO_LWORD(DWORD#100);
		ret.negative := DWORD_TO_LWORD(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.positive, 100u64);
    assert_eq!(maintype.negative, 4294967295u64);
}

#[test]
fn dword_to_word() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD; positive : WORD; negative : WORD;
		max_overflow: WORD; min_overflow: WORD;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		max : DWORD := 65535;
		min : DWORD := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_WORD(DWORD#0);
		ret.positive := DWORD_TO_WORD(DWORD#100);
		ret.negative := DWORD_TO_WORD(-1);
		ret.max_overflow := DWORD_TO_WORD(max+1);
		ret.min_overflow := DWORD_TO_WORD(min-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.positive, 100u16);
    assert_eq!(maintype.negative, 65535u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 65535u16);
}

#[test]
fn dword_to_byte() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE; positive : BYTE; negative : BYTE;
		max_overflow: BYTE; min_overflow: BYTE;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		max : DWORD := 255;
		min : DWORD := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DWORD_TO_BYTE(DWORD#0);
		ret.positive := DWORD_TO_BYTE(DWORD#100);
		ret.negative := DWORD_TO_BYTE(-1);
		ret.max_overflow := DWORD_TO_BYTE(max+1);
		ret.min_overflow := DWORD_TO_BYTE(min-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.positive, 100u8);
    assert_eq!(maintype.negative, 255u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 255u8);
}

#[test]
fn dword_to_bool() {
    let src = r"
	TYPE myType : STRUCT
		true_ : BOOL; false_ : BOOL;
		max_overflow : BOOL; min_overflow : BOOL;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.true_ := DWORD_TO_BOOL(DWORD#1);
		ret.false_ := DWORD_TO_BOOL(DWORD#0);
		ret.max_overflow := DWORD_TO_BOOL(DWORD#2);
		ret.min_overflow := DWORD_TO_BOOL(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = BoolType::default();
    let _res: bool = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.true_, true);
    assert_eq!(maintype.false_, false);
    assert_eq!(maintype.max_overflow, false);
    assert_eq!(maintype.min_overflow, true);
}

#[test]
fn word_to_lword() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD; positive : LWORD; negative : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := WORD_TO_LWORD(WORD#0);
		ret.positive := WORD_TO_LWORD(WORD#100);
		ret.negative := WORD_TO_LWORD(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.positive, 100u64);
    assert_eq!(maintype.negative, 65535u64);
}

#[test]
fn word_to_dword() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD; positive : DWORD; negative : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := WORD_TO_DWORD(WORD#0);
		ret.positive := WORD_TO_DWORD(WORD#100);
		ret.negative := WORD_TO_DWORD(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.positive, 100u32);
    assert_eq!(maintype.negative, 65535u32);
}

#[test]
fn word_to_byte() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE; positive : BYTE; negative : BYTE;
		max_overflow: BYTE; min_overflow: BYTE;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		max : WORD := 255;
		min : WORD := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := WORD_TO_BYTE(WORD#0);
		ret.positive := WORD_TO_BYTE(WORD#100);
		ret.negative := WORD_TO_BYTE(-1);
		ret.max_overflow := WORD_TO_BYTE(max+1);
		ret.min_overflow := WORD_TO_BYTE(min-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.positive, 100u8);
    assert_eq!(maintype.negative, 255u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 255u8);
}

#[test]
fn word_to_bool() {
    let src = r"
	TYPE myType : STRUCT
		true_ : BOOL; false_ : BOOL;
		max_overflow : BOOL; min_overflow : BOOL;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.true_ := WORD_TO_BOOL(WORD#1);
		ret.false_ := WORD_TO_BOOL(WORD#0);
		ret.max_overflow := WORD_TO_BOOL(WORD#2);
		ret.min_overflow := WORD_TO_BOOL(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = BoolType::default();
    let _res: bool = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.true_, true);
    assert_eq!(maintype.false_, false);
    assert_eq!(maintype.max_overflow, false);
    assert_eq!(maintype.min_overflow, true);
}

#[test]
fn byte_to_lword() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD; positive : LWORD; negative : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BYTE_TO_LWORD(BYTE#0);
		ret.positive := BYTE_TO_LWORD(BYTE#100);
		ret.negative := BYTE_TO_LWORD(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.positive, 100u64);
    assert_eq!(maintype.negative, 255u64);
}

#[test]
fn byte_to_dword() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD; positive : DWORD; negative : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BYTE_TO_DWORD(BYTE#0);
		ret.positive := BYTE_TO_DWORD(BYTE#100);
		ret.negative := BYTE_TO_DWORD(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.positive, 100u32);
    assert_eq!(maintype.negative, 255u32);
}

#[test]
fn byte_to_word() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD; positive : WORD; negative : WORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BYTE_TO_WORD(BYTE#0);
		ret.positive := BYTE_TO_WORD(BYTE#100);
		ret.negative := BYTE_TO_WORD(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.positive, 100u16);
    assert_eq!(maintype.negative, 255u16);
}

#[test]
fn byte_to_bool() {
    let src = r"
	TYPE myType : STRUCT
		true_ : BOOL; false_ : BOOL;
		max_overflow : BOOL; min_overflow : BOOL;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.true_ := BYTE_TO_BOOL(BYTE#1);
		ret.false_ := BYTE_TO_BOOL(BYTE#0);
		ret.max_overflow := BYTE_TO_BOOL(BYTE#2);
		ret.min_overflow := BYTE_TO_BOOL(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = BoolType::default();
    let _res: bool = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.true_, true);
    assert_eq!(maintype.false_, false);
    assert_eq!(maintype.max_overflow, false);
    assert_eq!(maintype.min_overflow, true);
}

#[test]
fn bool_to_lword() {
    let src = r"
	TYPE myType : STRUCT
		zero : LWORD; positive : LWORD; negative : LWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_LWORD(BOOL#0);
		ret.positive := BOOL_TO_LWORD(BOOL#1);
		ret.negative := BOOL_TO_LWORD(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.positive, 1u64);
    assert_eq!(maintype.negative, 2u64);
}

#[test]
fn bool_to_dword() {
    let src = r"
	TYPE myType : STRUCT
		zero : DWORD; positive : DWORD; negative : DWORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_DWORD(BOOL#0);
		ret.positive := BOOL_TO_DWORD(BOOL#1);
		ret.negative := BOOL_TO_DWORD(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.positive, 1u32);
    assert_eq!(maintype.negative, 2u32);
}

#[test]
fn bool_to_word() {
    let src = r"
	TYPE myType : STRUCT
		zero : WORD; positive : WORD; negative : WORD;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_WORD(BOOL#0);
		ret.positive := BOOL_TO_WORD(BOOL#1);
		ret.negative := BOOL_TO_WORD(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.positive, 1u16);
    assert_eq!(maintype.negative, 2u16);
}

#[test]
fn bool_to_byte() {
    let src = r"
	TYPE myType : STRUCT
		zero : BYTE; positive : BYTE; negative : BYTE;
	END_STRUCT END_TYPE

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := BOOL_TO_BYTE(BOOL#0);
		ret.positive := BOOL_TO_BYTE(BOOL#1);
		ret.negative := BOOL_TO_BYTE(-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "bit_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.positive, 1u8);
    assert_eq!(maintype.negative, 2u8);
}
