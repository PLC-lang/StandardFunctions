// Definitions of the core standard function modules for IEC61131-3

use std::{ffi::CStr, os::raw::c_char};

use chrono::TimeZone;

pub mod bit_shift;
pub mod time;
pub mod timers;
pub mod utils;
pub mod validation;

#[no_mangle]
pub static PI_LREAL: f64 = std::f64::consts::PI;
#[no_mangle]
pub static PI_REAL: f32 = std::f32::consts::PI;
#[no_mangle]
pub static FRAC_PI_2_LREAL: f64 = std::f64::consts::FRAC_PI_2;
#[no_mangle]
pub static FRAC_PI_2_REAL: f32 = std::f32::consts::FRAC_PI_2;
#[no_mangle]
pub static FRAC_PI_4_LREAL: f64 = std::f64::consts::FRAC_PI_4;
#[no_mangle]
pub static FRAC_PI_4_REAL: f32 = std::f32::consts::FRAC_PI_4;
#[no_mangle]
pub static E_REAL: f32 = std::f32::consts::E;
#[no_mangle]
pub static E_LREAL: f64 = std::f64::consts::E;

/// .
/// Rounds a REAL (f32) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ROUND__REAL(input: f32) -> f32 {
    input.round()
}

/// .
/// Rounds a LREAL (f64) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ROUND__LREAL(input: f64) -> f64 {
    input.round()
}

/// .
/// Calculates the square root of the given (f32) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQRT__REAL(input: f32) -> f32 {
    f32::sqrt(input)
}

/// .
/// Calculates the square root of the given (f64) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQRT__LREAL(input: f64) -> f64 {
    f64::sqrt(input)
}

/// .
/// Calculates the natural logarithm of the given (f32) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LN__REAL(input: f32) -> f32 {
    f32::ln(input)
}

/// .
/// Calculates the natural logarithm of the given (f64) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LN__LREAL(input: f64) -> f64 {
    f64::ln(input)
}

/// .
/// Calculates the base 10 logarithm of the given (f32) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LOG__REAL(input: f32) -> f32 {
    f32::log10(input)
}

/// .
/// Calculates the natural logarithm of the given (f64) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LOG__LREAL(input: f64) -> f64 {
    f64::log10(input)
}

/// .
/// The natural exponential function (e)
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXP__REAL(input: f32) -> f32 {
    f32::exp(input)
}

/// .
/// The natural exponential function (e)
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXP__LREAL(input: f64) -> f64 {
    f64::exp(input)
}

///
/// .
/// Calculates the sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SIN__REAL(input: f32) -> f32 {
    f32::sin(input)
}

/// .
/// Calculates the sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SIN__LREAL(input: f64) -> f64 {
    f64::sin(input)
}

/// .
/// Calculates the cosine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn COS__REAL(input: f32) -> f32 {
    f32::cos(input)
}

/// .
/// Calculates the cosine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn COS__LREAL(input: f64) -> f64 {
    f64::cos(input)
}

/// .
/// Calculates the tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TAN__REAL(input: f32) -> f32 {
    f32::tan(input)
}

/// .
/// Calculates the tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TAN__LREAL(input: f64) -> f64 {
    f64::tan(input)
}

/// .
/// Calculates the arc sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ASIN__REAL(input: f32) -> f32 {
    f32::asin(input)
}

/// .
/// Calculates the arc sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ASIN__LREAL(input: f64) -> f64 {
    f64::asin(input)
}

/// .
/// Calculates the arc cosine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ACOS__REAL(input: f32) -> f32 {
    f32::acos(input)
}

