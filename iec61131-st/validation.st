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

(******************************************************************************
Description: Validation function for ANY_BIT
Validate if the binary representation of the input
would result in a valid BCD representation
Input:
  - IN: ANY_BIT
Return:
  - TRUE: Value of each 4bits(nibble) is between 0-9, e.g 0110 => 6
  - FALSE: If any 4bits(nibble) is between 10-15, e.g 1101 => 13
******************************************************************************)
FUNCTION IS_VALID_BCD <T: ANY_BIT> : BOOL
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION