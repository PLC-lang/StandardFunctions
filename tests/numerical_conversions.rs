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
		ret.max_overflow := LINT_to_DINT(MAX+1);
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
		zero : INT; negative : INT; positive : INT;
		max_minus_one : INT; min_plus_one : INT; max_overflow : INT; min_overflow : INT;
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
		ret.max_overflow := LINT_to_INT(MAX+1);
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
		zero : SINT; negative : SINT; positive : SINT;
		max_minus_one : SINT; min_plus_one : SINT; max_overflow : SINT; min_overflow : SINT;
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
		ret.max_overflow := LINT_to_SINT(MAX+1);
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
		zero : ULINT; negative : ULINT; positive : ULINT;
		max_minus_one : ULINT; min_plus_one : ULINT; max_overflow : ULINT; min_overflow : ULINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : LINT := 9223372036854775807;
		MIN : LINT := -9223372036854775808;
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
		ret.max_overflow := LINT_to_ULINT(MAX+1);
		ret.min_overflow := LINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 18446744073709551615u64);
    assert_eq!(maintype.positive, 22u64);
    assert_eq!(maintype.max_minus_one, 9223372036854775806u64);
    assert_eq!(maintype.min_plus_one, 9223372036854775809u64);
    assert_eq!(maintype.max_overflow, 9223372036854775808u64);
    assert_eq!(maintype.min_overflow, 9223372036854775807u64);
}

#[test]
fn lint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT; negative : UDINT; positive : UDINT;
		max_minus_one : UDINT; min_plus_one : UDINT; max_overflow : UDINT; min_overflow : UDINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : LINT := 4294967295;
		MIN : LINT := 0;
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
		ret.max_overflow := LINT_to_UDINT(MAX+1);
		ret.min_overflow := LINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 4294967295u32);
    assert_eq!(maintype.positive, 22u32);
    assert_eq!(maintype.max_minus_one, 4294967294u32);
    assert_eq!(maintype.min_plus_one, 1u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 4294967295u32);
}

#[test]
fn lint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT; negative : UINT; positive : UINT;
		max_minus_one : UINT; min_plus_one : UINT; max_overflow : UINT; min_overflow : UINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : LINT := 65535;
		MIN : LINT := 0;
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
		ret.max_overflow := LINT_to_UINT(MAX+1);
		ret.min_overflow := LINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 65535u16);
    assert_eq!(maintype.positive, 22u16);
    assert_eq!(maintype.max_minus_one, 65534u16);
    assert_eq!(maintype.min_plus_one, 1u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 65535u16);
}

#[test]
fn lint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT; negative : USINT; positive : USINT;
		max_minus_one : USINT; min_plus_one : USINT; max_overflow : USINT; min_overflow : USINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : LINT := 255;
		MIN : LINT := 0;
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
		ret.max_overflow := LINT_to_USINT(MAX+1);
		ret.min_overflow := LINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 255u8);
    assert_eq!(maintype.positive, 22u8);
    assert_eq!(maintype.max_minus_one, 254u8);
    assert_eq!(maintype.min_plus_one, 1u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 255u8);
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
		zero : LINT; negative : LINT; positive : LINT;
		max_minus_one : LINT; min_plus_one : LINT; max_overflow : LINT; min_overflow : LINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 2147483647;
		MIN : DINT := -2147483648;
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
		ret.max_overflow := DINT_to_LINT(MAX+1);
		ret.min_overflow := DINT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, -11i64);
    assert_eq!(maintype.positive, 22i64);
    assert_eq!(maintype.max_minus_one, 2147483646i64);
    assert_eq!(maintype.min_plus_one, -2147483647i64);
    assert_eq!(maintype.max_overflow, -2147483648i64);
    assert_eq!(maintype.min_overflow, 2147483647i64);
}

