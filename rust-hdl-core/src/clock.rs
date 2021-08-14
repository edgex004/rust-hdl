use std::fmt::Debug;

// We don't want clock types to be multibit or other weird things...
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct Clock(pub bool);

pub const NANOS_PER_FEMTO: f64 = 1_000_000.0;

pub fn freq_hz_to_period_femto(freq: f64) -> f64 {
    (1.0e15 / freq).round()
}

#[test]
fn test_freq_to_period_mapping() {
    assert_eq!(freq_hz_to_period_femto(1.0), 1.0e15)
}

impl std::ops::Not for Clock {
    type Output = Clock;

    fn not(self) -> Self::Output {
        Clock(!self.0)
    }
}

impl From<bool> for Clock {
    fn from(x: bool) -> Clock {
        Clock(x)
    }
}
