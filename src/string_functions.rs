use std::char::decode_utf16;

/// Gets length of the given character string.
/// UTF8
///
/// Works on raw pointers, inherently unsafe.
/// May return an incorrect value if passed an
/// array filled with (non-zero) garbage values.
/// 
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LEN__STRING(src: *const u8) -> i32 {
    u8_ptr_to_str(src, None)
    .chars()
    .count() as i32
}

/// Gets length of the given string.
/// UTF16
///
/// Works on raw pointers, inherently unsafe.
/// May return an incorrect value if passed an
/// array filled with (non-zero) garbage values.
/// 
/// # Safety
///
/// Works on raw pointers, inherently unsafe
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LEN__WSTRING(src: *const u16) -> i32 {
    u16_ptr_to_utf16_iter(src, None).count() as i32
}

/// Finds the first occurance of the second string (in2)
/// within the first string (in1).
/// UTF8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn FIND__STRING(src1: *const u8, src2: *const u8) -> i32 {
    let haystack = u8_ptr_to_str(src1, None);
    let needle = u8_ptr_to_str(src2, None);

    if needle.len() > haystack.len() || haystack.len() == 0 || needle.len() == 0 {
        return 0;
    }

    if let Some(byte_idx) = haystack.find(needle) {
        // match found. get utf8 char len until byte index
        let pos = u8_ptr_to_str(src1, Some(byte_idx)).chars().count() as i32;

        pos + 1 
    } else {
        0
    }    
}

/// Finds the first occurance of the second string (src2)
/// within the first string (src1).
/// UTF16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn FIND__WSTRING(src1: *const u16, src2: *const u16) -> i32 {
    let nchars1 = u16_ptr_to_utf16_iter(src1, None).count();
    let nchars2 = u16_ptr_to_utf16_iter(src1, None).count();

    if nchars2 > nchars1 || nchars1 == 0 || nchars2 == 0 {
        return 0;
    }

    let haystack = u16_ptr_to_slice(src1, None);
    let needle = u16_ptr_to_slice(src2, None);
    
    if let Some(pos) = haystack.windows(needle.len()).position(|window| window == needle) {
        // match found. count utf16 chars to window position
        let n_chars_before_pos = 
            u16_ptr_to_utf16_iter(src1, Some(pos)).count();

        // correct indexing for ST
        n_chars_before_pos as i32 + 1
    } else {
        0
    }
}

/// Writes a substring of a specified length from the given string,
/// to the destination buffer, beginning with the first (leftmost) character.
/// UTF8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length is either negative or
/// longer than the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LEFT_EXT__STRING(src: *const u8, substr_len: i32, dest: *mut u8) -> i32 {
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }
    let mut dest = dest;
    let substr_len = substr_len as usize;
    let res =  u8_ptr_to_str(src, None).split_at(substr_len).0;
    
    dest = write_str_to_pointer(res, dest);
    *dest = 0;

    0
}

/// Writes a substring of a specified length from the given string,
/// to the destination buffer, beginning with the first (leftmost) character.
/// UTF16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length is either negative or
/// longer than the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LEFT_EXT__WSTRING(
    src: *const u16,
    substr_len: i32,
    dest: *mut u16,
) -> i32 {
    let mut dest = dest;
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }

    let iter = u16_ptr_to_utf16_iter(src, None);

    for c in iter.take(substr_len as usize) {
        dest = write_utf16_to_pointer(c, dest);
    }

    *dest = 0;
        
    0
}

/// Writes a substring of a specified length from the given string,
/// to the destination buffer, ending with the last (rightmost) character.
/// UTF8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length is either negative or
/// longer than the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn RIGHT_EXT__STRING(src: *const u8, substr_len: i32, dest: *mut u8) -> i32 {
    let substr_len = substr_len as usize;
    let res = u8_ptr_to_str(src, None).split_at(substr_len).1;
    let mut dest = dest;

    dest = write_str_to_pointer(res, dest);
    *dest = 0;

    0
}

