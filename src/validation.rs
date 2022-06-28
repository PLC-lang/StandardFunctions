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
