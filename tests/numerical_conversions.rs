use rusty::runner::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[derive(Default)]
struct MainType {
    zero: i32,
    negative: i32,
    positive: i32,
    max_minus_one: i32,
    min_plus_one: i32,
    max_overflow: i32,
    min_overflow: i32,
}

#[test]
fn lint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT 
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE
	
	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_TO_DINT(LINT#0);
		ret.negative := LINT_TO_DINT(LINT#-1111);
		ret.positive := LINT_TO_DINT(LINT#2222);
		ret.max_minus_one := LINT_TO_DINT(LINT#2147483646);
		ret.min_plus_one := LINT_TO_DINT(LINT#-2147483647);
		// overflow DINT max = 2147483647 by 1
		ret.max_overflow := LINT_TO_DINT(LINT#2147483648);
		// overflow DINT min = -2147483648 by 1
		ret.min_overflow := LINT_TO_DINT(LINT#-2147483649);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = MainType::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, -1111i32);
    assert_eq!(maintype.positive, 2222i32);
    assert_eq!(maintype.max_minus_one, 2147483646i32);
    assert_eq!(maintype.min_plus_one, -2147483647i32);
    // should be DINT min = -2147483648
    assert_eq!(maintype.max_overflow, -2147483648i32);
    // should be DINT max = -2147483648
    assert_eq!(maintype.min_overflow, 2147483647i32);
}
