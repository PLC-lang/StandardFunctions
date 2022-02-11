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
