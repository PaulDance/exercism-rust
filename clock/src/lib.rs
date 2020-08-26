use std::fmt;

/// The desired `Clock` structure.
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    /// The only attribute of `Clock`: the raw time as a number of minutes.
    time: u16,
}

impl Clock {
    /// The maximum value `Clock::time` can reach.
    const MAX_TIME: u16 = 24 * 60;

    /// Builds a new `Clock` by transforming the given `hours` and `minutes` into raw minutes
    /// included in the interval [|0; 24 * 60|].
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            time: (60 * hours + minutes).rem_euclid(Self::MAX_TIME as i32) as u16,
        }
    }

    /// Computes and returns the number of hours stored in the `Clock`.
    pub fn hours(&self) -> u8 {
        (self.time / 60) as u8
    }

    /// Computes and returns the number of minutes stored in the `Clock`.
    pub fn minutes(&self) -> u8 {
        (self.time % 60) as u8
    }

    /// Adds the given amount of `minutes` to the `Clock` into a new one and returns it.
    ///
    /// Simply uses the raw `Clock::time` in order to perform the addition and then brings the
    /// result back in the [|0; 24 * 60|] interval.
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.time as i32 + minutes)
    }
}

/// Desired `fmt::Display` implementation: the `Clock`'s time will be displayed using the hh:mm
/// format.
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
