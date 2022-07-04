use std::slice;

/// Integer type macros
macro_rules! define_int_type {
    ($max_name:ident, $rust_type:ty) => {
        //Odering operations
        define_order_type!($max_name, $rust_type);
    };
}

/// Integer type macros
macro_rules! define_order_type {
    ($max_name:ident, $rust_type:ty) => {
        //Max impl
        #[allow(non_snake_case)]
        pub extern "C" fn $max_name(size : u32, value : *const $rust_type) -> $rust_type {
            crate::types::max(size, value)
        }
    };
}

///Finds the max of a set of values
/// # Safety 
/// Works with raw pointers
pub(crate) fn max<T>(size: u32, value : *const T) -> T
where T :  Ord + Copy {

    // Declare array for value
    if value.is_null() {
        panic!("Null pointer for value");
    }
    let arr = unsafe {
        slice::from_raw_parts(value, size as usize)
    };

    *arr.iter().max().expect("A max will always exist")
}

define_order_type!(MAX__BOOL,u8);
define_int_type!(MAX__SINT,i8);
define_int_type!(MAX__USINT,u8);
define_order_type!(MAX__BYTE,u8);
define_int_type!(MAX__CHAR,u8);
define_int_type!(MAX__INT,i16);
define_int_type!(MAX__UINT,u16);
define_order_type!(MAX__WORD,u16);
define_order_type!(MAX__WCHAR,u16);
define_int_type!(MAX__DINT,i32);
define_int_type!(MAX__UDINT,u32);
define_order_type!(MAX__DWORD,u32);
define_int_type!(MAX__LINT,i64);
define_int_type!(MAX__ULINT,u64);
define_order_type!(MAX__LWORD,u64);
define_int_type!(MAX_DATE,i64);
define_int_type!(MAX__DATE_AND_TIME,i64);
define_int_type!(MAX__TIME,i64);
define_int_type!(MAX__TIME_OF_DAY,i64);
