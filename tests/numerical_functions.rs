// Import common functionality into the integration tests
mod common;

use common::add_std;

use crate::common::compile_and_run;

#[derive(Default)]
struct MainType<T: Default> {
    a: T,
    b: T,
    c: T,
}

#[test]
fn absolute_on_sint_test() {
    let src = r"FUNCTION main : SINT
            VAR
                a,b,c : SINT;
            END_VAR
            a := ABS(SINT#-120);
            b := ABS(SINT#-0);
            c := ABS(SINT#121);
            main := ABS(SINT#0);
            END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::<i8>::default();
    let res: i8 = compile_and_run(sources, &mut maintype);

    assert_eq!(res, 0i8);
    assert_eq!(maintype.a, 120i8);
    assert_eq!(maintype.b, 0i8);
    assert_eq!(maintype.c, 121i8);
}

#[test]
fn absolute_on_int_test() {
    let src = r"FUNCTION main : INT
            VAR
                a,b,c : INT;
            END_VAR
            a := ABS(INT#-99);
            b := ABS(INT#-0);
            c := ABS(INT#98);
            main := ABS(INT#0);
            END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::<i16>::default();
    let res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 0i16);
    assert_eq!(maintype.a, 99i16);
    assert_eq!(maintype.b, 0i16);
    assert_eq!(maintype.c, 98i16);
}

#[test]
fn absolute_on_dint_test() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b,c : DINT;
            END_VAR
            a := ABS(-77);
            b := ABS(-0);
            c := ABS(78);
            main := ABS(0);
            END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::<i32>::default();
    let res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 0i32);
    assert_eq!(maintype.a, 77i32);
    assert_eq!(maintype.b, 0i32);
    assert_eq!(maintype.c, 78i32);
}

#[test]
fn absolute_on_lint_test() {
    let src = r"FUNCTION main : LINT
            VAR
                a,b,c : LINT;
            END_VAR
            a := ABS(LINT#-88);
            b := ABS(LINT#-0);
            c := ABS(LINT#89);
            main := ABS(LINT#0);
            END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::<i64>::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 0i64);
    assert_eq!(maintype.a, 88i64);
    assert_eq!(maintype.b, 0i64);
    assert_eq!(maintype.c, 89i64);
}

#[test]
fn absolute_on_real_test() {
    let src = r"FUNCTION main : REAL
            VAR
                a,b,c : REAL;
            END_VAR
            a := ABS(REAL#-3.2);
            b := ABS(REAL#-0);
            c := ABS(REAL#3.3);
            main := ABS(REAL#0);
            END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::<f32>::default();
    let res: f32 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 0.0f32);
    assert_eq!(maintype.a, 3.2f32);
    assert_eq!(maintype.b, 0.0f32);
    assert_eq!(maintype.c, 3.3f32);
}

#[test]
fn absolute_on_lreal_test() {
    let src = r"FUNCTION main : LREAL
            VAR
                a,b,c : LREAL;
            END_VAR
            a := ABS(LREAL#-2.5);
            b := ABS(LREAL#-0);
            c := ABS(LREAL#2.6);
            main := ABS(LREAL#0);
            END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::<f64>::default();
    let res: f64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 0.0f64);
    assert_eq!(maintype.a, 2.5f64);
    assert_eq!(maintype.b, 0.0f64);
    assert_eq!(maintype.c, 2.6f64);
}

#[test]
fn test_round_real() {
    let src = r"
        FUNCTION main : REAL
            main := ROUND(REAL#2.5);
        END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = rusty::runner::MainType::default();
    let res: f32 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 3.0f32);
}

#[test]
fn test_round_lreal() {
    let src = r"
        FUNCTION main : LREAL
            main := ROUND(LREAL#2.5);
        END_FUNCTION
        ";
    let mut maintype = rusty::runner::MainType::default();
    let sources = add_std!(src, "num.st");
    let res: f64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 3.0f64);
}
