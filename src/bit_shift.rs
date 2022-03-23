/// Defines shift operations

#[repr(C)]
pub struct BitwiseParam<T> {
    pub input: T,
    pub n: u32,
}

#[allow(non_snake_case)]
#[no_mangle]
/// Shift left operation on bytes
pub fn SHL__BYTE(param: &BitwiseParam<u8>) -> u8 {
    param.input << param.n
}

#[allow(non_snake_case)]
#[no_mangle]
/// Shift left operation on word
pub fn SHL__WORD(param: &BitwiseParam<u16>) -> u16 {
    param.input << param.n
}

#[allow(non_snake_case)]
#[no_mangle]
/// Shift left operation on dword
pub fn SHL__DWORD(param: &BitwiseParam<u32>) -> u32 {
    param.input << param.n
}

#[allow(non_snake_case)]
#[no_mangle]
/// Shift left operation on lword
pub fn SHL__LWORD(param: &BitwiseParam<u64>) -> u64 {
    param.input << param.n
}

#[allow(non_snake_case)]
#[no_mangle]
/// Shift right operation on bytes
pub fn SHR__BYTE(param: &BitwiseParam<u8>) -> u8 {
    param.input >> param.n
}

#[allow(non_snake_case)]
#[no_mangle]
/// Shift right operation on word
pub fn SHR__WORD(param: &BitwiseParam<u16>) -> u16 {
    param.input >> param.n
}

#[allow(non_snake_case)]
#[no_mangle]
/// Shift right operation on dword
pub fn SHR__DWORD(param: &BitwiseParam<u32>) -> u32 {
    param.input >> param.n
}

#[allow(non_snake_case)]
#[no_mangle]
/// Shift right operation on lword
pub fn SHR__LWORD(param: &BitwiseParam<u64>) -> u64 {
    param.input >> param.n
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate left operation on bytes
pub fn ROL__BYTE(param: &BitwiseParam<u8>) -> u8 {
    param.input.rotate_left(param.n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate left operation on word
pub fn ROL__WORD(param: &BitwiseParam<u16>) -> u16 {
    param.input.rotate_left(param.n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate left operation on dword
pub fn ROL__DWORD(param: &BitwiseParam<u32>) -> u32 {
    param.input.rotate_left(param.n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate left operation on lword
pub fn ROL__LWORD(param: &BitwiseParam<u64>) -> u64 {
    param.input.rotate_left(param.n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate right operation on bytes
pub fn ROR__BYTE(param: &BitwiseParam<u8>) -> u8 {
    param.input.rotate_right(param.n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate right operation on word
pub fn ROR__WORD(param: &BitwiseParam<u16>) -> u16 {
    param.input.rotate_right(param.n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate right operation on dword
pub fn ROR__DWORD(param: &BitwiseParam<u32>) -> u32 {
    param.input.rotate_right(param.n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate right operation on lword
pub fn ROR__LWORD(param: &BitwiseParam<u64>) -> u64 {
    param.input.rotate_right(param.n)
}
