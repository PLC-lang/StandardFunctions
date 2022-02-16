// Import common functionality into the integration tests
mod common;

use common::add_std;

use crate::common::compile_and_run;

#[derive(Default)]
struct MainType<T: Default> {
    a: T,
    b: T,
}

#[test]
fn sqrt_called_on_real() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : REAL;
            END_VAR
            a := SQRT(REAL#4.0);
            b := SQRT(REAL#1.0);
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert_eq!(maintype.a, 2.0f32);
    assert_eq!(maintype.b, 1.0f32);
}

#[test]
fn sqrt_called_on_lreal() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : LREAL;
            END_VAR
            a := SQRT(LREAL#4.0);
            b := SQRT(LREAL#1.0);
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert_eq!(maintype.a, 2.0f64);
    assert_eq!(maintype.b, 1.0f64);
}

#[test]
#[ignore = "No auto conversion of generic types, we need the conversion function to be done PR#21"]
fn sqrt_called_on_none_real() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : DINT;
            END_VAR
            a := SQRT(DINT_TO_REAL(2));
            b := SQRT(DINT_TO_REAL(1));
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st", "num.st");
    let mut maintype = MainType::<i32>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert_eq!(maintype.a, 2);
    assert_eq!(maintype.b, 1);
}

#[test]
fn ln_called_on_real() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : REAL;
            END_VAR
            a := LN(REAL#2.7182818) - 1.0;
            b := LN(REAL#1.0);
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert!(maintype.a <= f32::EPSILON);
    assert!(maintype.b <= f32::EPSILON);
}

#[test]
fn ln_called_on_lreal() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : LREAL;
            END_VAR
            a := LN(LREAL#2.7182818) - 1.0;
            b := LN(LREAL#1.0);
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert!(maintype.a <= f64::EPSILON);
    assert!(maintype.b <= f64::EPSILON);
}

#[test]
fn log_called_on_real() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : REAL;
            END_VAR
                a := LOG(REAL#10);
                b := LOG(REAL#100);
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert_eq!(maintype.a, 1.0f32);
    assert_eq!(maintype.b, 2.0f32);
}

#[test]
fn log_called_on_lreal() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : LREAL;
            END_VAR
                a := LOG(LREAL#10);
                b := LOG(LREAL#100);
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert_eq!(maintype.a, 1.0f64);
    assert_eq!(maintype.b, 2.0f64);
}

#[test]
fn exp_called_on_real() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : REAL;
            END_VAR
            a := EXP(REAL#1.0) - REAL#2.7182818;
            b := EXP(REAL#0.0) - REAL#1.0;
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert!(maintype.a <= f32::EPSILON);
    assert!(maintype.b <= f32::EPSILON);
}

#[test]
fn exp_called_on_lreal() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : LREAL;
            END_VAR
            a := EXP(LREAL#1.0) - LREAL#2.718281849;
            b := EXP(LREAL#0.0) - LREAL#1.0;
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert!(maintype.a <= f64::EPSILON);
    assert!(maintype.b <= f64::EPSILON);
}

#[test]
fn sin_called_on_real() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : REAL;
            END_VAR
            a := SIN(REAL#3.14159274) - REAL#1.0;
            b := SIN(REAL#0.0); 
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert!(maintype.a <= f32::EPSILON);
    assert!(maintype.b <= f32::EPSILON);
}

#[test]
fn sin_called_on_lreal() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : LREAL;
            END_VAR
            a := SIN(LREAL#3.1415926535897931) - LREAL#1.0;
            b := SIN(LREAL#0.0); 
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert!(maintype.a <= f64::EPSILON);
    assert!(maintype.b <= f64::EPSILON);
}

#[test]
fn cos_called_on_real() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : REAL;
            END_VAR
            a := COS(REAL#3.1415926535897931) + REAL#1.0; 
            b := COS(REAL#0.0) - REAL#1.0; 
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert!(maintype.a <= f32::EPSILON);
    assert!(maintype.b <= f32::EPSILON);
}

#[test]
fn cos_called_on_lreal() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : LREAL;
            END_VAR
            a := COS(LREAL#3.1415926535897931) + LREAL#1.0; 
            b := COS(LREAL#0.0) - LREAL#1.0; 
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert!(maintype.a <= f64::EPSILON);
    assert!(maintype.b <= f64::EPSILON);
}

#[test]
fn tan_called_on_real() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : REAL;
            END_VAR
            a := TAN(REAL#3.1415926535897931/4.0) - REAL#1.0; 
            b := TAN(REAL#3.1415926535897931); 
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f32>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert!(maintype.a <= f32::EPSILON);
    assert!(maintype.b <= f32::EPSILON);
}

#[test]
fn tan_called_on_lreal() {
    let src = r"FUNCTION main : DINT
            VAR
                a,b : LREAL;
            END_VAR
            a := TAN(LREAL#3.1415926535897931/4) - LREAL#1.0; 
            a := TAN(LREAL#3.1415926535897931); 
            END_FUNCTION
        ";
    let sources = add_std!(src, "math.st");
    let mut maintype = MainType::<f64>::default();
    let _: i32 = compile_and_run(sources, &mut maintype);

    assert!(maintype.a <= f64::EPSILON);
    assert!(maintype.b <= f64::EPSILON);
}
