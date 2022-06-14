(********************
*
* Concatenates DATE and TOD to DT
*
*********************)
@EXTERNAL
FUNCTION CONCAT_DATE_TOD : DT
VAR_INPUT
	date_input : DATE;
	tod_input : TOD;
END_VAR
END_FUNCTION

(********************
*
* Concatenates DATE and LTOD to DT
*
*********************)
FUNCTION CONCAT_DATE_LTOD : DT
VAR_INPUT
	date_input : DATE;
	tod_input : LTOD;
END_VAR
	CONCAT_DATE_LTOD := CONCAT_DATE_TOD(date_input, tod_input);
END_FUNCTION

(********************
*
* Concatenates year, month and day of type ANY_INT to DATE
*
*********************)
@EXTERNAL
FUNCTION CONCAT_DATE <T: ANY_INT> : DATE
VAR_INPUT
	year : T;
	month : T;
	day : T;
END_VAR
END_FUNCTION

(********************
*
* Concatenates hour, minute, second, millisecond of type ANY_INT to TOD
*
*********************)
@EXTERNAL
FUNCTION CONCAT_TOD <T: ANY_INT> : TOD
VAR_INPUT
	hour : T;
	minute : T;
	second : T;
	millisecond : T;
END_VAR
END_FUNCTION

(********************
*
* Concatenates hour, minute, second, millisecond of type ANY_INT to LTOD
*
*********************)
FUNCTION CONCAT_LTOD <T: ANY_INT> : LTOD
VAR_INPUT
	hour : T;
	minute : T;
	second : T;
	millisecond : T;
END_VAR
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for SINT *)
FUNCTION CONCAT_LTOD__SINT : LTOD
VAR_INPUT
	hour : SINT;
	minute : SINT;
	second : SINT;
	millisecond : SINT;
END_VAR
	CONCAT_LTOD__SINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for USINT *)
FUNCTION CONCAT_LTOD__USINT : LTOD
VAR_INPUT
	hour : USINT;
	minute : USINT;
	second : USINT;
	millisecond : USINT;
END_VAR
	CONCAT_LTOD__USINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for INT *)
FUNCTION CONCAT_LTOD__INT : LTOD
VAR_INPUT
	hour : INT;
	minute : INT;
	second : INT;
	millisecond : INT;
END_VAR
	CONCAT_LTOD__INT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for UINT *)
FUNCTION CONCAT_LTOD__UINT : LTOD
VAR_INPUT
	hour : UINT;
	minute : UINT;
	second : UINT;
	millisecond : UINT;
END_VAR
	CONCAT_LTOD__UINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for DINT *)
FUNCTION CONCAT_LTOD__DINT : LTOD
VAR_INPUT
	hour : DINT;
	minute : DINT;
	second : DINT;
	millisecond : DINT;
END_VAR
	CONCAT_LTOD__DINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for UDINT *)
FUNCTION CONCAT_LTOD__UDINT : LTOD
VAR_INPUT
	hour : UDINT;
	minute : UDINT;
	second : UDINT;
	millisecond : UDINT;
END_VAR
	CONCAT_LTOD__UDINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for LINT *)
FUNCTION CONCAT_LTOD__LINT : LTOD
VAR_INPUT
	hour : LINT;
	minute : LINT;
	second : LINT;
	millisecond : LINT;
END_VAR
	CONCAT_LTOD__LINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(* Specialized implementation of CONCAT_LTOD for ULINT *)
FUNCTION CONCAT_LTOD__ULINT : LTOD
VAR_INPUT
	hour : ULINT;
	minute : ULINT;
	second : ULINT;
	millisecond : ULINT;
END_VAR
	CONCAT_LTOD__ULINT := CONCAT_TOD(hour, minute, second, millisecond);
END_FUNCTION

(********************
*
* Concatenates year, month, day, hour, minute, second, millisecond of type ANY_INT to DT
*
*********************)
FUNCTION CONCAT_DT <T: ANY_INT> : DT
VAR_INPUT
	year : T;
	month : T;
	day : T;
	hour : T;
	minute : T;
	second : T;
	millisecond : T;
END_VAR
END_FUNCTION

