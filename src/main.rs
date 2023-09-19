#![allow(unused)]

mod bubble;
mod cow_loader;
mod mode;

use std::io::Read;
use bubble::{bubble, Style};
use cow_loader::load_cow;
use mode::Mode;

struct Options {
    mode: Mode,
    style: Style,
    cow: &'static str,
    use_stdin: bool,
    input: Option<&'static str>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            style: Style::Say,
            cow: "cows/default.cow",
            use_stdin: true,
            input: None,
        }
    }
}

fn main() {

    let options = get_options_from_args();
    let mut input = String::new();

    if options.use_stdin{
        std::io::stdin().read_to_string(&mut input);
    }

    println!("{}", bubble(&format!("{}",&input.trim()), &options.style));

    let cow = load_cow(options.cow, &options.mode);
    println!("{}", cow.ok().unwrap());
}

fn get_options_from_args() -> Options {
    let mut args = std::env::args().peekable();

    let mut options = Options::default();

    while let Some(arg) = args.peek() {
        match &arg[..] {
            "-m" => {
                options.mode = Mode::Tired
            }
            _ => break
        }

    }

    options
}