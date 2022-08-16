(******************************************************************************
Description: String length
Input: 
    - IN:   A character string
Return: String length
******************************************************************************)
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
FUNCTION LEFT <T: ANY_STRING, U: ANY_INT> : T
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : U;
END_VAR
END_FUNCTION

{external}
FUNCTION LEFT_EXT<T: ANY_STRING, U: ANY_INT> : DINT
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : U;
END_VAR
VAR_IN_OUT
    OUT: T;
END_VAR
END_FUNCTION

FUNCTION LEFT__STRING__DINT : STRING 
VAR_INPUT {ref}
	IN : STRING;
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    LEFT_EXT(IN, L, LEFT__STRING__DINT);
END_FUNCTION

FUNCTION LEFT__STRING__LINT : STRING 
VAR_INPUT {ref}
	IN : STRING;
END_VAR
VAR_INPUT
    L  : LINT;
END_VAR
    LEFT_EXT(IN, L, LEFT__STRING__LINT);
END_FUNCTION

FUNCTION LEFT__WSTRING__DINT : WSTRING
VAR_INPUT {ref}
	IN : WSTRING;
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    LEFT_EXT(IN, L, LEFT__WSTRING__DINT);
END_FUNCTION

FUNCTION LEFT__WSTRING__LINT : WSTRING
VAR_INPUT {ref}
	IN : WSTRING;
END_VAR
VAR_INPUT
    L  : LINT;
END_VAR
    LEFT_EXT(IN, L, LEFT__WSTRING__LINT);
END_FUNCTION

(******************************************************************************
Description: Right
Input:
    - IN:   A character string
    - L:    The length of the substring
Return: A substring consisting of the rightmost L characters of IN
******************************************************************************)
FUNCTION RIGHT <T: ANY_STRING, U: ANY_INT> : T
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : U;
END_VAR
END_FUNCTION

{external}
FUNCTION RIGHT_EXT<T: ANY_STRING, U: ANY_INT> : DINT
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : U;
END_VAR
VAR_IN_OUT
    OUT: T;
END_VAR
END_FUNCTION

FUNCTION RIGHT__STRING__DINT : STRING[1024] 
VAR_INPUT {ref}
	IN : STRING[1024];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    RIGHT_EXT(IN, L, RIGHT__STRING__DINT);
END_FUNCTION

FUNCTION RIGHT__STRING__LINT : STRING[1024] 
VAR_INPUT {ref}
	IN : STRING[1024];
END_VAR
VAR_INPUT
    L  : LINT;
END_VAR
    RIGHT_EXT(IN, L, RIGHT__STRING__LINT);
END_FUNCTION

FUNCTION RIGHT__WSTRING__DINT : WSTRING[1024] 
VAR_INPUT {ref}
	IN : WSTRING[1024];
END_VAR
VAR_INPUT
    L  : DINT;
END_VAR
    RIGHT_EXT(IN, L, RIGHT__WSTRING__DINT);
END_FUNCTION

FUNCTION RIGHT__WSTRING__LINT : WSTRING[1024] 
VAR_INPUT {ref}
	IN : WSTRING[1024];
END_VAR
VAR_INPUT
    L  : LINT;
END_VAR
    RIGHT_EXT(IN, L, RIGHT__WSTRING__LINT);
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
END_VAR
VAR_INPUT
    L  : U;
    P  : U;
END_VAR
END_FUNCTION

{external}
FUNCTION MID_EXT<T: ANY_STRING, U: ANY_INT> : DINT
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : U;
    P  : U;
END_VAR
VAR_IN_OUT
    OUT: T;
END_VAR
END_FUNCTION

FUNCTION MID__STRING__DINT : STRING[1024] 
VAR_INPUT {ref}
	IN : STRING[1024];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    MID_EXT(IN, L, MID);
END_FUNCTION

FUNCTION MID__STRING__LINT : STRING[1024] 
VAR_INPUT {ref}
	IN : STRING[1024];
END_VAR
VAR_INPUT
    L  : LINT;
    P  : LINT;
END_VAR
    MID_EXT(IN, L, MID);
END_FUNCTION

FUNCTION MID__WSTRING__DINT : WSTRING[1024] 
VAR_INPUT {ref}
	IN : WSTRING[1024];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    MID_EXT(IN, L, MID);
END_FUNCTION

FUNCTION MID__WSTRING__LINT : WSTRING[1024] 
VAR_INPUT {ref}
	IN : WSTRING[1024];
END_VAR
VAR_INPUT
    L  : LINT;
    P  : LINT;
END_VAR
    MID_EXT(IN, L, MID);
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
END_VAR
VAR_INPUT    
    P   : U;
END_VAR
END_FUNCTION

{external}
FUNCTION INSERT_EXT<T: ANY_STRING, U: ANY_INT> : DINT
VAR_INPUT {ref}
	IN1 : T;
    IN2 : T;
END_VAR
VAR_INPUT    
    P   : U;
END_VAR
VAR_IN_OUT
    OUT: T;
END_VAR
END_FUNCTION

