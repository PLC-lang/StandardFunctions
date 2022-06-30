(******************************************************************************
Description: Validation function for ANY_REAL
Input:
  - IN: ANY_REAL
Return:
  - FALSE: If input is Not-a-Number(NaN) or infinite(+Inf, -Inf)
  - TRUE: Otherwise
******************************************************************************)
FUNCTION IS_VALID <T: ANY_REAL> : BOOL
VAR_INPUT
	IN : T;
END_VAR
END_FUNCTION

(* Specialized implementation of IS_VALID for REAL *)
{external}
FUNCTION IS_VALID__REAL : BOOL
VAR_INPUT
	IN : REAL;
END_VAR
END_FUNCTION

(* Specialized implementation of IS_VALID for LREAL *)
{external}
FUNCTION IS_VALID__LREAL : BOOL
VAR_INPUT
	IN : LREAL;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Validation function for ANY_BIT
Validate if the binary representation of the input
would result in a valid BCD representation
Input:
  - IN: ANY_BIT
Return:
  - TRUE: The value of each 4bits represented digit must be between 0-9
  - FALSE: If the value of any 4bits represented digit is higher than 9
******************************************************************************)
FUNCTION IS_VALID_BCD <T: ANY_BIT> : BOOL
VAR_INPUT
	IN : T;
END_VAR
END_FUNCTION

(* Specialized implementation of IS_VALID_BCD for BYTE *)
{external}
FUNCTION IS_VALID_BCD__BYTE : BOOL
VAR_INPUT
	IN : BYTE;
END_VAR
END_FUNCTION

(* Specialized implementation of IS_VALID_BCD for WORD *)
{external}
FUNCTION IS_VALID_BCD__WORD : BOOL
VAR_INPUT
	IN : WORD;
END_VAR
END_FUNCTION

(* Specialized implementation of IS_VALID_BCD for DWORD *)
{external}
FUNCTION IS_VALID_BCD__DWORD : BOOL
VAR_INPUT
	IN : DWORD;
END_VAR
END_FUNCTION

(* Specialized implementation of IS_VALID_BCD for LWORD *)
{external}
FUNCTION IS_VALID_BCD__LWORD : BOOL
VAR_INPUT
	IN : LWORD;
END_VAR
END_FUNCTION