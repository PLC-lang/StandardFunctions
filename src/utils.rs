use std::cell::Cell;

/// Trait describing a digital signal
pub trait Signal {
    /// Returns the state of the signal given the new value information
    fn evaluate(&self, value: bool) -> bool;
}

/// A Rising edge only becomes true on High Signal (True is only returned once per signal change)
#[derive(Default, Debug, PartialEq, Eq)]
pub struct RisingEdge {
    value: Cell<bool>,
}

impl Signal for RisingEdge {
    fn evaluate(&self, value: bool) -> bool {
        let current_value = self.value.get();
        if current_value {
            self.value.set(false);
            false
        } else {
            self.value.set(value);
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RisingEdge, Signal};

    #[test]
    fn rising_edge_subsequent_true() {
        let re = RisingEdge::default();
        assert!(re.evaluate(true));
        assert!(!re.evaluate(true));
    }

    #[test]
    fn rising_edge_true_false_true_true() {
        let re = RisingEdge::default();
        assert!(re.evaluate(true));
        assert!(!re.evaluate(false));
        assert!(re.evaluate(true));
        assert!(!re.evaluate(true));
    }

    #[test]
    fn rising_edge_false_true_true_false_true() {
        let re = RisingEdge::default();
        assert!(!re.evaluate(false));
        assert!(re.evaluate(true));
        assert!(!re.evaluate(true));
        assert!(!re.evaluate(false));
        assert!(re.evaluate(true));
    }
}
