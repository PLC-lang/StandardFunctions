/* (************************** */
/*  * */ 
/*  *This operator returns the absolute value of a number. */
/*  * */ 
/* *************************) */
/* FUNCTION ABS<T : ANY_NUM> : T */ 
/* VAR_INPUT */
/* 	IN : T; */
/* END_VAR */

/* END_FUNCTION */

(* Specialized implementation of ABS for DINT *)
/* FUNCTION ABS__DINT : DINT */
FUNCTION ABS : DINT //__DINT : DINT
VAR_INPUT
	IN : DINT;
END_VAR
	ABS := IN;
	/* ABS__DINT := IN; */
	IF ABS < 0 THEN
	/* IF ABS__DINT < 0 THEN */
		ABS := -1 * ABS;
		/* ABS__DINT := -1 * ABS__DINT; */
	END_IF
END_FUNCTION

/* (* Specialized implementation of ABS for LINT *) */
/* FUNCTION ABS__LINT : LINT */
/* VAR_INPUT */
/* 	IN : LINT; */
/* END_VAR */
/* 	ABS__LINT := IN; */
/* 	IF ABS__LINT < 0 THEN */
/* 		ABS__LINT := -1 * ABS__LINT; */
/* 	END_IF */
/* END_FUNCTION */
