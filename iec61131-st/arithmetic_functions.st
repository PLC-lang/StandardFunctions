(* Definitions of arithmetic functions defined by the IEC61131-3 standard *)

{external}
VAR_GLOBAL
	PI_REAL : REAL;
	PI_LREAL : LREAL;
	FRAC_PI_2_REAL : REAL;
	FRAC_PI_2_LREAL : LREAL;
	FRAC_PI_4_REAL : REAL;
	FRAC_PI_4_LREAL : LREAL;
	E_REAL : REAL;
	E_LREAL : LREAL;
END_VAR

(* Calculates the square root of a given value *)
{external}
FUNCTION SQRT<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(** Logarithmic functions **)


(* Calculates the natural logarithm of the given value *)
{external}
FUNCTION LN<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION


(* Calculates the base 10 logarithm of the given value *)
{external}
FUNCTION LOG<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(* Calculates the natural exporenent of the given value *)
{external}
FUNCTION EXP<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(** Trigonometric functions **)


(* Calculates the sine of the given value in radiants *)
{external}
FUNCTION SIN<T : ANY_REAL> : T
VAR_INPUT
	rad : T;
END_VAR
END_FUNCTION

(* Calculates the cosine of the given value in radiants *)
{external}
FUNCTION COS<T : ANY_REAL> : T
VAR_INPUT
	rad : T;
END_VAR
END_FUNCTION

(* Calculates the tangent of the given value in radiants *)
{external}
FUNCTION TAN<T : ANY_REAL> : T
VAR_INPUT
	rad : T;
END_VAR
END_FUNCTION

(* Calculates the arc sine of the given value *)
{external}
FUNCTION ASIN<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(* Calculates the arc cosine of the given value *)
{external}
FUNCTION ACOS<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(* Calculates the arc tangent of the given value *)
{external}
FUNCTION ATAN<T : ANY_REAL> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(* Calculate the four quadrant arc tangent of the given values *)
{external}
FUNCTION ATAN2<T : ANY_REAL> : T
VAR_INPUT
	y : T;
	x : T;
END_VAR
END_FUNCTION

{external}
FUNCTION EXPT<T: ANY_REAL, U: ANY_NUM> : T
VAR_INPUT 
	in1 : T;
	in2 : U;
END_VAR	
END_FUNCTION
