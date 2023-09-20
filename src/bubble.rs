use regex::Replacer;
use std::cmp::{max, min};

enum Line {
    First,
    Last,
    Middle,
    Only,
}

pub enum Style {
    Say,
    Think,
}

impl Style {
    fn get_delimiters(&self, line: Line) -> (&'static str, &'static str) {
        match self {
            Style::Think => (" (", ")"),
            Style::Say => match line {
                Line::First => (" /", "\\"),
                Line::Last => (" \\", "/"),
                Line::Middle => (" |", "|"),
                Line::Only => (" <", ">"),
            },
        }
    }
}

pub fn bubble(text: &str, style: &Style, width: Option<usize>) -> String {
    let max_width = match width {
        Some(x) => x,
        None => usize::MAX,
    };

    let (lines, max_width) = split(text, max_width);
    let count = lines.len() - 1;
    let mut out = vec![];

    out.push(top(min(max_width, text.len())));
    for (i, line) in lines.iter().enumerate() {
        let line_type = if text.len() > max_width {
            match i {
                0 => Line::First,
                x if { x == count } => Line::Last,
                _ => Line::Middle,
            }
        } else {
            Line::Only
        };

        let pads = style.get_delimiters(line_type);

        out.push(format!("{} {} {}", pads.0, line, pads.1));
    }
    out.push(bottom(min(max_width, text.len())));

    out.join("\r\n")
}

fn top(length: usize) -> String {
    format!("  {}", "_".repeat(length + 2))
}

fn bottom(length: usize) -> String {
    format!("  {}", "-".repeat(length + 2))
}

fn split(text: &str, max_width: usize) -> (Vec<String>, usize) {
    if text.is_empty() {
        return (vec![String::new()], 0);
    }

    let mut max_length = 0;
    let mut result = vec![];
    let mut line = String::new();

    for input_line in text.lines() {
        let mut words = input_line.split_whitespace();
        for word in words {
            max_length = max(max_length, line.trim().len());
            line.push(' ');
            if line.len() + word.len() > max_width {
                result.push(line);
                line = String::new();
            }

            line.push_str(word);
        }

        max_length = max(max_length, line.trim().len());
        result.push(line);
        line = String::new();
    }


    (
        result
            .iter()
            .map(|line| format!("{:<max_length$}", line.trim()))
            .collect(),
        max_length,
    )
}
