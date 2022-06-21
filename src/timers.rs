use std::time::Duration;
#[cfg(not(feature = "mock_time"))]
use std::time::Instant;

#[cfg(feature = "mock_time")]
use test_time_helpers::Instant;

use crate::utils::{RisingEdge, Signal};

#[cfg(feature = "mock_time")]
pub mod test_time_helpers;

pub type Time = i64;

#[repr(C)]
#[derive(Debug)]
pub struct TimerParams {
    input: bool,
    preset_time: Time,
    output: *mut bool,
    elapsed_time: *mut Time,
    input_edge: RisingEdge,
    start_time: Option<Instant>,
}

impl Default for TimerParams {
    fn default() -> Self {
        Self {
            input: Default::default(),
            preset_time: Default::default(),
            output: std::ptr::null_mut(),
            elapsed_time: std::ptr::null_mut(),
            input_edge: Default::default(),
            start_time: Default::default(),
        }
    }
}

impl TimerParams {
    fn is_running(&self) -> bool {
        self.start_time.is_some()
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.set_elapsed_time(Duration::default());
    }

    fn stop(&mut self) {
        self.start_time = None;
        self.set_elapsed_time(Duration::default());
    }

    fn set_elapsed_time(&mut self, duration: Duration) {
        unsafe {
            if !self.elapsed_time.is_null() {
                *self.elapsed_time = duration.as_nanos() as i64;
            }
        };
    }

    fn update_elapsed_time(&mut self) {
        if self.is_running() {
            self.set_elapsed_time(self.get_run_time().expect("Timer should be running"));
        }
    }

    fn in_range(&self) -> bool {
        let duration = Duration::from_nanos(self.preset_time as u64);
        self.get_run_time().map_or(false, |it| it <= duration)
    }

    fn get_run_time(&self) -> Option<Duration> {
        self.start_time.map(|it| it.elapsed())
    }

    fn set_output(&mut self, value: bool) {
        unsafe {
            if !self.output.is_null() {
                *self.output = value;
            }
        }
    }

    /// Returns true iff the input is fresh
    /// i.e. the current input is true, and the previous input is false
    fn has_new_input(&self) -> bool {
        self.input_edge.evaluate(self.input)
    }

    fn reset_input(&mut self) {
        self.input_edge = RisingEdge::default();
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TP(params: &mut TimerParams) {
    //If timer is active (start time set)
    let output = if params.is_running() {
        // If time elapsed within range
        if params.in_range() {
            params.update_elapsed_time();
            true
        } else {
            params.stop();
            false
        }
    } else if params.has_new_input() {
        params.start();
        true
    } else {
        false
    };
    params.set_output(output);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TON(params: &mut TimerParams) {
    let output = if params.input {
        //Timer was strarted at some point
        if params.is_running() {
            //Timer is still running
            if params.in_range() {
                params.update_elapsed_time();
                false
            } else {
                //Elapsed time reached
                params.stop();
                params.reset_input();
                true
            }
            //Timer stopped, but the input is new
        } else if params.has_new_input() {
            params.start();
            false
            //Timer stopped, input didn't change (still true from last time)
        } else {
            true
        }
    } else {
        //Input is false, stop timer regardless
        params.stop();
        params.reset_input();
        false
    };
    params.set_output(output);
}

// Aliases

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TP_TIME(params: &mut TimerParams) {
    TP(params)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TP_LTIME(params: &mut TimerParams) {
    TP(params)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TON_TIME(params: &mut TimerParams) {
    TON(params)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TON_LTIME(params: &mut TimerParams) {
    TON(params)
}
