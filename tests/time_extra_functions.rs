use common::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[allow(dead_code)]
#[repr(C)]
struct MainType {
    a: [usize; 1000],
}

impl Default for MainType {
    fn default() -> Self {
        MainType { a: [0; 1000] }
    }
}

#[test]
fn concat_date_tod() {
    let src = "
	FUNCTION main : DT
		main := CONCAT_DATE_TOD(D#2010-03-12, TOD#12:30:15.123);
	END_FUNCTION";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 1268397015122);
}

#[test]
fn concat_date_ltod() {
    let src = "
	FUNCTION main : DT
		main := CONCAT_DATE_LTOD(D#2010-03-12, LTOD#12:30:15.123);
	END_FUNCTION";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 1268397015122);
}

#[test]
fn concat_date() {
    let src = "
	FUNCTION main : DATE
		main := CONCAT_DATE(INT#2000,SINT#01,SINT#01);
	END_FUNCTION";
    let sources = add_std!(src, "time_extra_functions.st");
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 946684800000);
}