#[test]
fn dint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : INT; negative : INT; positive : INT;
		max_minus_one : INT; min_plus_one : INT; max_overflow : INT; min_overflow : INT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 32767;
		MIN : DINT := -32768;
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
		ret.max_overflow := DINT_to_INT(MAX+1);
		ret.min_overflow := DINT_to_INT(MIN-1);
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
fn dint_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : SINT; negative : SINT; positive : SINT;
		max_minus_one : SINT; min_plus_one : SINT; max_overflow : SINT; min_overflow : SINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 127;
		MIN : DINT := -128;
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
		ret.max_overflow := DINT_to_SINT(MAX+1);
		ret.min_overflow := DINT_to_SINT(MIN-1);
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
fn dint_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : ULINT; negative : ULINT; positive : ULINT;
		max_minus_one : ULINT; min_plus_one : ULINT; max_overflow : ULINT; min_overflow : ULINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 2147483647;
		MIN : DINT := -2147483648;
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
		ret.max_overflow := DINT_to_ULINT(MAX+1);
		ret.min_overflow := DINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 18446744073709551615u64);
    assert_eq!(maintype.positive, 22u64);
    assert_eq!(maintype.max_minus_one, 2147483646u64);
    assert_eq!(maintype.min_plus_one, 18446744071562067969u64);
    assert_eq!(maintype.max_overflow, 18446744071562067968u64);
    assert_eq!(maintype.min_overflow, 2147483647u64);
}

#[test]
fn dint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT; negative : UDINT; positive : UDINT;
		max_minus_one : UDINT; min_plus_one : UDINT; max_overflow : UDINT; min_overflow : UDINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 2147483647;
		MIN : DINT := -2147483648;
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
		ret.max_overflow := DINT_to_UDINT(MAX+1);
		ret.min_overflow := DINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 4294967295u32);
    assert_eq!(maintype.positive, 22u32);
    assert_eq!(maintype.max_minus_one, 2147483646u32);
    assert_eq!(maintype.min_plus_one, 2147483649u32);
    assert_eq!(maintype.max_overflow, 2147483648u32);
    assert_eq!(maintype.min_overflow, 2147483647u32);
}

#[test]
fn dint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT; negative : UINT; positive : UINT;
		max_minus_one : UINT; min_plus_one : UINT; max_overflow : UINT; min_overflow : UINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 65535;
		MIN : DINT := 0;
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
		ret.max_overflow := DINT_to_UINT(MAX+1);
		ret.min_overflow := DINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 65535u16);
    assert_eq!(maintype.positive, 22u16);
    assert_eq!(maintype.max_minus_one, 65534u16);
    assert_eq!(maintype.min_plus_one, 1u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 65535u16);
}

#[test]
fn dint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT; negative : USINT; positive : USINT;
		max_minus_one : USINT; min_plus_one : USINT; max_overflow : USINT; min_overflow : USINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : DINT := 255;
		MIN : DINT := 0;
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
		ret.max_overflow := DINT_to_USINT(MAX+1);
		ret.min_overflow := DINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 255u8);
    assert_eq!(maintype.positive, 22u8);
    assert_eq!(maintype.max_minus_one, 254u8);
    assert_eq!(maintype.min_plus_one, 1u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 255u8);
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
		zero : LINT; negative : LINT; positive : LINT;
		max_minus_one : LINT; min_plus_one : LINT; max_overflow : LINT; min_overflow : LINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 32767;
		MIN : INT := -32768;
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
		ret.max_overflow := INT_to_LINT(MAX+1);
		ret.min_overflow := INT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, -11i64);
    assert_eq!(maintype.positive, 22i64);
    assert_eq!(maintype.max_minus_one, 32766i64);
    assert_eq!(maintype.min_plus_one, -32767i64);
    assert_eq!(maintype.max_overflow, -32768i64);
    assert_eq!(maintype.min_overflow, 32767i64);
}

#[test]
fn int_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 32767;
		MIN : INT := -32768;
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
		ret.max_overflow := INT_to_DINT(MAX+1);
		ret.min_overflow := INT_to_DINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, -11i32);
    assert_eq!(maintype.positive, 22i32);
    assert_eq!(maintype.max_minus_one, 32766i32);
    assert_eq!(maintype.min_plus_one, -32767i32);
    assert_eq!(maintype.max_overflow, -32768i32);
    assert_eq!(maintype.min_overflow, 32767i32);
}

#[test]
fn int_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : SINT; negative : SINT; positive : SINT;
		max_minus_one : SINT; min_plus_one : SINT; max_overflow : SINT; min_overflow : SINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 127;
		MIN : INT := -128;
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
		ret.max_overflow := INT_to_SINT(MAX+1);
		ret.min_overflow := INT_to_SINT(MIN-1);
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
fn int_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : ULINT; negative : ULINT; positive : ULINT;
		max_minus_one : ULINT; min_plus_one : ULINT; max_overflow : ULINT; min_overflow : ULINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 32767;
		MIN : INT := -32768;
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
		ret.max_overflow := INT_to_ULINT(MAX+1);
		ret.min_overflow := INT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 18446744073709551615u64);
    assert_eq!(maintype.positive, 22u64);
    assert_eq!(maintype.max_minus_one, 32766u64);
    assert_eq!(maintype.min_plus_one, 18446744073709518849u64);
    assert_eq!(maintype.max_overflow, 18446744073709518848u64);
    assert_eq!(maintype.min_overflow, 32767u64);
}

