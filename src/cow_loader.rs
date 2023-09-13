use regex::Regex;

const COMMENT: &'static str = r##"^\s*#+"##;
const SUBSTITUTION: &'static str = r##"^\s*($\w+)\s*=\s*"(\w+)";"##;

pub fn load_cow(path: &str) -> Result<&'static str, std::io::Error> {
    let comment = Regex::new(COMMENT).unwrap();
    let mut substitutions = vec![("$thoughts", "\\")];

    let input = std::fs::read_to_string(path)?;
    for line in input.lines() {
        if comment.is_match(line) || line.is_empty() {
            continue;
        }
        let mut line = line.to_string();
        for substitution in &substitutions {
            line = line.replace(substitution.0, substitution.1);
        }
    }

    Ok("test")
}
