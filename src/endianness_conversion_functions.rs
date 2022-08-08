use paste::paste;
macro_rules! define_endianness_for_int_types {
    ( $st_type:tt, $t:ty ) => {
        paste! {
            #[allow(non_snake_case)]
            #[no_mangle]
            pub fn [<TO_BIG_ENDIAN__ $st_type>](input: $t) -> $t {
                return input.to_be();
            }

            #[allow(non_snake_case)]
            #[no_mangle]
            pub fn [<TO_LITTLE_ENDIAN__ $st_type>](input: $t) -> $t {
                return input.to_le();
            }
            
            #[allow(non_snake_case)]
            #[no_mangle]
            pub fn [<FROM_BIG_ENDIAN__ $st_type>](input: $t) -> $t {
                return input.to_le();
            }

            #[allow(non_snake_case)]
            #[no_mangle]
            pub fn [<FROM_LITTLE_ENDIAN__ $st_type>](input: $t) -> $t {
                return input.to_be();
            }
        }
    }
}

define_endianness_for_int_types!(INT, i16);
define_endianness_for_int_types!(DINT, i32);
define_endianness_for_int_types!(LINT, i64);
define_endianness_for_int_types!(UINT, u16);
define_endianness_for_int_types!(UDINT, u32);
define_endianness_for_int_types!(ULINT, u64);
define_endianness_for_int_types!(WORD, u16);
define_endianness_for_int_types!(DWORD, u32);
define_endianness_for_int_types!(LWORD, u64);
define_endianness_for_int_types!(WCHAR, u16);
define_endianness_for_int_types!(DATE, i64);
define_endianness_for_int_types!(TIME_OF_DAY, i64);
define_endianness_for_int_types!(DATE_AND_TIME, i64);

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
