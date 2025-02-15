use colorsys::Hsl;
fn main() {
    let scheme = colorscheme::ColorScheme::dyad(Hsl::new(90.0, 0.5, 0.5, Some(1.0)));
    println!("{:?}", scheme);
}
