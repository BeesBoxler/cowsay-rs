#![allow(unused)]

mod bubble;
mod cow_loader;
mod mode;

use bubble::{bubble, Style};
use cow_loader::load_cow;
use mode::Mode;
use std::io::Read;

struct Options {
    mode: Mode,
    style: Style,
    cow: &'static str,
    input: Option<String>,
    width: Option<usize>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            style: Style::Say,
            cow: "cows/default.cow",
            input: None,
            width: Some(40),
        }
    }
}

fn main() {
    let options = get_options_from_args();
    let mut input = String::new();

    match options.input {
        Some(inp) => input = inp,
        None => {
            std::io::stdin().read_to_string(&mut input);
        }
    }

    println!("{}", bubble(input.trim(), &options.style, options.width));

    let cow = load_cow(options.cow, &options.mode);
    println!("{}", cow.ok().unwrap());
}

fn get_options_from_args() -> Options {
    let mut options = Options::default();

    let args = std::env::args().collect::<Vec<String>>()[1..].join(" ");
    let mut chars = args.chars().peekable();

    let mut custom_eyes = None;
    let mut custom_tongue = None;

    let mut buffer = vec![];

    while let Some(c) = chars.next() {
        match c {
            '-' => {
                'args: while let Some(c) = chars.peek() {
                    match c {
                        'e' => {
                            let mut eyes = vec![];
                            chars.next();
                            'eyes: while let Some(c) = chars.peek() {
                                match c {
                                    ' ' => {
                                        if !eyes.is_empty() {
                                            break 'eyes;
                                        }
                                    }
                                    c => eyes.push(*c),
                                }
                                chars.next();
                            }
                            if !eyes.is_empty() {
                                let mut eyes = eyes.iter().collect::<String>();
                                eyes.truncate(2);
                                custom_eyes = Some(eyes)
                            }
                        }
                        'T' => {
                            let mut tongue = vec![];
                            chars.next();
                            'tongue: while let Some(c) = chars.peek() {
                                match c {
                                    ' ' => {
                                        if !tongue.is_empty() {
                                            break 'tongue;
                                        }
                                    }
                                    c => tongue.push(*c),
                                }
                                chars.next();
                            }
                            if !tongue.is_empty() {
                                let mut tongue = tongue.iter().collect::<String>();
                                tongue.truncate(2);
                                custom_tongue = Some(tongue)
                            }
                        }
                        'W' => {
                            let mut width = vec![];
                            chars.next();
                            'width: while let Some(c) = chars.peek() {
                                match c {
                                    ' ' => {
                                        if !width.is_empty() {
                                            break 'width;
                                        }
                                    }
                                    c => width.push(*c),
                                }
                                chars.next();
                            }
                            if !width.is_empty() {
                                match width.iter().collect::<String>().parse::<usize>() {
                                    Ok(x) => options.width = Some(x),
                                    Err(_) => options.width = Some(0),
                                }
                            }
                        }
                        'n' => options.width = None,
                        'b' => options.mode = Mode::Borg,
                        'd' => options.mode = Mode::Dead,
                        'g' => options.mode = Mode::Greedy,
                        'p' => options.mode = Mode::Paranoid,
                        's' => options.mode = Mode::Stoned,
                        't' => options.mode = Mode::Tired,
                        'w' => options.mode = Mode::Wired,
                        'y' => options.mode = Mode::Youthful,
                        _ => break 'args,
                    }
                    chars.next();
                }
            }
            ' ' => {}
            c => {
                buffer.push(c);
                for c in chars.by_ref() {
                    buffer.push(c);
                }
            }
        }
    }

    if (custom_eyes.is_some() || custom_tongue.is_some()) && options.mode == Mode::Default {
        options.mode = Mode::Custom(custom_eyes, custom_tongue);
    }

    let buffer = buffer.iter().collect::<String>();

    if !buffer.is_empty() {
        options.input = Some(buffer);
    }

    options
}
