

/// .
///-------------------------------INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__INT(input: i16) -> i16 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__INT(input: i16) -> i16 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__INT(input: i16) -> i16 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__INT(input: i16) -> i16 {
    input.to_be()
}

/// .
///-------------------------------DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__DINT(input: i32) -> i32 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__DINT(input: i32) -> i32 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__DINT(input: i32) -> i32 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__DINT(input: i32) -> i32 {
    input.to_be()
}

/// .
///-------------------------------LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__LINT(input: i64) -> i64 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__LINT(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__LINT(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__LINT(input: i64) -> i64 {
    input.to_be()
}

/// .
///-------------------------------UINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__UINT(input: u16) -> u16 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__UINT(input: u16) -> u16 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__UINT(input: u16) -> u16 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__UINT(input: u16) -> u16 {
    input.to_be()
}

/// .
///-------------------------------UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__UDINT(input: u32) -> u32 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__UDINT(input: u32) -> u32 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__UDINT(input: u32) -> u32 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__UDINT(input: u32) -> u32 {
    input.to_be()
}

/// .
///-------------------------------ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__ULINT(input: u64) -> u64 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__ULINT(input: u64) -> u64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__ULINT(input: u64) -> u64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__ULINT(input: u64) -> u64 {
    input.to_be()
}

/// .
///-------------------------------REAL
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__REAL(input: f32) -> f32 {
    f32::from_be_bytes(input.to_be_bytes())
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__REAL(input: f32) -> f32 {
    f32::from_le_bytes(input.to_le_bytes())
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__REAL(input: f32) -> f32 {
    f32::from_le_bytes(input.to_le_bytes())
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__REAL(input: f32) -> f32 {
    f32::from_be_bytes(input.to_be_bytes())
}

/// .
///-------------------------------LREAL
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__LREAL(input: f64) -> f64 {
    f64::from_be_bytes(input.to_be_bytes())
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__LREAL(input: f64) -> f64 {
    f64::from_le_bytes(input.to_le_bytes())
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__LREAL(input: f64) -> f64 {
    f64::from_le_bytes(input.to_le_bytes())
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__LREAL(input: f64) -> f64 {
    f64::from_be_bytes(input.to_be_bytes())
}

/// .
///-------------------------------WORD
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__WORD(input: u16) -> u16 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__WORD(input: u16) -> u16 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__WORD(input: u16) -> u16 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__WORD(input: i16) -> i16 {
    input.to_be()
}

/// .
///-------------------------------DWORD
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__DWORD(input: u32) -> u32 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__DWORD(input: u32) -> u32 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__DWORD(input: u32) -> u32 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__DWORD(input: u32) -> u32 {
    input.to_be()
}

/// .
///-------------------------------LWORD
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__LWORD(input: u64) -> u64 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__LWORD(input: u64) -> u64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__LWORD(input: u64) -> u64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__LWORD(input: u64) -> u64 {
    input.to_be()
}

/// .
///-------------------------------WCHAR
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__WCHAR(input: u16) -> u16 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__WCHAR(input: u16) -> u16 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__WCHAR(input: u16) -> u16 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__WCHAR(input: u16) -> u16 {
    input.to_be()
}

/// .
///------------------------------DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__DATE(input: i64) -> i64 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__DATE(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__DATE(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__DATE(input: i64) -> i64 {
    input.to_be()
}

/// .
///-------------------------------LDATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__LDATE(input: i64) -> i64 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__LDATE(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__LDATE(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__LDATE(input: i64) -> i64 {
    input.to_be()
}

/// .
///-------------------------------TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__TOD(input: i64) -> i64 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__TOD(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__TOD(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__TOD(input: i64) -> i64 {
    input.to_be()
}

/// .
///-------------------------------LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__LTOD(input: i64) -> i64 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__LTOD(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__LTOD(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__LTOD(input: i64) -> i64 {
    input.to_be()
}

/// .
///-------------------------------DT
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__DT(input: i64) -> i64 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__DT(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__DT(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__DT(input: i64) -> i64 {
    input.to_be()
}

/// .
///-------------------------------LDT
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__LDT(input: i64) -> i64 {
    input.to_be()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__LDT(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__LDT(input: i64) -> i64 {
    input.to_le()
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__LDT(input: i64) -> i64 {
    input.to_be()
}