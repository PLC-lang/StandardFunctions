use common::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[derive(Default)]
struct MainType {
    max: bool,
    min: bool,
    zero: bool,
    inf: bool,
    nan: bool,
}

#[test]
fn is_valid_real() {
    let src = "
	VAR_GLOBAL
		MAX : REAL := 3.4028235e38;
		MIN : REAL := -3.4028235e38;
		INF : REAL := 1.0 / 0.0;
		NaN : REAL := 0.0 / 0.0;
	END_VAR

	PROGRAM main
	VAR
		max_ : BOOL;
		min_ : BOOL;
		zero : BOOL;
		inf_ : BOOL;
		nan_ : BOOL;
	END_VAR
		max_ := IS_VALID(MAX);
		min_ := IS_VALID(MIN);
		zero := IS_VALID(REAL#0.0);
		inf_ := IS_VALID(INF);
		nan_ := IS_VALID(NaN);
	END_PROGRAM";
    let sources = add_std!(src, "validation.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    // MIN valid
    assert!(maintype.max);
    // MAX valid
    assert!(maintype.min);
    // ZERO valid
    assert!(maintype.zero);
    // inf invalid
    assert!(!maintype.inf);
    // NaN invalid
    assert!(!maintype.nan);
}

#[test]
fn is_valid_lreal() {
    let src = "
	VAR_GLOBAL
		MAX : LREAL := 1.7976931348623157e308;
		MIN : LREAL := -1.7976931348623157e308;
		INF : LREAL := 1.0 / 0.0;
		NaN : LREAL := 0.0 / 0.0;
	END_VAR

	PROGRAM main
	VAR
		max_ : BOOL;
		min_ : BOOL;
		zero : BOOL;
		inf_ : BOOL;
		nan_ : BOOL;
	END_VAR
		max_ := IS_VALID(MAX);
		min_ := IS_VALID(MIN);
		zero := IS_VALID(LREAL#0.0);
		inf_ := IS_VALID(INF);
		nan_ := IS_VALID(NaN);
	END_PROGRAM";
    let sources = add_std!(src, "validation.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    // MIN valid
    assert!(maintype.max);
    // MAX valid
    assert!(maintype.min);
    // ZERO valid
    assert!(maintype.zero);
    // inf invalid
    assert!(!maintype.inf);
    // NaN invalid
    assert!(!maintype.nan);
}

// BCD 4 bit per decimal digit
// valid values are :
// 0000 0001 0010 0011 0100 0101 0110 0111 1000 1001
//  0    1    2    3    4    5    6    7    8    9
// invalid values are :
// 1010 1011 1100 1101 1110 1111
//  10   11   12   13   14   15

#[derive(Default)]
struct StructBCD {
    valid: bool,
    invalid: bool,
}

#[test]
fn is_valid_byte() {
    let src = "
	PROGRAM main
	VAR
		valid	: BOOL;
		invalid	: BOOL;
	END_VAR
		valid	:= IS_VALID_BCD(BYTE#50); // 50 => 0011_0010 in BCD 3_2 => VALID
		invalid	:= IS_VALID_BCD(BYTE#44); // 44 => 0010_1100 in BCD 2_[12] => INVALID
	END_PROGRAM";
    let sources = add_std!(src, "validation.st");
    let mut maintype = StructBCD::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert!(maintype.valid);
    assert!(!maintype.invalid);
}

#[test]
fn is_valid_word() {
    let src = "
	PROGRAM main
	VAR
		valid	: BOOL;
		invalid	: BOOL;
	END_VAR
		valid	:= IS_VALID_BCD(WORD#18_545); // 18_545 => 0100_1000_0111_0001 in BCD 4_8_7_1 => VALID
		invalid	:= IS_VALID_BCD(WORD#19_313); // 19_313 => 0100_1011_0111_0001 in BCD 4_[11]_7_1 => INVALID
	END_PROGRAM";
    let sources = add_std!(src, "validation.st");
    let mut maintype = StructBCD::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert!(maintype.valid);
    assert!(!maintype.invalid);
}

#[test]
fn is_valid_dword() {
    let src = "
	PROGRAM main
	VAR
		valid	: BOOL;
		invalid	: BOOL;
	END_VAR
		valid	:= IS_VALID_BCD(DWORD#1_215_383_665); // 1_215_383_665 => 0100_1000_0111_0001_0100_1000_0111_0001 in BCD 4_8_7_1_4_8_7_1 => VALID
		invalid	:= IS_VALID_BCD(DWORD#1_265_716_081); // 1_265_716_081 => 0100_1011_0111_0001_0100_1011_0111_0001 in BCD 4_[11]_7_1_4_11_7_1 => INVALID
	END_PROGRAM";
    let sources = add_std!(src, "validation.st");
    let mut maintype = StructBCD::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert!(maintype.valid);
    assert!(!maintype.invalid);
}

#[test]
fn is_valid_lword() {
    let src = "
	PROGRAM main
	VAR
		valid	: BOOL;
		invalid	: BOOL;
	END_VAR
		valid	:= IS_VALID_BCD(LWORD#4_611_686_018_427_387_904); // 4_611_686_018_427_387_904 => 0100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000 in BCD 4_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0 => VALID
		invalid	:= IS_VALID_BCD(LWORD#4_611_686_018_427_387_919); // 4_611_686_018_427_387_919 => 0100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1111 in BCD 4_0_0_0_0_0_0_0_0_0_0_0_0_0_0_[15] => INVALID
	END_PROGRAM";
    let sources = add_std!(src, "validation.st");
    let mut maintype = StructBCD::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert!(maintype.valid);
    assert!(!maintype.invalid);
}
