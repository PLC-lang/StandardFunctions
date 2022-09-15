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

impl<I: Iterator<Item = char>> CharsEncoder<u8> for I {
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

impl<I: Iterator<Item = Result<char, DecodeUtf16Error>>> CharsEncoder<u16> for I {
    unsafe fn encode(self, dest: &mut *mut u16) {
        for c in self {
            let mut temp = [0_u16; 2];
            let slice = c.unwrap().encode_utf16(&mut temp);
            for word in slice {
                **dest = *word;
                *dest = dest.add(1);
            }
        }

        **dest = 0;
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

    // let chars = dbg!(EncodedCharsIter::decode(src));
    // let chars = dbg!(chars.take(pos));
    // let chars = dbg!(chars.chain(EncodedCharsIter::decode(src).skip(pos + ndel)));
    // chars.encode(&mut dest);
    // dbg!(ptr_to_slice(dest));

    EncodedCharsIter::decode(src)
        .take(pos)
        .chain(EncodedCharsIter::decode(src).skip(ndel + pos))
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
        let src = "ϕϚϡϗabcd\0";
        unsafe {
            let res = LEN__STRING(src.as_ptr());
            assert_eq!(res, 8)
        }
    }

    #[test]
    fn test_find_index_correct() {
        let haystack = "hϗllo wϕrld\0";
        let needle = "llo\0";
        unsafe {
            let res = FIND__STRING(haystack.as_ptr(), needle.as_ptr());
            assert_eq!(res, 3)
        }
    }

    #[test]
    fn test_find_index_correct_edge_case() {
        let haystack = "hello wϕrld\0";
        let needle = "h\0";
        unsafe {
            let res = FIND__STRING(haystack.as_ptr(), needle.as_ptr());
            assert_eq!(res, 1)
        }
    }

    #[test]
    fn test_find_index_correct_edge_case2() {
        let haystack = "hello world\0";
        let needle = "d\0";
        unsafe {
            let res = FIND__STRING(haystack.as_ptr(), needle.as_ptr());
            assert_eq!(res, 11)
        }
    }

    #[test]
    fn test_find_index_correct_multibyte() {
        let haystack = "hello ϕϚϡϗ\0";
        let needle = "ϗ\0";
        unsafe {
            let res = FIND__STRING(haystack.as_ptr(), needle.as_ptr());
            assert_eq!(res, 10)
        }
    }

    #[test]
    fn test_left_ext_str() {
        let src = "ϕϚϡϗ hello\0";
        let len = 7;
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            LEFT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("ϕϚϡϗ he", string)
        }
    }

