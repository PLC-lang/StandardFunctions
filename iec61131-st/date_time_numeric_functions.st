(********************
*
* This operator returns the value of adding up the operands.
*
*********************)
FUNCTION ADD <T: ANY> : T
VAR_INPUT
	IN1 : T;
	IN2 : T;
END_VAR
END_FUNCTION

(* Specialized implementation of ADD for TIME *)
FUNCTION ADD__TIME : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : TIME;
END_VAR
	ADD__TIME := ADD_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of ADD for LTIME *)
FUNCTION ADD__LTIME : LTIME
VAR_INPUT
	IN1 : LTIME;
	IN2 : LTIME;
END_VAR
	ADD__LTIME := ADD_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator returns the value of adding up two TIME operands.
*
*********************)
@EXTERNAL
FUNCTION ADD_TIME : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator returns the value of adding up two LTIME operands.
*
*********************)
FUNCTION ADD_LTIME : LTIME
VAR_INPUT
	IN1 : LTIME;
	IN2 : LTIME;
END_VAR
	ADD_LTIME := ADD_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator returns the value of adding up TOD and TIME.
* If the result overflows 0 will be returned
*
*********************)
@EXTERNAL
FUNCTION ADD_TOD_TIME : TOD
VAR_INPUT
	IN1 : TOD;
	IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator returns the value of adding up LTOD and LTIME.
* If the result overflows 0 will be returned
*
*********************)
FUNCTION ADD_LTOD_LTIME : LTOD
VAR_INPUT
	IN1 : LTOD;
	IN2 : LTIME;
END_VAR
	ADD_LTOD_LTIME := ADD_TOD_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator returns the value of adding up DT and TIME.
* If the result overflows 0 will be returned
*
*********************)
@EXTERNAL
FUNCTION ADD_DT_TIME : DT
VAR_INPUT
	IN1 : DT;
	IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator returns the value of adding up LDT and LTIME.
* If the result overflows 0 will be returned
*
*********************)
FUNCTION ADD_LDT_LTIME : LDT
VAR_INPUT
	IN1 : LDT;
	IN2 : LTIME;
END_VAR
	ADD_LDT_LTIME := ADD_DT_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of the operands.
*
*********************)
FUNCTION SUB <T: ANY> : T
VAR_INPUT
	IN1 : T;
	IN2 : T;
END_VAR
END_FUNCTION

(* Specialized implementation of SUB for TIME *)
FUNCTION SUB__TIME : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : TIME;
END_VAR
	SUB__TIME := SUB_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for LTIME *)
FUNCTION SUB__LTIME : LTIME
VAR_INPUT
	IN1 : LTIME;
	IN2 : LTIME;
END_VAR
	SUB__LTIME := SUB_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two TIME operands
*
*********************)
@EXTERNAL
FUNCTION SUB_TIME : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LTIME operands
*
*********************)
FUNCTION SUB_LTIME : LTIME
VAR_INPUT
	IN1 : LTIME;
	IN2 : LTIME;
END_VAR
	SUB_LTIME := SUB_TIME(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for DATE *)
FUNCTION SUB__DATE : TIME
VAR_INPUT
	IN1 : DATE;
	IN2 : DATE;
END_VAR
	SUB__DATE := SUB_DATE_DATE(IN1, IN2);
END_FUNCTION

(* Specialized implementation of SUB for LDATE *)
FUNCTION SUB__LDATE : LTIME
VAR_INPUT
	IN1 : LDATE;
	IN2 : LDATE;
END_VAR
	SUB__LDATE := SUB_DATE_DATE(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two DATE operands returning TIME
* If the result overflows 0 will be returned
*
*********************)
@EXTERNAL
FUNCTION SUB_DATE_DATE : TIME
VAR_INPUT
	IN1 : DATE;
	IN2 : DATE;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LDATE operands returning LTIME
* If the result overflows 0 will be returned
*
*********************)
FUNCTION SUB_LDATE_LDATE : LTIME
VAR_INPUT
	IN1 : LDATE;
	IN2 : LDATE;
END_VAR
	SUB_LDATE_LDATE := SUB_DATE_DATE(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of TOD and TIME returning TOD
* If the result overflows 0 will be returned
*
*********************)
@EXTERNAL
FUNCTION SUB_TOD_TIME : TOD
VAR_INPUT
	IN1 : TOD;
	IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of LTOD and LTIME returning LTOD
* If the result overflows 0 will be returned
*
*********************)
FUNCTION SUB_LTOD_LTIME : LTOD
VAR_INPUT
	IN1 : LTOD;
	IN2 : LTIME;
END_VAR
	SUB_LTOD_LTIME := SUB_TOD_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two TOD operands returning TIME
* If the result overflows 0 will be returned
*
*********************)
@EXTERNAL
FUNCTION SUB_TOD_TOD : TIME
VAR_INPUT
	IN1 : TOD;
	IN2 : TOD;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LTOD operands returning LTIME
* If the result overflows 0 will be returned
*
*********************)
FUNCTION SUB_LTOD_LTOD : LTIME
VAR_INPUT
	IN1 : LTOD;
	IN2 : LTOD;
END_VAR
	SUB_LTOD_LTOD := SUB_TOD_TOD(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of DT and TIME returning DT
* If the result overflows 0 will be returned
*
*********************)
@EXTERNAL
FUNCTION SUB_DT_TIME : DT
VAR_INPUT
	IN1 : DT;
	IN2 : TIME;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of LDT and LTIME returning LDT
* If the result overflows 0 will be returned
*
*********************)
FUNCTION SUB_LDT_LTIME : LDT
VAR_INPUT
	IN1 : LDT;
	IN2 : LTIME;
END_VAR
	SUB_LDT_LTIME := SUB_DT_TIME(IN1, IN2);
END_FUNCTION

(********************
*
* This operator produces the subtraction of two DT operands returning TIME
* If the result overflows 0 will be returned
*
*********************)
@EXTERNAL
FUNCTION SUB_DT_DT : TIME
VAR_INPUT
	IN1 : DT;
	IN2 : DT;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the subtraction of two LDT operands returning LTIME
* If the result overflows 0 will be returned
*
*********************)
FUNCTION SUB_LDT_LDT : LTIME
VAR_INPUT
	IN1 : LDT;
	IN2 : LDT;
END_VAR
	SUB_LDT_LDT := SUB_DT_DT(IN1, IN2);
END_FUNCTION