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