/// Writes a substring of a specified length from the given string
/// to the destination buffer, ending with the last (rightmost) character.
/// UTF16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length is either negative or
/// longer than the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn RIGHT_EXT__WSTRING(
    src: *const u16,
    substr_len: i32,
    dest: *mut u16,
) -> i32 {
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }

    let mut dest = dest;    
    let count = u16_ptr_to_utf16_iter(src, None).count();
    let iter = u16_ptr_to_utf16_iter(src, None);

    for c in iter.skip(count - substr_len as usize) {
        dest = write_utf16_to_pointer(c, dest);
    }

    *dest = 0;
        
    0
}

/// Writes a substring of a specified length from the given string
/// to the destination buffer, beginning at the given index.
/// UTF8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length or position are negative
/// or the substring length exceeds the remaining characters from the
/// starting position of the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn MID_EXT__STRING(
    src: *const u8,
    substr_len: i32,
    start_index: i32,
    dest: *mut u8,
) -> i32 {
    let mut dest = dest;
    let substr_len = substr_len as usize;
    let res = u8_ptr_to_str(src, None)
        .split_at(start_index as usize).1;
    let res = &res[..substr_len];

    dest = write_str_to_pointer(res, dest);
    *dest = 0;

    0
}

/// Writes a substring of a specified length from the given string
/// to the destination buffer, beginning at the given index.
/// UTF16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length or position are negative
/// or the substring length exceeds the remaining characters from the
/// starting position of the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn MID_EXT__WSTRING(
    src: *const u16,
    substr_len: i32,
    start_index: i32,
    dest: *mut u16,
) -> i32 {    
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }

    let mut dest = dest;
    let iter = u16_ptr_to_utf16_iter(src, None)
        .skip(start_index as usize)
        .take(substr_len as usize);

    for c in iter {
        dest = write_utf16_to_pointer(c, dest);
    }

    *dest = 0;
        
    0
}

/// Inserts a string into another string at the
/// specified position and writes the resulting string to
/// the destination buffer.
/// UTF8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the position parameter exceeds the
/// source array bounds.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn INSERT_EXT__STRING(
    src_base: *const u8,
    src_to_insert: *const u8,
    pos: i32,
    dest: *mut u8,
) -> i32 {
    if pos < 0 {
        panic!("Positional parameter cannot be negative.");
    }

    let mut dest = dest;
    let (first, second) = u8_ptr_to_str(src_base, None)
        .split_at(pos as usize);
    
    let to_insert = u8_ptr_to_str(src_to_insert, None);
    
    dest = write_str_to_pointer(first, dest);
    dest = write_str_to_pointer(to_insert, dest);
    dest = write_str_to_pointer(second, dest);

    *dest = 0;

    0
}

/// Inserts a string into another string at the
/// specified position and writes the resulting string to
/// the destination buffer.
/// UTF16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the position parameter exceeds the
/// source array bounds.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn INSERT_EXT__WSTRING(
    src_base: *const u16,
    src_to_insert: *const u16,
    pos: i32,
    dest: *mut u16,
) -> i32 {
    if pos < 0 {
        panic!("Positional parameter cannot be negative.");
    }

    let mut dest = dest;
    let iter_first = u16_ptr_to_utf16_iter(src_base, None)
        .take(pos as usize);
    let iter_to_insert = u16_ptr_to_utf16_iter(src_to_insert, None);
    let n_to_insert = u16_ptr_to_utf16_iter(src_to_insert, None).count();
    let iter_second = u16_ptr_to_utf16_iter(src_base, None)
        .skip(pos as usize + n_to_insert);

    for c in iter_first {
        dest = write_utf16_to_pointer(c, dest);
    }

    for c in iter_to_insert {
        dest = write_utf16_to_pointer(c, dest);
    }

    for c in iter_second {
        dest = write_utf16_to_pointer(c, dest);
    }

    *dest = 0;
        
    0

}


