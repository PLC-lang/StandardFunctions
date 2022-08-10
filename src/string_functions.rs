use std::{ffi::CStr, os::raw::c_char};

// --- utf8

/// .
/// 
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn LEN__STRING(input: &str) -> i32 {
    input.len()
}

/// .
/// 
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn FIND__STRING(in1: &str, in2: &str) -> i32 {
    in1.find(in2).unwrap_or_default() as i32
}

/// .
/// 
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LEFT_EXT__STRING(src: *const u8, substr_len: u16, dest: *mut u8) {
    let mut len = 0;
    // find length of src string 
    while !(src.add(len).is_null() || *(src.add(len)) == 0) {
        len += 1;
    };

    if len == 0 || len < substr_len as usize {
        return;
    }  
    
    for i in 0..substr_len as usize{
        *(dest.add(i)) = *(src.add(i));
    }

    *(dest.add(len)) = b'\0';
}

/// .
/// 
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn RIGHT_EXT__STRING(src: *const u8, substr_len: u16, dest: *mut u8) {
    let mut len = 0;
    // find length of src string 
    while !(src.add(len).is_null() || *(src.add(len)) == 0) {
        len += 1;
    };

    if len == 0 || len < substr_len as usize {
        return;
    }  
    
    let offset = len - substr_len as usize;
    for i in 0..substr_len as usize {
        *(dest.add(i)) = *(src.add(i + offset));
    }

    *(dest.add(len)) = 0;
}