/// .
/// Calculates the arc cosine of the given value in radiants
///
/// # Examples
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ACOS__LREAL(input: f64) -> f64 {
    f64::acos(input)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN__REAL(input: f32) -> f32 {
    f32::atan(input)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN__LREAL(input: f64) -> f64 {
    f64::atan(input)
}

/// .
/// Calculates the four quadrant arc tangent of the value with another value
///
/// # Examples
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN2__REAL(in1: f32, in2: f32) -> f32 {
    in1.atan2(in2)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN2__LREAL(in1: f64, in2: f64) -> f64 {
    in1.atan2(in2)
}

/// .
/// Converts LWORD to LREAL
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LWORD_TO_LREAL(input: u64) -> f64 {
    f64::from_bits(input)
}

/// .
/// Converts DWORD to REAL
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DWORD_TO_REAL(input: u32) -> f32 {
    f32::from_bits(input)
}

/// .
/// Converts LREAL to LWORD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LREAL_TO_LWORD(input: f64) -> u64 {
    f64::to_bits(input)
}

/// .
/// Converts REAL to DWORD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn REAL_TO_DWORD(input: f32) -> u32 {
    f32::to_bits(input)
}

/// .
/// Converts WSTRING to STRING
/// Limited by a return type of 80 charachters
///
/// # Safety
///
/// Works on string pointer conversion, inherently unsafe
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn WSTRING_TO_STRING(input: *const i16) -> Wrapper<[u8; 81]> {
    let mut widestring = input;
    let mut count = 0;
    let len = loop {
        if widestring.is_null() || *widestring == 0 {
            break count;
        }
        widestring = widestring.add(1);
        count += 1;
    };

    let input = std::slice::from_raw_parts(input as *const u16, len);

    let string = String::from_utf16_lossy(input);
    let mut arr = [0; 81];
    for (idx, b) in string.bytes().enumerate() {
        //Don't fill the null terminator
        if idx < arr.len() - 1 {
            arr[idx] = b;
        } else {
            //no need to loop further, the target size of 80 is done
            break;
        }
    }
    Wrapper { inner: arr }
}

/// .
/// Converts STRING to WSTRING
/// Limited by a return type of 80 charachters
///
/// # Safety
///
/// Works on string pointer conversion, inherently unsafe
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_WSTRING(input: *const c_char) -> Wrapper<[u16; 81]> {
    //find the \0
    let string = CStr::from_ptr(input).to_string_lossy();
    let mut arr: [u16; 81] = [0; 81];
    for (i, e) in string.encode_utf16().enumerate() {
        //Don't fill the null terminator
        if i < arr.len() - 1 {
            arr[i] = e;
        } else {
            //No need to go further if the string is bigger than the target string
            break;
        }
    }
    Wrapper { inner: arr }
}

/// .
/// Converts WCHAR to CHAR
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn WCHAR_TO_CHAR(input: u16) -> u8 {
    let u16_arr = [input];
    let mut res_iter = char::decode_utf16(u16_arr.into_iter())
        .map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER));
    let mut res_arr = [u8::MAX; 80];
    if let Some(res) = res_iter.next() {
        if res_iter.next().is_none() {
            res.encode_utf8(&mut res_arr);
        }
    }
    res_arr[0]
}

/// .
/// Converts CHAR to WCHAR
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CHAR_TO_WCHAR(input: u8) -> u16 {
    let res: char = input.into();
    let mut arr = [u16::MAX; 2];
    res.encode_utf16(&mut arr);
    arr[0]
}

/// .
/// This operator returns the value of adding up two TIME operands.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ADD_TIME(in1: i64, in2: i64) -> i64 {
    chrono::Duration::milliseconds(in1)
        .checked_add(&chrono::Duration::milliseconds(in2))
        .unwrap()
        .num_milliseconds()
}

/// .
/// This operator returns the value of adding up TOD and TIME.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ADD_TOD_TIME(in1: i64, in2: i64) -> i64 {
    add_datetime_time(in1, in2)
}

/// .
/// This operator returns the value of adding up DT and TIME.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ADD_DT_TIME(in1: i64, in2: i64) -> i64 {
    add_datetime_time(in1, in2)
}

fn add_datetime_time(in1: i64, in2: i64) -> i64 {
    chrono::Utc
        .timestamp_millis(in1)
        .checked_add_signed(chrono::Duration::nanoseconds(in2))
        .unwrap()
        .timestamp_millis()
}

