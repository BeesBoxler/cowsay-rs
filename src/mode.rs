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
}

impl Mode {
    pub fn get_eyes(&self) -> &'static str {
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
        }
    }

    pub fn get_mouth(&self) -> &'static str {
        match self {
            Mode::Dead | Mode::Stoned => "U ",
            _ => "  ",
        }
    }
}