(* Specialized implementation of CONCAT_DT for INT *)
FUNCTION CONCAT_DT__INT : DT
VAR_INPUT
	year : INT;
	month : INT;
	day : INT;
	hour : INT;
	minute : INT;
	second : INT;
	millisecond : INT;
END_VAR
	CONCAT_DT__INT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_DT for UINT *)
FUNCTION CONCAT_DT__UINT : DT
VAR_INPUT
	year : UINT;
	month : UINT;
	day : UINT;
	hour : UINT;
	minute : UINT;
	second : UINT;
	millisecond : UINT;
END_VAR
	CONCAT_DT__UINT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_DT for DINT *)
FUNCTION CONCAT_DT__DINT : DT
VAR_INPUT
	year : DINT;
	month : DINT;
	day : DINT;
	hour : DINT;
	minute : DINT;
	second : DINT;
	millisecond : DINT;
END_VAR
	CONCAT_DT__DINT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_DT for UDINT *)
FUNCTION CONCAT_DT__UDINT : DT
VAR_INPUT
	year : UDINT;
	month : UDINT;
	day : UDINT;
	hour : UDINT;
	minute : UDINT;
	second : UDINT;
	millisecond : UDINT;
END_VAR
	CONCAT_DT__UDINT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_DT for LINT *)
FUNCTION CONCAT_DT__LINT : DT
VAR_INPUT
	year : LINT;
	month : LINT;
	day : LINT;
	hour : LINT;
	minute : LINT;
	second : LINT;
	millisecond : LINT;
END_VAR
	CONCAT_DT__LINT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_DT for ULINT *)
FUNCTION CONCAT_DT__ULINT : DT
VAR_INPUT
	year : ULINT;
	month : ULINT;
	day : ULINT;
	hour : ULINT;
	minute : ULINT;
	second : ULINT;
	millisecond : ULINT;
END_VAR
	CONCAT_DT__ULINT := CONCAT_DATE_TOD(CONCAT_DATE(year, month, day), CONCAT_TOD(hour, minute, second, millisecond));
END_FUNCTION

(********************
*
* Concatenates year, month, day, hour, minute, second, millisecond of type ANY_INT to LDT
*
*********************)
FUNCTION CONCAT_LDT <T: ANY_INT> : LDT
VAR_INPUT
	year : T;
	month : T;
	day : T;
	hour : T;
	minute : T;
	second : T;
	millisecond : T;
END_VAR
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for INT *)
FUNCTION CONCAT_LDT__INT : LDT
VAR_INPUT
	year : INT;
	month : INT;
	day : INT;
	hour : INT;
	minute : INT;
	second : INT;
	millisecond : INT;
END_VAR
	CONCAT_LDT__INT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for UINT *)
FUNCTION CONCAT_LDT__UINT : LDT
VAR_INPUT
	year : UINT;
	month : UINT;
	day : UINT;
	hour : UINT;
	minute : UINT;
	second : UINT;
	millisecond : UINT;
END_VAR
	CONCAT_LDT__UINT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for DINT *)
FUNCTION CONCAT_LDT__DINT : LDT
VAR_INPUT
	year : DINT;
	month : DINT;
	day : DINT;
	hour : DINT;
	minute : DINT;
	second : DINT;
	millisecond : DINT;
END_VAR
	CONCAT_LDT__DINT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for UDINT *)
FUNCTION CONCAT_LDT__UDINT : LDT
VAR_INPUT
	year : UDINT;
	month : UDINT;
	day : UDINT;
	hour : UDINT;
	minute : UDINT;
	second : UDINT;
	millisecond : UDINT;
END_VAR
	CONCAT_LDT__UDINT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for LINT *)
FUNCTION CONCAT_LDT__LINT : LDT
VAR_INPUT
	year : LINT;
	month : LINT;
	day : LINT;
	hour : LINT;
	minute : LINT;
	second : LINT;
	millisecond : LINT;
END_VAR
	CONCAT_LDT__LINT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(* Specialized implementation of CONCAT_LDT for ULINT *)
FUNCTION CONCAT_LDT__ULINT : LDT
VAR_INPUT
	year : ULINT;
	month : ULINT;
	day : ULINT;
	hour : ULINT;
	minute : ULINT;
	second : ULINT;
	millisecond : ULINT;
