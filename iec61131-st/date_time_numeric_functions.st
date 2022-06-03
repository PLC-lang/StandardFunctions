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
*
*********************)
FUNCTION ADD_LDT_LTIME : LDT
VAR_INPUT
	IN1 : LDT;
	IN2 : LTIME;
END_VAR
	ADD_LDT_LTIME := ADD_DT_TIME(IN1, IN2);
END_FUNCTION