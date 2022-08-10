(******************************************************************************
Description: 
Input:
Return:
******************************************************************************)
{external}
FUNCTION LEN <T: ANY_STRING> : DINT
VAR_INPUT {ref}
	IN : T;
END_VAR
END_FUNCTION

(******************************************************************************
Description: 
Input:
Return:
******************************************************************************)
{external}
FUNCTION LEFT <T: ANY_STRING> : T
VAR_INPUT {ref}
	IN : T;
END_VAR
END_FUNCTION

(******************************************************************************
Description: 
Input:
Return:
******************************************************************************)
{external}
FUNCTION RIGHT <T: ANY_STRING, U: ANY_INT> : T
VAR_INPUT {ref}
	IN : T;
    L  : U;
END_VAR
END_FUNCTION

(******************************************************************************
Description: 
Input:
Return:
******************************************************************************)
{external}
FUNCTION MID <T: ANY_STRING, U: ANY_INT> : T
VAR_INPUT {ref}
	IN : T;
    L  : U;
    P  : U;
END_VAR
END_FUNCTION

(******************************************************************************
Description: 
Input:
Return:
******************************************************************************)
{external}
FUNCTION CONCAT <T: ANY_STRING> : T
VAR_INPUT {ref}
	IN : {sized} T...;
END_VAR
END_FUNCTION

(******************************************************************************
Description: 
Input:
Return:
******************************************************************************)
{external}
FUNCTION INSERT <T: ANY_STRING, U: ANY_INT> : T
VAR_INPUT {ref}
	IN1 : T;
    IN2 : T;
    P   : U;
END_VAR
END_FUNCTION

(******************************************************************************
Description: 
Input:
Return:
******************************************************************************)
{external}
FUNCTION DELETE <T: ANY_STRING, U: ANY_INT> : T
VAR_INPUT {ref}
	IN : T;
    L  : U;
    P  : U;
END_VAR
END_FUNCTION

(******************************************************************************
Description: 
Input:
Return:
******************************************************************************)
{external}
FUNCTION REPLACE <T: ANY_STRING, U: ANY_INT> : T
VAR_INPUT {ref}
	IN1 : T;
    IN2 : T;
    L  :  U;
    P  :  U;
END_VAR
END_FUNCTION

(******************************************************************************
Description: 
Input:
Return:
******************************************************************************)
{external}
FUNCTION FIND <T: ANY_STRING> : DINT
VAR_IN_OUT {ref}
	IN1 : T;
    IN2 : U;
END_VAR
END_FUNCTION