/// .
/// This operator produces the subtraction of two TIME operands
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SUB_TIME(in1: i64, in2: i64) -> i64 {
    chrono::Duration::milliseconds(in1)
        .checked_sub(&chrono::Duration::milliseconds(in2))
        .unwrap()
        .num_milliseconds()
}

/// .
/// This operator produces the subtraction of two DATE operands
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SUB_DATE_DATE(in1: i64, in2: i64) -> i64 {
    sub_datetimes(in1, in2)
}

/// .
/// This operator produces the subtraction of TOD and TIME
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SUB_TOD_TIME(in1: i64, in2: i64) -> i64 {
    sub_datetime_duration(in1, in2)
}

/// .
/// This operator produces the subtraction of two TOD operands
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SUB_TOD_TOD(in1: i64, in2: i64) -> i64 {
    sub_datetimes(in1, in2)
}

fn sub_datetimes(in1: i64, in2: i64) -> i64 {
    chrono::Utc
        .timestamp_millis(in1)
        .signed_duration_since(chrono::Utc.timestamp_millis(in2))
        .num_nanoseconds()
        .unwrap()
}

/// .
/// This operator produces the subtraction of DT and TIME
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SUB_DT_TIME(in1: i64, in2: i64) -> i64 {
    sub_datetime_duration(in1, in2)
}

fn sub_datetime_duration(in1: i64, in2: i64) -> i64 {
    chrono::Utc
        .timestamp_millis(in1)
        .checked_sub_signed(chrono::Duration::nanoseconds(in2))
        .unwrap()
        .timestamp_millis()
}

/// .
/// This operator produces the subtraction of two DT operands
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SUB_DT_DT(in1: i64, in2: i64) -> i64 {
    sub_datetimes(in1, in2)
}

/// .
/// Multiply TIME with SINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL__TIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with INT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL__TIME__INT(in1: i64, in2: i16) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with DINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL__TIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with LINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL__TIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply TIME with SINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_TIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with INT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_TIME__INT(in1: i64, in2: i16) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with DINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_TIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with LINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_TIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply LTIME with SINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_LTIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with INT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_LTIME__INT(in1: i64, in2: i16) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with DINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_LTIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with LINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_LTIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply TIME/LTIME with ANY_SIGNED_INT
/// Panic on overflow
///
fn checked_mul_time_with_signed_int(in1: i64, in2: i64) -> i64 {
    in1.checked_mul(in2).unwrap()
}

/// .
/// Multiply TIME with USINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL__TIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL__TIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UDINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL__TIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with ULINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL__TIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply TIME with USINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_TIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_TIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UDINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_TIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with ULINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_TIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply LTIME with USINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_LTIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with UINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_LTIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with UDINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with ULINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply TIME/LTIME with ANY_UNSIGNED_INT
/// Panic on overflow
///
fn checked_mul_time_with_unsigned_int(in1: i64, in2: u64) -> i64 {
    // convert in2 [u64] to [i64]
    // if in2 is to large for [i64] the multiplication will allways overflow -> panic on try_into()
    in1.checked_mul(in2.try_into().unwrap()).unwrap()
}

/// .
/// Divide TIME by SINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV__TIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by INT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV__TIME__INT(in1: i64, in2: i16) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by DINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV__TIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by LINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV__TIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_div_time_by_signed_int(in1, in2)
}

/// .
/// Divide TIME by SINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_TIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by INT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_TIME__INT(in1: i64, in2: i16) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by DINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_TIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by LINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_TIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_div_time_by_signed_int(in1, in2)
}

/// .
/// Divide LTIME by SINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_LTIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by INT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_LTIME__INT(in1: i64, in2: i16) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by DINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_LTIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by LINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_LTIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_div_time_by_signed_int(in1, in2)
}

