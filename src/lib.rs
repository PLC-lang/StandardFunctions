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
/// Calculates the square root of the given (f32) value
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
/// Calculates the square root of the given (f64) value
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

/// .
/// Calculates the natural logarithm of the given (f32) value
///
/// # Examples
///
/// ```
/// use iec61131_std::LN__REAL;
///
/// assert_eq!(LN__REAL(1.0), 0.0);
/// assert!(LN__REAL(2.718281828) - 1.0 <= f32::EPSILON);
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LN__REAL(input: f32) -> f32 {
    f32::ln(input)
}

/// .
/// Calculates the natural logarithm of the given (f64) value
///
/// # Examples
///
/// ```
/// use iec61131_std::LN__LREAL;
///
/// assert_eq!(LN__LREAL(1.0), 0.0);
/// assert!(LN__LREAL(2.718281828) - 1.0 <= f64::EPSILON);
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LN__LREAL(input: f64) -> f64 {
    f64::ln(input)
}

/// .
/// The natural exponential function (e)
///
/// # Examples
///
/// ```
/// use iec61131_std::EXP__REAL;
///
/// assert_eq!(EXP__REAL(0.0), 1.0);
/// assert!(EXP__REAL(1.0) - 2.718281828 <= f32::EPSILON);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXP__REAL(input: f32) -> f32 {
    f32::exp(input)
}

/// .
/// The natural exponential function (e)
///
/// # Examples
///
/// ```
/// use iec61131_std::EXP__LREAL;
///
/// assert_eq!(EXP__LREAL(0.0), 1.0);
/// assert!(EXP__LREAL(1.0) - 2.71828182849 <= f64::EPSILON);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXP__LREAL(input: f64) -> f64 {
    f64::exp(input)
}
