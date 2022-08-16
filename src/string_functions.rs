/// Helper function
/// Gets the amount of continuous characters in u8 array before
/// the first null-terminator.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// May return a false value if passed an
/// array filled with (non-zero) garbage values.
unsafe fn get_null_terminated_len_utf8(src: *const u8) -> usize {
    if src.is_null() {
        return 0;
    }

    (0..).take_while(|&i| *src.offset(i) != 0).count() as usize
}

/// Helper function
/// Gets the amount of continuous characters in u16 array before
/// the first null-terminator.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// May return a false value if passed an
/// array filled with (non-zero) garbage values.
unsafe fn get_null_terminated_len_utf16(src: *const u16) -> usize {
    if src.is_null() {
        return 0;
    }

    (0..).take_while(|&i| *src.offset(i) != 0).count() as usize
}

/// Gets length of the given character string.
/// UTF8
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn LEN__STRING(src: &str) -> i32 {
    src.len() as i32
}

/// Gets length of the given string.
/// UTF16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe fn LEN__WSTRING(src: *const u16) -> i32 {
    get_null_terminated_len_utf16(src) as i32
}

/// Finds the first occurance of the second string (in2)
/// within the first string (in1).
/// UTF8
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn FIND__STRING(in1: &str, in2: &str) -> i32 {
    in1.find(in2).unwrap_or_default() as i32
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
pub unsafe fn FIND__WSTRING(src1: *const u16, src2: *const u16) -> i32 {
    let len1 = get_null_terminated_len_utf16(src1);
    let len2 = get_null_terminated_len_utf16(src2);

    if len2 > len1 || len1 == 0 || len2 == 0 {
        return 0;
    }

    for i in 0..(len1 - len2) {
        let mut consecutive_matches = 0;

        for j in 0..len2 {
            if *src1.add(i + j) == *src2.add(j) {
                consecutive_matches += 1;
            } else {
                break;
            }
        }

        if consecutive_matches == len2 {
            return (i + 1) as i32;
        }
    }

    0
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

    let len = get_null_terminated_len_utf8(src);

    if len < substr_len as usize {
        panic!("Substring length exceeds string length");
    }

    for i in 0..substr_len as usize {
        *dest.add(i) = *src.add(i);
    }

    // null-terminate created string
    *dest.add(len) = b'\0';

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
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }

    let len = get_null_terminated_len_utf16(src);

    if len < substr_len as usize {
        panic!("Substring length exceeds string length");
    }

    for i in 0..substr_len as usize {
        *dest.add(i) = *src.add(i);
    }

    // null-terminate created string
    *dest.add(len) = 0;

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
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }

    let len = get_null_terminated_len_utf8(src);

    if len < substr_len as usize {
        panic!("Substring length exceeds string length");
    }

    let offset = len - substr_len as usize;
    for i in 0..substr_len as usize {
        *dest.add(i) = *src.add(i + offset);
    }

    // null-terminate created string
    *dest.add(len) = b'\0';

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

    let len = get_null_terminated_len_utf16(src);

    if len < substr_len as usize {
        panic!("Substring length exceeds string length");
    }

    let offset = len - substr_len as usize;
    for i in 0..substr_len as usize {
        *dest.add(i) = *src.add(i + offset);
    }

    // null-terminate created string
    *dest.add(len) = 0;

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
    if substr_len < 0 || start_index < 0 {
        panic!("Length/Position parameters cannot be negative.");
    }
    let len = get_null_terminated_len_utf8(src);

    if len < substr_len as usize {
        panic!("Substring length exceeds string length");
    }

    // correct for 0-indexing
    let start_index: i32 = start_index as i32 - 1;

    if start_index < 0 || (start_index + substr_len as i32) > len as i32 {
        panic!("Index out of range. Check Length and Position parameters.");
    }

    for i in 0..substr_len as usize {
        *dest.add(i) = *src.add(i + start_index as usize);
    }

    // null-terminate created string
    *dest.add(len) = b'\0';

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
    if substr_len < 0 || start_index < 0 {
        panic!("Length/Position parameters cannot be negative.");
    }
    let len = get_null_terminated_len_utf16(src);

    if len < substr_len as usize {
        panic!("Substring length exceeds string length");
    }

    // correct for 0-indexing
    let start_index: i32 = start_index as i32 - 1;

    if start_index < 0 || (start_index + substr_len as i32) > len as i32 {
        panic!("Index out of range. Check Length and Position parameters.");
    }

    for i in 0..substr_len as usize {
        *dest.add(i) = *src.add(i + start_index as usize);
    }

    // null-terminate created string
    *dest.add(len) = 0;

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

    let len1 = get_null_terminated_len_utf8(src_base);
    let len2 = get_null_terminated_len_utf8(src_to_insert);
    let pos = pos as usize;

    if pos > len1 {
        panic!("Positional parameter cannot exceed base string length.")
    }

    // copy characters until point of insertion
    for i in 0..pos {
        *dest.add(i) = *src_base.add(i);
    }

    // add to-insert characters
    for i in 0..len2 {
        *dest.add(i + pos) = *src_to_insert.add(i);
    }

    // copy remaining characters from original string
    for i in (len2 + pos)..(len1 + len2) {
        *dest.add(i) = *src_base.add(i - len2);
    }

    // null-terminate created string
    *dest.add(len1 + len2) = b'\0';

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

    let len1 = get_null_terminated_len_utf16(src_base);
    let len2 = get_null_terminated_len_utf16(src_to_insert);
    let pos = pos as usize;

    if pos > len1 {
        panic!("Positional parameter cannot exceed base string length.")
    }

    // copy characters until point of insertion
    for i in 0..pos {
        *dest.add(i) = *src_base.add(i);
    }

    // add to-insert characters
    for i in 0..len2 {
        *dest.add(i + pos) = *src_to_insert.add(i);
    }

    // copy remaining characters from original string
    for i in (len2 + pos)..(len1 + len2) {
        *dest.add(i) = *src_base.add(i - len2);
    }

    // null-terminate created string
    *dest.add(len1 + len2) = 0;

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
    let len = get_null_terminated_len_utf8(src);

    if pos < 1 || pos > len as i32 {
        panic!("Index out of bounds.")
    }
    // correct for 0-indexing
    let pos = (pos - 1) as usize;
    let ndel = num_chars_to_delete as usize;

    if ndel + pos > len {
        panic!(
            r#"Cannot delete {} characters starting from index {}.
            Index out of bounds.            
            "#,
            num_chars_to_delete,
            pos + 1
        )
    }

    // copy characters until point of deletion
    for i in 0..pos {
        *dest.add(i) = *src.add(i)
    }

    // skip the amount of characters to be deleted and
    // add the remaining characters
    for i in pos..(len - ndel) {
        *dest.add(i) = *src.add(i + ndel)
    }

    // null-terminate created string
    *dest.add(len - ndel) = b'\0';

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
    let len = get_null_terminated_len_utf16(src);

    if pos < 1 || pos > len as i32 {
        panic!("Index out of bounds.")
    }
    // correct for 0-indexing
    let pos = (pos - 1) as usize;
    let ndel = num_chars_to_delete as usize;

    if ndel + pos > len {
        panic!(
            r#"Cannot delete {} characters starting from index {}.
            Index out of bounds.            
            "#,
            num_chars_to_delete,
            pos + 1
        )
    }

    // copy characters until point of deletion
    for i in 0..pos {
        *dest.add(i) = *src.add(i)
    }

    // skip the amount of characters to be deleted and
    // add the remaining characters
    for i in pos..(len - ndel) {
        *dest.add(i) = *src.add(i + ndel)
    }

    // null-terminate created string
    *dest.add(len - ndel) = 0;

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
    let len1 = get_null_terminated_len_utf8(src_base);
    let len2 = get_null_terminated_len_utf8(src_replacement);

    if pos < 1 || pos > len1 as i32 {
        panic!("Index out of bounds.")
    }

    // correct for 0-indexing
    let pos = (pos - 1) as usize;
    let nreplace = num_chars_to_replace as usize;
    let resulting_len = len1 + len2 - nreplace;

    if nreplace + pos > len1 {
        panic!(
            r#"Cannot replace {} characters starting from index {}.
            Index out of bounds.            
            "#,
            nreplace,
            pos + 1
        )
    }

    // copy characters until point of replacement
    for i in 0..pos {
        *dest.add(i) = *src_base.add(i)
    }

    // add replacement characters
    for i in 0..len2 {
        *dest.add(i + pos) = *src_replacement.add(i)
    }

    // skip over the characters to be replaced and add the remainder
    for i in pos..(len1 - nreplace) {
        *dest.add(i + len2) = *src_base.add(i + nreplace)
    }

    // null-terminate created string
    *dest.add(resulting_len) = b'\0';

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
    let len1 = get_null_terminated_len_utf16(src_base);
    let len2 = get_null_terminated_len_utf16(src_replacement);

    if pos < 1 || pos > len1 as i32 {
        panic!("Index out of bounds.")
    }

    // correct for 0-indexing
    let pos = (pos - 1) as usize;
    let nreplace = num_chars_to_replace as usize;
    let resulting_len = len1 + len2 - nreplace;

    if nreplace + pos > len1 {
        panic!(
            r#"Cannot replace {} characters starting from index {}.
            Index out of bounds.            
            "#,
            nreplace,
            pos + 1
        )
    }

    // copy characters until point of replacement
    for i in 0..pos {
        *dest.add(i) = *src_base.add(i)
    }

    // add replacement characters
    for i in 0..len2 {
        *dest.add(i + pos) = *src_replacement.add(i)
    }

    // skip over the characters to be replaced and add the remainder
    for i in pos..(len1 - nreplace) {
        *dest.add(i + len2) = *src_base.add(i + nreplace)
    }

    // null-terminate created string
    *dest.add(resulting_len) = 0;

    0
}