#[test]
fn int_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT; negative : UDINT; positive : UDINT;
		max_minus_one : UDINT; min_plus_one : UDINT; max_overflow : UDINT; min_overflow : UDINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 32767;
		MIN : INT := -32768;
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
		ret.max_overflow := INT_to_UDINT(MAX+1);
		ret.min_overflow := INT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 4294967295u32);
    assert_eq!(maintype.positive, 22u32);
    assert_eq!(maintype.max_minus_one, 32766u32);
    assert_eq!(maintype.min_plus_one, 4294934529u32);
    assert_eq!(maintype.max_overflow, 4294934528u32);
    assert_eq!(maintype.min_overflow, 32767u32);
}

#[test]
fn int_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT; negative : UINT; positive : UINT;
		max_minus_one : UINT; min_plus_one : UINT; max_overflow : UINT; min_overflow : UINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 32767;
		MIN : INT := -32768;
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
		ret.max_overflow := INT_to_UINT(MAX+1);
		ret.min_overflow := INT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 65535u16);
    assert_eq!(maintype.positive, 22u16);
    assert_eq!(maintype.max_minus_one, 32766u16);
    assert_eq!(maintype.min_plus_one, 32769u16);
    assert_eq!(maintype.max_overflow, 32768u16);
    assert_eq!(maintype.min_overflow, 32767u16);
}

#[test]
fn int_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT; negative : USINT; positive : USINT;
		max_minus_one : USINT; min_plus_one : USINT; max_overflow : USINT; min_overflow : USINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : INT := 255;
		MIN : INT := 0;
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
		ret.max_overflow := INT_to_USINT(MAX+1);
		ret.min_overflow := INT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 255u8);
    assert_eq!(maintype.positive, 22u8);
    assert_eq!(maintype.max_minus_one, 254u8);
    assert_eq!(maintype.min_plus_one, 1u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 255u8);
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
		zero : LINT; negative : LINT; positive : LINT;
		max_minus_one : LINT; min_plus_one : LINT; max_overflow : LINT; min_overflow : LINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 127;
		MIN : SINT := -128;
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
		ret.max_overflow := SINT_to_LINT(MAX+1);
		ret.min_overflow := SINT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, -11i64);
    assert_eq!(maintype.positive, 22i64);
    assert_eq!(maintype.max_minus_one, 126i64);
    assert_eq!(maintype.min_plus_one, -127i64);
    assert_eq!(maintype.max_overflow, -128i64);
    assert_eq!(maintype.min_overflow, 127i64);
}

#[test]
fn sint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 127;
		MIN : SINT := -128;
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
		ret.max_overflow := SINT_to_DINT(MAX+1);
		ret.min_overflow := SINT_to_DINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, -11i32);
    assert_eq!(maintype.positive, 22i32);
    assert_eq!(maintype.max_minus_one, 126i32);
    assert_eq!(maintype.min_plus_one, -127i32);
    assert_eq!(maintype.max_overflow, -128i32);
    assert_eq!(maintype.min_overflow, 127i32);
}

#[test]
fn sint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : INT; negative : INT; positive : INT;
		max_minus_one : INT; min_plus_one : INT; max_overflow : INT; min_overflow : INT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 127;
		MIN : SINT := -128;
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
		ret.max_overflow := SINT_to_INT(MAX+1);
		ret.min_overflow := SINT_to_INT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.negative, -11i16);
    assert_eq!(maintype.positive, 22i16);
    assert_eq!(maintype.max_minus_one, 126i16);
    assert_eq!(maintype.min_plus_one, -127i16);
    assert_eq!(maintype.max_overflow, -128i16);
    assert_eq!(maintype.min_overflow, 127i16);
}

