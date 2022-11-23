use std::io::Write;

const DEFAULT_STRING_LEN: usize = 81;

// --------- x_TO_STRING
#[allow(non_snake_case)]
pub unsafe extern "C" fn BYTE_TO_STRING_EXT(input: u8, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);
    write!(&mut buf[..], "{:08b}", input).unwrap();

    0
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn LWORD_TO_STRING_EXT(input: u64, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);
    write!(&mut buf[..], "{:064b}", input).unwrap();

    0
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn LINT_TO_STRING_EXT(input: i64, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);
    write!(&mut buf[..], "{}", input).unwrap();

    0
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn LREAL_TO_STRING_EXT(input: f64, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);
    // float: 23 bits are used for the mantissa (about 7 decimal digits)
    // double: 52 bits are used for the mantissa (about 16 decimal digits)

    // m200 -> scientific notation if >= 15 digits before the comma

    // TODO: discuss when scientific notation should be displayed
    if input.floor() < 1e14 {
        write!(&mut buf[..], "{:.13}", input).unwrap()
    } else {
        write!(&mut buf[..], "{:.13e}", input).unwrap()
    }

    let float = 123456789123456.123456_f32;
    // println!("{}", float);
    dbg!(float);

    0
}

// tests
#[test]
fn byte_to_string_conversion() {
    let byte = 0b1010_1010_u8;
    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();

    let _ = unsafe { BYTE_TO_STRING_EXT(byte, dest_ptr) };
    let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!("10101010", res.trim_end_matches('\0'));
}

#[test]
fn lword_to_string_conversion() {
    let lword = 0xFF_00_FF_00_00_FF_00_FF_u64;
    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();

    let _ = unsafe { LWORD_TO_STRING_EXT(lword, dest_ptr) };
    let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!(
        "1111111100000000111111110000000000000000111111110000000011111111",
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
    let e_notation = 123_456_789_123_456.125125;
    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();
    let _ = unsafe { LREAL_TO_STRING_EXT(lreal, dest_ptr) };
    let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!(format!("{:.13}", lreal), res.trim_end_matches('\0'));

    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();
    let _ = unsafe { LREAL_TO_STRING_EXT(lreal_neg, dest_ptr) };
    let res_neg =
        std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!(format!("{:.13}", lreal_neg), res_neg.trim_end_matches('\0'));

    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();
    let _ = unsafe { LREAL_TO_STRING_EXT(pre_e_notation, dest_ptr) };
    let res_large =
        std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!(
        format!("{:.13}", pre_e_notation),
        res_large.trim_end_matches('\0')
    );

    let mut dest = [0_u8; 81];
    let dest_ptr = dest.as_mut_ptr();
    let _ = unsafe { LREAL_TO_STRING_EXT(e_notation, dest_ptr) };
    let res_scientific =
        std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

    assert_eq!(
        format!("{:.13e}", e_notation),
        res_scientific.trim_end_matches('\0')
    );
}
