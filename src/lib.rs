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
/// Calculates the base 10 logarithm of the given (f32) value
///
/// # Examples
///
/// ```
/// use iec61131_std::LOG__REAL;
///
/// assert_eq!(LOG__REAL(1.0), 0.0);
/// assert_eq!(LOG__REAL(10.0), 1.0);
/// assert_eq!(LOG__REAL(100.0), 2.0);
/// assert_eq!(LOG__REAL(1000.0), 3.0);
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LOG__REAL(input: f32) -> f32 {
    f32::log10(input)
}

/// .
/// Calculates the natural logarithm of the given (f64) value
///
/// # Examples
///
/// ```
/// use iec61131_std::LOG__LREAL;
///
/// assert_eq!(LOG__LREAL(1.0), 0.0);
/// assert_eq!(LOG__LREAL(10.0), 1.0);
/// assert_eq!(LOG__LREAL(100.0), 2.0);
/// assert_eq!(LOG__LREAL(1000.0), 3.0);
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LOG__LREAL(input: f64) -> f64 {
    f64::log10(input)
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

///
/// .
/// Calculates the sine of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::SIN__REAL;
///
/// assert_eq!(SIN__REAL(0.0), 0.0);
/// assert!(SIN__REAL(std::f32::consts::PI) - 1.0 <= f32::EPSILON);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SIN__REAL(input: f32) -> f32 {
    f32::sin(input)
}

/// .
/// Calculates the sine of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::SIN__LREAL;
///
/// assert_eq!(SIN__LREAL(0.0), 0.0);
/// assert!(SIN__LREAL(std::f64::consts::PI) - 1.0 <= f64::EPSILON);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SIN__LREAL(input: f64) -> f64 {
    f64::sin(input)
}

/// .
/// Calculates the cosine of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::COS__REAL;
///
/// assert!(COS__REAL(std::f32::consts::PI) + 1.0 <= f32::EPSILON);
/// assert!(COS__REAL(0.0) - 1.0 <= f32::EPSILON);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn COS__REAL(input: f32) -> f32 {
    f32::cos(input)
}

/// .
/// Calculates the cosine of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::COS__LREAL;
///
/// assert!(COS__LREAL(std::f64::consts::PI) + 1.0 <= f64::EPSILON);
/// assert!(COS__LREAL(0.0) - 1.0 <= f64::EPSILON);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn COS__LREAL(input: f64) -> f64 {
    f64::cos(input)
}

/// .
/// Calculates the tangent of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::TAN__REAL;
///
/// assert!(TAN__REAL(std::f32::consts::FRAC_PI_4) - 1.0 <= f32::EPSILON);
/// assert!(TAN__REAL(std::f32::consts::PI) <= f32::EPSILON);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TAN__REAL(input: f32) -> f32 {
    f32::tan(input)
}

/// .
/// Calculates the tangent of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::TAN__LREAL;
///
/// assert!(TAN__LREAL(std::f64::consts::FRAC_PI_4) - 1.0 <= f64::EPSILON);
/// assert!(TAN__LREAL(std::f64::consts::PI) <= f64::EPSILON);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TAN__LREAL(input: f64) -> f64 {
    f64::tan(input)
}

///
/// .
/// Calculates the arc sine of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::ASIN__REAL;
///
/// assert_eq!(ASIN__REAL(1.0), std::f32::consts::FRAC_PI_2);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ASIN__REAL(input: f32) -> f32 {
    f32::asin(input)
}

/// .
/// Calculates the arc sine of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::ASIN__LREAL;
///
/// assert_eq!(ASIN__LREAL(1.0), std::f64::consts::FRAC_PI_2);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ASIN__LREAL(input: f64) -> f64 {
    f64::asin(input)
}

/// .
/// Calculates the arc cosine of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::ACOS__REAL;
///
/// assert_eq!(ACOS__REAL(0.0), std::f32::consts::FRAC_PI_2);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ACOS__REAL(input: f32) -> f32 {
    f32::acos(input)
}

/// .
/// Calculates the arc cosine of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::ACOS__LREAL;
///
/// assert_eq!(ACOS__LREAL(0.0), std::f64::consts::FRAC_PI_2);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ACOS__LREAL(input: f64) -> f64 {
    f64::acos(input)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::ATAN__REAL;
///
/// assert_eq!(ATAN__REAL(1.0), std::f32::consts::FRAC_PI_4);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN__REAL(input: f32) -> f32 {
    f32::atan(input)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::ATAN__LREAL;
///
/// assert_eq!(ATAN__LREAL(1.0), std::f64::consts::FRAC_PI_4);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN__LREAL(input: f64) -> f64 {
    f64::atan(input)
}

/// .
/// Calculates the four quadrant arc tangent of the value with another value
///
/// # Examples
///
/// ```
/// use iec61131_std::ATAN2__REAL;
///
/// assert_eq!(ATAN2__REAL(-3.0, 3.0), -std::f32::consts::FRAC_PI_4);
/// assert_eq!(ATAN2__REAL(3.0, -3.0), 3.0*std::f32::consts::FRAC_PI_4);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN2__REAL(x: f32, y: f32) -> f32 {
    x.atan2(y)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
/// # Examples
///
/// ```
/// use iec61131_std::ATAN2__LREAL;
///
/// assert_eq!(ATAN2__LREAL(-3.0, 3.0), -std::f64::consts::FRAC_PI_4);
/// assert_eq!(ATAN2__LREAL(3.0, -3.0), 3.0*std::f64::consts::FRAC_PI_4);
///
/// ```
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN2__LREAL(x: f64, y: f64) -> f64 {
    x.atan2(y)
}
