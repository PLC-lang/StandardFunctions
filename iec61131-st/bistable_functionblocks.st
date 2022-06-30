(******************************************************************************
Description: Bistable function, set dominant
Input:
  - SET1: Dominant input of bistable function.
  - RESET: input of bistable function.
Output:
  - Q1: Result of Bistable function. 
Return: Output is used as return
******************************************************************************)
{external}
FUNCTION_BLOCK SR
VAR_INPUT
	SET1: BOOL;	
	RESET: BOOL;
END_VAR
VAR_OUTPUT
	Q1: BOOL;
END_VAR
END_FUNCTION_BLOCK

(******************************************************************************
Description: Bistable function, reset dominant.
Input:
  - SET: Bistable input 
  - RESET1: Dominmant bistable input. 
Output:
  - Q1: Result of bistable function 
Return: Output variable used as return value.
******************************************************************************)
{external}
FUNCTION_BLOCK RS
VAR_INPUT
	SET: 	BOOL;
	RESET1: BOOL;
END_VAR
VAR_OUTPUT
	Q1:		BOOL;
END_VAR
END_FUNCTION_BLOCK