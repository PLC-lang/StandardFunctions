// Import common functionality into the integration tests
mod common;

use common::add_std;

use crate::common::compile_and_run_no_params;

#[test]
fn test_mux() {
    let src = r"FUNCTION main : DINT
    VAR
        a,b,c,d : DINT;
    END_VAR
    a := 1;
    b := 2;
    c := 3;
    main := MUX(2,a,b,c);
    END_FUNCTION";

    let res: i32 = compile_and_run_no_params(src);
    assert_eq!(res, 3);
}

#[test]
fn test_sel() {
    let src = r"FUNCTION main : DINT
    VAR
        a,b,c : DINT;
    END_VAR
    a := 1;
    b := 2;
    main := SEL(FALSE,a,b);
    END_FUNCTION";
    
    let res: i32 = compile_and_run_no_params(src);
    assert_eq!(res, 1);
}

#[test]
#[ignore = "Not yet implemented, needs to be builtin"]
fn test_move() {
    let src = r"FUNCTION main : DINT
    VAR
        a : DINT;
    END_VAR
    a := 1;
    main := MOVE(a);
    END_FUNCTION";
    
    let res: i32 = compile_and_run_no_params(src);
    assert_eq!(res, 1);
}

#[test]
fn test_max_int() {
    let src = r"FUNCTION main : INT
    main := MAX(INT#5,INT#2,INT#1,INT#3,INT#4,INT#7,INT#-1);
    END_FUNCTION";
    
    let src = add_std!(src, "selectors.st");
    let res: i16 = compile_and_run_no_params(src);
    assert_eq!(res, 7);
}

#[test]
fn test_max_dint() {
    let src = r"FUNCTION main : DINT
    main := MAX(DINT#5,DINT#2,DINT#1,DINT#3,DINT#4,DINT#7,DINT#-1);
    END_FUNCTION";
    
    let src = add_std!(src, "selectors.st");
    let res: i32 = compile_and_run_no_params(src);
    assert_eq!(res, 7);
}

#[test]
fn test_max_lint() {
    let src = r"FUNCTION main : LINT
    main := MAX(LINT#5,LINT#2,LINT#1,LINT#3,LINT#4,LINT#7,LINT#-1);
    END_FUNCTION";
    
    let src = add_std!(src, "selectors.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, 7);
}

#[test]
fn test_max_char() {
    let src = r"FUNCTION main : CHAR
    main := MAX(CHAR#'a',CHAR#'d',CHAR#'e',CHAR#'g',CHAR#'f',CHAR#'c',CHAR#'b');
    END_FUNCTION";
    
    let src = add_std!(src, "selectors.st");
    let res: u8 = compile_and_run_no_params(src);
    assert_eq!(res, b'g');
}

#[test]
fn test_max_date() {
    let src = r"FUNCTION main : CHAR
    main := MAX(T#35ms, T#40ms,T#1ms,T#30ms);
    END_FUNCTION";
    
    let src = add_std!(src, "selectors.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, 70_000_000);
}


#[test]
fn test_min_int() {
    let src = r"FUNCTION main : INT
    main := MIN(INT#5,INT#2,INT#-1,INT#3,INT#4,INT#7,INT#1);
    END_FUNCTION";
    
    let src = add_std!(src, "selectors.st");
    let res: i16 = compile_and_run_no_params(src);
    assert_eq!(res, -1);
}

#[test]
fn test_min_dint() {
    let src = r"FUNCTION main : DINT
    main := MIN(DINT#5,DINT#2,DINT#-1,DINT#3,DINT#4,DINT#7,DINT#1);
    END_FUNCTION";
    
    let src = add_std!(src, "selectors.st");
    let res: i32 = compile_and_run_no_params(src);
    assert_eq!(res, -1);
}

#[test]
fn test_min_lint() {
    let src = r"FUNCTION main : LINT
    main := MIN(LINT#5,LINT#2,LINT#-1,LINT#3,LINT#4,LINT#7,LINT#1);
    END_FUNCTION";
    
    let src = add_std!(src, "selectors.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, -1);
}

#[test]
fn test_min_char() {
    let src = r"FUNCTION main : CHAR
    main := MIN(CHAR#'b',CHAR#'d',CHAR#'e',CHAR#'g',CHAR#'f',CHAR#'a',CHAR#'c');
    END_FUNCTION";
    
    let src = add_std!(src, "selectors.st");
    let res: u8 = compile_and_run_no_params(src);
    assert_eq!(res, b'a');
}

#[test]
fn test_min_date() {
    let src = r"FUNCTION main : CHAR
    main := MIN(T#40ms,T#1d,T#30ms,T#5m);
    END_FUNCTION";
    
    let src = add_std!(src, "selectors.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, 30_000_000);
}

#[test]
fn test_limit_int() {}

#[test]
fn test_limit_dint() {}

#[test]
fn test_limit_lint() {}

#[test]
fn test_limit_char() {}

#[test]
fn test_limit_date() {}