#[test]
fn sint_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : ULINT; negative : ULINT; positive : ULINT;
		max_minus_one : ULINT; min_plus_one : ULINT; max_overflow : ULINT; min_overflow : ULINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 127;
		MIN : SINT := -128;
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
		ret.max_overflow := SINT_to_ULINT(MAX+1);
		ret.min_overflow := SINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 18446744073709551615u64);
    assert_eq!(maintype.positive, 22u64);
    assert_eq!(maintype.max_minus_one, 126u64);
    assert_eq!(maintype.min_plus_one, 18446744073709551489u64);
    assert_eq!(maintype.max_overflow, 18446744073709551488u64);
    assert_eq!(maintype.min_overflow, 127u64);
}

#[test]
fn sint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT; negative : UDINT; positive : UDINT;
		max_minus_one : UDINT; min_plus_one : UDINT; max_overflow : UDINT; min_overflow : UDINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 127;
		MIN : SINT := -128;
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
		ret.max_overflow := SINT_to_UDINT(MAX+1);
		ret.min_overflow := SINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 4294967295u32);
    assert_eq!(maintype.positive, 22u32);
    assert_eq!(maintype.max_minus_one, 126u32);
    assert_eq!(maintype.min_plus_one, 4294967169u32);
    assert_eq!(maintype.max_overflow, 4294967168u32);
    assert_eq!(maintype.min_overflow, 127u32);
}

#[test]
fn sint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT; negative : UINT; positive : UINT;
		max_minus_one : UINT; min_plus_one : UINT; max_overflow : UINT; min_overflow : UINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 127;
		MIN : SINT := -128;
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
		ret.max_overflow := SINT_to_UINT(MAX+1);
		ret.min_overflow := SINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 65535u16);
    assert_eq!(maintype.positive, 22u16);
    assert_eq!(maintype.max_minus_one, 126u16);
    assert_eq!(maintype.min_plus_one, 65409u16);
    assert_eq!(maintype.max_overflow, 65408u16);
    assert_eq!(maintype.min_overflow, 127u16);
}

#[test]
fn sint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT; negative : USINT; positive : USINT;
		max_minus_one : USINT; min_plus_one : USINT; max_overflow : USINT; min_overflow : USINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : SINT := 127;
		MIN : SINT := -128;
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
		ret.max_overflow := SINT_to_USINT(MAX+1);
		ret.min_overflow := SINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 255u8);
    assert_eq!(maintype.positive, 22u8);
    assert_eq!(maintype.max_minus_one, 126u8);
    assert_eq!(maintype.min_plus_one, 129u8);
    assert_eq!(maintype.max_overflow, 128u8);
    assert_eq!(maintype.min_overflow, 127u8);
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
		zero : LINT; negative : LINT; positive : LINT;
		max_minus_one : LINT; min_plus_one : LINT; max_overflow : LINT; min_overflow : LINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 9223372036854775807;
		MIN : ULINT := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_to_LINT(ULINT#0);
		ret.positive := ULINT_to_LINT(ULINT#22);
		ret.max_minus_one := ULINT_to_LINT(MAX-1);
		ret.min_plus_one := ULINT_to_LINT(MIN+1);
		ret.max_overflow := ULINT_to_LINT(MAX+1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.positive, 22i64);
    assert_eq!(maintype.max_minus_one, 9223372036854775806i64);
    assert_eq!(maintype.min_plus_one, 1i64);
    assert_eq!(maintype.max_overflow, -9223372036854775808i64);
}

#[test]
fn ulint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 2147483647;
		MIN : ULINT := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_to_DINT(ULINT#0);
		ret.positive := ULINT_to_DINT(ULINT#22);
		ret.max_minus_one := ULINT_to_DINT(MAX-1);
		ret.min_plus_one := ULINT_to_DINT(MIN+1);
		ret.max_overflow := ULINT_to_DINT(MAX+1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.positive, 22i32);
    assert_eq!(maintype.max_minus_one, 2147483646i32);
    assert_eq!(maintype.min_plus_one, 1i32);
    assert_eq!(maintype.max_overflow, -2147483648i32);
}

#[test]
fn ulint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : INT; negative : INT; positive : INT;
		max_minus_one : INT; min_plus_one : INT; max_overflow : INT; min_overflow : INT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 32767;
		MIN : ULINT := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_to_INT(ULINT#0);
		ret.positive := ULINT_to_INT(ULINT#22);
		ret.max_minus_one := ULINT_to_INT(MAX-1);
		ret.min_plus_one := ULINT_to_INT(MIN+1);
		ret.max_overflow := ULINT_to_INT(MAX+1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.positive, 22i16);
    assert_eq!(maintype.max_minus_one, 32766i16);
    assert_eq!(maintype.min_plus_one, 1i16);
    assert_eq!(maintype.max_overflow, -32768i16);
}

#[test]
fn ulint_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : SINT; negative : SINT; positive : SINT;
		max_minus_one : SINT; min_plus_one : SINT; max_overflow : SINT; min_overflow : SINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 127;
		MIN : ULINT := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := ULINT_to_SINT(ULINT#0);
		ret.positive := ULINT_to_SINT(ULINT#22);
		ret.max_minus_one := ULINT_to_SINT(MAX-1);
		ret.min_plus_one := ULINT_to_SINT(MIN+1);
		ret.max_overflow := ULINT_to_SINT(MAX+1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.positive, 22i8);
    assert_eq!(maintype.max_minus_one, 126i8);
    assert_eq!(maintype.min_plus_one, 1i8);
    assert_eq!(maintype.max_overflow, -128i8);
}

#[test]
fn ulint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT; negative : UDINT; positive : UDINT;
		max_minus_one : UDINT; min_plus_one : UDINT; max_overflow : UDINT; min_overflow : UDINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 4294967295;
		MIN : ULINT := 0;
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
		ret.max_overflow := ULINT_to_UDINT(MAX+1);
		ret.min_overflow := ULINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 4294967295u32);
    assert_eq!(maintype.positive, 22u32);
    assert_eq!(maintype.max_minus_one, 4294967294u32);
    assert_eq!(maintype.min_plus_one, 1u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 4294967295u32);
}

#[test]
fn ulint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT; negative : UINT; positive : UINT;
		max_minus_one : UINT; min_plus_one : UINT; max_overflow : UINT; min_overflow : UINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 65535;
		MIN : ULINT := 0;
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
		ret.max_overflow := ULINT_to_UINT(MAX+1);
		ret.min_overflow := ULINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 65535u16);
    assert_eq!(maintype.positive, 22u16);
    assert_eq!(maintype.max_minus_one, 65534u16);
    assert_eq!(maintype.min_plus_one, 1u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 65535u16);
}

