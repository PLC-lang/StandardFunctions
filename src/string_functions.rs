use std::char::{decode_utf16, DecodeUtf16Error};

use num::PrimInt;

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
fn get_null_terminated_len<T: num::PrimInt>(src: *const T) -> usize {
    unsafe {
        if src.is_null() {
            return 0;
        }

        (0..).take_while(|&i| !(*src.add(i)).is_zero()).count() as usize
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
unsafe fn ptr_to_slice<'a, T: num::PrimInt>(src: *const T) -> &'a [T] {
    let nbytes = get_null_terminated_len(src);
    std::slice::from_raw_parts(src, nbytes)
}

type Utf16Iterator<'a> = std::char::DecodeUtf16<std::iter::Copied<std::slice::Iter<'a, u16>>>;
type Utf8Iterator<'a> = core::str::Chars<'a>;

trait CharsDecoder<T: PrimInt> {
    type IteratorType: Iterator;
    unsafe fn decode(src: *const T) -> EncodedCharsIter<Self::IteratorType>;
}

trait CharsEncoder<T: PrimInt>: Iterator {
    unsafe fn encode(self, dest: &mut *mut T);
}

#[derive(Debug)]
struct EncodedCharsIter<T: Iterator> {
    iter: T,
}

impl<T: Iterator> Iterator for EncodedCharsIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> CharsDecoder<u8> for EncodedCharsIter<Utf8Iterator<'a>> {
    type IteratorType = Utf8Iterator<'a>;
    unsafe fn decode(src: *const u8) -> Self {
        let slice = ptr_to_slice(src);
        Self {
            iter: std::str::from_utf8(slice).unwrap().chars(),
        }
    }
}

impl<'a, I: Iterator<Item = char>> CharsEncoder<u8> for I {
    unsafe fn encode(self, dest: &mut *mut u8) {
        for char in self {
            let mut temp = [0; 4];
            let slice = char.encode_utf8(&mut temp);
            for byte in slice.as_bytes() {
                **dest = *byte;
                *dest = dest.add(1);
            }
        }

        **dest = 0;
    }
}

impl<'a, I: Iterator<Item = Result<char, DecodeUtf16Error>>> CharsEncoder<u16> for I {
    unsafe fn encode(self, dest: &mut *mut u16) {
        dbg!(&dest);
        for c in self {
            let mut temp = [0_u16; 2];
            let slice = dbg!(c.unwrap().encode_utf16(&mut temp));
            for word in slice {
                **dest = *word;
                *dest = dest.add(1);
            }
        }

        **dest = 0;
        dbg!(dest);
    }
}

impl<'a> CharsDecoder<u16> for EncodedCharsIter<Utf16Iterator<'a>> {
    type IteratorType = Utf16Iterator<'a>;
    unsafe fn decode(src: *const u16) -> Self {
        let src = ptr_to_slice(src);
        Self {
            iter: decode_utf16(src.iter().copied()),
        }
    }
}

// enum EitherIter<CharsEncoder, CharsDecoder>{
//     U8Decoder(CharsDecoder),
//     U8Encoder(CharsEncoder),
//     U16Decoder(CharsDecoder),
//     U16Encoder(CharsEncoder),
// }

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
    EncodedCharsIter::decode(src).count() as i32
}

