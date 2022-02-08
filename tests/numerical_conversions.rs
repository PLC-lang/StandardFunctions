use rusty::runner::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[derive(Default)]
struct I64Type {
    zero: i64,
    negative: i64,
    positive: i64,
    max_minus_one: i64,
    min_plus_one: i64,
    max_overflow: i64,
    min_overflow: i64,
}
#[derive(Default)]
struct U64Type {
    zero: u64,
    negative: u64,
    positive: u64,
    max_minus_one: u64,
    min_plus_one: u64,
    max_overflow: u64,
    min_overflow: u64,
}
#[derive(Default)]
struct I32Type {
    zero: i32,
    negative: i32,
    positive: i32,
    max_minus_one: i32,
    min_plus_one: i32,
    max_overflow: i32,
    min_overflow: i32,
}
#[derive(Default)]
struct U32Type {
    zero: u32,
    negative: u32,
    positive: u32,
    max_minus_one: u32,
    min_plus_one: u32,
    max_overflow: u32,
    min_overflow: u32,
}

#[derive(Default)]
struct I16Type {
    zero: i16,
    negative: i16,
    positive: i16,
    max_minus_one: i16,
    min_plus_one: i16,
    max_overflow: i16,
    min_overflow: i16,
}

#[derive(Default)]
struct U16Type {
    zero: u16,
    negative: u16,
    positive: u16,
    max_minus_one: u16,
    min_plus_one: u16,
    max_overflow: u16,
    min_overflow: u16,
}

#[derive(Default)]
struct I8Type {
    zero: i8,
    negative: i8,
    positive: i8,
    max_minus_one: i8,
    min_plus_one: i8,
    max_overflow: i8,
    min_overflow: i8,
}
#[derive(Default)]
struct U8Type {
    zero: u8,
    negative: u8,
    positive: u8,
    max_minus_one: u8,
    min_plus_one: u8,
    max_overflow: u8,
    min_overflow: u8,
}

// #[test]
// fn LREAL_to_REAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : LREAL := 1;
// 		MIN : LREAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := LREAL_to_REAL(LREAL#0);
// 		ret.negative := LREAL_to_REAL(LREAL#-11);
// 		ret.positive := LREAL_to_REAL(LREAL#22);
// 		ret.max_minus_one := LREAL_to_REAL(MAX-1);
// 		ret.min_plus_one := LREAL_to_REAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := LREAL_to_REAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := LREAL_to_REAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn LREAL_to_LINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : LREAL := 1;
// 		MIN : LREAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := LREAL_to_LINT(LREAL#0);
// 		ret.negative := LREAL_to_LINT(LREAL#-11);
// 		ret.positive := LREAL_to_LINT(LREAL#22);
// 		ret.max_minus_one := LREAL_to_LINT(MAX-1);
// 		ret.min_plus_one := LREAL_to_LINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := LREAL_to_LINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := LREAL_to_LINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn LREAL_to_DINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : LREAL := 1;
// 		MIN : LREAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := LREAL_to_DINT(LREAL#0);
// 		ret.negative := LREAL_to_DINT(LREAL#-11);
// 		ret.positive := LREAL_to_DINT(LREAL#22);
// 		ret.max_minus_one := LREAL_to_DINT(MAX-1);
// 		ret.min_plus_one := LREAL_to_DINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := LREAL_to_DINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := LREAL_to_DINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn LREAL_to_INT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : LREAL := 1;
// 		MIN : LREAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := LREAL_to_INT(LREAL#0);
// 		ret.negative := LREAL_to_INT(LREAL#-11);
// 		ret.positive := LREAL_to_INT(LREAL#22);
// 		ret.max_minus_one := LREAL_to_INT(MAX-1);
// 		ret.min_plus_one := LREAL_to_INT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := LREAL_to_INT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := LREAL_to_INT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn LREAL_to_SINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : LREAL := 1;
// 		MIN : LREAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := LREAL_to_SINT(LREAL#0);
// 		ret.negative := LREAL_to_SINT(LREAL#-11);
// 		ret.positive := LREAL_to_SINT(LREAL#22);
// 		ret.max_minus_one := LREAL_to_SINT(MAX-1);
// 		ret.min_plus_one := LREAL_to_SINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := LREAL_to_SINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := LREAL_to_SINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn LREAL_to_ULINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : LREAL := 1;
// 		MIN : LREAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := LREAL_to_ULINT(LREAL#0);
// 		ret.negative := LREAL_to_ULINT(LREAL#-11);
// 		ret.positive := LREAL_to_ULINT(LREAL#22);
// 		ret.max_minus_one := LREAL_to_ULINT(MAX-1);
// 		ret.min_plus_one := LREAL_to_ULINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := LREAL_to_ULINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := LREAL_to_ULINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn LREAL_to_UDINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : LREAL := 1;
// 		MIN : LREAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := LREAL_to_UDINT(LREAL#0);
// 		ret.negative := LREAL_to_UDINT(LREAL#-11);
// 		ret.positive := LREAL_to_UDINT(LREAL#22);
// 		ret.max_minus_one := LREAL_to_UDINT(MAX-1);
// 		ret.min_plus_one := LREAL_to_UDINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := LREAL_to_UDINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := LREAL_to_UDINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn LREAL_to_UINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : LREAL := 1;
// 		MIN : LREAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := LREAL_to_UINT(LREAL#0);
// 		ret.negative := LREAL_to_UINT(LREAL#-11);
// 		ret.positive := LREAL_to_UINT(LREAL#22);
// 		ret.max_minus_one := LREAL_to_UINT(MAX-1);
// 		ret.min_plus_one := LREAL_to_UINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := LREAL_to_UINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := LREAL_to_UINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn LREAL_to_USINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : LREAL := 1;
// 		MIN : LREAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := LREAL_to_USINT(LREAL#0);
// 		ret.negative := LREAL_to_USINT(LREAL#-11);
// 		ret.positive := LREAL_to_USINT(LREAL#22);
// 		ret.max_minus_one := LREAL_to_USINT(MAX-1);
// 		ret.min_plus_one := LREAL_to_USINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := LREAL_to_USINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := LREAL_to_USINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn REAL_to_LREAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : REAL := 1;
// 		MIN : REAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := REAL_to_LREAL(REAL#0);
// 		ret.negative := REAL_to_LREAL(REAL#-11);
// 		ret.positive := REAL_to_LREAL(REAL#22);
// 		ret.max_minus_one := REAL_to_LREAL(MAX-1);
// 		ret.min_plus_one := REAL_to_LREAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := REAL_to_LREAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := REAL_to_LREAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn REAL_to_LINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : REAL := 1;
// 		MIN : REAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := REAL_to_LINT(REAL#0);
// 		ret.negative := REAL_to_LINT(REAL#-11);
// 		ret.positive := REAL_to_LINT(REAL#22);
// 		ret.max_minus_one := REAL_to_LINT(MAX-1);
// 		ret.min_plus_one := REAL_to_LINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := REAL_to_LINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := REAL_to_LINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn REAL_to_DINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : REAL := 1;
// 		MIN : REAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := REAL_to_DINT(REAL#0);
// 		ret.negative := REAL_to_DINT(REAL#-11);
// 		ret.positive := REAL_to_DINT(REAL#22);
// 		ret.max_minus_one := REAL_to_DINT(MAX-1);
// 		ret.min_plus_one := REAL_to_DINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := REAL_to_DINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := REAL_to_DINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn REAL_to_INT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : REAL := 1;
// 		MIN : REAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := REAL_to_INT(REAL#0);
// 		ret.negative := REAL_to_INT(REAL#-11);
// 		ret.positive := REAL_to_INT(REAL#22);
// 		ret.max_minus_one := REAL_to_INT(MAX-1);
// 		ret.min_plus_one := REAL_to_INT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := REAL_to_INT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := REAL_to_INT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn REAL_to_SINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : REAL := 1;
// 		MIN : REAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := REAL_to_SINT(REAL#0);
// 		ret.negative := REAL_to_SINT(REAL#-11);
// 		ret.positive := REAL_to_SINT(REAL#22);
// 		ret.max_minus_one := REAL_to_SINT(MAX-1);
// 		ret.min_plus_one := REAL_to_SINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := REAL_to_SINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := REAL_to_SINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn REAL_to_ULINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : REAL := 1;
// 		MIN : REAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := REAL_to_ULINT(REAL#0);
// 		ret.negative := REAL_to_ULINT(REAL#-11);
// 		ret.positive := REAL_to_ULINT(REAL#22);
// 		ret.max_minus_one := REAL_to_ULINT(MAX-1);
// 		ret.min_plus_one := REAL_to_ULINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := REAL_to_ULINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := REAL_to_ULINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn REAL_to_UDINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : REAL := 1;
// 		MIN : REAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := REAL_to_UDINT(REAL#0);
// 		ret.negative := REAL_to_UDINT(REAL#-11);
// 		ret.positive := REAL_to_UDINT(REAL#22);
// 		ret.max_minus_one := REAL_to_UDINT(MAX-1);
// 		ret.min_plus_one := REAL_to_UDINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := REAL_to_UDINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := REAL_to_UDINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn REAL_to_UINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : REAL := 1;
// 		MIN : REAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := REAL_to_UINT(REAL#0);
// 		ret.negative := REAL_to_UINT(REAL#-11);
// 		ret.positive := REAL_to_UINT(REAL#22);
// 		ret.max_minus_one := REAL_to_UINT(MAX-1);
// 		ret.min_plus_one := REAL_to_UINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := REAL_to_UINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := REAL_to_UINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn REAL_to_USINT_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : REAL := 1;
// 		MIN : REAL := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := REAL_to_USINT(REAL#0);
// 		ret.negative := REAL_to_USINT(REAL#-11);
// 		ret.positive := REAL_to_USINT(REAL#22);
// 		ret.max_minus_one := REAL_to_USINT(MAX-1);
// 		ret.min_plus_one := REAL_to_USINT(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := REAL_to_USINT(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := REAL_to_USINT(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn LINT_to_LREAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : LINT := 1;
// 		MIN : LINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := LINT_to_LREAL(LINT#0);
// 		ret.negative := LINT_to_LREAL(LINT#-11);
// 		ret.positive := LINT_to_LREAL(LINT#22);
// 		ret.max_minus_one := LINT_to_LREAL(MAX-1);
// 		ret.min_plus_one := LINT_to_LREAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := LINT_to_LREAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := LINT_to_LREAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn LINT_to_REAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : LINT := 1;
// 		MIN : LINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := LINT_to_REAL(LINT#0);
// 		ret.negative := LINT_to_REAL(LINT#-11);
// 		ret.positive := LINT_to_REAL(LINT#22);
// 		ret.max_minus_one := LINT_to_REAL(MAX-1);
// 		ret.min_plus_one := LINT_to_REAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := LINT_to_REAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := LINT_to_REAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

