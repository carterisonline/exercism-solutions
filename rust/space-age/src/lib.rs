#[derive(Debug)]
pub struct Duration {
    years: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            // Unsafe representation because `f64::from` doesn't implement conversions for type `u64` >:(
            years: s as f64 / 31556952.0f64
        }
    }
}

pub trait Planet {
    const YEAR_RELATIVE: f64;
    fn years_during(d: &Duration) -> f64 {
        d.years * Self::YEAR_RELATIVE
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
    const YEAR_RELATIVE: f64 = 4.1520773f64;
}
impl Planet for Venus {
    const YEAR_RELATIVE: f64 = 1.62438807f64;
}
impl Planet for Earth {
    const YEAR_RELATIVE: f64 = 1.0f64;
}
impl Planet for Mars {
    const YEAR_RELATIVE: f64 = 0.531614894f64;
}
impl Planet for Jupiter {
    const YEAR_RELATIVE: f64 = 0.084317032f64;
}
impl Planet for Saturn {
    const YEAR_RELATIVE: f64 = 0.0339443313f64;
}
impl Planet for Uranus {
    const YEAR_RELATIVE: f64 = 0.0119033448f64;
}
impl Planet for Neptune {
    const YEAR_RELATIVE: f64 = 0.00606832939f64;
}
