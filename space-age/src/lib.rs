//! Numbers are mostly taken as floats in order to represent fully-fledged real numbers, which is
//! more convenient for time in physics, and in order to avoid useless type conversions.

/// One earth year expressed in seconds.
const EARTH_PERIOD: f64 = 31557600.0;

/// An "absolute" time duration.
pub struct Duration {
    /// The only attribute of this structure: the time duration in seconds.
    time: f64,
}

/// `From<u64>` implementation for `Duration`: takes the number and stores it in as a `f64`.
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { time: s as f64 }
    }
}

/// Defines a trait for planets, consisting of an associated constant `RELATIVE_PERIOD` and an
/// associated function `years_during` with a default implementation.
pub trait Planet {
    /// The ratio between one planet year and one Earth year.
    const RELATIVE_PERIOD: f64;

    /// Converts the given `Duration` in planet years.
    fn years_during(d: &Duration) -> f64 {
        d.time / (Self::RELATIVE_PERIOD * EARTH_PERIOD)
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const RELATIVE_PERIOD: f64 = 0.2408467;
}

impl Planet for Venus {
    const RELATIVE_PERIOD: f64 = 0.61519726;
}

impl Planet for Earth {
    const RELATIVE_PERIOD: f64 = 1.0;
}

impl Planet for Mars {
    const RELATIVE_PERIOD: f64 = 1.8808158;
}

impl Planet for Jupiter {
    const RELATIVE_PERIOD: f64 = 11.862615;
}

impl Planet for Saturn {
    const RELATIVE_PERIOD: f64 = 29.447498;
}

impl Planet for Uranus {
    const RELATIVE_PERIOD: f64 = 84.016846;
}

impl Planet for Neptune {
    const RELATIVE_PERIOD: f64 = 164.79132;
}
