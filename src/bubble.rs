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
            Style::Think => ("( ", " )"),
            Style::Say => match line {
                Line::First => (" /", "\\"),
                Line::Last => (" \\", "/"),
                Line::Middle => ("| ", " |"),
                Line::Only => (" <", ">"),
            },
        }
    }
}

pub fn bubble(text: &str, style: Style) -> String {
    let lines = split(text);
    let count = lines.len() - 1;
    let mut out = vec![];

    out.push(top(std::cmp::min(LINE_WIDTH, text.len())));
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
    out.push(bottom(std::cmp::min(LINE_WIDTH, text.len())));

    out.join("\r\n")
}

fn top(length: usize) -> String {
    format!("  {}", "_".repeat(length + 2))
}

fn bottom(length: usize) -> String {
    format!("  {}", "-".repeat(length + 2))
}

fn split(text: &str) -> Vec<String> {
    let mut result = if text.len() > LINE_WIDTH {
        vec![]
    } else {
        vec![text.to_string()]
    };
    let mut cur = if text.len() > LINE_WIDTH { text } else { "" };
    while !cur.is_empty() {
        let (chunk, rest) = cur.split_at(std::cmp::min(LINE_WIDTH, cur.len()));
        result.push(format!("{}{}", chunk, " ".repeat(LINE_WIDTH - chunk.len())));
        cur = rest;
    }
    result
}
