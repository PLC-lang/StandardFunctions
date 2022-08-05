// ANY_REAL and ANY_BIT bit-width constraint >=16 -> how?

(********************
*
* Converts little endian data format to big endian data format
*
*********************)
{external}
FUNCTION TO_BIG_ENDIAN<T: ANY_INT> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(********************
*
* Converts big endian data format to little endian data format
*
*********************)
{external}
FUNCTION TO_LITTLE_ENDIAN<T: ANY_INT> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(********************
*
* Converts to little endian data format from big endian data format
*
*********************)
{external}
FUNCTION FROM_BIG_ENDIAN<T: ANY_INT> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION

(********************
*
* Converts to big endian data format from little endian data format
*
*********************)
{external}
FUNCTION FROM_LITTLE_ENDIAN<T: ANY_INT> : T
VAR_INPUT
	in : T;
END_VAR
END_FUNCTION






// (********************
// *
// * Converts little endian data format to big endian data format
// *
// *********************)
// {external}
// FUNCTION TO_BIG_ENDIAN__INT: INT
// VAR_INPUT
// 	in : INT;
// END_VAR
//     TO_BIG_ENDIAN__INT := in;
// END_FUNCTION

// (********************
// *
// * Converts big endian data format to little endian data format
// *
// *********************)
// {external}
// FUNCTION TO_LITTLE_ENDIAN__INT: INT
// VAR_INPUT
// 	in : INT;
// END_VAR
//     TO_BIG_ENDIAN__INT := in;
// END_FUNCTION

// (********************
// *
// * Converts to little endian data format from big endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_BIG_ENDIAN__INT: INT
// VAR_INPUT
// 	in : INT;
// END_VAR
//     TO_BIG_ENDIAN__INT := in;
// END_FUNCTION

// (********************
// *
// * Converts to big endian data format from little endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_LITTLE_ENDIAN__INT: INT
// VAR_INPUT
// 	in : INT;
// END_VAR
//     TO_BIG_ENDIAN__INT := in;
// END_FUNCTION

// (********************
// *
// * Converts little endian data format to big endian data format
// *
// *********************)
// {external}
// FUNCTION TO_BIG_ENDIAN__DINT: DINT
// VAR_INPUT
// 	in : INT;
// END_VAR
//     TO_BIG_ENDIAN__INT := in;
// END_FUNCTION

// (********************
// *
// * Converts big endian data format to little endian data format
// *
// *********************)
// {external}
// FUNCTION TO_LITTLE_ENDIAN__DINT: DINT
// VAR_INPUT
// 	in : INT;
// END_VAR
//     TO_BIG_ENDIAN__INT := in;
// END_FUNCTION

// (********************
// *
// * Converts to little endian data format from big endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_BIG_ENDIAN__DINT: DINT

// END_FUNCTION

// (********************
// *
// * Converts to big endian data format from little endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_LITTLE_ENDIAN__DINT: DINT

// END_FUNCTION

// (********************
// *
// * Converts little endian data format to big endian data format
// *
// *********************)
// {external}
// FUNCTION TO_BIG_ENDIAN__LINT: LINT

// END_FUNCTION

// (********************
// *
// * Converts big endian data format to little endian data format
// *
// *********************)
// {external}
// FUNCTION TO_LITTLE_ENDIAN__LINT: LINT

// END_FUNCTION

// (********************
// *
// * Converts to little endian data format from big endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_BIG_ENDIAN__LINT: LINT

// END_FUNCTION

// (********************
// *
// * Converts to big endian data format from little endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_LITTLE_ENDIAN__LINT: LINT

// END_FUNCTION

// (********************
// *
// * Converts little endian data format to big endian data format
// *
// *********************)
// {external}
// FUNCTION TO_BIG_ENDIAN__UINT: UINT

// END_FUNCTION

