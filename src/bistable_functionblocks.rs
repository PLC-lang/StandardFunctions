/// .
/// ...
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SR(set1: bool, reset: bool, q1: *mut bool) {}

/// .
/// ...
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn RS(set: bool, reset1: bool, q1: *mut bool) {}
