#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Hue(f64);

impl Hue {
    fn clamp(n: f64) -> f64 {
        if n > 360.0 {
            n - 360.0
        } else if n < 0.0 {
            n + 360.0
        } else {
            n
        }
    }
    pub fn new(n: f64) -> Self {
        Self(Self::clamp(n))
    }
}

impl From<f64> for Hue {
    fn from(value: f64) -> Self {
        Self(Self::clamp(value))
    }
}

impl From<Hue> for f64 {
    fn from(Hue(v): Hue) -> Self {
        v
    }
}

impl std::ops::Add<f64> for Hue {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self::new(self.0 + other)
    }
}

impl std::ops::Sub<f64> for Hue {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self::new(self.0 - other)
    }
}
