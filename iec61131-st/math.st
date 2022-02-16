(* Definitions of arithmetic functions defined by the IEC61131-3 standard *)

(* Calculates the square root of a given value *)
@EXTERNAL
FUNCTION SQRT<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(** Logarithmic functions **)


(* Calculates the natural logarithm of the given value *)
@EXTERNAL
FUNCTION LN<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION


(* Calculates the base 10 logarithm of the given value *)
@EXTERNAL
FUNCTION LOG<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(* Calculates the natural exporenent of the given value *)
@EXTERNAL
FUNCTION EXP<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(** Trigonometric functions **)


(* Calculates the sine of the given value in radiants *)
@EXTERNAL
FUNCTION SIN<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(* Calculates the cosine of the given value in radiants *)
@EXTERNAL
FUNCTION COS<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(* Calculates the tangent of the given value in radiants *)
@EXTERNAL
FUNCTION TAN<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(* Calculates the arc sine of the given value *)
@EXTERNAL
FUNCTION ASIN<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(* Calculates the arc cosine of the given value *)
@EXTERNAL
FUNCTION ACOS<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(* Calculates the arc tangent of the given value *)
@EXTERNAL
FUNCTION ATAN<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(* Calculate the four quadrant arc tangent of the given values *)
@EXTERNAL
FUNCTION ATAN2<T : ANY_REAL> : T
VAR_INPUT
	x : T;
	y : T;
END_VAR
END_FUNCTION