// unsafe fn len<T: PrimInt>(src: *const T) -> i32 {
//     EncodedCharsIter::decode(src)
//         .count() as i32
// }

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
    EncodedCharsIter::decode(src).count() as i32
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
    let haystack = ptr_to_slice(src1);
    let needle = ptr_to_slice(src2);

    if needle.len() > haystack.len() || haystack.is_empty() || needle.is_empty() {
        return 0;
    }

    if let Some(idx) = haystack
        .windows(needle.len())
        .position(|window| window == needle)
    {
        // get chars until byte index
        let char_index = core::str::from_utf8(std::slice::from_raw_parts(src1, idx))
            .unwrap()
            .chars()
            .count();
        // correct for ST indexing
        char_index as i32 + 1
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
    let haystack = ptr_to_slice(src1);
    let needle = ptr_to_slice(src2);

    if needle.len() > haystack.len() || haystack.is_empty() || needle.is_empty() {
        return 0;
    }

    if let Some(idx) = haystack
        .windows(needle.len())
        .position(|window| window == needle)
    {
        // match found. count utf16 chars to window position
        let char_index =
            decode_utf16(std::slice::from_raw_parts(src1, idx).iter().copied()).count();

        // correct indexing for ST
        char_index as i32 + 1
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
    let nchars = EncodedCharsIter::decode(src).count();
    if nchars < substr_len {
        panic!("Requested substring length exceeds string length.")
    }
    let chars = EncodedCharsIter::decode(src).take(substr_len);
    chars.encode(&mut dest);

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
    let mut dest = dest;
    let substr_len = substr_len as usize;
    let nchars = EncodedCharsIter::decode(src).count();
    if nchars < substr_len {
        panic!("Requested substring length exceeds string length.")
    }
    let chars = EncodedCharsIter::decode(src).take(substr_len);
    chars.encode(&mut dest);

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
    let mut dest = dest;
    let substr_len = substr_len as usize;
    let nchars = EncodedCharsIter::decode(src).count();
    if nchars < substr_len {
        panic!("Requested substring length exceeds string length.")
    }
    let chars = EncodedCharsIter::decode(src).skip(nchars - substr_len);
    chars.encode(&mut dest);

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
    if dbg!(substr_len) < 0 {
        panic!("Length parameter cannot be negative.");
    }
    let mut dest = dbg!(dest);
    let substr_len = substr_len as usize;
    let nchars = dbg!(EncodedCharsIter::decode(src)).count();
    if nchars < substr_len {
        panic!("Requested substring length exceeds string length.")
    }
    let chars = EncodedCharsIter::decode(src).skip(nchars - substr_len);
    dbg!(chars).encode(&mut dest);

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
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }
    let mut dest = dest;
    let substr_len = substr_len as usize;
    let start_index = start_index as usize;
    let nchars = EncodedCharsIter::decode(src).count();
    if start_index < 1 || start_index > nchars {
        panic!("Position is out of bounds.")
    }
    // correct for 0-indexing
    let start_index = start_index - 1;
    if nchars < substr_len + start_index {
        panic!(
            "Requested substring length {} from position {} exceeds string length.",
            substr_len, start_index
        )
    }
    let chars = EncodedCharsIter::decode(src)
        .skip(start_index)
        .take(substr_len);
    chars.encode(&mut dest);

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
    let substr_len = substr_len as usize;
    let start_index = start_index as usize;
    let nchars = EncodedCharsIter::decode(src).count();
    if start_index < 1 || start_index > nchars {
        panic!("Position is out of bounds.")
    }
    // correct for 0-indexing
    let start_index = start_index - 1;
    if nchars < substr_len + start_index {
        panic!(
            "Requested substring length {} from position {} exceeds string length.",
            substr_len, start_index
        )
    }
    let chars = EncodedCharsIter::decode(src)
        .skip(start_index)
        .take(substr_len);
    chars.encode(&mut dest);

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
    let mut dest = dest;
    let nchars = EncodedCharsIter::decode(src_base).count();
    if pos < 0 || pos > nchars as i32 {
        panic! {"Positional parameter is out of bounds."}
    }
    let pos = pos as usize;
    EncodedCharsIter::decode(src_base)
        .take(pos)
        .chain(EncodedCharsIter::decode(src_to_insert))
        .chain(EncodedCharsIter::decode(src_base).skip(pos))
        .encode(&mut dest);

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
    let mut dest = dest;
    let nchars = EncodedCharsIter::decode(src_base).count();
    if pos < 0 || pos > nchars as i32 {
        panic! {"Positional parameter is out of bounds."}
    }
    let pos = pos as usize;
    EncodedCharsIter::decode(src_base)
        .take(pos)
        .chain(EncodedCharsIter::decode(src_to_insert))
        .chain(EncodedCharsIter::decode(src_base).skip(pos))
        .encode(&mut dest);

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
    let mut dest = dest;
    let nchars = EncodedCharsIter::decode(src).count();
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
    EncodedCharsIter::decode(src)
        .take(pos)
        .chain(EncodedCharsIter::decode(src).skip(pos + ndel))
        .encode(&mut dest);

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
    let nchars = EncodedCharsIter::decode(src).count();
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

    EncodedCharsIter::decode(src)
        .take(pos)
        .chain(EncodedCharsIter::decode(src).skip(pos + ndel))
        .encode(&mut dest);

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
    let mut dest = dest;
    let nbase = EncodedCharsIter::decode(src_base).count();
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
    EncodedCharsIter::decode(src_base)
        .take(pos)
        .chain(
            EncodedCharsIter::decode(src_replacement)
                .chain(EncodedCharsIter::decode(src_base).skip(pos + nreplace)),
        )
        .encode(&mut dest);

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
    let mut dest = dest;
    let nbase = EncodedCharsIter::decode(src_base).count();
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
    EncodedCharsIter::decode(src_base)
        .take(pos)
        .chain(
            EncodedCharsIter::decode(src_replacement)
                .chain(EncodedCharsIter::decode(src_base).skip(pos + nreplace)),
        )
        .encode(&mut dest);

    0
}

