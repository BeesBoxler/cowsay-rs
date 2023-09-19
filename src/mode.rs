#[derive(PartialEq)]
pub enum Mode {
    Default,
    Stoned,
    Borg,
    Dead,
    Greedy,
    Paranoid,
    Tired,
    Wired,
    Youthful,
    Custom(Option<String>, Option<String>),
}

impl Mode {
    pub fn get_eyes(&self) -> String {
        match self {
            Mode::Default => "oo",
            Mode::Stoned => "**",
            Mode::Borg => "==",
            Mode::Dead => "xx",
            Mode::Greedy => "$$",
            Mode::Paranoid => "@@",
            Mode::Tired => "--",
            Mode::Wired => "OO",
            Mode::Youthful => "..",
            Mode::Custom(Some(x), _) => x,
            Mode::Custom(None, _) => "oo",
        }
        .to_string()
    }

    pub fn get_mouth(&self) -> String {
        match self {
            Mode::Dead | Mode::Stoned => "U ",
            Mode::Custom(_, Some(x)) => x,
            _ => "  ",
        }
        .to_string()
    }
}