FUNCTION INSERT__STRING__DINT : STRING[1024] 
VAR_INPUT {ref}
	IN1 : STRING[1024];
    IN2 : STRING[1024];
END_VAR
VAR_INPUT    
    P   : DINT;
END_VAR
    INSERT_EXT(IN1, IN2, P, INSERT);
END_FUNCTION

FUNCTION INSERT__STRING__LINT : STRING[1024] 
VAR_INPUT {ref}
	IN1 : STRING[1024];
    IN2 : STRING[1024];
END_VAR
VAR_INPUT    
    P   : LINT;
END_VAR
    INSERT_EXT(IN1, IN2, P, INSERT);
END_FUNCTION

FUNCTION INSERT__WSTRING__DINT : WSTRING[1024] 
VAR_INPUT {ref}
	IN1 : WSTRING[1024];
    IN2 : WSTRING[1024];
END_VAR
VAR_INPUT    
    P   : DINT;
END_VAR
    INSERT_EXT(IN1, IN2, P, INSERT);
END_FUNCTION

FUNCTION INSERT__WSTRING__LINT : WSTRING[1024] 
VAR_INPUT {ref}
	IN1 : WSTRING[1024];
    IN2 : WSTRING[1024];
END_VAR
VAR_INPUT    
    P   : LINT;
END_VAR
    INSERT_EXT(IN1, IN2, P, INSERT);
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
END_VAR
VAR_INPUT
    L  : U;
    P  : U;
END_VAR
END_FUNCTION

{external}
FUNCTION DELETE_EXT<T: ANY_STRING, U: ANY_INT> : DINT
VAR_INPUT {ref}
	IN : T;
END_VAR
VAR_INPUT
    L  : U;
    P  : U;
END_VAR
VAR_IN_OUT
    OUT: T;
END_VAR
END_FUNCTION

FUNCTION DELETE__STRING__DINT : STRING[1024] 
VAR_INPUT {ref}
	IN : STRING[1024];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    DELETE_EXT(IN, L, P, DELETE);
END_FUNCTION

FUNCTION DELETE__STRING__LINT : STRING[1024] 
VAR_INPUT {ref}
	IN : STRING[1024];
END_VAR
VAR_INPUT
    L  : LINT;
    P  : LINT;
END_VAR
    DELETE_EXT(IN, L, P, DELETE);
END_FUNCTION

FUNCTION DELETE__WSTRING__DINT : WSTRING[1024] 
VAR_INPUT {ref}
	IN : WSTRING[1024];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    DELETE_EXT(IN, L, P, DELETE);
END_FUNCTION

FUNCTION DELETE__WSTRING__LINT : WSTRING[1024] 
VAR_INPUT {ref}
	IN : WSTRING[1024];
END_VAR
VAR_INPUT
    L  : LINT;
    P  : LINT;
END_VAR
    DELETE_EXT(IN, L, P, DELETE);
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
END_VAR
VAR_INPUT
    L  : U;
    P  : U;
END_VAR
END_FUNCTION

{external}
FUNCTION REPLACE_EXT <T: ANY_STRING, U: ANY_INT> : DINT
VAR_INPUT {ref}
    IN1 : T;
    IN2 : T;
END_VAR
VAR_INPUT 
    L   : U;
    P   : U;
END_VAR
VAR_IN_OUT
    OUT : T;
END_VAR    
END_FUNCTION

{external}
FUNCTION REPLACE__STRING__DINT : STRING[1024]
VAR_INPUT {ref}
	IN1 : STRING[1024];
    IN2 : STRING[1024];
END_VAR
VAR_INPUT
    L  : DINT;
    P  : DINT;
END_VAR
    REPLACE_EXT(IN1, IN2, L, P, REPLACE);
END_FUNCTION

{external}
FUNCTION REPLACE__STRING__LINT : STRING[1024]
VAR_INPUT {ref}
    IN1 : STRING[1024];
    IN2 : STRING[1024];
END_VAR
VAR_INPUT
    L   : LINT;
    P   : LINT;
END_VAR
    REPLACE_EXT(IN1, IN2, L, P, REPLACE);
END_FUNCTION

{external}
FUNCTION REPLACE__WSTRING__DINT : WSTRING[1024]
VAR_INPUT {ref}
    IN1 : WSTRING[1024];
    IN2 : WSTRING[1024];
END_VAR    
VAR_INPUT 
    L   : DINT;
    P   : DINT;
END_VAR
    REPLACE_EXT(IN1, IN2, L, P, REPLACE);
END_FUNCTION

{external}
FUNCTION REPLACE__WSTRING__LINT : WSTRING[1024]
VAR_INPUT {ref}
    IN1 : WSTRING[1024];
    IN2 : WSTRING[1024];
END_VAR    
VAR_INPUT 
    L   : LINT;
    P   : LINT;
END_VAR
    REPLACE_EXT(IN1, IN2, L, P, REPLACE);
END_FUNCTION

(******************************************************************************
Description: Find
Input:
Return:
******************************************************************************)
{external}
FUNCTION FIND <T: ANY_STRING> : DINT
VAR_INPUT {ref}
	IN1 : T;
    IN2 : T;
END_VAR
END_FUNCTION