// -------------------------------------------------unit tests-----------------------------------------
#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CStr;

    // -----------------------------------UTF8
    #[test]
    fn test_left_ext_str() {
        let s = "hello\0";
        let len = 4;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            LEFT_EXT__STRING(raw_src, len, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("hell", str_slice)
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
        let s = "hello world\0";
        let len = 6;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            RIGHT_EXT__STRING(raw_src, len, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!(" world", str_slice)
        }
    }

    #[test]
    fn test_mid_ext_str() {
        let s = "hello world\0";
        let len = 5;
        let start_index = 3;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            MID_EXT__STRING(raw_src, len, start_index, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("llo w", str_slice)
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
        unsafe { MID_EXT__STRING(raw_src, len, start_index, raw_dest) }
    }

    #[test]
    fn test_insert_ext_str() {
        let base = "hello world\0";
        let s = "brave new \0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__STRING(raw_src1, raw_src2, 6, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("hello brave new world", str_slice)
        }
    }

    #[test]
    fn test_insert_ext_str_insert_at_zero() {
        let base = "hello world\0";
        let s = "brave new \0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__STRING(raw_src1, raw_src2, 0, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("brave new hello world", str_slice)
        }
    }

    #[test]
    fn test_insert_ext_str_insert_at_end() {
        let base = "hello world\0";
        let s = "brave new \0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__STRING(raw_src1, raw_src2, (base.len() - 1) as i32, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("hello worldbrave new ", str_slice)
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
        let base = "hello world\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = base.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__STRING(raw_src, 6, 3, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("herld", str_slice)
        }
    }

    #[test]
    fn test_delete_ext_str_delete_all() {
        let base = "hello world\0";
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
        let base = "hello world\0";
        let s = "brave new \0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__STRING(raw_src1, raw_src2, 6, 1, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("brave new world", str_slice)
        }
    }

    #[test]
    fn test_replace_ext_str_replace_at_middle() {
        let base = "hello world\0";
        let s = " is out of this \0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__STRING(raw_src1, raw_src2, 2, 5, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("hell is out of this world", str_slice)
        }
    }

    #[test]
    fn test_replace_ext_str_replace_at_end() {
        let base = "hello world\0";
        let s = "aldo, how are you\0";
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src1 = base.as_ptr();
        let raw_src2 = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__STRING(raw_src1, raw_src2, 4, 8, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("hello waldo, how are you", str_slice)
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
    fn test_find_wstring() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "lo\0".encode_utf16().collect();
        let mut base = [0_u16; 20];
        let mut find = [0_u16; 20];

        for (place, element) in base.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in find.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

        let raw_src1 = base.as_ptr();
        let raw_src2 = find.as_ptr();
        unsafe {
            let res = FIND__WSTRING(raw_src1, raw_src2);
            assert_eq!(4, res)
        }
    }

    #[test]
    fn test_find_wstring_no_match() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "zzzzzz\0".encode_utf16().collect();
        let mut base = [0_u16; 20];
        let mut find = [0_u16; 20];

        for (place, element) in base.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in find.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

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
        let mut base = [0_u16; 20];
        let mut find = [0_u16; 20];

        for (place, element) in base.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in find.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

        let raw_src1 = base.as_ptr();
        let raw_src2 = find.as_ptr();
        unsafe {
            let res = FIND__WSTRING(raw_src1, raw_src2);
            assert_eq!(0, res)
        }
    }

    #[test]
    fn test_left_ext_wstring() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let mut arr = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        let raw_src = arr.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            LEFT_EXT__WSTRING(raw_src, 4, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("hell", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_left_ext_wstring_len_out_of_range() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let mut arr = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        let raw_src = arr.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            LEFT_EXT__WSTRING(raw_src, 14, raw_dest);
        }
    }

    #[test]
    fn test_right_ext_wstring() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let mut arr = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        let raw_src = arr.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            RIGHT_EXT__WSTRING(raw_src, 7, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("o world", res)
        }
    }

    #[test]
    fn test_right_ext_wstring_zero_length_strings() {
        let v1: Vec<u16> = "\0".encode_utf16().collect();
        let mut arr = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

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
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let mut arr = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        let raw_src = arr.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            MID_EXT__WSTRING(raw_src, 4, 6, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!(" wor", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_mid_ext_wstring_index_out_of_range() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let mut arr = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        let raw_src = arr.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            MID_EXT__WSTRING(raw_src, 4, 12, raw_dest);
        }
    }

    #[test]
    fn test_insert_ext_wstring() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "brave new \0".encode_utf16().collect();
        let mut arr1 = [0_u16; 20];
        let mut arr2 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in arr2.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__WSTRING(raw_src1, raw_src2, 6, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("hello brave new world", res)
        }
    }

    #[test]
    fn test_insert_ext_wstring_insert_at_zero() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "brave new \0".encode_utf16().collect();
        let mut arr1 = [0_u16; 20];
        let mut arr2 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in arr2.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__WSTRING(raw_src1, raw_src2, 0, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("brave new hello world", res)
        }
    }

    #[test]
    fn test_insert_ext_wstring_insert_at_end() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "brave new \0".encode_utf16().collect();
        let mut arr1 = [0_u16; 20];
        let mut arr2 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in arr2.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__WSTRING(raw_src1, raw_src2, 11, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("hello worldbrave new ", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_insert_ext_wstring_pos_out_of_range() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "brave new \0".encode_utf16().collect();
        let mut arr1 = [0_u16; 20];
        let mut arr2 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in arr2.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            INSERT_EXT__WSTRING(raw_src1, raw_src2, 12, raw_dest);
        }
    }

    #[test]
    fn test_delete_ext_wstring() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let mut arr1 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        let raw_src = arr1.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__WSTRING(raw_src, 6, 3, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("herld", res)
        }
    }

    #[test]
    fn test_delete_ext_wstring_delete_all() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let mut arr1 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

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
        let mut arr1 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

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
        let mut arr1 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

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
        let mut arr1 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        let raw_src = arr1.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            DELETE_EXT__WSTRING(raw_src, 9, 12, raw_dest);
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_beginning() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "brave new \0".encode_utf16().collect();
        let mut arr1 = [0_u16; 20];
        let mut arr2 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in arr2.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__WSTRING(raw_src1, raw_src2, 6, 1, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("brave new world", res)
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_middle() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = " is out of this \0".encode_utf16().collect();
        let mut arr1 = [0_u16; 20];
        let mut arr2 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in arr2.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__WSTRING(raw_src1, raw_src2, 2, 5, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("hell is out of this world", res)
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_end() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = "aldo, how are you?\0".encode_utf16().collect();
        let mut arr1 = [0_u16; 20];
        let mut arr2 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in arr2.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__WSTRING(raw_src1, raw_src2, 4, 8, raw_dest);
            let slice =
                std::slice::from_raw_parts(raw_dest, get_null_terminated_len_utf16(raw_dest));
            let res = String::from_utf16_lossy(slice);

            assert_eq!("hello waldo, how are you?", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_wstring_replace_too_many_chars() {
        let v1: Vec<u16> = "hello world\0".encode_utf16().collect();
        let v2: Vec<u16> = " is out of this \0".encode_utf16().collect();
        let mut arr1 = [0_u16; 20];
        let mut arr2 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in arr2.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

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
        let mut arr1 = [0_u16; 20];
        let mut arr2 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in arr2.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

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
        let mut arr1 = [0_u16; 20];
        let mut arr2 = [0_u16; 20];
        let dest: &mut [u16; 1024] = &mut [0; 1024];

        for (place, element) in arr1.iter_mut().zip(v1.iter()) {
            *place = *element;
        }

        for (place, element) in arr2.iter_mut().zip(v2.iter()) {
            *place = *element;
        }

        let raw_src1 = arr1.as_ptr();
        let raw_src2 = arr2.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            REPLACE_EXT__WSTRING(raw_src1, raw_src2, 8, 12, raw_dest);
        }
    }
}
