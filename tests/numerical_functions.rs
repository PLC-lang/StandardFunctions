use inkwell::context::Context;
use rusty::runner::{compile_and_run, run, MainType};

// Import common functionality into the integration tests
mod common;

use common::add_std;

use crate::common::compile_with_native;

#[test]
fn absolute_on_sint_test() {
    let src = r"FUNCTION main : SINT
            main := ABS(SINT#-120);
            END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::default();
    let res: i8 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 120i8);
}

#[test]
fn absolute_on_int_test() {
    let src = r"FUNCTION main : INT
            main := ABS(INT#-99);
            END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::default();
    let res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 99i16);
}

#[test]
fn absolute_on_dint_test() {
    let src = r"FUNCTION main : DINT
            main := ABS(-77);
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::default();
    let res: i32 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 77i32);
}

#[test]
fn absolute_on_lint_test() {
    let src = r"FUNCTION main : LINT
            main := ABS(LINT#-88);
            END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 88i64);
}

#[test]
fn absolute_on_real_test() {
    let src = r"FUNCTION main : REAL
            main := ABS(REAL#-3.2);
            END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::default();
    let res: f32 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 3.2f32);
}

#[test]
fn absolute_on_lreal_test() {
    let src = r"FUNCTION main : LREAL
            main := ABS(LREAL#-2.5);
            END_FUNCTION
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::default();
    let res: f64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 2.5f64);
}

#[test]
fn test_round_real() {
    let src = r"
        FUNCTION main : REAL
            main := ROUND(REAL#2.5);
        END_FUNCTION
        ";
    let context = Context::create();
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::default();
    let engine = compile_with_native(&context, sources);
    let res: f32 = run(&engine, "main", &mut maintype);
    assert_eq!(res, 3.0f32);
}

#[test]
fn test_round_lreal() {
    let src = r"
        FUNCTION main : LREAL
            main := ROUND(LREAL#2.5);
        END_FUNCTION
        ";
    let context = Context::create();
    let mut maintype = MainType::default();
    let sources = add_std!(src, "num.st");
    let engine = compile_with_native(&context, sources);
    let res: f64 = run(&engine, "main", &mut maintype);
    assert_eq!(res, 3.0f64);
}
