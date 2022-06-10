// Definitions of the core standard function modules for IEC61131-3

use chrono::{TimeZone, Timelike};
use std::ffi::CStr;

pub mod bit_shift;

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
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn WSTRING_TO_STRING(input: Wrapper<[u16; 81]>) -> Wrapper<[u8; 81]> {
    let terminator = input
        .inner
        .iter()
        .position(|c| *c == 0)
        .unwrap_or(input.inner.len());
    let string = String::from_utf16_lossy(&input.inner[..terminator]);
    let mut arr = [0; 81];
    for (idx, b) in string.bytes().enumerate() {
        if idx < arr.len() {
            arr[idx] = b;
        }
    }
    Wrapper { inner: arr }
}

/// .
/// Converts STRING to WSTRING
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn STRING_TO_WSTRING(input: Wrapper<[u8; 81]>) -> Wrapper<[u16; 81]> {
    //find the \0
    let terminator = input
        .inner
        .iter()
        .position(|c| *c == 0)
        .map(|it| it + 1)
        .unwrap_or(input.inner.len());
    let string = CStr::from_bytes_with_nul(&input.inner[..terminator])
        .map_or(Ok(""), CStr::to_str)
        .unwrap_or("");
    let mut arr: [u16; 81] = [0; 81];
    for (i, e) in string.encode_utf16().enumerate() {
        if i < arr.len() {
            arr[i] = e;
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
/// Converts DT/LDT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_DATE(input: i64) -> i64 {
    let date_time = chrono::Utc.timestamp_millis(input);

    let new_date_time = date_time.date().and_hms(0, 0, 0);
    new_date_time.timestamp_millis()
}

/// .
/// Converts DT/LDT to TOD/LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_TIME_OF_DAY(input: i64) -> i64 {
    let date_time = chrono::Utc.timestamp_millis(input);
    let hour = date_time.hour();
    let min = date_time.minute();
    let sec = date_time.second();
    let milli = date_time.timestamp_subsec_millis();

    let new_date_time =
        chrono::NaiveDate::from_ymd(1970, 1, 1).and_hms_milli(hour, min, sec, milli);

    new_date_time.timestamp_millis()
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
/// Panic on onverflow
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn add_datetime_time(in1: i64, in2: i64) -> i64 {
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn sub_datetimes(in1: i64, in2: i64) -> i64 {
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn sub_datetime_duration(in1: i64, in2: i64) -> i64 {
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
/// Multiply TIME/LTIME with ANY_SIGNED_INT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CHECKED_MUL_SIGNED(in1: i64, in2: i64) -> i64 {
    in1.checked_mul(in2).unwrap()
}

/// .
/// Multiply TIME/LTIME with ANY_UNSIGNED_INT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CHECKED_MUL_UNSIGNED(in1: i64, in2: u64) -> i64 {
    // convert in2 [u64] to [i64]
    // if in2 is to large for [i64] the multiplication will allways overflow -> panic on try_into()
    in1.checked_mul(in2.try_into().unwrap()).unwrap()
}

/// .
/// Divide TIME/LTIME with ANY_SIGNED_INT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CHECKED_DIV_SIGNED(in1: i64, in2: i64) -> i64 {
    in1.checked_div(in2).unwrap()
}

/// .
/// Divide TIME/LTIME with ANY_UNSIGNED_INT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CHECKED_DIV_UNSIGNED(in1: i64, in2: u64) -> i64 {
    // convert in2 [u64] to [i64]
    // if in2 is to large for [i64] the division will allways fail -> panic on try_into()
    in1.checked_div(in2.try_into().unwrap()).unwrap()
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

/// .
/// Multiply TIME/LTIME with REAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CHECKED_MUL_F32(in1: i64, in2: f32) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = match is_in1_negative {
        true => std::time::Duration::from_nanos(-in1 as u64),
        false => std::time::Duration::from_nanos(in1 as u64),
    };

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = match is_in2_negative {
        true => duration.mul_f32(-in2).as_nanos().try_into().unwrap(),
        false => duration.mul_f32(in2).as_nanos().try_into().unwrap(),
    };

    // convert to negative if necessary
    let should_res_be_negative = should_result_be_negative(is_in1_negative, is_in2_negative);
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}

/// .
/// Multiply TIME/LTIME with LREAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CHECKED_MUL_F64(in1: i64, in2: f64) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = match is_in1_negative {
        true => std::time::Duration::from_nanos(-in1 as u64),
        false => std::time::Duration::from_nanos(in1 as u64),
    };

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = match is_in2_negative {
        true => duration.mul_f64(-in2).as_nanos().try_into().unwrap(),
        false => duration.mul_f64(in2).as_nanos().try_into().unwrap(),
    };

    // convert to negative if necessary
    let should_res_be_negative = should_result_be_negative(is_in1_negative, is_in2_negative);
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}

/// .
/// Divide TIME/LTIME with ANY_UNSIGNED_INT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CHECKED_DIV_F32(in1: i64, in2: f32) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = match is_in1_negative {
        true => std::time::Duration::from_nanos(-in1 as u64),
        false => std::time::Duration::from_nanos(in1 as u64),
    };

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = match is_in2_negative {
        true => duration.div_f32(-in2).as_nanos().try_into().unwrap(),
        false => duration.div_f32(in2).as_nanos().try_into().unwrap(),
    };

    // convert to negative if necessary
    let should_res_be_negative = should_result_be_negative(is_in1_negative, is_in2_negative);
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}

/// .
/// Divide TIME/LTIME with ANY_UNSIGNED_INT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CHECKED_DIV_F64(in1: i64, in2: f64) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = match is_in1_negative {
        true => std::time::Duration::from_nanos(-in1 as u64),
        false => std::time::Duration::from_nanos(in1 as u64),
    };

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = match is_in2_negative {
        true => duration.div_f64(-in2).as_nanos().try_into().unwrap(),
        false => duration.div_f64(in2).as_nanos().try_into().unwrap(),
    };

    // convert to negative if necessary
    let should_res_be_negative = should_result_be_negative(is_in1_negative, is_in2_negative);
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Wrapper<T> {
    pub inner: T,
}