/// .
/// Divide TIME/LTIME with ANY_SIGNED_INT
/// Panic on overflow or division by zero
///
fn checked_div_time_by_signed_int(in1: i64, in2: i64) -> i64 {
    in1.checked_div(in2).unwrap()
}

/// .
/// Divide TIME by USINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV__TIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV__TIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UDINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV__TIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by ULINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV__TIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide TIME by USINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_TIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_TIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UDINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_TIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by ULINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_TIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide LTIME by USINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_LTIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by UINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_LTIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by UDINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by ULINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide TIME/LTIME with ANY_UNSIGNED_INT
/// Panic on overflow or division by zero
///
fn checked_div_time_by_unsigned_int(in1: i64, in2: u64) -> i64 {
    // convert in2 [u64] to [i64]
    // if in2 is to large for [i64] the division will allways fail -> panic on try_into()
    in1.checked_div(in2.try_into().unwrap()).unwrap()
}

/// .
/// Multiply TIME with REAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL__TIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_mul_time_with_f32(in1, in2)
}

/// .
/// Multiply TIME with REAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_TIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_mul_time_with_f32(in1, in2)
}

/// .
/// Multiply LTIME with REAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_LTIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_mul_time_with_f32(in1, in2)
}

fn checked_mul_time_with_f32(in1: i64, in2: f32) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = std::time::Duration::from_nanos(in1.abs() as u64);

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = duration.mul_f32(in2.abs()).as_nanos().try_into().unwrap();

    // convert to negative if necessary
    let should_res_be_negative = should_result_be_negative(is_in1_negative, is_in2_negative);
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}

/// .
/// Multiply TIME with LREAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL__TIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_mul_time_with_f64(in1, in2)
}

/// .
/// Multiply TIME with LREAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_TIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_mul_time_with_f64(in1, in2)
}

/// .
/// Multiply LTIME with LREAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn MUL_LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_mul_time_with_f64(in1, in2)
}

fn checked_mul_time_with_f64(in1: i64, in2: f64) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = std::time::Duration::from_nanos(in1.abs() as u64);

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = duration.mul_f64(in2.abs()).as_nanos().try_into().unwrap();

    // convert to negative if necessary
    let should_res_be_negative = should_result_be_negative(is_in1_negative, is_in2_negative);
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}

/// .
/// Divide TIME by REAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV__TIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_div_time_by_f32(in1, in2)
}

/// .
/// Divide TIME by REAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_TIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_div_time_by_f32(in1, in2)
}

/// .
/// Divide LTIME by REAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_LTIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_div_time_by_f32(in1, in2)
}

fn checked_div_time_by_f32(in1: i64, in2: f32) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = std::time::Duration::from_nanos(in1.abs() as u64);

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = duration.div_f32(in2.abs()).as_nanos().try_into().unwrap();

    // convert to negative if necessary
    let should_res_be_negative = should_result_be_negative(is_in1_negative, is_in2_negative);
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}

/// .
/// Divide TIME by LREAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV__TIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_div_time_by_f64(in1, in2)
}

/// .
/// Divide TIME by LREAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_TIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_div_time_by_f64(in1, in2)
}

/// .
/// Divide LTIME by LREAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DIV_LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_div_time_by_f64(in1, in2)
}

fn checked_div_time_by_f64(in1: i64, in2: f64) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = std::time::Duration::from_nanos(in1.abs() as u64);

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = duration.div_f64(in2.abs()).as_nanos().try_into().unwrap();

    // convert to negative if necessary
    let should_res_be_negative = should_result_be_negative(is_in1_negative, is_in2_negative);
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}

fn should_result_be_negative(in1_is_negative: bool, in2_is_negative: bool) -> bool {
    if !in1_is_negative & !in2_is_negative {
        // if both params are negative, result will be positive
        false
    } else {
        // if any of the params is nagative and the other positive
        // result will be negative
        !in1_is_negative | !in2_is_negative
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Wrapper<T> {
    pub inner: T,
}
