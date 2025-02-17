#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Hue(f64);

impl Hue {
    fn clamp(n: f64) -> f64 {
        n.clamp(0.0, 360.0)
    }
    fn wrap(n: f64) -> f64 {
        if n < 0.0 {
            360.0 + n
        } else if n > 360.0 {
            n - 360.0
        } else {
            n
        }
    }
    fn wrapped(n: f64) -> Self {
        Self(Self::wrap(n))
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
        Self::wrapped(self.0 + other)
    }
}

impl std::ops::Sub<f64> for Hue {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self::wrapped(self.0 - other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sub() {
        let h = Hue::new(90.0);
        let new = h - 120.0;
        assert_eq!(330.0, new.0);
    }
    #[test]
    fn test_add() {
        let h = Hue::new(270.0);
        let new = h + 120.0;
        assert_eq!(30.0, new.0);
    }
}
