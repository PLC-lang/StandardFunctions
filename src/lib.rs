// Definitions of the core standard function modules for IEC61131-3

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
pub extern "C" fn ROUND__REAL(input: &SingleParam<f32>) -> f32 {
    input.in1.round()
}

/// .
/// Rounds a LREAL (f64) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ROUND__LREAL(input: &SingleParam<f64>) -> f64 {
    input.in1.round()
}

/// .
/// Calculates the square root of the given (f32) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQRT__REAL(input: &SingleParam<f32>) -> f32 {
    f32::sqrt(input.in1)
}

/// .
/// Calculates the square root of the given (f64) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQRT__LREAL(input: &SingleParam<f64>) -> f64 {
    f64::sqrt(input.in1)
}

/// .
/// Calculates the natural logarithm of the given (f32) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LN__REAL(input: &SingleParam<f32>) -> f32 {
    f32::ln(input.in1)
}

/// .
/// Calculates the natural logarithm of the given (f64) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LN__LREAL(input: &SingleParam<f64>) -> f64 {
    f64::ln(input.in1)
}

/// .
/// Calculates the base 10 logarithm of the given (f32) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LOG__REAL(input: &SingleParam<f32>) -> f32 {
    f32::log10(input.in1)
}

/// .
/// Calculates the natural logarithm of the given (f64) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LOG__LREAL(input: &SingleParam<f64>) -> f64 {
    f64::log10(input.in1)
}

/// .
/// The natural exponential function (e)
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXP__REAL(input: &SingleParam<f32>) -> f32 {
    f32::exp(input.in1)
}

/// .
/// The natural exponential function (e)
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXP__LREAL(input: &SingleParam<f64>) -> f64 {
    f64::exp(input.in1)
}

///
/// .
/// Calculates the sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SIN__REAL(input: &SingleParam<f32>) -> f32 {
    f32::sin(input.in1)
}

/// .
/// Calculates the sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SIN__LREAL(input: &SingleParam<f64>) -> f64 {
    f64::sin(input.in1)
}

/// .
/// Calculates the cosine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn COS__REAL(input: &SingleParam<f32>) -> f32 {
    f32::cos(input.in1)
}

/// .
/// Calculates the cosine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn COS__LREAL(input: &SingleParam<f64>) -> f64 {
    f64::cos(input.in1)
}

/// .
/// Calculates the tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TAN__REAL(input: &SingleParam<f32>) -> f32 {
    f32::tan(input.in1)
}

/// .
/// Calculates the tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TAN__LREAL(input: &SingleParam<f64>) -> f64 {
    f64::tan(input.in1)
}

/// .
/// Calculates the arc sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ASIN__REAL(input: &SingleParam<f32>) -> f32 {
    f32::asin(input.in1)
}

/// .
/// Calculates the arc sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ASIN__LREAL(input: &SingleParam<f64>) -> f64 {
    f64::asin(input.in1)
}

/// .
/// Calculates the arc cosine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ACOS__REAL(input: &SingleParam<f32>) -> f32 {
    f32::acos(input.in1)
}

/// .
/// Calculates the arc cosine of the given value in radiants
///
/// # Examples
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ACOS__LREAL(input: &SingleParam<f64>) -> f64 {
    f64::acos(input.in1)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN__REAL(input: &SingleParam<f32>) -> f32 {
    f32::atan(input.in1)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN__LREAL(input: &SingleParam<f64>) -> f64 {
    f64::atan(input.in1)
}

/// .
/// Calculates the four quadrant arc tangent of the value with another value
///
/// # Examples
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN2__REAL(inter: &DoubleParam<f32>) -> f32 {
    inter.in1.atan2(inter.in2)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN2__LREAL(inter: &DoubleParam<f64>) -> f64 {
    inter.in1.atan2(inter.in2)
}

/// .
/// Converts LWORD to LREAL
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LWORD_TO_LREAL(input: &SingleParam<u64>) -> f64 {
    f64::from_bits(input.in1)
}

/// .
/// Converts DWORD to REAL
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DWORD_TO_REAL(input: &SingleParam<u32>) -> f32 {
    f32::from_bits(input.in1)
}

/// .
/// Converts LREAL to LWORD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LREAL_TO_LWORD(input: &SingleParam<f64>) -> u64 {
    f64::to_bits(input.in1)
}

/// .
/// Converts REAL to DWORD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn REAL_TO_DWORD(input: &SingleParam<f32>) -> u32 {
    f32::to_bits(input.in1)
}

#[repr(C)]
pub struct SingleParam<T> {
    pub in1: T,
}

#[repr(C)]
pub struct DoubleParam<T> {
    pub in1: T,
    pub in2: T,
}
