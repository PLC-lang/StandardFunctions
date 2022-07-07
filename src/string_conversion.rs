use std::{ffi::CStr, os::raw::c_char};

#[repr(C)]
#[derive(Debug)]
pub struct Wrapper<T> {
    pub inner: T,
}

/// .
/// Converts WSTRING to STRING
/// Limited by a return type of 80 charachters
///
/// # Safety
///
/// Works on string pointer conversion, inherently unsafe
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn WSTRING_TO_STRING(input: *const i16) -> Wrapper<[u8; 81]> {
    let mut widestring = input;
    let mut count = 0;
    let len = loop {
        if widestring.is_null() || *widestring == 0 {
            break count;
        }
        widestring = widestring.add(1);
        count += 1;
    };

    let input = std::slice::from_raw_parts(input as *const u16, len);

    let string = String::from_utf16_lossy(input);
    let mut arr = [0; 81];
    for (idx, b) in string.bytes().enumerate() {
        //Don't fill the null terminator
        if idx < arr.len() - 1 {
            arr[idx] = b;
        } else {
            //no need to loop further, the target size of 80 is done
            break;
        }
    }
    Wrapper { inner: arr }
}

/// .
/// Converts STRING to WSTRING
/// Limited by a return type of 80 charachters
///
/// # Safety
///
/// Works on string pointer conversion, inherently unsafe
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_WSTRING(input: *const c_char) -> Wrapper<[u16; 81]> {
    //find the \0
    let string = CStr::from_ptr(input).to_string_lossy();
    let mut arr: [u16; 81] = [0; 81];
    for (i, e) in string.encode_utf16().enumerate() {
        //Don't fill the null terminator
        if i < arr.len() - 1 {
            arr[i] = e;
        } else {
            //No need to go further if the string is bigger than the target string
            break;
        }
    }
    Wrapper { inner: arr }
}

/// .
/// Converts WCHAR to CHAR
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn WCHAR_TO_CHAR(input: u16) -> u8 {
    let u16_arr = [input];
    let mut res_iter = char::decode_utf16(u16_arr.into_iter())
        .map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER));
    let mut res_arr = [u8::MAX; 80];
    if let Some(res) = res_iter.next() {
        if res_iter.next().is_none() {
            res.encode_utf8(&mut res_arr);
        }
    }
    res_arr[0]
}

/// .
/// Converts CHAR to WCHAR
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CHAR_TO_WCHAR(input: u8) -> u16 {
    let res: char = input.into();
    let mut arr = [u16::MAX; 2];
    res.encode_utf16(&mut arr);
    arr[0]
}
