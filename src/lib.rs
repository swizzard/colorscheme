use colorsys::Hsl;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::ops::{Add, Sub};

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

impl Add<f64> for Hue {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self::new(self.0 + other)
    }
}

impl Sub<f64> for Hue {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self::new(self.0 - other)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Scheme {
    Dyad,
    Triad,
    Tetrad,
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq)]
pub struct ColorScheme(BTreeMap<String, Hsl>);

impl ColorScheme {
    pub fn dyad(primary: Hsl) -> Self {
        let complementary = rotate(&primary, 180.0.into());
        let m: BTreeMap<String, Hsl> = [
            ColorScheme::primary_variable(primary),
            (String::from("--complementary"), complementary),
        ]
        .into();
        Self(m)
    }
    pub fn triad(primary: Hsl) -> Self {
        let clockwise = rotate(&primary, 120.0.into());
        let counterclockwise = rotate(&primary, (-120.0).into());
        let m = [
            ColorScheme::primary_variable(primary),
            (String::from("--clockwise"), clockwise),
            (String::from("--counterclockwise"), counterclockwise),
        ]
        .into();
        Self(m)
    }
    pub fn tetrad(primary: Hsl) -> Self {
        let by: Hue = 90.0.into();
        let upper_right = rotate(&primary, by);
        let lower_right = rotate(&upper_right, by);
        let lower_left = rotate(&lower_right, by);
        let m = [
            ColorScheme::primary_variable(primary),
            (String::from("--upper-right"), upper_right),
            (String::from("--lower-right"), lower_right),
            (String::from("--lower-left"), lower_left),
        ]
        .into();
        Self(m)
    }
    pub fn as_css(&self, selector: Option<&str>) -> String {
        let sel = selector.unwrap_or(":root");
        let mut s = format!("{} {{", sel);
        for (var_name, color) in self.0.iter() {
            write!(s, "\n\t{}: {};", var_name, hsl_to_css(color)).unwrap();
        }
        write!(s, "\n}};").unwrap();
        s
    }
    fn primary_variable(primary: Hsl) -> (String, Hsl) {
        (String::from("--primary"), primary)
    }
    #[cfg(test)]
    fn scheme(self) -> BTreeMap<String, Hsl> {
        self.0
    }
}

fn rotate(color: &Hsl, by: Hue) -> Hsl {
    let v = color.iter().collect::<Vec<f64>>();
    let new_hue = by + v[0];
    Hsl::new(new_hue.into(), v[1], v[2], v.get(3).copied())
}
fn hsl_to_css(h: &Hsl) -> String {
    colorsys::Rgb::from(h).to_hex_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn _new_hsl(h: f64) -> Hsl {
        Hsl::new(h, 100.0, 50.0, Some(1.0))
    }
    #[test]
    fn test_rotate() {
        let o = Hsl::new(270.0, 50.0, 50.0, Some(1.0));
        let by = Hue::new(91.0);
        let r = rotate(&o, by);
        assert_eq!(r.hue(), 1.0);
    }
    #[test]
    fn test_dyad() {
        let primary: f64 = 90.0;
        let expected_complementary: f64 = 270.0;
        let expected = [
            (String::from("--primary"), _new_hsl(primary)),
            (
                String::from("--complementary"),
                _new_hsl(expected_complementary),
            ),
        ]
        .into();
        let dyad = ColorScheme::dyad(_new_hsl(primary)).scheme();
        assert_eq!(dyad, expected);
    }
    #[test]
    fn test_triad() {
        let primary: f64 = 90.0;
        let expected_clockwise: f64 = 210.0;
        let expected_counterclockwise: f64 = 330.0;
        let expected = [
            (String::from("--primary"), _new_hsl(primary)),
            (String::from("--clockwise"), _new_hsl(expected_clockwise)),
            (
                String::from("--counterclockwise"),
                _new_hsl(expected_counterclockwise),
            ),
        ]
        .into();
        let triad = ColorScheme::triad(_new_hsl(primary)).scheme();
        assert_eq!(triad, expected);
    }
    #[test]
    fn test_tetrad() {
        let primary: f64 = 90.0;
        let expected_upper_right = 180.0;
        let expected_lower_right = 270.0;
        let expected_lower_left = 360.0;
        let expected = [
            (String::from("--primary"), _new_hsl(primary)),
            (
                String::from("--upper-right"),
                _new_hsl(expected_upper_right),
            ),
            (
                String::from("--lower-right"),
                _new_hsl(expected_lower_right),
            ),
            (String::from("--lower-left"), _new_hsl(expected_lower_left)),
        ]
        .into();
        let tetrad = ColorScheme::tetrad(_new_hsl(primary)).scheme();
        assert_eq!(tetrad, expected);
    }
    #[test]
    fn test_hsl_to_css() {
        let red = _new_hsl(0.0);
        let expected = String::from("#ff0000");
        assert_eq!(hsl_to_css(&red), expected);
    }
    #[test]
    fn test_as_css() {
        let primary = _new_hsl(0.0);
        let expected =
            String::from(":root {\n\t--complementary: #00ffff;\n\t--primary: #ff0000;\n};");
        let dyad = ColorScheme::dyad(primary);
        let actual = dyad.as_css(None);
        assert_eq!(actual, expected);
    }
}
