use clap::Parser;
use colorscheme::{cli::Args, scheme::ColorScheme};
fn main() -> Result<(), String> {
    let args = Args::parse();
    if let Some(primary) = args.primary() {
        let scheme = ColorScheme::new(primary, args.scheme());
        println!("{}", scheme.as_css(args.selector.as_deref()));
        Ok(())
    } else {
        Err(String::from("invalid primary color"))
    }
}
