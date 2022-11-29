// #[cfg(not(feature = "mock_time"))]
// use std::time::Instant;

// #[cfg(feature = "mock_time")]
// use test_time_helpers::Instant;

use crate::string_functions::ptr_to_slice;
use num::{Float, PrimInt};
use std::{fmt::Display, io::Write, str::FromStr};

// can't determine string buffer length of an empty string, therefore
// _TO_STRING functions use the default string length.
const DEFAULT_STRING_LEN: usize = 81;
// --------- x_TO_STRING

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn BYTE_TO_STRING_EXT(input: u8, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);

    write!(&mut *buf, "{}", input).unwrap();

    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LWORD_TO_STRING_EXT(input: u64, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);

    write!(&mut *buf, "{}", input).unwrap();

    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LINT_TO_STRING_EXT(input: i64, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);
    write!(&mut *buf, "{}", input).unwrap();

    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LREAL_TO_STRING_EXT(input: f64, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);
    // double: 52 bits are used for the mantissa (about 16 decimal digits)
    if input.floor() < 1e14 {
        write!(&mut *buf, "{:.6}", input).unwrap()
    } else {
        write!(&mut *buf, "{:.6e}", input).unwrap()
    }

    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn REAL_TO_STRING_EXT(input: f64, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);
    // float: 23 bits are used for the mantissa (about 7 decimal digits)

    // TODO: discuss when scientific notation should be displayed
    if input.floor() < 1e6 {
        write!(&mut *buf, "{:.6}", input).unwrap()
    } else {
        write!(&mut *buf, "{:.6e}", input).unwrap()
    }

    0
}

unsafe fn string_to_int<T>(src: *const u8) -> T
where
    T: PrimInt,
    <T as num::Num>::FromStrRadixErr: std::fmt::Display,
{
    let slice = ptr_to_slice(src);
    let (string, radix) = match slice {
        [b'1', b'6', b'#', ..] => (std::str::from_utf8(&slice[3..]), 16),
        [b'0', b'x', ..] | [b'0', b'X', ..] => (std::str::from_utf8(&slice[2..]), 16),
        [b'8', b'#', ..] => (std::str::from_utf8(&slice[2..]), 8), // support c-style octal prefixes? e.g. 010 -> 10 octal
        [b'2', b'#', ..] | [b'0', b'b', ..] | [b'0', b'B', ..] => {
            (std::str::from_utf8(&slice[2..]), 2)
        }
        _ => (std::str::from_utf8(slice), 10),
    };

    match string {
        Ok(s) => match T::from_str_radix(s, radix) {
            Ok(number) => number,
            Err(e) => panic!("Could not parse number from '{s}': {e}"),
        },
        Err(e) => panic!("Encoding error: {e}"),
    }
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_LINT(src: *const u8) -> i64 {
    string_to_int(src)
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_DINT(src: *const u8) -> i32 {
    string_to_int(src)
}

unsafe fn string_to_float<T>(src: *const u8) -> T
where
    T: Float + FromStr,
    <T as FromStr>::Err: Display,
{
    let slice = ptr_to_slice(src);
    let string = std::str::from_utf8(slice);

    match string {
        Ok(s) => match s.parse() {
            Ok(number) => number,
            Err(e) => panic!("Could not parse number from '{s}': {e}"),
        },
        Err(e) => panic!("Encoding error: {e}"),
    }
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_LREAL(src: *const u8) -> f64 {
    string_to_float(src)
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_REAL(src: *const u8) -> f32 {
    string_to_float(src)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TIME() -> i64 {
    let now = std::time::SystemTime::now();
    let since = now.duration_since(std::time::UNIX_EPOCH);
    match since {
        Ok(time) => time.as_nanos() as i64,
        Err(e) => panic!("Could not retrieve system time: {e}"),
    }

    // let now = Instant::now();
}

// tests
#[test]
fn byte_to_string_conversion() {
    let byte = 0b1010_1010_u8;
    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();

    let _ = unsafe { BYTE_TO_STRING_EXT(byte, dest_ptr) };
    let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!(0b1010_1010_u8.to_string(), res.trim_end_matches('\0'));
}

#[test]
fn lword_to_string_conversion() {
    let lword = 0xFF_00_FF_00_00_FF_00_FF_u64;
    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();

    let _ = unsafe { LWORD_TO_STRING_EXT(lword, dest_ptr) };
    let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!(
        0xFF_00_FF_00_00_FF_00_FF_u64.to_string(),
        res.trim_end_matches('\0')
    );
}

#[test]
fn lint_to_string_conversion() {
    let lint = 100_200_300_400_500_i64;
    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();

    let _ = unsafe { LINT_TO_STRING_EXT(lint, dest_ptr) };
    let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!("100200300400500", res.trim_end_matches('\0'));
}

#[test]
fn lreal_to_string_conversion() {
    let lreal = 10230.2321123121;
    let lreal_neg = lreal * -1.0;
    let pre_e_notation = 99_999_999_999_999.25;
    let e_notation = 123_456_789_123_456.13;
    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();
    let _ = unsafe { LREAL_TO_STRING_EXT(lreal, dest_ptr) };
    let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!(format!("{:.6}", lreal), res.trim_end_matches('\0'));

    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();
    let _ = unsafe { LREAL_TO_STRING_EXT(lreal_neg, dest_ptr) };
    let res_neg =
        std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!(format!("{:.6}", lreal_neg), res_neg.trim_end_matches('\0'));

    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();
    let _ = unsafe { LREAL_TO_STRING_EXT(pre_e_notation, dest_ptr) };
    let res_large =
        std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!(
        format!("{:.6}", pre_e_notation),
        res_large.trim_end_matches('\0')
    );

    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();
    let _ = unsafe { LREAL_TO_STRING_EXT(e_notation, dest_ptr) };
    let res_scientific =
        std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!(
        format!("{:.6e}", e_notation),
        res_scientific.trim_end_matches('\0')
    );
}

#[test]
fn string_to_lint_conversion() {
    let string = "12345\0";
    let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
    assert_eq!(12345_i64, result);

    let string = "2#1111\0";
    let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
    assert_eq!(15_i64, result);

    let string = "8#77\0";
    let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
    assert_eq!(63_i64, result);

    let string = "16#FF\0";
    let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
    assert_eq!(255_i64, result);

    let string = "0b1111\0";
    let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
    assert_eq!(15_i64, result);

    let string = "0B1111\0";
    let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
    assert_eq!(15_i64, result);

    let string = "0xFF\0";
    let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
    assert_eq!(255_i64, result);

    let string = "0XFF\0";
    let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
    assert_eq!(255_i64, result);
}

#[test]
#[should_panic]
fn string_to_lint_conversion_panics_if_given_invalid_string() {
    let string = "ab456\0";
    let _ = unsafe { STRING_TO_LINT(string.as_ptr()) };
}

#[test]
fn string_to_lreal_conversion() {
    let string = "1.25\0";
    let result = unsafe { STRING_TO_LREAL(string.as_ptr()) };
    assert_eq!(1.25, result);
}

#[test]
fn string_to_real_conversion() {
    let string = "1.25\0";
    let result = unsafe { STRING_TO_REAL(string.as_ptr()) };
    assert_eq!(1.25, result);
}

#[test]
#[should_panic]
fn string_to_lreal_conversion_panics_if_given_invalid_string() {
    let string = "1,25f\0";
    let _ = unsafe { STRING_TO_LREAL(string.as_ptr()) };
}