#[test]
fn ulint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT; negative : USINT; positive : USINT;
		max_minus_one : USINT; min_plus_one : USINT; max_overflow : USINT; min_overflow : USINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : ULINT := 255;
		MIN : ULINT := 0;
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
		ret.max_overflow := ULINT_to_USINT(MAX+1);
		ret.min_overflow := ULINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 255u8);
    assert_eq!(maintype.positive, 22u8);
    assert_eq!(maintype.max_minus_one, 254u8);
    assert_eq!(maintype.min_plus_one, 1u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 255u8);
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
		zero : LINT; negative : LINT; positive : LINT;
		max_minus_one : LINT; min_plus_one : LINT; max_overflow : LINT; min_overflow : LINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 4294967295;
		MIN : UDINT := 0;
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
		ret.max_overflow := UDINT_to_LINT(MAX+1);
		ret.min_overflow := UDINT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, 4294967295i64);
    assert_eq!(maintype.positive, 22i64);
    assert_eq!(maintype.max_minus_one, 4294967294i64);
    assert_eq!(maintype.min_plus_one, 1i64);
    assert_eq!(maintype.max_overflow, 0i64);
    assert_eq!(maintype.min_overflow, 4294967295i64);
}

#[test]
fn udint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 2147483647;
		MIN : UDINT := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_to_DINT(UDINT#0);
		ret.positive := UDINT_to_DINT(UDINT#22);
		ret.max_minus_one := UDINT_to_DINT(MAX-1);
		ret.min_plus_one := UDINT_to_DINT(MIN+1);
		ret.max_overflow := UDINT_to_DINT(MAX+1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.positive, 22i32);
    assert_eq!(maintype.max_minus_one, 2147483646i32);
    assert_eq!(maintype.min_plus_one, 1i32);
    assert_eq!(maintype.max_overflow, -2147483648i32);
}

#[test]
fn udint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : INT; negative : INT; positive : INT;
		max_minus_one : INT; min_plus_one : INT; max_overflow : INT; min_overflow : INT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 32767;
		MIN : UDINT := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_to_INT(UDINT#0);
		ret.positive := UDINT_to_INT(UDINT#22);
		ret.max_minus_one := UDINT_to_INT(MAX-1);
		ret.min_plus_one := UDINT_to_INT(MIN+1);
		ret.max_overflow := UDINT_to_INT(MAX+1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.positive, 22i16);
    assert_eq!(maintype.max_minus_one, 32766i16);
    assert_eq!(maintype.min_plus_one, 1i16);
    assert_eq!(maintype.max_overflow, -32768i16);
}

#[test]
fn udint_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : SINT; negative : SINT; positive : SINT;
		max_minus_one : SINT; min_plus_one : SINT; max_overflow : SINT; min_overflow : SINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 127;
		MIN : UDINT := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UDINT_to_SINT(UDINT#0);
		ret.positive := UDINT_to_SINT(UDINT#22);
		ret.max_minus_one := UDINT_to_SINT(MAX-1);
		ret.min_plus_one := UDINT_to_SINT(MIN+1);
		ret.max_overflow := UDINT_to_SINT(MAX+1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.positive, 22i8);
    assert_eq!(maintype.max_minus_one, 126i8);
    assert_eq!(maintype.min_plus_one, 1i8);
    assert_eq!(maintype.max_overflow, -128i8);
}

#[test]
fn udint_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : ULINT; negative : ULINT; positive : ULINT;
		max_minus_one : ULINT; min_plus_one : ULINT; max_overflow : ULINT; min_overflow : ULINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 4294967295;
		MIN : UDINT := 0;
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
		ret.max_overflow := UDINT_to_ULINT(MAX+1);
		ret.min_overflow := UDINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 4294967295u64);
    assert_eq!(maintype.positive, 22u64);
    assert_eq!(maintype.max_minus_one, 4294967294u64);
    assert_eq!(maintype.min_plus_one, 1u64);
    assert_eq!(maintype.max_overflow, 0u64);
    assert_eq!(maintype.min_overflow, 4294967295u64);
}

#[test]
fn udint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT; negative : UINT; positive : UINT;
		max_minus_one : UINT; min_plus_one : UINT; max_overflow : UINT; min_overflow : UINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 65535;
		MIN : UDINT := 0;
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
		ret.max_overflow := UDINT_to_UINT(MAX+1);
		ret.min_overflow := UDINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 65535u16);
    assert_eq!(maintype.positive, 22u16);
    assert_eq!(maintype.max_minus_one, 65534u16);
    assert_eq!(maintype.min_plus_one, 1u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 65535u16);
}

