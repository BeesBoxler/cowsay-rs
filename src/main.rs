#![allow(unused)]

mod bubble;
mod cow_loader;
mod mode;

use bubble::{bubble, Style};

fn main() {
    println!("\"Say\" tests:");
    println!("{}", bubble("This is a test message", Style::Say));
    println!(
        "{}", 
        bubble(
            "This is a test message, except this one is so much longer and should therefore split over multiple lines. Let's see how this works...",
            Style::Say
        )
    );
    println!("\"Think\" tests:");
    println!("{}", bubble("This is a test message", Style::Think));
    println!(
        "{}", 
        bubble(
            "This is a test message, except this one is so much longer and should therefore split over multiple lines. Let's see how this works...",
            Style::Think
        )
    );
}
