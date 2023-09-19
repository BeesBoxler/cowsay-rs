use regex::Regex;

use crate::mode::Mode;

const COMMENT: &str = r"^\s*#+";
const SUBSTITUTION: &str = r#"^\s*(\$\w+)\s*=\s*"(.+)";"#;
const THE_COW: &str = r#"\s*\$the_cow\s*=\s*<<\s*"?EOC"?"#;
const EOC: &str = r"\s*EOC";

pub fn load_cow(path: &str, mode: &Mode) -> Result<String, std::io::Error> {
    let comment = Regex::new(COMMENT).unwrap();
    let sub = Regex::new(SUBSTITUTION).unwrap();
    let the_cow = Regex::new(THE_COW).unwrap();
    let eoc = Regex::new(EOC).unwrap();

    let mut cow = vec![];
    let mut substitutions = vec![
        ("$thoughts".to_string(), "\\".to_string()),
        ("$eyes".to_string(), mode.get_eyes().to_string()),
        ("${eyes}".to_string(), mode.get_eyes().to_string()),
        ("$tongue".to_string(), mode.get_mouth().to_string()),
        ("${tongue}".to_string(), mode.get_mouth().to_string()),
        ("\\\\".to_string(), "\\".to_string()),
        ("\\@".to_string(), "@".to_string()),
    ];

    let input = std::fs::read_to_string(path)?;
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if comment.is_match(line) || line.is_empty() {
            continue;
        }
        let mut line = line.to_string();

        if sub.is_match(&line) {
            for substitution in &substitutions {
                line = line.replace(&substitution.0, &substitution.1);
            }
            let substitution = sub.captures(&line).unwrap();
            substitutions.push((
                substitution.get(1).unwrap().as_str().to_string(),
                substitution.get(2).unwrap().as_str().to_string(),
            ));

            continue;
        }

        if the_cow.is_match(&line) {
            for cow_line in lines.by_ref() {
                if eoc.is_match(cow_line) {
                    break;
                }
                cow.push(cow_line);
            }
        }
    }

    let mut cow = cow.join("\r\n");
    for substitution in &substitutions {
        cow = cow.replace(&substitution.0, &substitution.1);
    }
    cow = cow.replace(r"\e[", "\x1b[");

    Ok(cow)
}
