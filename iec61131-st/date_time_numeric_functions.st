(********************
*
* This operator returns the value of adding up the operands.
*
*********************)
FUNCTION ADD <T1: ANY, T2: ANY> : T1
VAR_INPUT
	IN1 : T1;
	IN2 : T2;
END_VAR
END_FUNCTION

(* Specialized implementation of ADD for TIME *)
FUNCTION ADD__TIME__TIME : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : TIME;
END_VAR
	ADD__TIME__TIME := ADD_TIME(IN1, IN2);
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

(* Specialized implementation of ADD for TOD *)
FUNCTION ADD__TIME_OF_DAY__TIME : TOD
VAR_INPUT
	IN1 : TOD;
	IN2 : TIME;
END_VAR
	ADD__TIME_OF_DAY__TIME := ADD_TOD_TIME(IN1, IN2);
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

(* Specialized implementation of ADD for DT *)
FUNCTION ADD__DATE_AND_TIME__TIME : DT
VAR_INPUT
	IN1 : DT;
	IN2 : TIME;
END_VAR
	ADD__DATE_AND_TIME__TIME := ADD_DT_TIME(IN1, IN2);
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
FUNCTION SUB <T1: ANY, T2: ANY> : T1
VAR_INPUT
	IN1 : T1;
	IN2 : T2;
END_VAR
END_FUNCTION

(* Specialized implementation of SUB for TIME *)
FUNCTION SUB__TIME__TIME : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : TIME;
END_VAR
	SUB__TIME__TIME := SUB_TIME(IN1, IN2);
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
FUNCTION SUB__DATE__DATE : TIME
VAR_INPUT
	IN1 : DATE;
	IN2 : DATE;
END_VAR
	SUB__DATE__DATE := SUB_DATE_DATE(IN1, IN2);
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

(* Specialized implementation of SUB for TOD and TIME *)
FUNCTION SUB__TIME_OF_DAY__TIME : TOD
VAR_INPUT
	IN1 : TOD;
	IN2 : TIME;
END_VAR
	SUB__TIME_OF_DAY__TIME := SUB_TOD_TIME(IN1, IN2);
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

(* Specialized implementation of SUB for TOD *)
FUNCTION SUB__TIME_OF_DAY__TIME_OF_DAY : TIME
VAR_INPUT
	IN1 : TOD;
	IN2 : TOD;
END_VAR
	SUB__TIME_OF_DAY__TIME_OF_DAY := SUB_TOD_TOD(IN1, IN2);
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

(* Specialized implementation of SUB for DT and TIME *)
FUNCTION SUB__DATE_AND_TIME__TIME : DT
VAR_INPUT
	IN1 : DT;
	IN2 : TIME;
END_VAR
	SUB__DATE_AND_TIME__TIME := SUB_DT_TIME(IN1, IN2);
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

(* Specialized implementation of SUB for DT *)
FUNCTION SUB__DATE_AND_TIME__DATE_AND_TIME : TIME
VAR_INPUT
	IN1 : DT;
	IN2 : DT;
END_VAR
	SUB__DATE_AND_TIME__DATE_AND_TIME := SUB_DT_DT(IN1, IN2);
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

(********************
*
* This operator produces the multiplication of the operands.
*
*********************)
FUNCTION MUL <T1: ANY, T2: ANY> : T1
VAR_INPUT
	IN1 : T1;
	IN2 : T2;
END_VAR
END_FUNCTION

(* Specialized implementation of MUL for TIME and SINT *)
FUNCTION MUL__TIME__SINT : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : SINT;
END_VAR
	MUL__TIME__SINT := CHECKED_MUL_SIGNED(IN1, IN2);
END_FUNCTION

(* Specialized implementation of MUL for TIME and INT *)
FUNCTION MUL__TIME__INT : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : INT;
END_VAR
	MUL__TIME__INT := CHECKED_MUL_SIGNED(IN1, IN2);
END_FUNCTION

(* Specialized implementation of MUL for TIME and DINT *)
FUNCTION MUL__TIME__DINT : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : DINT;
END_VAR
	MUL__TIME__DINT := CHECKED_MUL_SIGNED(IN1, IN2);
END_FUNCTION

(* Specialized implementation of MUL for TIME and LINT *)
FUNCTION MUL__TIME__LINT : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : LINT;
END_VAR
	MUL__TIME__LINT := CHECKED_MUL_SIGNED(IN1, IN2);
END_FUNCTION


(********************
*
* This operator produces the multiplication of TIME with ANY_SIGNED integer
* If the result overflows 0 will be returned
*
*********************)
@EXTERNAL
FUNCTION CHECKED_MUL_SIGNED : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : LINT;
END_VAR
END_FUNCTION

(* Specialized implementation of MUL for TIME and USINT *)
FUNCTION MUL__TIME__USINT : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : USINT;
END_VAR
	MUL__TIME__USINT := CHECKED_MUL_UNSIGNED(IN1, IN2);
END_FUNCTION

(* Specialized implementation of MUL for TIME and UINT *)
FUNCTION MUL__TIME__UINT : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : UINT;
END_VAR
	MUL__TIME__UINT := CHECKED_MUL_UNSIGNED(IN1, IN2);
END_FUNCTION

(* Specialized implementation of MUL for TIME and UDINT *)
FUNCTION MUL__TIME__UDINT : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : UDINT;
END_VAR
	MUL__TIME__UDINT := CHECKED_MUL_UNSIGNED(IN1, IN2);
END_FUNCTION

(* Specialized implementation of MUL for TIME and ULINT *)
FUNCTION MUL__TIME__ULINT : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : ULINT;
END_VAR
	MUL__TIME__ULINT := CHECKED_MUL_UNSIGNED(IN1, IN2);
END_FUNCTION


(********************
*
* This operator produces the multiplication of TIME with ANY_UNSIGNED integer
* If the result overflows 0 will be returned
*
*********************)
@EXTERNAL
FUNCTION CHECKED_MUL_UNSIGNED : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : ULINT;
END_VAR
END_FUNCTION

(********************
*
* This operator produces the multiplication of TIME and REAL
* If the result overflows 0 will be returned
*
*********************)
@EXTERNAL
FUNCTION CHECKED_MUL_F32 : TIME
VAR_INPUT
	IN1 : TIME;
	IN2 : REAL;
END_VAR
END_FUNCTION