/// Deletes the given amount of characters in a string,
/// starting from the specified position. Writes the resulting
/// string into a destination buffer.
/// UTF8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the position parameter is out of bounds of the
/// array or if trying to delete too many characters.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DELETE_EXT__STRING(
    src: *const u8,
    num_chars_to_delete: i32,
    pos: i32,
    dest: *mut u8,
) -> i32 {
    let res = u8_ptr_to_str(src, None);
    let nchars = res.chars().count();
    
    if pos < 1 || pos > nchars as i32 {
        panic!("Index out of bounds.")
    }

    // correct for 0-indexing
    let pos = pos as usize - 1;
    let ndel = num_chars_to_delete as usize;

    if ndel + pos > nchars {
        panic!(
            r#"Cannot delete {} characters starting from index {}.
            Index out of bounds.            
            "#,
            num_chars_to_delete,
            pos + 1
        )
    }

    let mut dest = dest;
    let first_half = &res[..pos];
    let second_half = &res[(pos + ndel)..];

    dest = write_str_to_pointer(first_half, dest);
    dest = write_str_to_pointer(second_half, dest);

    *dest = 0;

    0
}


/// Deletes the given amount of characters in a string,
/// starting from the specified position. Writes the resulting
/// string into a destination buffer.
/// UTF16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the position parameter is out of bounds of the
/// array or if trying to delete too many characters.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DELETE_EXT__WSTRING(
    src: *const u16,
    num_chars_to_delete: i32,
    pos: i32,
    dest: *mut u16,
) -> i32 {
    let mut dest = dest;
    let nchars = u16_ptr_to_utf16_iter(src, None).count();

    if pos < 1 || pos > nchars as i32{
        panic!("Index out of bounds.")
    }

    let ndel = num_chars_to_delete as usize;
    // correct for 0-indexing
    let pos = pos as usize - 1;

    if ndel + pos > nchars {
        panic!(
            r#"Cannot delete {} characters starting from index {}.
            Index out of bounds.            
            "#,
            num_chars_to_delete,
            pos + 1
        )
    }

    let first_half = u16_ptr_to_utf16_iter(src, None).take(pos);
    let second_half = u16_ptr_to_utf16_iter(src, None).skip(pos + ndel);

    for c in first_half {
        dest = write_utf16_to_pointer(c, dest);
    }

    for c in second_half {
        dest = write_utf16_to_pointer(c, dest);
    }

    *dest = 0;

    0
}

/// Replaces the given amount of characters in a string, starting
/// from a specified location in the string, with another string and
/// writes it to the destination buffer.
/// UTF8
///  
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if trying to index outside of the array or trying
/// to replace more characters than remaining.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn REPLACE_EXT__STRING(
    src_base: *const u8,
    src_replacement: *const u8,
    num_chars_to_replace: i32,
    pos: i32,
    dest: *mut u8,
) -> i32 {
    let base = u8_ptr_to_str(src_base, None);
    let nbase = base.chars().count();

    if pos < 1 || pos > nbase as i32 {
        panic!("Index out of bounds.")
    }

    // correct for 0-indexing
    let pos = (pos - 1) as usize;
    let nreplace = num_chars_to_replace as usize;

    if nreplace + pos > nbase {
        panic!(
            r#"Cannot replace {} characters starting from index {}.
            Index out of bounds.            
            "#,
            nreplace,
            pos + 1
        )
    }

    let replacement = u8_ptr_to_str(src_replacement, None);
    let first = &base[..pos];
    let second = &base[(pos + nreplace)..];

    let mut dest = dest;
    dest = write_str_to_pointer(first, dest);
    dest = write_str_to_pointer(replacement, dest);
    dest = write_str_to_pointer(second, dest);
    
    *dest = 0;

    0
}

