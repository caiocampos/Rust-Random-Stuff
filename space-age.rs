#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    const YEAR: f64;

    fn years_during(d: &Duration) -> f64 {
        (d.seconds * 100.0 / Self::YEAR).round() / 100.0
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
    const YEAR: f64 = 31557600f64 * 0.2408467;
}
impl Planet for Venus {
    const YEAR: f64 = 31557600f64 * 0.61519726;
}
impl Planet for Earth {
    const YEAR: f64 = 31557600f64;
}
impl Planet for Mars {
    const YEAR: f64 = 31557600f64 * 1.8808158;
}
impl Planet for Jupiter {
    const YEAR: f64 = 31557600f64 * 11.862615;
}
impl Planet for Saturn {
    const YEAR: f64 = 31557600f64 * 29.447498;
}
impl Planet for Uranus {
    const YEAR: f64 = 31557600f64 * 84.016846;
}
impl Planet for Neptune {
    const YEAR: f64 = 31557600f64 * 164.79132;
}
