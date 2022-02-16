// Definitions of the core standard function modules for IEC61131-3

/// .
/// Rounds a REAL (f32) value
///
/// # Examples
///
/// ```
/// use iec61131_std::ROUND__REAL;
///
/// assert_eq!(ROUND__REAL(-2.4), -2.0);
/// assert_eq!(ROUND__REAL(-2.5), -3.0);
/// assert_eq!(ROUND__REAL(2.5), 3.0);
/// assert_eq!(ROUND__REAL(2.4), 2.0);
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ROUND__REAL(input: f32) -> f32 {
    input.round()
}

/// .
/// Rounds a LREAL (f64) value
///
/// # Examples
///
/// ```
/// use iec61131_std::ROUND__LREAL;
///
/// assert_eq!(ROUND__LREAL(-2.4), -2.0);
/// assert_eq!(ROUND__LREAL(-2.5), -3.0);
/// assert_eq!(ROUND__LREAL(2.5), 3.0);
/// assert_eq!(ROUND__LREAL(2.4), 2.0);
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ROUND__LREAL(input: f64) -> f64 {
    input.round()
}

/// .
/// Calculates tne square root of the given (f32) value
///
/// # Examples
///
/// ```
/// use iec61131_std::SQRT__REAL;
///
/// assert_eq!(SQRT__REAL(4.0), 2.0);
/// assert_eq!(SQRT__REAL(1.0), 1.0);
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQRT__REAL(input: f32) -> f32 {
    f32::sqrt(input)
}

/// .
/// Calculates tne square root of the given (f64) value
///
/// # Examples
///
/// ```
/// use iec61131_std::SQRT__LREAL;
///
/// assert_eq!(SQRT__LREAL(4.0), 2.0);
/// assert_eq!(SQRT__LREAL(1.0), 1.0);
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQRT__LREAL(input: f64) -> f64 {
    f64::sqrt(input)
}