/// Replaces the given amount of characters in a string, starting
/// from a specified location in the string, with another string and
/// writes it to the destination buffer.
/// UTF16
///  
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if trying to index outside of the array or trying
/// to replace more characters than remaining.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn REPLACE_EXT__WSTRING(
    src_base: *const u16,
    src_replacement: *const u16,
    num_chars_to_replace: i32,
    pos: i32,
    dest: *mut u16,
) -> i32 {
    let nchars1 = u16_ptr_to_utf16_iter(src_base, None).count();

    if pos < 1 || pos > nchars1 as i32 {
        panic!("Index out of bounds.")
    }

    // correct for 0-indexing
    let pos = (pos - 1) as usize;
    let nreplace = num_chars_to_replace as usize;

    if nreplace + pos > nchars1 {
        panic!(
            r#"Cannot replace {} characters starting from index {}.
            Index out of bounds.            
            "#,
            nreplace,
            pos + 1
        )
    }
    
    let first = u16_ptr_to_utf16_iter(src_base, None).take(pos);
    let replacement = u16_ptr_to_utf16_iter(src_replacement, None);
    let second = u16_ptr_to_utf16_iter(src_base, None).skip(pos + nreplace);
    let mut dest = dest;

    for c in first {
        dest = write_utf16_to_pointer(c, dest);
    }

    for c in replacement {
        dest = write_utf16_to_pointer(c, dest);
    }

    for c in second {
        dest = write_utf16_to_pointer(c, dest);
    }

    *dest = 0;

    0
}

/// # Helper function
/// 
/// Gets the amount of continuous characters in u8 array before
/// the first null-terminator.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// May return an incorrect value if passed an
/// array filled with (non-zero) garbage values.
fn get_null_terminated_len_utf8(src: *const u8) -> usize {
    unsafe {
        if src.is_null() {
            return 0;
        }

        (0..).take_while(|&i| *src.add(i) != 0).count() as usize
    }
}

/// # Helper function
/// 
/// Gets the amount of continuous characters in u16 array before
/// the first null-terminator.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// May return an incorrect value if passed an
/// array filled with (non-zero) garbage values.
fn get_null_terminated_len_utf16(src: *const u16) -> usize {
    unsafe {
        if src.is_null() {
            return 0;
        }

        (0..).take_while(|&i| *src.add(i) != 0).count() as usize
    }
}

/// # Helper function
/// 
/// Get slice from null-terminated u8 pointer.
/// If no number of bytes is given, nbytes will be determined
/// by finding the nul-terminator.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
unsafe fn u8_ptr_to_slice<'a>(src: *const u8, nbytes: Option<usize>) -> &'a [u8] {
    if let Some(n) = nbytes {
        std::slice::from_raw_parts(src, n)        
    } else {        
        let nbytes = get_null_terminated_len_utf8(src);
        std::slice::from_raw_parts(src, nbytes)   
    }
}

/// # Helper function
/// 
/// Get str from null-terminated u8 pointer.
/// If no number of bytes is given, nbytes will be determined
/// by finding the nul-terminator.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
unsafe fn u8_ptr_to_str<'a>(src: *const u8, nbytes: Option<usize>) -> &'a str {
    let src = u8_ptr_to_slice(src, nbytes);
    std::str::from_utf8(src).unwrap()
}
/// # Helper function
/// 
/// Get slice from null-terminated u16 pointer.
/// If no number of words is given, nwords will be determined
/// by finding the nul-terminator.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
unsafe fn u16_ptr_to_slice<'a>(src: *const u16, nwords: Option<usize>) -> &'a [u16] {
    if let Some(n) = nwords {
        std::slice::from_raw_parts(src, n)
    } else {
        let nwords = get_null_terminated_len_utf16(src);
        std::slice::from_raw_parts(src, nwords)
    }
}

type Utf16Iterator<'a> = std::char::DecodeUtf16<std::iter::Copied<std::slice::Iter<'a, u16>>>;

/// # Helper function
/// 
/// Get string slice from null-terminated u16 pointer.
/// If no number of words is given, nwords will be determined
/// by finding the nul-terminator.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
unsafe fn u16_ptr_to_utf16_iter<'a>(src: *const u16, nwords: Option<usize>) -> Utf16Iterator<'a> {
    let src = u16_ptr_to_slice(src, nwords);
    decode_utf16(src.iter().copied())
}

/// # Helper function
/// 
/// Encodes character to UTF16, writes it into pre-allocated out-buffer and increments the pointer.
/// 
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
unsafe fn write_utf16_to_pointer(c: Result<char, std::char::DecodeUtf16Error>, dest: *mut u16) -> *mut u16{
    let mut dest = dest;
    let mut temp = [0; 2];
    let slice = c.unwrap().encode_utf16(&mut temp);
    for word in slice {
        *dest = *word;
        dest = dest.add(1);
    }

    dest
}

