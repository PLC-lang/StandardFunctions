(******************************************************************************
Description: String length
Input: 
    - IN:   A character string
Return: String length
******************************************************************************)
{external}
FUNCTION LEN <T: ANY_STRING> : DINT
VAR_INPUT {ref}
	IN : T;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Left
Input:
    - IN:   A character string
    - L:    The length of the substring
Return: A substring consisting of the leftmost L characters of IN
******************************************************************************)
{external}
FUNCTION LEFT <T: ANY_STRING, U: ANY_INT> : T
VAR_INPUT {ref}
	IN : T;
    L  : U;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Right
Input:
    - IN:   A character string
    - L:    The length of the substring
Return: A substring consisting of the rightmost L characters of IN
******************************************************************************)
{external}
FUNCTION RIGHT <T: ANY_STRING, U: ANY_INT> : T
VAR_INPUT {ref}
	IN : T;
    L  : U;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Middle
Input:
    - IN:   A character string
    - L:    The length of the substring
    - P:    The starting index position
Return: 
    A substring that contains L characters starting 
    from position P in a string.
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
Description: Extensible concatenation
Input:
    - IN:   Two or more comma-separated strings
Return:
    A string combining all given input strings in the same order
    as the given string parameters.
******************************************************************************)
{external}
FUNCTION CONCAT <T: ANY_STRING> : T
VAR_INPUT {ref}
	IN : {sized} T...;
END_VAR
END_FUNCTION

(******************************************************************************
Description: Insert
Input:
    IN1:    The string to insert into
    IN2:    The string to insert
    P:      The position at which to insert
Return:
    A string consisting of IN2 inserted at position P into string IN1
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
Description: Delete
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
Description: Replace
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
Description: Find
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

(******************************************************************************
Description: Left extended
Input:
    - IN:   A character string
    - L:    The length of the substring
Return: A substring consisting of the leftmost L characters of IN
******************************************************************************)
{external}
FUNCTION LEFT_EXT <T: ANY_STRING, U: ANY_INT> : T
VAR_INPUT {ref}
	IN : T;
    L  : U;
END_VAR
END_FUNCTION