    #[test]
    fn test_left_ext_long_str() {
        let src = "     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.\0";
        let len = 85;
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            LEFT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("     this is   a  very   long           sentence   with plenty  of    characters and ", string)
        }
    }

    #[test]
    fn test_left_ext_str_w_escape_sequence() {
        let src = "ϕ\"Ϛ\"ϡϗ hello\0";
        let len = 6;
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            LEFT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("ϕ\"Ϛ\"ϡϗ", string)
        }
    }

    #[test]
    fn test_left_ext_str_edge_case() {
        let src = "ϕϚϡϗ hello\0";
        let len = 10;
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            LEFT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("ϕϚϡϗ hello", string)
        }
    }

    #[test]
    #[should_panic]
    fn test_left_ext_str_len_out_of_range() {
        let src = "hello\0 world";
        let len = 7;
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            LEFT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_right_ext_str() {
        let src = "ϕϚϡϗ hello\0";
        let len = 5;
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            RIGHT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("hello", string)
        }
    }

    #[test]
    fn test_right_ext_str_multi_byte() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let len = 8;
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            RIGHT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("xϗ wϕrld", string)
        }
    }

    #[test]
    fn test_mid_ext_str() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let len = 6;
        let start_index = 3;
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            MID_EXT__STRING(src.as_ptr(), len, start_index, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("ϡxϗ wϕ", string)
        }
    }

    #[test]
    fn test_mid_ext_str_edge_case() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let len = 11;
        let start_index = 1;
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            MID_EXT__STRING(src.as_ptr(), len, start_index, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("ϕϚϡxϗ wϕrld", string)
        }
    }

    #[test]
    #[should_panic]
    fn test_mid_ext_str_start_index_out_of_range() {
        let src = "hello world\0";
        let len = 5;
        let start_index = 12;
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe { MID_EXT__STRING(src.as_ptr(), len, start_index, dest.as_mut_ptr()) };
    }

    #[test]
    fn test_insert_ext_str() {
        let base = "ϕϚϡxϗ wϕrld\0";
        let insert = "brave new \0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            INSERT_EXT__STRING(base.as_ptr(), insert.as_ptr(), 6, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("ϕϚϡxϗ brave new wϕrld", string)
        }
    }

    #[test]
    fn test_insert_ext_str_insert_at_zero() {
        let base = "hello world\0";
        let insert = "ϕϚϡxϗ new \0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            INSERT_EXT__STRING(base.as_ptr(), insert.as_ptr(), 0, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("ϕϚϡxϗ new hello world", string)
        }
    }

    #[test]
    fn test_insert_ext_str_insert_at_end() {
        let base = "hello world\0";
        let insert = "ϕϚϡxϗ new \0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            INSERT_EXT__STRING(
                base.as_ptr(),
                insert.as_ptr(),
                (base.len() - 1) as i32,
                dest.as_mut_ptr(),
            );
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("hello worldϕϚϡxϗ new ", string)
        }
    }

    #[test]
    #[should_panic]
    fn test_insert_ext_str_pos_out_of_range() {
        let base = "hello world\0";
        let insert = "brave new \0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            INSERT_EXT__STRING(
                base.as_ptr(),
                insert.as_ptr(),
                base.len() as i32,
                dest.as_mut_ptr(),
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_insert_ext_str_pos_negative() {
        let base = "hello world\0";
        let insert = "brave new \0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            INSERT_EXT__STRING(base.as_ptr(), insert.as_ptr(), -2, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_delete_ext_str() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 9, 3, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("ϕϚ", string)
        }
    }

    #[test]
    fn test_delete_ext_str_delete_all() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 11, 1, dest.as_mut_ptr());
            let c_str: &CStr = CStr::from_ptr(dest.as_mut_ptr() as *const i8);
            let string: &str = c_str.to_str().unwrap();
            assert_eq!("", string)
        }
    }

    #[test]
    fn test_delete_ext_str_delete_last() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 1, 11, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("ϕϚϡxϗ wϕrl", string)
        }
    }

    #[test]
    fn test_delete_ext_str_delete_first() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 1, 1, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("Ϛϡxϗ wϕrld", string)
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_str_too_many_del_chars() {
        let src = "hello world\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 12, 1, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_str_pos_out_of_range_lower() {
        let src = "hello world\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 11, 0, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_str_pos_out_of_range_upper() {
        let src = "hello world\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 11, 12, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_replace_ext_str_replace_at_beginning() {
        let base = "ϕϚϡxϗ wϕrld\0";
        let replacement = "brϡxϗ new \0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__STRING(base.as_ptr(), replacement.as_ptr(), 6, 1, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("brϡxϗ new wϕrld", string)
        }
    }

    #[test]
    fn test_replace_ext_str_replace_at_middle() {
        let base = "hellϕ wϕrld\0";
        let replacement = "brϡxϗ new\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__STRING(base.as_ptr(), replacement.as_ptr(), 3, 5, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("hellbrϡxϗ newϕrld", string)
        }
    }

    #[test]
    fn test_replace_ext_str_replace_at_end() {
        let base = "hællø wørlÞ\0";
        let replacement = "aldø, how are you😀\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__STRING(base.as_ptr(), replacement.as_ptr(), 4, 8, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("hællø waldø, how are you😀", string)
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_str_replace_too_many_chars() {
        let base = "hello world\0";
        let replacement = "aldo, how are you\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__STRING(
                base.as_ptr(),
                replacement.as_ptr(),
                12,
                1,
                dest.as_mut_ptr(),
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_str_pos_out_of_bounds_lower() {
        let base = "hello world\0";
        let replacement = "aldo, how are you\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__STRING(base.as_ptr(), replacement.as_ptr(), 8, 0, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_str_pos_out_of_bounds_upper() {
        let base = "hello world\0";
        let replacement = "aldo, how are you\0";
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__STRING(
                base.as_ptr(),
                replacement.as_ptr(),
                8,
                12,
                dest.as_mut_ptr(),
            );
        }
    }

    #[test]
    fn test_concat_ext_str() {
        let argv = [
            "hællø wørlÞ\0".as_ptr(),
            "hello world\0".as_ptr(),
            "𝄞music\0".as_ptr(),
        ];
        let argc = argv.len();
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            CONCAT_EXT__STRING(argc, argv.as_ptr(), dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("hællø wørlÞhello world𝄞music", string)
        }
    }

    #[test]
    fn test_concat_no_args() {
        let argv = [];
        let argc = argv.len();
        let mut dest: [u8; 1024] = [0; 1024];
        unsafe {
            CONCAT_EXT__STRING(argc, argv.as_ptr(), dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const i8).to_str().unwrap();
            assert_eq!("", string)
        }
    }

    // -----------------------------------UTF16
    #[test]
    fn test_len_correct_utf16_character_count() {
        let src = "𝄞music𝄞 😀𝄞ϕϚϡϗ😀\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        unsafe {
            let res = LEN__WSTRING(src_ptr);
            assert_eq!(res, 15)
        }
    }

    #[test]
    fn test_find_wstring() {
        let base = "𝄞music𝄞 world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let find = "c𝄞\0".encode_utf16().collect::<Vec<u16>>();
        let find_ptr = find.as_slice().as_ptr();
        unsafe {
            let res = FIND__WSTRING(base_ptr, find_ptr);
            assert_eq!(6, res)
        }
    }

    #[test]
    fn test_find_wstring_no_match() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let find = "zzzzzz\0".encode_utf16().collect::<Vec<u16>>();
        let find_ptr = find.as_slice().as_ptr();
        unsafe {
            let res = FIND__WSTRING(base_ptr, find_ptr);
            assert_eq!(0, res)
        }
    }

    #[test]
    fn test_find_wstring_base_string_too_short() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let find = "hello world oachkatzlschwoaf\0"
            .encode_utf16()
            .collect::<Vec<u16>>();
        let find_ptr = find.as_slice().as_ptr();
        unsafe {
            let res = FIND__WSTRING(base_ptr, find_ptr);
            assert_eq!(0, res)
        }
    }

    #[test]
    fn test_left_ext_wstring() {
        let src = "𝄞musϗ😀ic world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            LEFT_EXT__WSTRING(src_ptr, 7, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("𝄞musϗ😀i", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_left_ext_wstring_len_out_of_range() {
        let src = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            LEFT_EXT__WSTRING(src_ptr, 14, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_right_ext_wstring() {
        let src = "hello 𝄞musϗ😀\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            RIGHT_EXT__WSTRING(src_ptr, 8, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));

            assert_eq!("o 𝄞musϗ😀", res)
        }
    }

    #[test]
    fn test_right_ext_wstring_zero_length_strings() {
        let src = "\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            RIGHT_EXT__WSTRING(src_ptr, 0, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));

            assert_eq!("", res)
        }
    }

    #[test]
    fn test_mid_ext_wstring() {
        let src = "𝄞muϗ😀 world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            MID_EXT__WSTRING(src_ptr, 5, 5, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("😀 wor", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_mid_ext_wstring_index_out_of_range() {
        let src = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            MID_EXT__WSTRING(src_ptr, 4, 12, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_insert_ext_wstring() {
        let base = "𝄞muϗ😀 world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let to_insert = "brave 𝄞muϗ😀 \0".encode_utf16().collect::<Vec<u16>>();
        let to_insert_ptr = to_insert.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            INSERT_EXT__WSTRING(base_ptr, to_insert_ptr, 6, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("𝄞muϗ😀 brave 𝄞muϗ😀 world", res)
        }
    }

    #[test]
    fn test_insert_ext_wstring_insert_at_zero() {
        let base = "hello 𝄞muϗ😀\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let to_insert = "𝄞muϗ😀 new \0".encode_utf16().collect::<Vec<u16>>();
        let to_insert_ptr = to_insert.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            INSERT_EXT__WSTRING(base_ptr, to_insert_ptr, 0, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("𝄞muϗ😀 new hello 𝄞muϗ😀", res)
        }
    }

    #[test]
    fn test_insert_ext_wstring_insert_at_end() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_ptr();
        let to_insert = "brave 𝄞muϗ😀 \0".encode_utf16().collect::<Vec<u16>>();
        let to_insert_ptr = to_insert.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            INSERT_EXT__WSTRING(base_ptr, to_insert_ptr, 11, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("hello worldbrave 𝄞muϗ😀 ", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_insert_ext_wstring_pos_out_of_range() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let to_insert = "brave new \0".encode_utf16().collect::<Vec<u16>>();
        let to_insert_ptr = to_insert.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            INSERT_EXT__WSTRING(base_ptr, to_insert_ptr, 12, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_delete_ext_wstring() {
        let src = "h𝄞muϗ w😀rld\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__WSTRING(src_ptr, 5, 3, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("h𝄞😀rld", res)
        }
    }

    #[test]
    fn test_delete_ext_wstring_delete_all() {
        let src = "h𝄞muϗ w😀rld\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__WSTRING(src_ptr, 11, 1, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));

            assert_eq!("", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_wstring_too_many_del_chars() {
        let src = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__WSTRING(src_ptr, 10, 3, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_wstring_pos_out_of_range_lower() {
        let src = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__WSTRING(src_ptr, 9, 0, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_wstring_pos_out_of_range_upper() {
        let src = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            DELETE_EXT__WSTRING(src_ptr, 9, 12, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_beginning() {
        let base = "h𝄞muϗ w😀rld\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = "brave new \0".encode_utf16().collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 6, 1, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));

            assert_eq!("brave new w😀rld", res)
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_middle() {
        let base = "hello w😀rld𝄞\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = " is out of this \0".encode_utf16().collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 2, 5, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("hell is out of this w😀rld𝄞", res)
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_end() {
        let base = "hello w😀rld𝄞\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = "aldo, how are you? 😀\0"
            .encode_utf16()
            .collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 5, 8, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));

            assert_eq!("hello waldo, how are you? 😀", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_wstring_replace_too_many_chars() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = " is out of this \0".encode_utf16().collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 12, 1, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_wstring_pos_out_of_bounds_lower() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = " is out of this \0".encode_utf16().collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 8, 0, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_wstring_pos_out_of_bounds_upper() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = " is out of this \0".encode_utf16().collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 8, 12, dest.as_mut_ptr());
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
        let mut dest: [u16; 1024] = [0; 1024];
        unsafe {
            CONCAT_EXT__WSTRING(argc, argv.as_ptr(), dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("hællø wørlÞhello world𝄞music", res)
        }
    }
}