#[test]
fn udint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT; negative : USINT; positive : USINT;
		max_minus_one : USINT; min_plus_one : USINT; max_overflow : USINT; min_overflow : USINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UDINT := 255;
		MIN : UDINT := 0;
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
		ret.max_overflow := UDINT_to_USINT(MAX+1);
		ret.min_overflow := UDINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 255u8);
    assert_eq!(maintype.positive, 22u8);
    assert_eq!(maintype.max_minus_one, 254u8);
    assert_eq!(maintype.min_plus_one, 1u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 255u8);
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
		zero : LINT; negative : LINT; positive : LINT;
		max_minus_one : LINT; min_plus_one : LINT; max_overflow : LINT; min_overflow : LINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 65535;
		MIN : UINT := 0;
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
		ret.max_overflow := UINT_to_LINT(MAX+1);
		ret.min_overflow := UINT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, 65535i64);
    assert_eq!(maintype.positive, 22i64);
    assert_eq!(maintype.max_minus_one, 65534i64);
    assert_eq!(maintype.min_plus_one, 1i64);
    assert_eq!(maintype.max_overflow, 0i64);
    assert_eq!(maintype.min_overflow, 65535i64);
}

#[test]
fn uint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 65535;
		MIN : UINT := 0;
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
		ret.max_overflow := UINT_to_DINT(MAX+1);
		ret.min_overflow := UINT_to_DINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, 65535i32);
    assert_eq!(maintype.positive, 22i32);
    assert_eq!(maintype.max_minus_one, 65534i32);
    assert_eq!(maintype.min_plus_one, 1i32);
    assert_eq!(maintype.max_overflow, 0i32);
    assert_eq!(maintype.min_overflow, 65535i32);
}