/// # Helper function
/// 
/// Writes str slice into pre-allocated out-buffer and increments the pointer.
/// 
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
unsafe fn write_str_to_pointer(src: &str, dest: &mut *mut u8) -> *mut u8 {
    for byte in src.as_bytes() {
        **dest = *byte;
        *dest = dest.add(1);
    }

    *dest
}


// -------------------------------------------------unit tests-----------------------------------------
#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CStr;

    // -----------------------------------UTF8
    #[test]
    fn test_len_correct_utf8_character_count()
    {
        let s = "ϕϚϡϗabcd\0";
        let raw_src = s.as_ptr();
        unsafe {
            let res = LEN__STRING(raw_src);
            assert_eq!(res.to_bytes().len(), 12) 
        }
    }

    #[test]
    fn test_find_index_correct() {
        let s1 = "hϗllo wϕrld\0";
        let s2 = "llo\0";
        let raw_src1 = s1.as_ptr();
        let raw_src2 = s2.as_ptr();
        unsafe {
            let res = FIND__STRING(raw_src1, raw_src2);
            assert_eq!(res, 3) 
        }
    }

    #[test]
    fn test_find_index_correct_edge_case() {
        let s1 = "hello wϕrld\0";
        let s2 = "h\0";
        let raw_src1 = s1.as_ptr();
        let raw_src2 = s2.as_ptr();
        unsafe {
            let res = FIND__STRING(raw_src1, raw_src2);
            assert_eq!(res, 1) 
        }
    }

    #[test]
    fn test_find_index_correct_edge_case2() {
        let s1 = "hello world\0";
        let s2 = "d\0";
        let raw_src1 = s1.as_ptr();
        let raw_src2 = s2.as_ptr();
        unsafe {
            let res = FIND__STRING(raw_src1, raw_src2);
            assert_eq!(res, 11) 
        }
    }
    #[test]
    fn test_find_index_correct_multibyte() {
        let s1 = "hello ϕϚϡϗ\0";
        let s2 = "ϗ\0";
        let raw_src1 = s1.as_ptr();
        let raw_src2 = s2.as_ptr();
        unsafe {
            let res = FIND__STRING(raw_src1, raw_src2);
            assert_eq!(res, 10) 
        }
    }
    
    #[test]
    fn test_left_ext_str() {
        let s = "ϕϚϡϗ hello\0";
        let len = 7;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            LEFT_EXT__STRING(raw_src, len, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("ϕϚϡϗ he", str_slice)
        }
    }

    #[test]
    fn test_left_ext_str_edge_case() {
        let s = "ϕϚϡϗ hello\0";
        let len = 10;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            LEFT_EXT__STRING(raw_src, len, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("ϕϚϡϗ hello", str_slice)
        }
    }

    #[test]
    #[should_panic]
    fn test_left_ext_str_len_out_of_range() {
        let s = "hello\0 world";
        let len = 7;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            LEFT_EXT__STRING(raw_src, len, raw_dest);
        }
    }

    #[test]
    fn test_right_ext_str() {
        let s = "ϕϚϡϗ hello\0";
        let len = 5;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            RIGHT_EXT__STRING(raw_src, len, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("hello", str_slice)
        }
    }

    #[test]
    fn test_right_ext_str_multi_byte() {
        let s = "ϕϚϡxϗ wϕrld\0";
        let len = 8;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            RIGHT_EXT__STRING(raw_src, len, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("xϗ wϕrld", str_slice)
        }
    }

    #[test]
    fn test_mid_ext_str() {
        let s = "ϕϚϡxϗ wϕrld\0";
        let len = 6;
        let start_index = 3;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            MID_EXT__STRING(raw_src, len, start_index, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("ϡxϗ wϕ", str_slice)
        }
    }

    #[test]
    fn test_mid_ext_str_edge_case() {
        let s = "ϕϚϡxϗ wϕrld\0";
        let len = 11;
        let start_index = 1;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            MID_EXT__STRING(raw_src, len, start_index, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("ϕϚϡxϗ wϕrld", str_slice)
        }
    }

    #[test]
    #[should_panic]
    fn test_mid_ext_str_start_index_out_of_range() {
        let s = "hello world\0";
        let len = 5;
        let start_index = 12;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe { MID_EXT__STRING(raw_src, len, start_index, raw_dest) };
    }

    #[test]
    fn test_insert_ext_str() {
        let base = "ϕϚϡxϗ wϕrld\0";
        let s = "brave new \0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__STRING(raw_src1, raw_src2, 6, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("ϕϚϡxϗ brave new wϕrld", str_slice)
        }
    }

    #[test]
    fn test_insert_ext_str_insert_at_zero() {
        let base = "hello world\0";
        let s = "ϕϚϡxϗ new \0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__STRING(raw_src1, raw_src2, 0, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("ϕϚϡxϗ new hello world", str_slice)
        }
    }

    #[test]
    fn test_insert_ext_str_insert_at_end() {
        let base = "hello world\0";
        let s = "ϕϚϡxϗ new \0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__STRING(raw_src1, raw_src2, (base.len() - 1) as i32, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("hello worldϕϚϡxϗ new ", str_slice)
        }
    }

    #[test]
    #[should_panic]
    fn test_insert_ext_str_pos_out_of_range() {
        let base = "hello world\0";
        let s = "brave new \0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__STRING(raw_src1, raw_src2, base.len() as i32, raw_dest);
        }
    }

    #[test]
    #[should_panic]
    fn test_insert_ext_str_pos_negative() {
        let base = "hello world\0";
        let s = "brave new \0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__STRING(raw_src1, raw_src2, -2, raw_dest);
        }
    }

    #[test]
    fn test_delete_ext_str() {
        let base = "ϕϚϡxϗ wϕrld\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = base.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__STRING(raw_src, 9, 3, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("ϕϚ", str_slice)
        }
    }

    #[test]
    fn test_delete_ext_str_delete_all() {
        let base = "ϕϚϡxϗ wϕrld\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = base.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__STRING(raw_src, 11, 1, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("", str_slice)
        }
    }

    #[test]
    fn test_delete_ext_str_delete_last() {
        let base = "ϕϚϡxϗ wϕrld\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = base.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__STRING(raw_src, 1, 11, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("ϕϚϡxϗ wϕrl", str_slice)
        }
    }

    #[test]
    fn test_delete_ext_str_delete_first() {
        let base = "ϕϚϡxϗ wϕrld\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = base.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__STRING(raw_src, 1, 1, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("Ϛϡxϗ wϕrld", str_slice)
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_str_too_many_del_chars() {
        let base = "hello world\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = base.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__STRING(raw_src, 12, 1, raw_dest);
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_str_pos_out_of_range_lower() {
        let base = "hello world\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = base.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__STRING(raw_src, 11, 0, raw_dest);
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_str_pos_out_of_range_upper() {
        let base = "hello world\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = base.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__STRING(raw_src, 11, 12, raw_dest);
        }
    }

    #[test]
    fn test_replace_ext_str_replace_at_beginning() {
        let base = "ϕϚϡxϗ wϕrld\0";
        let s = "brϡxϗ new \0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__STRING(raw_src1, raw_src2, 6, 1, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("brϡxϗ new wϕrld", str_slice)
        }
    }

    #[test]
    fn test_replace_ext_str_replace_at_middle() {
        let base = "hellϕ wϕrld\0";
        let s = "brϡxϗ new\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__STRING(raw_src1, raw_src2, 3, 5, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("hellbrϡxϗ newϕrld", str_slice)
        }
    }

    #[test]
    fn test_replace_ext_str_replace_at_end() {
        // 
        let base = "hællø wørlÞ\0";
        let s = "aldø, how are you😀\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__STRING(raw_src1, raw_src2, 4, 8, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("hællø waldø, how are you😀", str_slice)
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_str_replace_too_many_chars() {
        let base = "hello world\0";
        let s = "aldo, how are you\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__STRING(raw_src1, raw_src2, 12, 1, raw_dest);
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_str_pos_out_of_bounds_lower() {
        let base = "hello world\0";
        let s = "aldo, how are you\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__STRING(raw_src1, raw_src2, 8, 0, raw_dest);
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_str_pos_out_of_bounds_upper() {
        let base = "hello world\0";
        let s = "aldo, how are you\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__STRING(raw_src1, raw_src2, 8, 12, raw_dest);
        }
    }

    // -----------------------------------UTF16
    #[test]
    fn test_len_correct_utf16_character_count()
    {
        let v1: Vec<u16> = "𝄞music𝄞 😀𝄞ϕϚϡϗ😀\0".encode_utf16().collect();
        let s = &v1[..];
        let raw_src = s.as_ptr();
        unsafe {
            let res = LEN__WSTRING(raw_src);
            assert_eq!(res, 15)
        }
        
    }

    #[test]
    fn test_find_wstring() {
        let v1: Vec<u16> = "𝄞music𝄞 world\0".encode_utf16().collect();
        let v2: Vec<u16> = "c𝄞\0".encode_utf16().collect();
        let base = &v1[..];
        let find = &v2[..];
        let raw_src1 = base.as_ptr();
        let raw_src2 = find.as_ptr();
        unsafe {
            let res = FIND__WSTRING(raw_src1, raw_src2);
            assert_eq!(6, res)
        }
    }

    #[test]
    fn test_find_wstring_no_match() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "zzzzzz\0".encode_utf16().collect();
        let base = &v1[..];
        let find = &v2[..];
        let raw_src1 = base.as_ptr();
        let raw_src2 = find.as_ptr();
        unsafe {
            let res = FIND__WSTRING(raw_src1, raw_src2);
            assert_eq!(0, res)
        }
    }

    #[test]
    fn test_find_wstring_base_string_too_short() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "hello world oachkatzlschwoaf\0".encode_utf16().collect();
        let base = &v1[..];
        let find = &v2[..];
        let raw_src1 = base.as_ptr();
        let raw_src2 = find.as_ptr();
        unsafe {
            let res = FIND__WSTRING(raw_src1, raw_src2);
            assert_eq!(0, res)
        }
    }

    #[test]
    fn test_left_ext_wstring() {
        let v1: Vec<u16> = "𝄞musϗ😀ic world\0".encode_utf16().collect();
        let arr = &v1[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src = arr.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            LEFT_EXT__WSTRING(raw_src, 7, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("𝄞musϗ😀i", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_left_ext_wstring_len_out_of_range() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let arr = &v1[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src = arr.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            LEFT_EXT__WSTRING(raw_src, 14, raw_dest);
        }
    }

    #[test]
    fn test_right_ext_wstring() {
        let v1: Vec<u16> = "hello 𝄞musϗ😀".encode_utf16().collect();
        let arr = &v1[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src = arr.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            RIGHT_EXT__WSTRING(raw_src, 8, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("o 𝄞musϗ😀", res)
        }
    }

    #[test]
    fn test_right_ext_wstring_zero_length_strings() {
        let v1: Vec<u16> = "\0".encode_utf16().collect();
        let arr = &v1[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src = arr.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            RIGHT_EXT__WSTRING(raw_src, 0, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("", res)
        }
    }

    #[test]
    fn test_mid_ext_wstring() {
        let v1: Vec<u16> = "𝄞muϗ😀 world\0".encode_utf16().collect();
        let arr = &v1[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src = arr.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            MID_EXT__WSTRING(raw_src, 5, 5, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("😀 wor", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_mid_ext_wstring_index_out_of_range() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let arr = &v1[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src = arr.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            MID_EXT__WSTRING(raw_src, 4, 12, raw_dest);
        }
    }

    #[test]
    fn test_insert_ext_wstring() {
        let v1: Vec<u16> = "𝄞muϗ😀 world\0".encode_utf16().collect();
        let v2: Vec<u16> = "brave 𝄞muϗ😀 \0".encode_utf16().collect();
        let arr1 = &v1[..];
        let arr2 = &v2[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            INSERT_EXT__WSTRING(raw_src1, raw_src2, 6, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("𝄞muϗ😀 brave 𝄞muϗ😀 world", res)
        }
    }

    #[test]
    fn test_insert_ext_wstring_insert_at_zero() {
        let v1: Vec<u16> = "hello 𝄞muϗ😀\0".encode_utf16().collect();
        let v2: Vec<u16> = "𝄞muϗ😀 new \0".encode_utf16().collect();
        let arr1 = &v1[..];
        let arr2 = &v2[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            INSERT_EXT__WSTRING(raw_src1, raw_src2, 0, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("𝄞muϗ😀 new hello 𝄞muϗ😀", res)
        }
    }

    #[test]
    fn test_insert_ext_wstring_insert_at_end() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "brave 𝄞muϗ😀 \0".encode_utf16().collect();
        let arr1 = &v1[..];
        let arr2 = &v2[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            INSERT_EXT__WSTRING(raw_src1, raw_src2, 11, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("hello worldbrave 𝄞muϗ😀 ", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_insert_ext_wstring_pos_out_of_range() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "brave new \0".encode_utf16().collect();
        let arr1 = &v1[..];
        let arr2 = &v2[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            INSERT_EXT__WSTRING(raw_src1, raw_src2, 12, raw_dest);
        }
    }

    #[test]
    fn test_delete_ext_wstring() {
        let v1: Vec<u16> = "h𝄞muϗ w😀rld\0".encode_utf16().collect();
        let arr1 = &v1[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src = arr1.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            DELETE_EXT__WSTRING(raw_src, 5, 3, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("h𝄞😀rld", res)
        }
    }

    #[test]
    fn test_delete_ext_wstring_delete_all() {
        let v1: Vec<u16> = "h𝄞muϗ w😀rld\0".encode_utf16().collect();
        let arr1 = &v1[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src = arr1.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__WSTRING(raw_src, 11, 1, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_wstring_too_many_del_chars() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let arr1 = &v1[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src = arr1.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__WSTRING(raw_src, 10, 3, raw_dest);
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_wstring_pos_out_of_range_lower() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let arr1 = &v1[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src = arr1.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__WSTRING(raw_src, 9, 0, raw_dest);
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_wstring_pos_out_of_range_upper() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let arr1 = &v1[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src = arr1.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__WSTRING(raw_src, 9, 12, raw_dest);
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_beginning() {
        let v1: Vec<u16> = "h𝄞muϗ w😀rld\0".encode_utf16().collect();
        let v2: Vec<u16> = "brave new \0".encode_utf16().collect();
        let arr1 = &v1[..];
        let arr2 = &v2[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__WSTRING(raw_src1, raw_src2, 6, 1, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("brave new w😀rld", res)
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_middle() {
        let v1: Vec<u16> = "hello w😀rld𝄞\0".encode_utf16().collect();
        let v2: Vec<u16> = " is out of this \0".encode_utf16().collect();
        let arr1 = &v1[..];
        let arr2 = &v2[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__WSTRING(raw_src1, raw_src2, 2, 5, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("hell is out of this w😀rld𝄞", res)
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_end() {
        let v1: Vec<u16> = "hello w😀rld𝄞\0".encode_utf16().collect();
        let v2: Vec<u16> = "aldo, how are you? 😀\0".encode_utf16().collect();
        let arr1 = &v1[..];
        let arr2 = &v2[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__WSTRING(raw_src1, raw_src2, 5, 8, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("hello waldo, how are you? 😀", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_wstring_replace_too_many_chars() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = " is out of this \0".encode_utf16().collect();
        let arr1 = &v1[..];
        let arr2 = &v2[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__WSTRING(raw_src1, raw_src2, 12, 1, raw_dest);
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_wstring_pos_out_of_bounds_lower() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = " is out of this \0".encode_utf16().collect();
        let arr1 = &v1[..];
        let arr2 = &v2[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__WSTRING(raw_src1, raw_src2, 8, 0, raw_dest);
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_wstring_pos_out_of_bounds_upper() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = " is out of this \0".encode_utf16().collect();
        let arr1 = &v1[..];
        let arr2 = &v2[..];
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__WSTRING(raw_src1, raw_src2, 8, 12, raw_dest);
        }
    }
}
