//! # color scheme generation
use crate::hue::Hue;
use colorsys::Hsl;
use std::fmt::Write;

/// color scheme variants    
///
/// it may be worth referring to [a
/// diagram of HSL](https://en.wikipedia.org/wiki/HSL_and_HSV#/media/File:HSL_color_solid_cylinder_saturation_gray.png)    
/// schemes primarily affect hue while preserving saturation and lightness except where noted
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Scheme {
    /// lighter and darker variants of the same hue    
    /// variable names: `--lighter`, `--darker`
    Column,
    /// the complementary color (180 degrees on the color wheel)    
    /// variable names: `--complementary`
    Complementary,
    /// an isoceles triangle (120 degrees clockwise and counterclockwise)    
    /// variable names: `--clockwise`, `--counterclockwise`
    Triad,
    /// a square with the primary color as the upper-left corner (90 degrees
    /// clockwise, 180 degrees clockwise, 90 degrees counterclockwise)    
    /// variable names: `--upper-right`, `--lower-right`, `--lower-left`
    Tetrad,
    /// a dark and saturated variant suitable for use as a font color
    /// variable names: `--text-primary`
    Text,
    /// a light and desaturated variant for use as a background color
    /// variable names: `--background-primary`
    Background,
}

type ColorVar = (&'static str, Hsl);

/// a colorscheme with a primary color and one or more additional colors
#[derive(Debug, Clone, PartialEq)]
pub struct ColorScheme {
    primary: Hsl,
    colors: Vec<ColorVar>,
}

impl ColorScheme {
    /// create a colorscheme from a primary color and scheme variant
    pub fn new(primary: Hsl, scheme: Scheme) -> Self {
        let colors = ColorScheme::colors(&primary, scheme);
        Self { primary, colors }
    }
    pub fn from_schemes(primary: Hsl, schemes: impl IntoIterator<Item = Scheme>) -> Self {
        let mut colors = Vec::new();
        for scheme in schemes {
            colors.extend(ColorScheme::colors(&primary, scheme));
        }
        Self { primary, colors }
    }
    /// add another scheme variant's colors
    pub fn and(mut self, scheme: Scheme) -> Self {
        self.colors
            .extend(ColorScheme::colors(&self.primary, scheme));
        self
    }
    fn colors(primary: &Hsl, scheme: Scheme) -> Vec<ColorVar> {
        match scheme {
            Scheme::Column => Self::column(primary),
            Scheme::Complementary => Self::complementary(primary),
            Scheme::Triad => Self::triad(primary),
            Scheme::Tetrad => Self::tetrad(primary),
            Scheme::Text => Self::text(primary),
            Scheme::Background => Self::background(primary),
        }
    }
    /// serialize the scheme to CSS variables defined under the provided selector or `:root`
    ///
    /// all colors are converted to RGB hex strings
    pub fn as_css(&self, selector: Option<&str>) -> String {
        let sel = selector.unwrap_or(":root");
        let mut s = format!("{} {{", sel);
        write!(s, "\n\t--primary: {};", hsl_to_css(&self.primary)).unwrap();
        for (var_name, color) in self.colors.iter() {
            write!(s, "\n\t{}: {};", var_name, hsl_to_css(color)).unwrap();
        }
        write!(s, "\n}};").unwrap();
        s
    }
    fn column(primary: &Hsl) -> Vec<ColorVar> {
        let lightness = primary.lightness();
        let lighter = with_lightness(primary, lightness * 1.5);
        let darker = with_lightness(primary, lightness * 0.5);
        vec![("--lighter", lighter), ("--darker", darker)]
    }
    fn complementary(primary: &Hsl) -> Vec<ColorVar> {
        let complementary = rotate(primary, 180.0);
        vec![("--complementary", complementary)]
    }
    fn triad(primary: &Hsl) -> Vec<ColorVar> {
        let clockwise = rotate(primary, 120.0);
        let counterclockwise = rotate(primary, -120.0);
        vec![
            ("--clockwise", clockwise),
            ("--counterclockwise", counterclockwise),
        ]
    }
    fn tetrad(primary: &Hsl) -> Vec<ColorVar> {
        let by: f64 = 90.0;
        let upper_right = rotate(primary, by);
        let lower_right = rotate(&upper_right, by);
        let lower_left = rotate(&lower_right, by);
        vec![
            ("--upper-right", upper_right),
            ("--lower-right", lower_right),
            ("--lower-left", lower_left),
        ]
    }
    fn text(primary: &Hsl) -> Vec<ColorVar> {
        let text_primary = with_saturation(primary, 0.75);
        let text_primary = with_lightness(&text_primary, 0.125);
        vec![("--text-primary", text_primary)]
    }
    fn background(primary: &Hsl) -> Vec<ColorVar> {
        let background_primary = with_saturation(primary, 0.25);
        let background_primary = with_lightness(&background_primary, 0.875);
        vec![("--background-primary", background_primary)]
    }
}

fn rotate(color: &Hsl, by: f64) -> Hsl {
    let mut c = color.clone();
    let new_hue = Hue::new(color.hue()) + by;
    c.set_hue(new_hue.into());
    c
}

// not used yet, maybe for e.g. pastelization
fn with_saturation(color: &Hsl, new_saturation: f64) -> Hsl {
    let mut c = color.clone();
    c.set_saturation(new_saturation);
    c
}
fn with_lightness(color: &Hsl, new_lightness: f64) -> Hsl {
    let mut c = color.clone();
    c.set_lightness(new_lightness);
    c
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
        let o = _new_hsl(270.0);
        let by = 91.0;
        let r = rotate(&o, by);
        assert_eq!(r.hue(), 1.0);
    }
    #[test]
    fn test_complementary() {
        let primary: f64 = 90.0;
        let expected_complementary: f64 = 270.0;
        let expected: Vec<ColorVar> = vec![("--complementary", _new_hsl(expected_complementary))];
        let complementary = ColorScheme::complementary(&_new_hsl(primary));
        assert_eq!(complementary, expected);
    }
    #[test]
    fn test_triad() {
        let primary: f64 = 90.0;
        let expected_clockwise: f64 = 210.0;
        let expected_counterclockwise: f64 = 330.0;
        let expected: Vec<ColorVar> = vec![
            ("--clockwise", _new_hsl(expected_clockwise)),
            ("--counterclockwise", _new_hsl(expected_counterclockwise)),
        ];
        let triad = ColorScheme::triad(&_new_hsl(primary));
        assert_eq!(triad, expected);
    }
    #[test]
    fn test_tetrad() {
        let primary: f64 = 90.0;
        let expected_upper_right = 180.0;
        let expected_lower_right = 270.0;
        let expected_lower_left = 360.0;
        let expected = vec![
            ("--upper-right", _new_hsl(expected_upper_right)),
            ("--lower-right", _new_hsl(expected_lower_right)),
            ("--lower-left", _new_hsl(expected_lower_left)),
        ];
        let tetrad = ColorScheme::tetrad(&_new_hsl(primary));
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
            String::from(":root {\n\t--primary: #ff0000;\n\t--complementary: #00ffff;\n};");
        let dyad = ColorScheme::new(primary, Scheme::Complementary);
        let actual = dyad.as_css(None);
        assert_eq!(actual, expected);
    }
}
