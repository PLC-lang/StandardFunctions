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