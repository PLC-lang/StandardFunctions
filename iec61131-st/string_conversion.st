(********************
*
* Converts WSTRING to STRING
*
*********************)
FUNCTION WSTRING_TO_STRING : STRING
VAR_INPUT
	in : WSTRING;
END_VAR
VAR
	ptr : REF_TO STRING;
END_VAR
	ptr := &in;
	WSTRING_TO_STRING := ptr^;
END_FUNCTION

(********************
*
* Converts WSTRING to WCHAR
*
*********************)
FUNCTION WSTRING_TO_WCHAR : WCHAR
VAR_INPUT
	in : WSTRING;
END_VAR
VAR
	ptr : REF_TO WCHAR;
END_VAR
	ptr := &in;
	WSTRING_TO_WCHAR := ptr^;
END_FUNCTION

(********************
*
* Converts STRING to WSTRING
*
*********************)
FUNCTION STRING_TO_WSTRING : WSTRING
VAR_INPUT
	in : STRING;
END_VAR
VAR
	ptr : REF_TO WSTRING;
END_VAR
	ptr := &in;
	STRING_TO_WSTRING := ptr^;
END_FUNCTION

(********************
*
* Converts STRING to CHAR
*
*********************)
FUNCTION STRING_TO_CHAR : CHAR
VAR_INPUT
	in : STRING;
END_VAR
VAR
	ptr : REF_TO CHAR;
END_VAR
	ptr := &in;
	STRING_TO_CHAR := ptr^;
END_FUNCTION

(********************
*
* Converts WCHAR to WSTRING
*
*********************)
FUNCTION WCHAR_TO_WSTRING : WSTRING
VAR_INPUT
	in : WCHAR;
END_VAR
VAR
	ptr : REF_TO WSTRING;
END_VAR
	ptr := &in;
	WCHAR_TO_WSTRING := ptr^;
END_FUNCTION

(********************
*
* Converts WCHAR to CHAR
*
*********************)
FUNCTION WCHAR_TO_CHAR : CHAR
VAR_INPUT
	in : WCHAR;
END_VAR
VAR
	ptr : REF_TO CHAR;
END_VAR
	ptr := &in;
	WCHAR_TO_CHAR := ptr^;
END_FUNCTION

(********************
*
* Converts CHAR to STRING
*
*********************)
FUNCTION CHAR_TO_STRING : STRING
VAR_INPUT
	in : CHAR;
END_VAR
VAR
	ptr : REF_TO STRING;
END_VAR
	ptr := &in;
	CHAR_TO_STRING := ptr^;
END_FUNCTION

(********************
*
* Converts CHAR to WCHAR
*
*********************)
FUNCTION CHAR_TO_WCHAR : WCHAR
VAR_INPUT
	in : CHAR;
END_VAR
VAR
	ptr : REF_TO WCHAR;
END_VAR
	ptr := &in;
	CHAR_TO_WCHAR := ptr^;
END_FUNCTION