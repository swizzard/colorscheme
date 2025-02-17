//! # cli parsing
use crate::scheme::Scheme;
use clap::{Parser, ValueEnum};
use css_named_colors::NamedColor;

/// cli-facing equivalent of [`crate::scheme::Scheme`]
#[derive(Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum CliScheme {
    /// lighter and darker variants of the same hue
    Column,
    /// complementary color (180 degrees on the color wheel)
    Dyad,
    /// an isoceles triangle (120 degrees clockwise and counterclockwise)
    Triad,
    /// a square with the primary color as the upper-left corner (90 degrees clockwise, 180 degrees clockwise, 90 degrees counterclockwise)
    Tetrad,
}

/// cli arguments
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        short = 's',
        long = "scheme",
        help = "color scheme to generate",
        value_name = "SCHEME"
    )]
    cli_scheme: CliScheme,
    #[arg(
        short,
        long = "primary",
        help = "primary scheme color (hex value or CSS color name)",
        value_name = "PRIMARY COLOR"
    )]
    primary_str: String,
    #[arg(
        short = 'e',
        long = "selector",
        help = "css selector under which variables are declared (default: `:root`)",
        value_name = "CSS SELECTOR"
    )]
    pub selector: Option<String>,
}

impl Args {
    /// try to parse the primary color string as either a hex string or [named CSS color](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color)
    pub fn primary(&self) -> Option<colorsys::Hsl> {
        Args::parse_primary(&self.primary_str)
    }
    /// convert from [`CliScheme`] to [`Scheme`]
    pub fn scheme(&self) -> Scheme {
        match self.cli_scheme {
            CliScheme::Column => Scheme::Column,
            CliScheme::Dyad => Scheme::Dyad,
            CliScheme::Triad => Scheme::Triad,
            CliScheme::Tetrad => Scheme::Tetrad,
        }
    }
    /// try to parse the provided input as either a hex string or CSS color name
    fn parse_primary(primary: &str) -> Option<colorsys::Hsl> {
        if let Some('#') = primary.chars().nth(0) {
            // hex string
            colorsys::Rgb::from_hex_str(primary).map(|c| c.into()).ok()
        } else if primary == NamedColor::TRANSPARENT.name() {
            // 'transparent' is a valid CSS color name but not useful to us
            None
        } else {
            // color name?
            let from_name = if let Some(nc) = NamedColor::from_name(primary) {
                // safety: we know `nc` is not `TRANSPARENT`
                let (r, g, b) = nc.rgb().unwrap();
                Some(colorsys::Rgb::new(r.into(), g.into(), b.into(), None).into())
            } else {
                None
            };
            if from_name.is_some() {
                from_name
            } else {
                // hex without the hash?
                colorsys::Rgb::from_hex_str(&format!("#{}", primary))
                    .map(|c| c.into())
                    .ok()
            }
        }
    }
}
