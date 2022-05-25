use common::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

#[test]
fn ltime_to_time_conversion() {
    let src = "
	FUNCTION main : TIME
	VAR
		res : TIME;
	END_VAR
		res := LTIME_TO_TIME();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}

#[test]
fn time_to_ltime_conversion() {
    let src = "
	FUNCTION main : LTIME
	VAR
		res : LTIME;
	END_VAR
		res := TIME_TO_LTIME();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}

#[test]
fn ldt_to_dt_conversion() {
    let src = "
	FUNCTION main : DT
	VAR
		res : DT;
	END_VAR
		res := LDT_TO_DT();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}

#[test]
fn ldt_to_date_conversion() {
    let src = "
	FUNCTION main : DATE
	VAR
		res : DATE;
	END_VAR
		res := LDT_TO_DATE();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}

#[test]
fn ldt_to_ltod_conversion() {
    let src = "
	FUNCTION main : LTOD
	VAR
		res : LTOD;
	END_VAR
		res := LDT_TO_LTOD();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}

#[test]
fn ldt_to_tod_conversion() {
    let src = "
	FUNCTION main : TOD
	VAR
		res : TOD;
	END_VAR
		res := LDT_TO_TOD();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}

#[test]
fn dt_to_ldt_conversion() {
    let src = "
	FUNCTION main : LDT
	VAR
		res : LDT;
	END_VAR
		res := DT_TO_LDT();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}

#[test]
fn dt_to_date_conversion() {
    let src = "
	FUNCTION main : DATE
	VAR
		res : DATE;
	END_VAR
		res := DT_TO_DATE();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}

#[test]
fn dt_to_ltod_conversion() {
    let src = "
	FUNCTION main : LTOD
	VAR
		res : LTOD;
	END_VAR
		res := DT_TO_LTOD();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}

#[test]
fn dt_to_tod_conversion() {
    let src = "
	FUNCTION main : TOD
	VAR
		res : TOD;
	END_VAR
		res := DT_TO_TOD();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}

#[test]
fn ltod_to_tod_conversion() {
    let src = "
	FUNCTION main : TOD
	VAR
		res : TOD;
	END_VAR
		res := LTOD_TO_TOD();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}

#[test]
fn tod_to_ltod_conversion() {
    let src = "
	FUNCTION main : LTOD
	VAR
		res : LTOD;
	END_VAR
		res := TOD_TO_LTOD();
	END_FUNCTION";
    let sources = add_std!(src, "date_time_conversion.st");
}