END_VAR
	CONCAT_LDT__ULINT := CONCAT_DATE_LTOD(CONCAT_DATE(year, month, day), CONCAT_LTOD(hour, minute, second, millisecond));
END_FUNCTION

(********************
*
* Splits DATE into year, month, day of type ANY_INT
*
*********************)
FUNCTION SPLIT_DATE <T: ANY_INT> : INT
VAR_INPUT
	in : DATE;
END_VAR
VAR_OUTPUT
	year : T;
	month : T;
	day : T;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for INT *)
@EXTERNAL
FUNCTION SPLIT_DATE__INT : INT
VAR_INPUT
	in : DATE;
END_VAR
VAR_OUTPUT
	year : INT;
	month : INT;
	day : INT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for UINT *)
@EXTERNAL
FUNCTION SPLIT_DATE__UINT : INT
VAR_INPUT
	in : DATE;
END_VAR
VAR_OUTPUT
	year : UINT;
	month : UINT;
	day : UINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for DINT *)
@EXTERNAL
FUNCTION SPLIT_DATE__DINT : INT
VAR_INPUT
	in : DATE;
END_VAR
VAR_OUTPUT
	year : DINT;
	month : DINT;
	day : DINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for UDINT *)
@EXTERNAL
FUNCTION SPLIT_DATE__UDINT : INT
VAR_INPUT
	in : DATE;
END_VAR
VAR_OUTPUT
	year : UDINT;
	month : UDINT;
	day : UDINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for LINT *)
@EXTERNAL
FUNCTION SPLIT_DATE__LINT : INT
VAR_INPUT
	in : DATE;
END_VAR
VAR_OUTPUT
	year : LINT;
	month : LINT;
	day : LINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_DATE for ULINT *)
@EXTERNAL
FUNCTION SPLIT_DATE__ULINT : INT
VAR_INPUT
	in : DATE;
END_VAR
VAR_OUTPUT
	year : ULINT;
	month : ULINT;
	day : ULINT;
END_VAR
END_FUNCTION

(********************
*
* Splits TOD into hour, minute, second, millisecond of type ANY_INT
*
*********************)
FUNCTION SPLIT_TOD <T: ANY_INT> : INT
VAR_INPUT
	in : TOD;
END_VAR
VAR_OUTPUT
	hour : T;
	minute : T;
	second : T;
	millisecond : T;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for INT *)
@EXTERNAL
FUNCTION SPLIT_TOD__INT : INT
VAR_INPUT
	in : TOD;
END_VAR
VAR_OUTPUT
	hour : INT;
	minute : INT;
	second : INT;
	millisecond : INT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for UINT *)
@EXTERNAL
FUNCTION SPLIT_TOD__UINT : INT
VAR_INPUT
	in : TOD;
END_VAR
VAR_OUTPUT
	hour : UINT;
	minute : UINT;
	second : UINT;
	millisecond : UINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for DINT *)
@EXTERNAL
FUNCTION SPLIT_TOD__DINT : INT
VAR_INPUT
	in : TOD;
END_VAR
VAR_OUTPUT
	hour : DINT;
	minute : DINT;
	second : DINT;
	millisecond : DINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for UDINT *)
@EXTERNAL
FUNCTION SPLIT_TOD__UDINT : INT
VAR_INPUT
	in : TOD;
END_VAR
VAR_OUTPUT
	hour : UDINT;
	minute : UDINT;
	second : UDINT;
	millisecond : UDINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for LINT *)
@EXTERNAL
FUNCTION SPLIT_TOD__LINT : INT
VAR_INPUT
	in : TOD;
END_VAR
VAR_OUTPUT
	hour : LINT;
	minute : LINT;
	second : LINT;
	millisecond : LINT;
END_VAR
END_FUNCTION

(* Specialized implementation of SPLIT_TOD for ULINT *)
@EXTERNAL
FUNCTION SPLIT_TOD__ULINT : INT
VAR_INPUT
	in : TOD;
END_VAR
VAR_OUTPUT
	hour : ULINT;
	minute : ULINT;
	second : ULINT;
	millisecond : ULINT;
END_VAR
END_FUNCTION