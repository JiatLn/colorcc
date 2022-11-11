#[derive(Debug)]
pub enum BaseColor {
    Red,
    Green,
    Blue,
    Random,
    Unknown,
}

impl From<String> for BaseColor {
    fn from(val: String) -> Self {
        match val.as_str() {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            "random" => Self::Random,
            _ => Self::Unknown,
        }
    }
}
