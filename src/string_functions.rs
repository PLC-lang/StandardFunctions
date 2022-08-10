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
pub fn LEN__STRING(in1: &str, in2: &str) -> i32 {
    in1.find(in2).unwrap_or_default() as i32
}

/// .
/// 
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LEFT_EXT__STRING(src: *const u8, substr_len: u16, dest: *mut u8) {
    let mut cnt = 0;
    // find length of src string 
    let len = loop {
        if src.add(cnt).is_null() || *(src.add(cnt)) == 0 {
            break cnt;
        }
        cnt += 1;
    };

    if len == 0 || len < substr_len {
        return;
    }  
    
    for i in 0..len{
        *(dest.add(i)) = *(src.add(i));
    }
}

/// .
/// 
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn RIGHT_EXT__STRING(src: *const i8, substr_len: u16, dest: *mut u8) {
    let mut cnt = 0;
    let len = loop {
        if src.add(cnt).is_null() || *(src.add(cnt)) == 0 {
            break cnt;
        }
        cnt += 1;
    };

    if len == 0 || len < substr_len {
        return;
    }  
    
    for i in 0..len {
        *(dest.add(i)) = *(src.add(i));
    }
}