#[test]
fn uint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : INT; negative : INT; positive : INT;
		max_minus_one : INT; min_plus_one : INT; max_overflow : INT; min_overflow : INT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 32767;
		MIN : UINT := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_to_INT(UINT#0);
		ret.positive := UINT_to_INT(UINT#22);
		ret.max_minus_one := UINT_to_INT(MAX-1);
		ret.min_plus_one := UINT_to_INT(MIN+1);
		ret.max_overflow := UINT_to_INT(MAX+1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.positive, 22i16);
    assert_eq!(maintype.max_minus_one, 32766i16);
    assert_eq!(maintype.min_plus_one, 1i16);
    assert_eq!(maintype.max_overflow, -32768i16);
}

#[test]
fn uint_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : SINT; negative : SINT; positive : SINT;
		max_minus_one : SINT; min_plus_one : SINT; max_overflow : SINT; min_overflow : SINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 127;
		MIN : UINT := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := UINT_to_SINT(UINT#0);
		ret.positive := UINT_to_SINT(UINT#22);
		ret.max_minus_one := UINT_to_SINT(MAX-1);
		ret.min_plus_one := UINT_to_SINT(MIN+1);
		ret.max_overflow := UINT_to_SINT(MAX+1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.positive, 22i8);
    assert_eq!(maintype.max_minus_one, 126i8);
    assert_eq!(maintype.min_plus_one, 1i8);
    assert_eq!(maintype.max_overflow, -128i8);
}

#[test]
fn uint_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : ULINT; negative : ULINT; positive : ULINT;
		max_minus_one : ULINT; min_plus_one : ULINT; max_overflow : ULINT; min_overflow : ULINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 65535;
		MIN : UINT := 0;
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
		ret.max_overflow := UINT_to_ULINT(MAX+1);
		ret.min_overflow := UINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 65535u64);
    assert_eq!(maintype.positive, 22u64);
    assert_eq!(maintype.max_minus_one, 65534u64);
    assert_eq!(maintype.min_plus_one, 1u64);
    assert_eq!(maintype.max_overflow, 0u64);
    assert_eq!(maintype.min_overflow, 65535u64);
}

#[test]
fn uint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT; negative : UDINT; positive : UDINT;
		max_minus_one : UDINT; min_plus_one : UDINT; max_overflow : UDINT; min_overflow : UDINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 65535;
		MIN : UINT := 0;
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
		ret.max_overflow := UINT_to_UDINT(MAX+1);
		ret.min_overflow := UINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 65535u32);
    assert_eq!(maintype.positive, 22u32);
    assert_eq!(maintype.max_minus_one, 65534u32);
    assert_eq!(maintype.min_plus_one, 1u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 65535u32);
}

#[test]
fn uint_to_usint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : USINT; negative : USINT; positive : USINT;
		max_minus_one : USINT; min_plus_one : USINT; max_overflow : USINT; min_overflow : USINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : UINT := 255;
		MIN : UINT := 0;
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
		ret.max_overflow := UINT_to_USINT(MAX+1);
		ret.min_overflow := UINT_to_USINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U8Type::default();
    let _res: u8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u8);
    assert_eq!(maintype.negative, 255u8);
    assert_eq!(maintype.positive, 22u8);
    assert_eq!(maintype.max_minus_one, 254u8);
    assert_eq!(maintype.min_plus_one, 1u8);
    assert_eq!(maintype.max_overflow, 0u8);
    assert_eq!(maintype.min_overflow, 255u8);
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
		zero : LINT; negative : LINT; positive : LINT;
		max_minus_one : LINT; min_plus_one : LINT; max_overflow : LINT; min_overflow : LINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 255;
		MIN : USINT := 0;
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
		ret.max_overflow := USINT_to_LINT(MAX+1);
		ret.min_overflow := USINT_to_LINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I64Type::default();
    let _res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i64);
    assert_eq!(maintype.negative, 255i64);
    assert_eq!(maintype.positive, 22i64);
    assert_eq!(maintype.max_minus_one, 254i64);
    assert_eq!(maintype.min_plus_one, 1i64);
    assert_eq!(maintype.max_overflow, 0i64);
    assert_eq!(maintype.min_overflow, 255i64);
}

#[test]
fn usint_to_dint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : DINT; negative : DINT; positive : DINT;
		max_minus_one : DINT; min_plus_one : DINT; max_overflow : DINT; min_overflow : DINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 255;
		MIN : USINT := 0;
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
		ret.max_overflow := USINT_to_DINT(MAX+1);
		ret.min_overflow := USINT_to_DINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I32Type::default();
    let _res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i32);
    assert_eq!(maintype.negative, 255i32);
    assert_eq!(maintype.positive, 22i32);
    assert_eq!(maintype.max_minus_one, 254i32);
    assert_eq!(maintype.min_plus_one, 1i32);
    assert_eq!(maintype.max_overflow, 0i32);
    assert_eq!(maintype.min_overflow, 255i32);
}

