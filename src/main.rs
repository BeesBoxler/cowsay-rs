#![allow(unused)]

mod bubble;
mod cow_loader;
mod mode;

use bubble::{bubble, Style};
use cow_loader::load_cow;
use mode::Mode;

fn main() {

    let mode = Mode::Default;
    let style = Style::Say;

    println!("{}", bubble("This is a test message", &style));
    println!(
        "{}", 
        bubble(
            "This is a test message, except this one is so much longer and should therefore split over multiple lines. Let's see how this works...",
           &style 
        )
    );

    let cow = load_cow("cows/butcher.cow", &mode);
    println!("{}", cow.ok().unwrap());
}
