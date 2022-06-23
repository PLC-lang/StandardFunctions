use crate::utils::Signal;

#[derive(Debug)]
#[repr(C)]
pub struct Trigger {
    clk : bool,
    output : *mut bool,
    internal : Signal,
}

impl Default for Trigger {
    fn default() -> Self {
        Self { clk: Default::default(), output: std::ptr::null_mut(), internal: Default::default() }
    }
}

impl Trigger {
    fn set_output(&mut self, val : bool) {
        if !self.output.is_null() {
            unsafe{
                *self.output = val;
            }
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn R_TRIG(trigger : &mut Trigger) {
    let res = trigger.internal.rising_edge(trigger.clk);
    trigger.set_output(res);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn F_TRIG(trigger : &mut Trigger) {
    let res = trigger.internal.falling_edge(trigger.clk);
    trigger.set_output(res);
}