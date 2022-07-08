use std::slice;

/// Integer type macros
macro_rules! define_int_type {
    ($max_name:ident, $min_name:ident, $rust_type:ty) => {
        //Odering operations
        define_order_type!($max_name, $min_name, $rust_type);
    };
}

///Float type macros
macro_rules! define_float_type {
    ($max_name:ident, $min_name:ident, $rust_type:ty) => {
        /// # Safety
        /// Dealing with raw pointers
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn $max_name(size: u32, value: *const $rust_type) -> $rust_type {
            // Declare array for value
            let arr = if !value.is_null() {
                slice::from_raw_parts(value, size as usize)
            } else {
                panic!("Null pointer for value");
            };

            arr.iter()
                .map(|it| *it)
                .reduce(<$rust_type>::max)
                .expect("A max will always exist")
        }
        /// # Safety
        /// Dealing with raw pointers
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn $min_name(size: u32, value: *const $rust_type) -> $rust_type {
            // Declare array for value
            let arr = if !value.is_null() {
                slice::from_raw_parts(value, size as usize)
            } else {
                panic!("Null pointer for value");
            };

            arr.iter()
                .map(|it| *it)
                .reduce(<$rust_type>::min)
                .expect("A max will always exist")
        }
    };
}

/// Ordered type macros
macro_rules! define_order_type {
    ($max_name:ident, $min_name:ident, $rust_type:ty) => {
        //Max impl
        /// # Safety
        /// Dealing with raw pointers
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn $max_name(size: u32, value: *const $rust_type) -> $rust_type {
            if !value.is_null() {
                let arr = slice::from_raw_parts(value, size as usize);
                *arr.iter().max().expect("A max will always exist")
            } else {
                panic!("Null pointer for value");
            }
        }
        //Min impl
        /// # Safety
        /// Dealing with raw pointers
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn $min_name(size: u32, value: *const $rust_type) -> $rust_type {
            if !value.is_null() {
                let arr = slice::from_raw_parts(value, size as usize);
                *arr.iter().min().expect("A min will always exist")
            } else {
                panic!("Null pointer for value");
            }
        }
    };
}

//Order
define_order_type!(MAX__DWORD, MIN__DWORD, u32);
define_order_type!(MAX__BOOL, MIN__BOOL, u8);
define_order_type!(MAX__BYTE, MIN__BYTE, u8);
define_order_type!(MAX__WORD, MIN__WORD, u16);
define_order_type!(MAX__WCHAR, MIN__WCHAR, u16);
define_order_type!(MAX__LWORD, MIN__LWORD, u64);
//Ints
define_int_type!(MAX__SINT, MIN__SINT, i8);
define_int_type!(MAX__USINT, MIN__USINT, u8);
define_int_type!(MAX__CHAR, MIN__CHAR, u8);
define_int_type!(MAX__INT, MIN__INT, i16);
define_int_type!(MAX__UINT, MIN__UINT, u16);
define_int_type!(MAX__DINT, MIN__DINT, i32);
define_int_type!(MAX__UDINT, MIN__UDINT, u32);
define_int_type!(MAX__LINT, MIN__LINT, i64);
define_int_type!(MAX__ULINT, MIN__ULINT, u64);
define_int_type!(MAX__DATE, MIN__DATE, i64);
define_int_type!(MAX__DATE_AND_TIME, MIN__DATE_AND_TIME, i64);
define_int_type!(MAX__TIME, MIN__TIME, i64);
define_int_type!(MAX__TIME_OF_DAY, MIN__TIME_OF_DAY, i64);
//Floats
define_float_type!(MAX__REAL, MIN__REAL, f32);
define_float_type!(MAX__LREAL, MIN__LREAL, f64);
