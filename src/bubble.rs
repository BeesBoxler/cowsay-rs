use std::cmp::{max, min};

const LINE_WIDTH: usize = 40;

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

pub fn bubble(text: &str, style: &Style) -> String {
    let lines = split(text);
    let count = lines.len() - 1;
    let mut out = vec![];

    out.push(top(min(LINE_WIDTH, text.len())));
    for (i, line) in lines.iter().enumerate() {
        let line_type = if text.len() > LINE_WIDTH {
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
    out.push(bottom(min(LINE_WIDTH, text.len())));

    out.join("\r\n")
}

fn top(length: usize) -> String {
    format!("  {}", "_".repeat(length + 2))
}

fn bottom(length: usize) -> String {
    format!("  {}", "-".repeat(length + 2))
}

fn split(text: &str) -> Vec<String> {
    if text.is_empty() {
        return vec![String::new()];
    }
    let mut max_length = 0;
    let mut result = vec![];
    for line in text.lines() {
        let mut cur = line;
        while !cur.is_empty() {
            let (chunk, rest) = cur.split_at(min(LINE_WIDTH, cur.len()));
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