#[test]
fn usint_to_int_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : INT; negative : INT; positive : INT;
		max_minus_one : INT; min_plus_one : INT; max_overflow : INT; min_overflow : INT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 255;
		MIN : USINT := 0;
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
		ret.max_overflow := USINT_to_INT(MAX+1);
		ret.min_overflow := USINT_to_INT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I16Type::default();
    let _res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i16);
    assert_eq!(maintype.negative, 255i16);
    assert_eq!(maintype.positive, 22i16);
    assert_eq!(maintype.max_minus_one, 254i16);
    assert_eq!(maintype.min_plus_one, 1i16);
    assert_eq!(maintype.max_overflow, 0i16);
    assert_eq!(maintype.min_overflow, 255i16);
}

#[test]
fn usint_to_sint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : SINT; negative : SINT; positive : SINT;
		max_minus_one : SINT; min_plus_one : SINT; max_overflow : SINT; min_overflow : SINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 127;
		MIN : USINT := 0;
	END_VAR

	PROGRAM main
	VAR
		ret : myType;
	END_VAR
		ret.zero := USINT_to_SINT(USINT#0);
		ret.positive := USINT_to_SINT(USINT#22);
		ret.max_minus_one := USINT_to_SINT(MAX-1);
		ret.min_plus_one := USINT_to_SINT(MIN+1);
		ret.max_overflow := USINT_to_SINT(MAX+1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = I8Type::default();
    let _res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0i8);
    assert_eq!(maintype.positive, 22i8);
    assert_eq!(maintype.max_minus_one, 126i8);
    assert_eq!(maintype.min_plus_one, 1i8);
    assert_eq!(maintype.max_overflow, -128i8);
}

#[test]
fn usint_to_ulint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : ULINT; negative : ULINT; positive : ULINT;
		max_minus_one : ULINT; min_plus_one : ULINT; max_overflow : ULINT; min_overflow : ULINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 255;
		MIN : USINT := 0;
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
		ret.max_overflow := USINT_to_ULINT(MAX+1);
		ret.min_overflow := USINT_to_ULINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U64Type::default();
    let _res: u64 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u64);
    assert_eq!(maintype.negative, 255u64);
    assert_eq!(maintype.positive, 22u64);
    assert_eq!(maintype.max_minus_one, 254u64);
    assert_eq!(maintype.min_plus_one, 1u64);
    assert_eq!(maintype.max_overflow, 0u64);
    assert_eq!(maintype.min_overflow, 255u64);
}

#[test]
fn usint_to_udint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UDINT; negative : UDINT; positive : UDINT;
		max_minus_one : UDINT; min_plus_one : UDINT; max_overflow : UDINT; min_overflow : UDINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 255;
		MIN : USINT := 0;
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
		ret.max_overflow := USINT_to_UDINT(MAX+1);
		ret.min_overflow := USINT_to_UDINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U32Type::default();
    let _res: u32 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u32);
    assert_eq!(maintype.negative, 255u32);
    assert_eq!(maintype.positive, 22u32);
    assert_eq!(maintype.max_minus_one, 254u32);
    assert_eq!(maintype.min_plus_one, 1u32);
    assert_eq!(maintype.max_overflow, 0u32);
    assert_eq!(maintype.min_overflow, 255u32);
}

#[test]
fn usint_to_uint_conversion() {
    let src = r"
	TYPE myType : STRUCT
		zero : UINT; negative : UINT; positive : UINT;
		max_minus_one : UINT; min_plus_one : UINT; max_overflow : UINT; min_overflow : UINT;
	END_STRUCT END_TYPE

	VAR_GLOBAL
		MAX : USINT := 255;
		MIN : USINT := 0;
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
		ret.max_overflow := USINT_to_UINT(MAX+1);
		ret.min_overflow := USINT_to_UINT(MIN-1);
    END_PROGRAM
        ";
    let sources = add_std!(src, "num_conversion.st");
    let mut maintype = U16Type::default();
    let _res: u16 = compile_and_run(sources, &mut maintype);
    assert_eq!(maintype.zero, 0u16);
    assert_eq!(maintype.negative, 255u16);
    assert_eq!(maintype.positive, 22u16);
    assert_eq!(maintype.max_minus_one, 254u16);
    assert_eq!(maintype.min_plus_one, 1u16);
    assert_eq!(maintype.max_overflow, 0u16);
    assert_eq!(maintype.min_overflow, 255u16);
}