// (********************
// *
// * Converts big endian data format to little endian data format
// *
// *********************)
// {external}
// FUNCTION TO_LITTLE_ENDIAN__UINT: UINT

// END_FUNCTION

// (********************
// *
// * Converts to little endian data format from big endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_BIG_ENDIAN__UINT: UINT

// END_FUNCTION

// (********************
// *
// * Converts to big endian data format from little endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_LITTLE_ENDIAN__UINT: UINT

// END_FUNCTION

// (********************
// *
// * Converts little endian data format to big endian data format
// *
// *********************)
// {external}
// FUNCTION TO_BIG_ENDIAN__UDINT: UDINT

// END_FUNCTION

// (********************
// *
// * Converts big endian data format to little endian data format
// *
// *********************)
// {external}
// FUNCTION TO_LITTLE_ENDIAN__UDINT: UDINT

// END_FUNCTION

// (********************
// *
// * Converts to little endian data format from big endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_BIG_ENDIAN__UDINT: UDINT

// END_FUNCTION

// (********************
// *
// * Converts to big endian data format from little endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_LITTLE_ENDIAN__UDINT: UDINT

// END_FUNCTION

// (********************
// *
// * Converts little endian data format to big endian data format
// *
// *********************)
// {external}
// FUNCTION TO_BIG_ENDIAN__ULINT: ULINT

// END_FUNCTION

// (********************
// *
// * Converts big endian data format to little endian data format
// *
// *********************)
// {external}
// FUNCTION TO_LITTLE_ENDIAN__ULINT: ULINT

// END_FUNCTION

// (********************
// *
// * Converts to little endian data format from big endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_BIG_ENDIAN__ULINT: ULINT

// END_FUNCTION

// (********************
// *
// * Converts to big endian data format from little endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_LITTLE_ENDIAN__ULINT: ULINT

// END_FUNCTION

// (********************
// *
// * Converts little endian data format to big endian data format
// *
// *********************)
// {external}
// FUNCTION TO_BIG_ENDIAN__REAL: REAL

// END_FUNCTION

// (********************
// *
// * Converts big endian data format to little endian data format
// *
// *********************)
// {external}
// FUNCTION TO_LITTLE_ENDIAN__REAL: REAL

// END_FUNCTION

// (********************
// *
// * Converts to little endian data format from big endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_BIG_ENDIAN__REAL: REAL

// END_FUNCTION

// (********************
// *
// * Converts to big endian data format from little endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_LITTLE_ENDIAN__REAL: REAL

// END_FUNCTION

// (********************
// *
// * Converts little endian data format to big endian data format
// *
// *********************)
// {external}
// FUNCTION TO_BIG_ENDIAN__LREAL: LREAL

// END_FUNCTION

// (********************
// *
// * Converts big endian data format to little endian data format
// *
// *********************)
// {external}
// FUNCTION TO_LITTLE_ENDIAN__LREAL: LREAL

// END_FUNCTION

// (********************
// *
// * Converts to little endian data format from big endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_BIG_ENDIAN__LREAL: LREAL

// END_FUNCTION

// (********************
// *
// * Converts to big endian data format from little endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_LITTLE_ENDIAN__LREAL: LREAL

// END_FUNCTION

// (********************
// *
// * Converts little endian data format to big endian data format
// *
// *********************)
// {external}
// FUNCTION TO_BIG_ENDIAN__WORD: WORD

// END_FUNCTION

// (********************
// *
// * Converts big endian data format to little endian data format
// *
// *********************)
// {external}
// FUNCTION TO_LITTLE_ENDIAN__WORD: WORD

// END_FUNCTION

// (********************
// *
// * Converts to little endian data format from big endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_BIG_ENDIAN__WORD: WORD

// END_FUNCTION

// (********************
// *
// * Converts to big endian data format from little endian data format
// *
// *********************)
// {external}
// FUNCTION FROM_LITTLE_ENDIAN__WORD: WORD

// END_FUNCTION