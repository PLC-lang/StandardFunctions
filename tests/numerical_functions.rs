use rusty::runner::{compile_and_run, MainType};

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[test]
fn absolute_on_int_test() {
    let src = r"FUNCTION main : INT
            main := ABS(-99);
        ";
    let sources = add_std!(src, "num.st");
    let mut maintype = MainType::default();
    let res: i16 = compile_and_run(sources, &mut maintype);
    assert_eq!(res, 99);
}
