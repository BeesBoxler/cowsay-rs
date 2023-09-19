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
                Line::Middle => ("| ", " |"),
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

    let lines = split(text, max_width);
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

fn split(text: &str, max_width: usize) -> Vec<String> {
    if text.is_empty() {
        return vec![String::new()];
    }
    let mut max_length = 0;
    let mut result = vec![];
    for line in text.lines() {
        let mut cur = line;
        while !cur.is_empty() {
            let (chunk, rest) = cur.split_at(min(max_width, cur.len()));
            max_length = max(max_length, chunk.len());
            result.push(chunk);
            cur = rest;
        }
    }

    let result = result
        .iter()
        .map(|line| format!("{:max_length$}", line))
        .collect();
    result
}
