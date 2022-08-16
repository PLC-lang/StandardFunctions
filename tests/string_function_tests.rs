// Import common functionality into the integration tests
mod common;

use common::add_std;

use crate::common::compile_and_run_no_params;

#[test]
fn len_string() {
    let src = r#"
	FUNCTION main : STRING
		main := LEN(STRING#"hello");
    END_FUNCTION
        "#;
    let sources = add_std!(src, "string_functions.st");
    let res: i32 = compile_and_run_no_params(sources);
    assert_eq!(5, res);
}