/// Concatenates all given strings in the order in which they are given.
/// Strings are passed as pointer of pointer to u8, where each pointer represents
/// the starting address of each string. The amount of strings must be passed as
/// argument.
/// UTF8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if trying to index outside of the array or trying
/// to replace more characters than remaining.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CONCAT_EXT__STRING(
    argc: usize,
    argv: *const *const u8,
    dest: *mut u8,
) -> i32 {
    if argv.is_null() || dest.is_null() {
        panic!("Received null-pointer.")
    }
    let mut dest = dest;
    let mut argv = argv;
    for _ in 0..argc {
        EncodedCharsIter::decode(*argv).encode(&mut dest);
        argv = argv.add(1);
    }

    0
}

/// Concatenates all given strings in the order in which they are given.
/// Strings are passed as pointer of pointer to u8, where each pointer represents
/// the starting address of each string. The amount of strings must be passed as
/// argument.
/// UTF16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if trying to index outside of the array or trying
/// to replace more characters than remaining.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CONCAT_EXT__WSTRING(
    argc: usize,
    argv: *const *const u16,
    dest: *mut u16,
) -> i32 {
    if argv.is_null() || dest.is_null() {
        panic!("Received null-pointer.")
    }
    let mut dest = dest;
    let mut argv = argv;
    for _ in 0..argc {
        EncodedCharsIter::decode(*argv).encode(&mut dest);
        argv = argv.add(1);
    }

    0
}
// -------------------------------------------------unit tests-----------------------------------------
#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CStr;
    // -----------------------------------UTF8
    #[test]
    fn test_len_correct_utf8_character_count() {
        let s = "ϕϚϡϗabcd\0";
        let raw_src = s.as_ptr();
        unsafe {
            let res = LEN__STRING(raw_src);
            assert_eq!(res, 8)
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
    fn test_left_ext_str_w_escape_sequence() {
        let s = "ϕ\"Ϛ\"ϡϗ hello\0";
        let len = 6;
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let raw_src = s.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            LEFT_EXT__STRING(raw_src, len, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("ϕ\"Ϛ\"ϡϗ", str_slice)
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

    #[test]
    fn test_concat_ext_str() {
        // let argv = [
        //     CString::new("hællø wørlÞ").unwrap().as_c_str().as_ptr(),
        //     CString::new("hello world").unwrap().as_c_str().as_ptr(),
        //     CString::new("𝄞music").unwrap().as_c_str().as_ptr(),
        // ];
        let argv = [
            "hællø wørlÞ\0".as_ptr(),
            "hello world\0".as_ptr(),
            "𝄞music\0".as_ptr(),
        ];
        let argc = argv.len();
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let argv = argv.as_ptr();
        let raw_dest = dest.as_mut_ptr();

        unsafe {
            CONCAT_EXT__STRING(argc, argv, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("hællø wørlÞhello world𝄞music", str_slice)
        }
    }

    #[test]
    fn test_concat_no_args() {
        let argv = [];
        let argc = argv.len();
        let dest: &mut [u8; 1024] = &mut [0; 1024];
        let argv = argv.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            CONCAT_EXT__STRING(argc, argv, raw_dest);
            let c_str: &CStr = CStr::from_ptr(raw_dest as *const i8);
            let str_slice: &str = c_str.to_str().unwrap();

            assert_eq!("", str_slice)
        }
    }

    // -----------------------------------UTF16
    #[test]
    fn test_len_correct_utf16_character_count() {
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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
            let res = String::from_utf16(slice).unwrap();

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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
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
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
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

    #[test]
    fn test_concat_ext_wstring() {
        let argvec: [Vec<u16>; 3] = [
            "hællø wørlÞ\0".encode_utf16().collect(),
            "hello world\0".encode_utf16().collect(),
            "𝄞music\0".encode_utf16().collect(),
        ];
        let mut argv: [*const u16; 3] = [std::ptr::null(); 3];
        for (i, arg) in argvec.iter().enumerate() {
            argv[i] = arg.as_ptr();
        }
        let argc = argv.len();
        let dest: &mut [u16; 1024] = &mut [0; 1024];
        let argv = argv.as_ptr();
        let raw_dest = dest.as_mut_ptr();
        unsafe {
            CONCAT_EXT__WSTRING(argc, argv, raw_dest);
            let slice = std::slice::from_raw_parts(raw_dest, get_null_terminated_len(raw_dest));
            let res = String::from_utf16_lossy(slice);
            assert_eq!("hællø wørlÞhello world𝄞music", res)
        }
    }
}
