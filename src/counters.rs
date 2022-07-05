use crate::utils::Signal;

#[repr(C)]
#[derive(Debug)]
pub struct CTUParams<T> {
    cu: bool,
    r: bool,
    pv: T,
    q: *mut bool,
    cv: *mut T,
    internal: Signal,
}

impl<T> Default for CTUParams<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            cu: Default::default(),
            r: Default::default(),
            pv: Default::default(),
            q: std::ptr::null_mut(),
            cv: std::ptr::null_mut(),
            internal: Default::default(),
        }
    }
}

impl<T> CTUParams<T> {
    unsafe fn set_q(&mut self, q_value: bool) {
        if !self.q.is_null() {
            *self.q = q_value;
        }
    }

    unsafe fn set_cv(&mut self, cv_value: T) {
        if !self.cv.is_null() {
            *self.cv = cv_value;
        }
    }
}

///.
/// Counter up for INT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTU_INT(params: &mut CTUParams<i16>) {
    if params.r {
        params.set_cv(0);
    } else if params.internal.rising_edge(params.cu) & (*params.cv < i16::MAX) {
        params.set_cv(*params.cv + 1);
    }
    params.set_q(*params.cv >= params.pv);
}

///.
/// Counter up for DINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTU_DINT(params: &mut CTUParams<i32>) {
    if params.r {
        params.set_cv(0);
    } else if params.internal.rising_edge(params.cu) & (*params.cv < i32::MAX) {
        params.set_cv(*params.cv + 1);
    }
    params.set_q(*params.cv >= params.pv);
}

///.
/// Counter up for DINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTU_UDINT(params: &mut CTUParams<u32>) {
    if params.r {
        params.set_cv(0);
    } else if params.internal.rising_edge(params.cu) & (*params.cv < u32::MAX) {
        params.set_cv(*params.cv + 1);
    }
    params.set_q(*params.cv >= params.pv);
}

///.
/// Counter up for LINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTU_LINT(params: &mut CTUParams<i64>) {
    if params.r {
        params.set_cv(0);
    } else if params.internal.rising_edge(params.cu) & (*params.cv < i64::MAX) {
        params.set_cv(*params.cv + 1);
    }
    params.set_q(*params.cv >= params.pv);
}

///.
/// Counter up for ULINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTU_ULINT(params: &mut CTUParams<u64>) {
    if params.r {
        params.set_cv(0);
    } else if params.internal.rising_edge(params.cu) & (*params.cv < u64::MAX) {
        params.set_cv(*params.cv + 1);
    }
    params.set_q(*params.cv >= params.pv);
}

#[repr(C)]
#[derive(Debug)]
pub struct CTDParams<T> {
    cd: bool,
    ld: bool,
    pv: T,
    q: *mut bool,
    cv: *mut T,
    internal: Signal,
}

impl<T> Default for CTDParams<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            cd: Default::default(),
            ld: Default::default(),
            pv: Default::default(),
            q: std::ptr::null_mut(),
            cv: std::ptr::null_mut(),
            internal: Default::default(),
        }
    }
}

impl<T> CTDParams<T> {
    unsafe fn set_q(&mut self, q_value: bool) {
        if !self.q.is_null() {
            *self.q = q_value;
        }
    }

    unsafe fn set_cv(&mut self, cv_value: T) {
        if !self.cv.is_null() {
            *self.cv = cv_value;
        }
    }
}

///.
/// Counter down for INT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTD_INT(params: &mut CTDParams<i16>) {
    if params.ld {
        params.set_cv(params.pv);
    } else if params.internal.rising_edge(params.cd) & (*params.cv > i16::MIN) {
        params.set_cv(*params.cv - 1);
    }
    params.set_q(*params.cv <= 0);
}

///.
/// Counter down for DINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTD_DINT(params: &mut CTDParams<i32>) {
    if params.ld {
        params.set_cv(params.pv);
    } else if params.internal.rising_edge(params.cd) & (*params.cv > i32::MIN) {
        params.set_cv(*params.cv - 1);
    }
    params.set_q(*params.cv <= 0);
}

///.
/// Counter down for UDINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTD_UDINT(params: &mut CTDParams<u32>) {
    if params.ld {
        params.set_cv(params.pv);
    } else if params.internal.rising_edge(params.cd) & (*params.cv > u32::MIN) {
        params.set_cv(*params.cv - 1);
    }
    params.set_q(*params.cv == 0);
}

///.
/// Counter down for LINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTD_LINT(params: &mut CTDParams<i64>) {
    if params.ld {
        params.set_cv(params.pv);
    } else if params.internal.rising_edge(params.cd) & (*params.cv > i64::MIN) {
        params.set_cv(*params.cv - 1);
    }
    params.set_q(*params.cv <= 0);
}

///.
/// Counter down for ULINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTD_ULINT(params: &mut CTDParams<u64>) {
    if params.ld {
        params.set_cv(params.pv);
    } else if params.internal.rising_edge(params.cd) & (*params.cv > u64::MIN) {
        params.set_cv(*params.cv - 1);
    }
    params.set_q(*params.cv == 0);
}
