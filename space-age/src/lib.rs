// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration{
    seconds: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            seconds: s as f64
        }
    }
}

impl Duration {
    fn calculate(&self, planet: f64) -> f64 {
        self.seconds/(EARTH_SECONDS*planet)
    }
}

pub trait Planet {
    fn period() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.calculate(Self::period())
    }
}

const EARTH_SECONDS: f64 = 31_557_600.0;
const EARTH: f64 = 1.0;
const MERCURE: f64 = 0.2408467;
const VENUS: f64 = 0.61519726;
const MARS: f64 = 1.8808158;
const JUPITER: f64 = 11.862615;
const SATURN: f64 = 29.447498;
const URANUS: f64 = 84.016846;
const NEPTUNE: f64 = 164.79132;

macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n; impl Planet for $n { fn period() -> f64 { $p } }
    }
}
planet!(Earth, EARTH);
planet!(Mercury, MERCURE);
planet!(Venus, VENUS);
planet!(Mars, MARS);
planet!(Jupiter, JUPITER);
planet!(Saturn, SATURN);
planet!(Uranus, URANUS);
planet!(Neptune, NEPTUNE);