#[test]
fn lint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT 
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : LINT := 2147483647;
		MIN : LINT := -2147483648;
	END_VAR
	
	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_to_DINT(LINT#0);
		ret.negative := LINT_to_DINT(LINT#-11);
		ret.positive := LINT_to_DINT(LINT#22);
		ret.max_minus_one := LINT_to_DINT(MAX-1);
		ret.min_plus_one := LINT_to_DINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := LINT_to_DINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := LINT_to_DINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, -11i32);
    assert_eq!(maintype.positive, 22i32);
    assert_eq!(maintype.max_minus_one, 2147483646i32);
    assert_eq!(maintype.min_plus_one, -2147483647i32);
    assert_eq!(maintype.max_overflow, -2147483648i32);
    assert_eq!(maintype.min_overflow, 2147483647i32);
}

#[test]
fn lint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : LINT := 32767;
		MIN : LINT := -32768;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_to_INT(LINT#0);
		ret.negative := LINT_to_INT(LINT#-11);
		ret.positive := LINT_to_INT(LINT#22);
		ret.max_minus_one := LINT_to_INT(MAX-1);
		ret.min_plus_one := LINT_to_INT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := LINT_to_INT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := LINT_to_INT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.negative, -11i16);
    assert_eq!(maintype.positive, 22i16);
    assert_eq!(maintype.max_minus_one, 32766i16);
    assert_eq!(maintype.min_plus_one, -32767i16);
    assert_eq!(maintype.max_overflow, -32768i16);
    assert_eq!(maintype.min_overflow, 32767i16);
}

#[test]
fn lint_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : LINT := 127;
		MIN : LINT := -128;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_to_SINT(LINT#0);
		ret.negative := LINT_to_SINT(LINT#-11);
		ret.positive := LINT_to_SINT(LINT#22);
		ret.max_minus_one := LINT_to_SINT(MAX-1);
		ret.min_plus_one := LINT_to_SINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := LINT_to_SINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := LINT_to_SINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.negative, -11i8);
    assert_eq!(maintype.positive, 22i8);
    assert_eq!(maintype.max_minus_one, 126i8);
    assert_eq!(maintype.min_plus_one, -127i8);
    assert_eq!(maintype.max_overflow, -128i8);
    assert_eq!(maintype.min_overflow, 127i8);
}

#[test]
fn lint_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : LINT := 1;
		MIN : LINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_to_ULINT(LINT#0);
		ret.negative := LINT_to_ULINT(LINT#-1);
		ret.positive := LINT_to_ULINT(LINT#22);
		ret.max_minus_one := LINT_to_ULINT(MAX-1);
		ret.min_plus_one := LINT_to_ULINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := LINT_to_ULINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := LINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 0u64);
    assert_eq!(maintype.positive, 0u64);
    assert_eq!(maintype.max_minus_one, 0u64);
    assert_eq!(maintype.min_plus_one, 0u64);
    assert_eq!(maintype.max_overflow, 0u64);
    assert_eq!(maintype.min_overflow, 0u64);
}

#[test]
fn lint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : LINT := 1;
		MIN : LINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_to_UDINT(LINT#0);
		ret.negative := LINT_to_UDINT(LINT#-1);
		ret.positive := LINT_to_UDINT(LINT#22);
		ret.max_minus_one := LINT_to_UDINT(MAX-1);
		ret.min_plus_one := LINT_to_UDINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := LINT_to_UDINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := LINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 0u32);
    assert_eq!(maintype.positive, 0u32);
    assert_eq!(maintype.max_minus_one, 0u32);
    assert_eq!(maintype.min_plus_one, 0u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 0u32);
}

#[test]
fn lint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : LINT := 1;
		MIN : LINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_to_UINT(LINT#0);
		ret.negative := LINT_to_UINT(LINT#-1);
		ret.positive := LINT_to_UINT(LINT#22);
		ret.max_minus_one := LINT_to_UINT(MAX-1);
		ret.min_plus_one := LINT_to_UINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := LINT_to_UINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := LINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 0u16);
    assert_eq!(maintype.positive, 0u16);
    assert_eq!(maintype.max_minus_one, 0u16);
    assert_eq!(maintype.min_plus_one, 0u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 0u16);
}

#[test]
fn lint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : LINT := 1;
		MIN : LINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := LINT_to_USINT(LINT#0);
		ret.negative := LINT_to_USINT(LINT#-1);
		ret.positive := LINT_to_USINT(LINT#22);
		ret.max_minus_one := LINT_to_USINT(MAX-1);
		ret.min_plus_one := LINT_to_USINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := LINT_to_USINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := LINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 0u8);
    assert_eq!(maintype.positive, 0u8);
    assert_eq!(maintype.max_minus_one, 0u8);
    assert_eq!(maintype.min_plus_one, 0u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 0u8);
}

// #[test]
// fn DINT_to_LREAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : DINT := 1;
// 		MIN : DINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := DINT_to_LREAL(DINT#0);
// 		ret.negative := DINT_to_LREAL(DINT#-11);
// 		ret.positive := DINT_to_LREAL(DINT#22);
// 		ret.max_minus_one := DINT_to_LREAL(MAX-1);
// 		ret.min_plus_one := DINT_to_LREAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := DINT_to_LREAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := DINT_to_LREAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn DINT_to_REAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : DINT := 1;
// 		MIN : DINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := DINT_to_REAL(DINT#0);
// 		ret.negative := DINT_to_REAL(DINT#-11);
// 		ret.positive := DINT_to_REAL(DINT#22);
// 		ret.max_minus_one := DINT_to_REAL(MAX-1);
// 		ret.min_plus_one := DINT_to_REAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := DINT_to_REAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := DINT_to_REAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

#[test]
fn dint_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 1;
		MIN : DINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DINT_to_LINT(DINT#0);
		ret.negative := DINT_to_LINT(DINT#-11);
		ret.positive := DINT_to_LINT(DINT#22);
		ret.max_minus_one := DINT_to_LINT(MAX-1);
		ret.min_plus_one := DINT_to_LINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := DINT_to_LINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := DINT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, 0i64);
    assert_eq!(maintype.positive, 0i64);
    assert_eq!(maintype.max_minus_one, 0i64);
    assert_eq!(maintype.min_plus_one, 0i64);
    assert_eq!(maintype.max_overflow, 0i64);
    assert_eq!(maintype.min_overflow, 0i64);
}

#[test]
fn dint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 1;
		MIN : DINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DINT_to_INT(DINT#0);
		ret.negative := DINT_to_INT(DINT#-11);
		ret.positive := DINT_to_INT(DINT#22);
		ret.max_minus_one := DINT_to_INT(MAX-1);
		ret.min_plus_one := DINT_to_INT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := DINT_to_INT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := DINT_to_INT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.negative, 0i16);
    assert_eq!(maintype.positive, 0i16);
    assert_eq!(maintype.max_minus_one, 0i16);
    assert_eq!(maintype.min_plus_one, 0i16);
    assert_eq!(maintype.max_overflow, 0i16);
    assert_eq!(maintype.min_overflow, 0i16);
}

#[test]
fn dint_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 1;
		MIN : DINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DINT_to_SINT(DINT#0);
		ret.negative := DINT_to_SINT(DINT#-11);
		ret.positive := DINT_to_SINT(DINT#22);
		ret.max_minus_one := DINT_to_SINT(MAX-1);
		ret.min_plus_one := DINT_to_SINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := DINT_to_SINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := DINT_to_SINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.negative, 0i8);
    assert_eq!(maintype.positive, 0i8);
    assert_eq!(maintype.max_minus_one, 0i8);
    assert_eq!(maintype.min_plus_one, 0i8);
    assert_eq!(maintype.max_overflow, 0i8);
    assert_eq!(maintype.min_overflow, 0i8);
}

#[test]
fn dint_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 1;
		MIN : DINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DINT_to_ULINT(DINT#0);
		ret.negative := DINT_to_ULINT(DINT#-1);
		ret.positive := DINT_to_ULINT(DINT#22);
		ret.max_minus_one := DINT_to_ULINT(MAX-1);
		ret.min_plus_one := DINT_to_ULINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := DINT_to_ULINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := DINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 0u64);
    assert_eq!(maintype.positive, 0u64);
    assert_eq!(maintype.max_minus_one, 0u64);
    assert_eq!(maintype.min_plus_one, 0u64);
    assert_eq!(maintype.max_overflow, 0u64);
    assert_eq!(maintype.min_overflow, 0u64);
}

#[test]
fn dint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 1;
		MIN : DINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DINT_to_UDINT(DINT#0);
		ret.negative := DINT_to_UDINT(DINT#-1);
		ret.positive := DINT_to_UDINT(DINT#22);
		ret.max_minus_one := DINT_to_UDINT(MAX-1);
		ret.min_plus_one := DINT_to_UDINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := DINT_to_UDINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := DINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 0u32);
    assert_eq!(maintype.positive, 0u32);
    assert_eq!(maintype.max_minus_one, 0u32);
    assert_eq!(maintype.min_plus_one, 0u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 0u32);
}

#[test]
fn dint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 1;
		MIN : DINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DINT_to_UINT(DINT#0);
		ret.negative := DINT_to_UINT(DINT#-1);
		ret.positive := DINT_to_UINT(DINT#22);
		ret.max_minus_one := DINT_to_UINT(MAX-1);
		ret.min_plus_one := DINT_to_UINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := DINT_to_UINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := DINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 0u16);
    assert_eq!(maintype.positive, 0u16);
    assert_eq!(maintype.max_minus_one, 0u16);
    assert_eq!(maintype.min_plus_one, 0u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 0u16);
}

#[test]
fn dint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 1;
		MIN : DINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := DINT_to_USINT(DINT#0);
		ret.negative := DINT_to_USINT(DINT#-1);
		ret.positive := DINT_to_USINT(DINT#22);
		ret.max_minus_one := DINT_to_USINT(MAX-1);
		ret.min_plus_one := DINT_to_USINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := DINT_to_USINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := DINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 0u8);
    assert_eq!(maintype.positive, 0u8);
    assert_eq!(maintype.max_minus_one, 0u8);
    assert_eq!(maintype.min_plus_one, 0u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 0u8);
}

// #[test]
// fn INT_to_LREAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : INT := 1;
// 		MIN : INT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := INT_to_LREAL(INT#0);
// 		ret.negative := INT_to_LREAL(INT#-11);
// 		ret.positive := INT_to_LREAL(INT#22);
// 		ret.max_minus_one := INT_to_LREAL(MAX-1);
// 		ret.min_plus_one := INT_to_LREAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := INT_to_LREAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := INT_to_LREAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn INT_to_REAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : INT := 1;
// 		MIN : INT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := INT_to_REAL(INT#0);
// 		ret.negative := INT_to_REAL(INT#-11);
// 		ret.positive := INT_to_REAL(INT#22);
// 		ret.max_minus_one := INT_to_REAL(MAX-1);
// 		ret.min_plus_one := INT_to_REAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := INT_to_REAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := INT_to_REAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

#[test]
fn int_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 1;
		MIN : INT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := INT_to_LINT(INT#0);
		ret.negative := INT_to_LINT(INT#-11);
		ret.positive := INT_to_LINT(INT#22);
		ret.max_minus_one := INT_to_LINT(MAX-1);
		ret.min_plus_one := INT_to_LINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := INT_to_LINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := INT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, 0i64);
    assert_eq!(maintype.positive, 0i64);
    assert_eq!(maintype.max_minus_one, 0i64);
    assert_eq!(maintype.min_plus_one, 0i64);
    assert_eq!(maintype.max_overflow, 0i64);
    assert_eq!(maintype.min_overflow, 0i64);
}

#[test]
fn int_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 1;
		MIN : INT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := INT_to_DINT(INT#0);
		ret.negative := INT_to_DINT(INT#-11);
		ret.positive := INT_to_DINT(INT#22);
		ret.max_minus_one := INT_to_DINT(MAX-1);
		ret.min_plus_one := INT_to_DINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := INT_to_DINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := INT_to_DINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, 0i32);
    assert_eq!(maintype.positive, 0i32);
    assert_eq!(maintype.max_minus_one, 0i32);
    assert_eq!(maintype.min_plus_one, 0i32);
    assert_eq!(maintype.max_overflow, 0i32);
    assert_eq!(maintype.min_overflow, 0i32);
}

#[test]
fn int_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 1;
		MIN : INT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := INT_to_SINT(INT#0);
		ret.negative := INT_to_SINT(INT#-11);
		ret.positive := INT_to_SINT(INT#22);
		ret.max_minus_one := INT_to_SINT(MAX-1);
		ret.min_plus_one := INT_to_SINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := INT_to_SINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := INT_to_SINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.negative, 0i8);
    assert_eq!(maintype.positive, 0i8);
    assert_eq!(maintype.max_minus_one, 0i8);
    assert_eq!(maintype.min_plus_one, 0i8);
    assert_eq!(maintype.max_overflow, 0i8);
    assert_eq!(maintype.min_overflow, 0i8);
}

#[test]
fn int_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 1;
		MIN : INT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := INT_to_ULINT(INT#0);
		ret.negative := INT_to_ULINT(INT#-1);
		ret.positive := INT_to_ULINT(INT#22);
		ret.max_minus_one := INT_to_ULINT(MAX-1);
		ret.min_plus_one := INT_to_ULINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := INT_to_ULINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := INT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 0u64);
    assert_eq!(maintype.positive, 0u64);
    assert_eq!(maintype.max_minus_one, 0u64);
    assert_eq!(maintype.min_plus_one, 0u64);
    assert_eq!(maintype.max_overflow, 0u64);
    assert_eq!(maintype.min_overflow, 0u64);
}

#[test]
fn int_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 1;
		MIN : INT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := INT_to_UDINT(INT#0);
		ret.negative := INT_to_UDINT(INT#-1);
		ret.positive := INT_to_UDINT(INT#22);
		ret.max_minus_one := INT_to_UDINT(MAX-1);
		ret.min_plus_one := INT_to_UDINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := INT_to_UDINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := INT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 0u32);
    assert_eq!(maintype.positive, 0u32);
    assert_eq!(maintype.max_minus_one, 0u32);
    assert_eq!(maintype.min_plus_one, 0u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 0u32);
}

#[test]
fn int_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 1;
		MIN : INT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := INT_to_UINT(INT#0);
		ret.negative := INT_to_UINT(INT#-1);
		ret.positive := INT_to_UINT(INT#22);
		ret.max_minus_one := INT_to_UINT(MAX-1);
		ret.min_plus_one := INT_to_UINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := INT_to_UINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := INT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 0u16);
    assert_eq!(maintype.positive, 0u16);
    assert_eq!(maintype.max_minus_one, 0u16);
    assert_eq!(maintype.min_plus_one, 0u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 0u16);
}

#[test]
fn int_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 1;
		MIN : INT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := INT_to_USINT(INT#0);
		ret.negative := INT_to_USINT(INT#-1);
		ret.positive := INT_to_USINT(INT#22);
		ret.max_minus_one := INT_to_USINT(MAX-1);
		ret.min_plus_one := INT_to_USINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := INT_to_USINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := INT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 0u8);
    assert_eq!(maintype.positive, 0u8);
    assert_eq!(maintype.max_minus_one, 0u8);
    assert_eq!(maintype.min_plus_one, 0u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 0u8);
}

// #[test]
// fn SINT_to_LREAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : SINT := 1;
// 		MIN : SINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := SINT_to_LREAL(SINT#0);
// 		ret.negative := SINT_to_LREAL(SINT#-11);
// 		ret.positive := SINT_to_LREAL(SINT#22);
// 		ret.max_minus_one := SINT_to_LREAL(MAX-1);
// 		ret.min_plus_one := SINT_to_LREAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := SINT_to_LREAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := SINT_to_LREAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn SINT_to_REAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : SINT := 1;
// 		MIN : SINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := SINT_to_REAL(SINT#0);
// 		ret.negative := SINT_to_REAL(SINT#-11);
// 		ret.positive := SINT_to_REAL(SINT#22);
// 		ret.max_minus_one := SINT_to_REAL(MAX-1);
// 		ret.min_plus_one := SINT_to_REAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := SINT_to_REAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := SINT_to_REAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

#[test]
fn sint_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 1;
		MIN : SINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := SINT_to_LINT(SINT#0);
		ret.negative := SINT_to_LINT(SINT#-11);
		ret.positive := SINT_to_LINT(SINT#22);
		ret.max_minus_one := SINT_to_LINT(MAX-1);
		ret.min_plus_one := SINT_to_LINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := SINT_to_LINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := SINT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, 0i64);
    assert_eq!(maintype.positive, 0i64);
    assert_eq!(maintype.max_minus_one, 0i64);
    assert_eq!(maintype.min_plus_one, 0i64);
    assert_eq!(maintype.max_overflow, 0i64);
    assert_eq!(maintype.min_overflow, 0i64);
}

#[test]
fn sint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 1;
		MIN : SINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := SINT_to_DINT(SINT#0);
		ret.negative := SINT_to_DINT(SINT#-11);
		ret.positive := SINT_to_DINT(SINT#22);
		ret.max_minus_one := SINT_to_DINT(MAX-1);
		ret.min_plus_one := SINT_to_DINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := SINT_to_DINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := SINT_to_DINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, 0i32);
    assert_eq!(maintype.positive, 0i32);
    assert_eq!(maintype.max_minus_one, 0i32);
    assert_eq!(maintype.min_plus_one, 0i32);
    assert_eq!(maintype.max_overflow, 0i32);
    assert_eq!(maintype.min_overflow, 0i32);
}

#[test]
fn sint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 1;
		MIN : SINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := SINT_to_INT(SINT#0);
		ret.negative := SINT_to_INT(SINT#-11);
		ret.positive := SINT_to_INT(SINT#22);
		ret.max_minus_one := SINT_to_INT(MAX-1);
		ret.min_plus_one := SINT_to_INT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := SINT_to_INT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := SINT_to_INT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.negative, 0i16);
    assert_eq!(maintype.positive, 0i16);
    assert_eq!(maintype.max_minus_one, 0i16);
    assert_eq!(maintype.min_plus_one, 0i16);
    assert_eq!(maintype.max_overflow, 0i16);
    assert_eq!(maintype.min_overflow, 0i16);
}

#[test]
fn sint_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 1;
		MIN : SINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := SINT_to_ULINT(SINT#0);
		ret.negative := SINT_to_ULINT(SINT#-1);
		ret.positive := SINT_to_ULINT(SINT#22);
		ret.max_minus_one := SINT_to_ULINT(MAX-1);
		ret.min_plus_one := SINT_to_ULINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := SINT_to_ULINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := SINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 0u64);
    assert_eq!(maintype.positive, 0u64);
    assert_eq!(maintype.max_minus_one, 0u64);
    assert_eq!(maintype.min_plus_one, 0u64);
    assert_eq!(maintype.max_overflow, 0u64);
    assert_eq!(maintype.min_overflow, 0u64);
}

#[test]
fn sint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 1;
		MIN : SINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := SINT_to_UDINT(SINT#0);
		ret.negative := SINT_to_UDINT(SINT#-1);
		ret.positive := SINT_to_UDINT(SINT#22);
		ret.max_minus_one := SINT_to_UDINT(MAX-1);
		ret.min_plus_one := SINT_to_UDINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := SINT_to_UDINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := SINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 0u32);
    assert_eq!(maintype.positive, 0u32);
    assert_eq!(maintype.max_minus_one, 0u32);
    assert_eq!(maintype.min_plus_one, 0u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 0u32);
}

#[test]
fn sint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 1;
		MIN : SINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := SINT_to_UINT(SINT#0);
		ret.negative := SINT_to_UINT(SINT#-1);
		ret.positive := SINT_to_UINT(SINT#22);
		ret.max_minus_one := SINT_to_UINT(MAX-1);
		ret.min_plus_one := SINT_to_UINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := SINT_to_UINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := SINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 0u16);
    assert_eq!(maintype.positive, 0u16);
    assert_eq!(maintype.max_minus_one, 0u16);
    assert_eq!(maintype.min_plus_one, 0u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 0u16);
}

#[test]
fn sint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 1;
		MIN : SINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := SINT_to_USINT(SINT#0);
		ret.negative := SINT_to_USINT(SINT#-1);
		ret.positive := SINT_to_USINT(SINT#22);
		ret.max_minus_one := SINT_to_USINT(MAX-1);
		ret.min_plus_one := SINT_to_USINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := SINT_to_USINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := SINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 0u8);
    assert_eq!(maintype.positive, 0u8);
    assert_eq!(maintype.max_minus_one, 0u8);
    assert_eq!(maintype.min_plus_one, 0u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 0u8);
}

// #[test]
// fn ULINT_to_LREAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : ULINT := 1;
// 		MIN : ULINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := ULINT_to_LREAL(ULINT#0);
// 		ret.negative := ULINT_to_LREAL(ULINT#-11);
// 		ret.positive := ULINT_to_LREAL(ULINT#22);
// 		ret.max_minus_one := ULINT_to_LREAL(MAX-1);
// 		ret.min_plus_one := ULINT_to_LREAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := ULINT_to_LREAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := ULINT_to_LREAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn ULINT_to_REAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : ULINT := 1;
// 		MIN : ULINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := ULINT_to_REAL(ULINT#0);
// 		ret.negative := ULINT_to_REAL(ULINT#-11);
// 		ret.positive := ULINT_to_REAL(ULINT#22);
// 		ret.max_minus_one := ULINT_to_REAL(MAX-1);
// 		ret.min_plus_one := ULINT_to_REAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := ULINT_to_REAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := ULINT_to_REAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

#[test]
fn ulint_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 1;
		MIN : ULINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_to_LINT(ULINT#0);
		ret.negative := ULINT_to_LINT(ULINT#-1);
		ret.positive := ULINT_to_LINT(ULINT#22);
		ret.max_minus_one := ULINT_to_LINT(MAX-1);
		ret.min_plus_one := ULINT_to_LINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := ULINT_to_LINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := ULINT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, 0i64);
    assert_eq!(maintype.positive, 0i64);
    assert_eq!(maintype.max_minus_one, 0i64);
    assert_eq!(maintype.min_plus_one, 0i64);
    assert_eq!(maintype.max_overflow, 0i64);
    assert_eq!(maintype.min_overflow, 0i64);
}

#[test]
fn ulint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 1;
		MIN : ULINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_to_DINT(ULINT#0);
		ret.negative := ULINT_to_DINT(ULINT#-1);
		ret.positive := ULINT_to_DINT(ULINT#22);
		ret.max_minus_one := ULINT_to_DINT(MAX-1);
		ret.min_plus_one := ULINT_to_DINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := ULINT_to_DINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := ULINT_to_DINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, 0i32);
    assert_eq!(maintype.positive, 0i32);
    assert_eq!(maintype.max_minus_one, 0i32);
    assert_eq!(maintype.min_plus_one, 0i32);
    assert_eq!(maintype.max_overflow, 0i32);
    assert_eq!(maintype.min_overflow, 0i32);
}

#[test]
fn ulint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 1;
		MIN : ULINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_to_INT(ULINT#0);
		ret.negative := ULINT_to_INT(ULINT#-1);
		ret.positive := ULINT_to_INT(ULINT#22);
		ret.max_minus_one := ULINT_to_INT(MAX-1);
		ret.min_plus_one := ULINT_to_INT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := ULINT_to_INT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := ULINT_to_INT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.negative, 0i16);
    assert_eq!(maintype.positive, 0i16);
    assert_eq!(maintype.max_minus_one, 0i16);
    assert_eq!(maintype.min_plus_one, 0i16);
    assert_eq!(maintype.max_overflow, 0i16);
    assert_eq!(maintype.min_overflow, 0i16);
}

#[test]
fn ulint_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 1;
		MIN : ULINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_to_SINT(ULINT#0);
		ret.negative := ULINT_to_SINT(ULINT#-1);
		ret.positive := ULINT_to_SINT(ULINT#22);
		ret.max_minus_one := ULINT_to_SINT(MAX-1);
		ret.min_plus_one := ULINT_to_SINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := ULINT_to_SINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := ULINT_to_SINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.negative, 0i8);
    assert_eq!(maintype.positive, 0i8);
    assert_eq!(maintype.max_minus_one, 0i8);
    assert_eq!(maintype.min_plus_one, 0i8);
    assert_eq!(maintype.max_overflow, 0i8);
    assert_eq!(maintype.min_overflow, 0i8);
}

#[test]
fn ulint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 1;
		MIN : ULINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_to_UDINT(ULINT#0);
		ret.negative := ULINT_to_UDINT(ULINT#-1);
		ret.positive := ULINT_to_UDINT(ULINT#22);
		ret.max_minus_one := ULINT_to_UDINT(MAX-1);
		ret.min_plus_one := ULINT_to_UDINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := ULINT_to_UDINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := ULINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 0u32);
    assert_eq!(maintype.positive, 0u32);
    assert_eq!(maintype.max_minus_one, 0u32);
    assert_eq!(maintype.min_plus_one, 0u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 0u32);
}

#[test]
fn ulint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 1;
		MIN : ULINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_to_UINT(ULINT#0);
		ret.negative := ULINT_to_UINT(ULINT#-1);
		ret.positive := ULINT_to_UINT(ULINT#22);
		ret.max_minus_one := ULINT_to_UINT(MAX-1);
		ret.min_plus_one := ULINT_to_UINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := ULINT_to_UINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := ULINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 0u16);
    assert_eq!(maintype.positive, 0u16);
    assert_eq!(maintype.max_minus_one, 0u16);
    assert_eq!(maintype.min_plus_one, 0u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 0u16);
}

#[test]
fn ulint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 1;
		MIN : ULINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_to_USINT(ULINT#0);
		ret.negative := ULINT_to_USINT(ULINT#-1);
		ret.positive := ULINT_to_USINT(ULINT#22);
		ret.max_minus_one := ULINT_to_USINT(MAX-1);
		ret.min_plus_one := ULINT_to_USINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := ULINT_to_USINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := ULINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 0u8);
    assert_eq!(maintype.positive, 0u8);
    assert_eq!(maintype.max_minus_one, 0u8);
    assert_eq!(maintype.min_plus_one, 0u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 0u8);
}

// #[test]
// fn UDINT_to_LREAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : UDINT := 1;
// 		MIN : UDINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := UDINT_to_LREAL(UDINT#0);
// 		ret.negative := UDINT_to_LREAL(UDINT#-11);
// 		ret.positive := UDINT_to_LREAL(UDINT#22);
// 		ret.max_minus_one := UDINT_to_LREAL(MAX-1);
// 		ret.min_plus_one := UDINT_to_LREAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := UDINT_to_LREAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := UDINT_to_LREAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn UDINT_to_REAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : UDINT := 1;
// 		MIN : UDINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := UDINT_to_REAL(UDINT#0);
// 		ret.negative := UDINT_to_REAL(UDINT#-11);
// 		ret.positive := UDINT_to_REAL(UDINT#22);
// 		ret.max_minus_one := UDINT_to_REAL(MAX-1);
// 		ret.min_plus_one := UDINT_to_REAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := UDINT_to_REAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := UDINT_to_REAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

#[test]
fn udint_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 1;
		MIN : UDINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_to_LINT(UDINT#0);
		ret.negative := UDINT_to_LINT(UDINT#-1);
		ret.positive := UDINT_to_LINT(UDINT#22);
		ret.max_minus_one := UDINT_to_LINT(MAX-1);
		ret.min_plus_one := UDINT_to_LINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UDINT_to_LINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UDINT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, 0i64);
    assert_eq!(maintype.positive, 0i64);
    assert_eq!(maintype.max_minus_one, 0i64);
    assert_eq!(maintype.min_plus_one, 0i64);
    assert_eq!(maintype.max_overflow, 0i64);
    assert_eq!(maintype.min_overflow, 0i64);
}

#[test]
fn udint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 1;
		MIN : UDINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_to_DINT(UDINT#0);
		ret.negative := UDINT_to_DINT(UDINT#-1);
		ret.positive := UDINT_to_DINT(UDINT#22);
		ret.max_minus_one := UDINT_to_DINT(MAX-1);
		ret.min_plus_one := UDINT_to_DINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UDINT_to_DINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UDINT_to_DINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, 0i32);
    assert_eq!(maintype.positive, 0i32);
    assert_eq!(maintype.max_minus_one, 0i32);
    assert_eq!(maintype.min_plus_one, 0i32);
    assert_eq!(maintype.max_overflow, 0i32);
    assert_eq!(maintype.min_overflow, 0i32);
}

#[test]
fn udint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 1;
		MIN : UDINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_to_INT(UDINT#0);
		ret.negative := UDINT_to_INT(UDINT#-1);
		ret.positive := UDINT_to_INT(UDINT#22);
		ret.max_minus_one := UDINT_to_INT(MAX-1);
		ret.min_plus_one := UDINT_to_INT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UDINT_to_INT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UDINT_to_INT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.negative, 0i16);
    assert_eq!(maintype.positive, 0i16);
    assert_eq!(maintype.max_minus_one, 0i16);
    assert_eq!(maintype.min_plus_one, 0i16);
    assert_eq!(maintype.max_overflow, 0i16);
    assert_eq!(maintype.min_overflow, 0i16);
}

#[test]
fn udint_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 1;
		MIN : UDINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_to_SINT(UDINT#0);
		ret.negative := UDINT_to_SINT(UDINT#-1);
		ret.positive := UDINT_to_SINT(UDINT#22);
		ret.max_minus_one := UDINT_to_SINT(MAX-1);
		ret.min_plus_one := UDINT_to_SINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UDINT_to_SINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UDINT_to_SINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.negative, 0i8);
    assert_eq!(maintype.positive, 0i8);
    assert_eq!(maintype.max_minus_one, 0i8);
    assert_eq!(maintype.min_plus_one, 0i8);
    assert_eq!(maintype.max_overflow, 0i8);
    assert_eq!(maintype.min_overflow, 0i8);
}

#[test]
fn udint_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 1;
		MIN : UDINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_to_ULINT(UDINT#0);
		ret.negative := UDINT_to_ULINT(UDINT#-1);
		ret.positive := UDINT_to_ULINT(UDINT#22);
		ret.max_minus_one := UDINT_to_ULINT(MAX-1);
		ret.min_plus_one := UDINT_to_ULINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UDINT_to_ULINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UDINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 0u64);
    assert_eq!(maintype.positive, 0u64);
    assert_eq!(maintype.max_minus_one, 0u64);
    assert_eq!(maintype.min_plus_one, 0u64);
    assert_eq!(maintype.max_overflow, 0u64);
    assert_eq!(maintype.min_overflow, 0u64);
}

#[test]
fn udint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 1;
		MIN : UDINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_to_UINT(UDINT#0);
		ret.negative := UDINT_to_UINT(UDINT#-1);
		ret.positive := UDINT_to_UINT(UDINT#22);
		ret.max_minus_one := UDINT_to_UINT(MAX-1);
		ret.min_plus_one := UDINT_to_UINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UDINT_to_UINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UDINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 0u16);
    assert_eq!(maintype.positive, 0u16);
    assert_eq!(maintype.max_minus_one, 0u16);
    assert_eq!(maintype.min_plus_one, 0u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 0u16);
}

#[test]
fn udint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 1;
		MIN : UDINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_to_USINT(UDINT#0);
		ret.negative := UDINT_to_USINT(UDINT#-1);
		ret.positive := UDINT_to_USINT(UDINT#22);
		ret.max_minus_one := UDINT_to_USINT(MAX-1);
		ret.min_plus_one := UDINT_to_USINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UDINT_to_USINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UDINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 0u8);
    assert_eq!(maintype.positive, 0u8);
    assert_eq!(maintype.max_minus_one, 0u8);
    assert_eq!(maintype.min_plus_one, 0u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 0u8);
}

// #[test]
// fn UINT_to_LREAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : UINT := 1;
// 		MIN : UINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := UINT_to_LREAL(UINT#0);
// 		ret.negative := UINT_to_LREAL(UINT#-11);
// 		ret.positive := UINT_to_LREAL(UINT#22);
// 		ret.max_minus_one := UINT_to_LREAL(MAX-1);
// 		ret.min_plus_one := UINT_to_LREAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := UINT_to_LREAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := UINT_to_LREAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn UINT_to_REAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : UINT := 1;
// 		MIN : UINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := UINT_to_REAL(UINT#0);
// 		ret.negative := UINT_to_REAL(UINT#-11);
// 		ret.positive := UINT_to_REAL(UINT#22);
// 		ret.max_minus_one := UINT_to_REAL(MAX-1);
// 		ret.min_plus_one := UINT_to_REAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := UINT_to_REAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := UINT_to_REAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

#[test]
fn uint_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 1;
		MIN : UINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_to_LINT(UINT#0);
		ret.negative := UINT_to_LINT(UINT#-1);
		ret.positive := UINT_to_LINT(UINT#22);
		ret.max_minus_one := UINT_to_LINT(MAX-1);
		ret.min_plus_one := UINT_to_LINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UINT_to_LINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UINT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, 0i64);
    assert_eq!(maintype.positive, 0i64);
    assert_eq!(maintype.max_minus_one, 0i64);
    assert_eq!(maintype.min_plus_one, 0i64);
    assert_eq!(maintype.max_overflow, 0i64);
    assert_eq!(maintype.min_overflow, 0i64);
}

#[test]
fn uint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 1;
		MIN : UINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_to_DINT(UINT#0);
		ret.negative := UINT_to_DINT(UINT#-1);
		ret.positive := UINT_to_DINT(UINT#22);
		ret.max_minus_one := UINT_to_DINT(MAX-1);
		ret.min_plus_one := UINT_to_DINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UINT_to_DINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UINT_to_DINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, 0i32);
    assert_eq!(maintype.positive, 0i32);
    assert_eq!(maintype.max_minus_one, 0i32);
    assert_eq!(maintype.min_plus_one, 0i32);
    assert_eq!(maintype.max_overflow, 0i32);
    assert_eq!(maintype.min_overflow, 0i32);
}

#[test]
fn uint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 1;
		MIN : UINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_to_INT(UINT#0);
		ret.negative := UINT_to_INT(UINT#-1);
		ret.positive := UINT_to_INT(UINT#22);
		ret.max_minus_one := UINT_to_INT(MAX-1);
		ret.min_plus_one := UINT_to_INT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UINT_to_INT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UINT_to_INT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.negative, 0i16);
    assert_eq!(maintype.positive, 0i16);
    assert_eq!(maintype.max_minus_one, 0i16);
    assert_eq!(maintype.min_plus_one, 0i16);
    assert_eq!(maintype.max_overflow, 0i16);
    assert_eq!(maintype.min_overflow, 0i16);
}

#[test]
fn uint_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 1;
		MIN : UINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_to_SINT(UINT#0);
		ret.negative := UINT_to_SINT(UINT#-1);
		ret.positive := UINT_to_SINT(UINT#22);
		ret.max_minus_one := UINT_to_SINT(MAX-1);
		ret.min_plus_one := UINT_to_SINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UINT_to_SINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UINT_to_SINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.negative, 0i8);
    assert_eq!(maintype.positive, 0i8);
    assert_eq!(maintype.max_minus_one, 0i8);
    assert_eq!(maintype.min_plus_one, 0i8);
    assert_eq!(maintype.max_overflow, 0i8);
    assert_eq!(maintype.min_overflow, 0i8);
}

#[test]
fn uint_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 1;
		MIN : UINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_to_ULINT(UINT#0);
		ret.negative := UINT_to_ULINT(UINT#-1);
		ret.positive := UINT_to_ULINT(UINT#22);
		ret.max_minus_one := UINT_to_ULINT(MAX-1);
		ret.min_plus_one := UINT_to_ULINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UINT_to_ULINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 0u64);
    assert_eq!(maintype.positive, 0u64);
    assert_eq!(maintype.max_minus_one, 0u64);
    assert_eq!(maintype.min_plus_one, 0u64);
    assert_eq!(maintype.max_overflow, 0u64);
    assert_eq!(maintype.min_overflow, 0u64);
}

#[test]
fn uint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 1;
		MIN : UINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_to_UDINT(UINT#0);
		ret.negative := UINT_to_UDINT(UINT#-1);
		ret.positive := UINT_to_UDINT(UINT#22);
		ret.max_minus_one := UINT_to_UDINT(MAX-1);
		ret.min_plus_one := UINT_to_UDINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UINT_to_UDINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 0u32);
    assert_eq!(maintype.positive, 0u32);
    assert_eq!(maintype.max_minus_one, 0u32);
    assert_eq!(maintype.min_plus_one, 0u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 0u32);
}

#[test]
fn uint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 1;
		MIN : UINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_to_USINT(UINT#0);
		ret.negative := UINT_to_USINT(UINT#-1);
		ret.positive := UINT_to_USINT(UINT#22);
		ret.max_minus_one := UINT_to_USINT(MAX-1);
		ret.min_plus_one := UINT_to_USINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := UINT_to_USINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := UINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 0u8);
    assert_eq!(maintype.positive, 0u8);
    assert_eq!(maintype.max_minus_one, 0u8);
    assert_eq!(maintype.min_plus_one, 0u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 0u8);
}

// #[test]
// fn USINT_to_LREAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : USINT := 1;
// 		MIN : USINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := USINT_to_LREAL(USINT#0);
// 		ret.negative := USINT_to_LREAL(USINT#-11);
// 		ret.positive := USINT_to_LREAL(USINT#22);
// 		ret.max_minus_one := USINT_to_LREAL(MAX-1);
// 		ret.min_plus_one := USINT_to_LREAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := USINT_to_LREAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := USINT_to_LREAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

// #[test]
// fn USINT_to_REAL_conversion() {
//     let src = r"
// 	TYPE myType : STRUCT
// 		zero : DINT; negative : DINT; positive : DINT;
// 		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
// 	END_STRUCT END_TYPE

// 	VAR_GLOBAL
// 		MAX : USINT := 1;
// 		MIN : USINT := 1;
// 	END_VAR

// 	PROGRAM main
// 	VAR
// 		ret : myType;
// 	END_VAR
// 		ret.zero := USINT_to_REAL(USINT#0);
// 		ret.negative := USINT_to_REAL(USINT#-11);
// 		ret.positive := USINT_to_REAL(USINT#22);
// 		ret.max_minus_one := USINT_to_REAL(MAX-1);
// 		ret.min_plus_one := USINT_to_REAL(MIN+1);
// 		// overflow DINT max = MAX by 1
// 		ret.max_overflow := USINT_to_REAL(MAX+1);
// 		// overflow DINT min = MIN by 1
// 		ret.min_overflow := USINT_to_REAL(MIN-1);
//     END_PROGRAM
//         ";
//     let sources = add_std!(src, "num_conversion.st");
//     let mut maintype = TYPE::default();
//     let _res: TYPE = compile_and_run(sources, &mut maintype);
//     assert_eq!(maintype.zero, 0TYPE);
//     assert_eq!(maintype.negative, 0TYPE);
//     assert_eq!(maintype.positive, 0TYPE);
//     assert_eq!(maintype.max_minus_one, 0TYPE);
//     assert_eq!(maintype.min_plus_one, 0TYPE);
//     assert_eq!(maintype.max_overflow, 0TYPE);
//     assert_eq!(maintype.min_overflow, 0TYPE);
// }

#[test]
fn usint_to_lint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 1;
		MIN : USINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_to_LINT(USINT#0);
		ret.negative := USINT_to_LINT(USINT#-1);
		ret.positive := USINT_to_LINT(USINT#22);
		ret.max_minus_one := USINT_to_LINT(MAX-1);
		ret.min_plus_one := USINT_to_LINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := USINT_to_LINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := USINT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, 0i64);
    assert_eq!(maintype.positive, 0i64);
    assert_eq!(maintype.max_minus_one, 0i64);
    assert_eq!(maintype.min_plus_one, 0i64);
    assert_eq!(maintype.max_overflow, 0i64);
    assert_eq!(maintype.min_overflow, 0i64);
}

#[test]
fn usint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 1;
		MIN : USINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_to_DINT(USINT#0);
		ret.negative := USINT_to_DINT(USINT#-1);
		ret.positive := USINT_to_DINT(USINT#22);
		ret.max_minus_one := USINT_to_DINT(MAX-1);
		ret.min_plus_one := USINT_to_DINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := USINT_to_DINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := USINT_to_DINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, 0i32);
    assert_eq!(maintype.positive, 0i32);
    assert_eq!(maintype.max_minus_one, 0i32);
    assert_eq!(maintype.min_plus_one, 0i32);
    assert_eq!(maintype.max_overflow, 0i32);
    assert_eq!(maintype.min_overflow, 0i32);
}

#[test]
fn usint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 1;
		MIN : USINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_to_INT(USINT#0);
		ret.negative := USINT_to_INT(USINT#-1);
		ret.positive := USINT_to_INT(USINT#22);
		ret.max_minus_one := USINT_to_INT(MAX-1);
		ret.min_plus_one := USINT_to_INT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := USINT_to_INT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := USINT_to_INT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.negative, 0i16);
    assert_eq!(maintype.positive, 0i16);
    assert_eq!(maintype.max_minus_one, 0i16);
    assert_eq!(maintype.min_plus_one, 0i16);
    assert_eq!(maintype.max_overflow, 0i16);
    assert_eq!(maintype.min_overflow, 0i16);
}

#[test]
fn usint_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 1;
		MIN : USINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_to_SINT(USINT#0);
		ret.negative := USINT_to_SINT(USINT#-1);
		ret.positive := USINT_to_SINT(USINT#22);
		ret.max_minus_one := USINT_to_SINT(MAX-1);
		ret.min_plus_one := USINT_to_SINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := USINT_to_SINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := USINT_to_SINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.negative, 0i8);
    assert_eq!(maintype.positive, 0i8);
    assert_eq!(maintype.max_minus_one, 0i8);
    assert_eq!(maintype.min_plus_one, 0i8);
    assert_eq!(maintype.max_overflow, 0i8);
    assert_eq!(maintype.min_overflow, 0i8);
}

#[test]
fn usint_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 1;
		MIN : USINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_to_ULINT(USINT#0);
		ret.negative := USINT_to_ULINT(USINT#-1);
		ret.positive := USINT_to_ULINT(USINT#22);
		ret.max_minus_one := USINT_to_ULINT(MAX-1);
		ret.min_plus_one := USINT_to_ULINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := USINT_to_ULINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := USINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 0u64);
    assert_eq!(maintype.positive, 0u64);
    assert_eq!(maintype.max_minus_one, 0u64);
    assert_eq!(maintype.min_plus_one, 0u64);
    assert_eq!(maintype.max_overflow, 0u64);
    assert_eq!(maintype.min_overflow, 0u64);
}

#[test]
fn usint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 1;
		MIN : USINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_to_UDINT(USINT#0);
		ret.negative := USINT_to_UDINT(USINT#-1);
		ret.positive := USINT_to_UDINT(USINT#22);
		ret.max_minus_one := USINT_to_UDINT(MAX-1);
		ret.min_plus_one := USINT_to_UDINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := USINT_to_UDINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := USINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 0u32);
    assert_eq!(maintype.positive, 0u32);
    assert_eq!(maintype.max_minus_one, 0u32);
    assert_eq!(maintype.min_plus_one, 0u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 0u32);
}

#[test]
fn usint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 1;
		MIN : USINT := 1;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_to_UINT(USINT#0);
		ret.negative := USINT_to_UINT(USINT#-1);
		ret.positive := USINT_to_UINT(USINT#22);
		ret.max_minus_one := USINT_to_UINT(MAX-1);
		ret.min_plus_one := USINT_to_UINT(MIN+1);
		// overflow DINT max = MAX by 1
		ret.max_overflow := USINT_to_UINT(MAX+1);
		// overflow DINT min = MIN by 1
		ret.min_overflow := USINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 0u16);
    assert_eq!(maintype.positive, 0u16);
    assert_eq!(maintype.max_minus_one, 0u16);
    assert_eq!(maintype.min_plus_one, 0u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 0u16);
}
