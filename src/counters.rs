use std::ops::{Add, Sub};

use num::{Bounded, One, Zero};

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

impl<T> CTUParams<T>
where
    T: Add<Output = T> + One + Zero + Copy + PartialOrd,
{
    unsafe fn update_q(&mut self) {
        if !self.q.is_null() {
            *self.q = *self.cv >= self.pv
        }
    }

    unsafe fn reset(&mut self) {
        if !self.cv.is_null() {
            *self.cv = Zero::zero();
        }
    }

    unsafe fn inc(&mut self) {
        if !self.cv.is_null() {
            *self.cv = *self.cv + One::one();
        }
    }

    fn r_edge(&mut self) -> bool {
        self.internal.rising_edge(self.cu)
    }
}

unsafe fn ctu<T>(params: &mut CTUParams<T>)
where
    T: Add<Output = T> + One + Zero + Copy + PartialOrd + Bounded,
{
    if params.r {
        params.reset();
    } else if params.r_edge() & (*params.cv < T::max_value()) {
        params.inc();
    }
    params.update_q();
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
    ctu(params);
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
    ctu(params);
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
    ctu(params);
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
    ctu(params);
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
    ctu(params);
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

impl<T> CTDParams<T>
where
    T: Sub<Output = T> + One + Zero + Copy + PartialOrd,
{
    unsafe fn update_q(&mut self) {
        if !self.q.is_null() {
            *self.q = *self.cv <= Zero::zero();
        }
    }

    unsafe fn load(&mut self) {
        if !self.cv.is_null() {
            *self.cv = self.pv;
        }
    }

    unsafe fn dec(&mut self) {
        if !self.cv.is_null() {
            *self.cv = *self.cv - One::one();
        }
    }

    fn r_edge(&mut self) -> bool {
        self.internal.rising_edge(self.cd)
    }
}

unsafe fn ctd<T>(params: &mut CTDParams<T>)
where
    T: Sub<Output = T> + One + Zero + Copy + PartialOrd + Bounded,
{
    if params.ld {
        params.load();
    } else if params.r_edge() & (*params.cv > T::min_value()) {
        params.dec();
    }
    params.update_q();
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
    ctd(params);
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
    ctd(params);
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
    ctd(params);
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
    ctd(params);
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
    ctd(params);
}

#[repr(C)]
#[derive(Debug)]
pub struct CTUDParams<T> {
    cu: bool,
    cd: bool,
    r: bool,
    ld: bool,
    pv: T,
    qu: *mut bool,
    qd: *mut bool,
    cv: *mut T,
    internal_up: Signal,
    internal_down: Signal,
}

impl<T> Default for CTUDParams<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            cu: Default::default(),
            cd: Default::default(),
            r: Default::default(),
            ld: Default::default(),
            pv: Default::default(),
            qu: std::ptr::null_mut(),
            qd: std::ptr::null_mut(),
            cv: std::ptr::null_mut(),
            internal_up: Default::default(),
            internal_down: Default::default(),
        }
    }
}

impl<T> CTUDParams<T>
where
    T: Add<Output = T> + Sub<Output = T> + One + Zero + Copy + PartialOrd,
{
    unsafe fn update_qu(&mut self) {
        if !self.qu.is_null() {
            *self.qu = *self.cv >= self.pv
        }
    }

    unsafe fn update_qd(&mut self) {
        if !self.qd.is_null() {
            *self.qd = *self.cv <= Zero::zero();
        }
    }

    unsafe fn reset(&mut self) {
        if !self.cv.is_null() {
            *self.cv = Zero::zero();
        }
    }

    unsafe fn load(&mut self) {
        if !self.cv.is_null() {
            *self.cv = self.pv;
        }
    }

    unsafe fn inc(&mut self) {
        if !self.cv.is_null() {
            *self.cv = *self.cv + One::one();
        }
    }

    unsafe fn dec(&mut self) {
        if !self.cv.is_null() {
            *self.cv = *self.cv - One::one();
        }
    }

    fn cu_r_edge(&mut self) -> bool {
        self.internal_up.rising_edge(self.cu)
    }

    fn cd_r_edge(&mut self) -> bool {
        self.internal_down.rising_edge(self.cd)
    }
}

unsafe fn ctud<T>(params: &mut CTUDParams<T>)
where
    T: Add<Output = T> + Sub<Output = T> + One + Zero + Copy + PartialOrd + Bounded,
{
    if params.r {
        params.reset();
    } else if params.ld {
        params.load();
    } else {
        let r_edge_up = params.cu_r_edge();
        let r_edge_down = params.cd_r_edge();
        if !(r_edge_up & r_edge_down) {
            if r_edge_up & (*params.cv < T::max_value()) {
                params.inc();
            } else if r_edge_down & (*params.cv > T::min_value()) {
                params.dec();
            }
        }
    }
    params.update_qu();
    params.update_qd();
}

///.
/// Counter up and down for INT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTUD_INT(params: &mut CTUDParams<i16>) {
    ctud(params);
}

///.
/// Counter up and down for DINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTUD_DINT(params: &mut CTUDParams<i32>) {
    ctud(params);
}

///.
/// Counter up and down for UDINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTUD_UDINT(params: &mut CTUDParams<u32>) {
    ctud(params);
}

///.
/// Counter up and down for LINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTUD_LINT(params: &mut CTUDParams<i64>) {
    ctud(params);
}

///.
/// Counter up and down for ULINT
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CTUD_ULINT(params: &mut CTUDParams<u64>) {
    ctud(params);
}
