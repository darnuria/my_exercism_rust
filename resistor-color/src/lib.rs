#[derive(Debug, PartialEq)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    let color = match value {
        0 => "Black",
        1 => "Brown",
        2 => "Red",
        3 => "Orange",
        4 => "Yellow",
        5 => "Green",
        6 => "Blue",
        7 => "Violet",
        8 => "Grey",
        9 => "White",
        _ => "value out of range",
    };
    String::from(color)
}

pub fn colors() -> Vec<ResistorColor> {
    vec![
        ResistorColor::Black,
        ResistorColor::Brown,
        ResistorColor::Red,
        ResistorColor::Orange,
        ResistorColor::Yellow,
        ResistorColor::Green,
        ResistorColor::Blue,
        ResistorColor::Violet,
        ResistorColor::Grey,
        ResistorColor::White,
    ]
}
