/// .
/// Check if input is a valid REAL
/// NaN or infinite will return FALSE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID__REAL(input: f32) -> bool {
    !(input.is_nan() || input.is_infinite())
}

/// .
/// Check if input is a valid LREAL
/// NaN or infinite will return FALSE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID__LREAL(input: f64) -> bool {
    !(input.is_nan() || input.is_infinite())
}

/// .
/// Check if input as a valid BCD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID_BCD__BYTE(input: u8) -> bool {
    let bits_per_bcd_digit = 4;
    let iterations = u8::BITS / bits_per_bcd_digit;
    let mut valid = true;
    for i in 0..iterations {
        if input >> (bits_per_bcd_digit * i) & 0b1111 > 9 {
            valid = false;
            break;
        }
    }
    valid
}

/// .
/// Check if input as a valid BCD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID_BCD__WORD(input: u16) -> bool {
    let bits_per_bcd_digit = 4;
    let iterations = u16::BITS / bits_per_bcd_digit;

    let mut valid = true;
    for i in 0..iterations {
        if input >> (bits_per_bcd_digit * i) & 0b1111 > 9 {
            valid = false;
            break;
        }
    }
    valid
}

/// .
/// Check if input as a valid BCD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID_BCD__DWORD(input: u32) -> bool {
    let bits_per_bcd_digit = 4;
    let iterations = u32::BITS / bits_per_bcd_digit;

    let mut valid = true;
    for i in 0..iterations {
        if input >> (bits_per_bcd_digit * i) & 0b1111 > 9 {
            valid = false;
            break;
        }
    }
    valid
}

/// .
/// Check if input as a valid BCD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID_BCD__LWORD(input: u64) -> bool {
    let bits_per_bcd_digit = 4;
    let iterations = u64::BITS / bits_per_bcd_digit;

    let mut valid = true;
    for i in 0..iterations {
        if input >> (bits_per_bcd_digit * i) & 0b1111 > 9 {
            valid = false;
            break;
        }
    }
    valid
}
