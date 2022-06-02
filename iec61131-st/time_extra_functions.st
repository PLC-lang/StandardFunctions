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