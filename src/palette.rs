use crate::color::Color;

pub const ANSIOPEN: &str = "\x1b[";
pub const ANSIRESET: &str = "\x1b[0m";

pub const CGACHAR: [char; 4] = [' ', '*', '+', '▒'];

pub fn palette_from_abbr(name: &str) -> [Color; 4] {
    match name {
        "0" => CGA0,
        "0i" => CGA0I,
        "1i" => CGA1I,
        "1" | _ => CGA1,
    }
}

pub const CGA0: [Color; 4] = [
    Color::Black(false),
    Color::Green(false),
    Color::Red(false),
    Color::Brown(false),
];
pub const CGA0I: [Color; 4] = [
    Color::Black(false),
    Color::Green(true),
    Color::Red(true),
    Color::Brown(true),
];
pub const CGA1: [Color; 4] = [
    Color::Black(false),
    Color::Cyan(false),
    Color::Magenta(false),
    Color::White(false),
];
pub const CGA1I: [Color; 4] = [
    Color::Black(false),
    Color::Cyan(true),
    Color::Magenta(true),
    Color::White(true),
];