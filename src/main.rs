mod bubble;
mod mode;

use bubble::{bubble, Style};

fn main() {
    println!("{}", bubble("This is a test message", Style::Say));
    println!("{}", bubble("This is a test message, except this one is so much longer and should therefore split over multiple lines. Let's see how this works...", Style::Say));
}
