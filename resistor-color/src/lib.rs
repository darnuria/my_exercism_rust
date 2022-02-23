use core::panic;

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

fn value_to_color(value: usize) -> ResistorColor {
    match value {
        0 => ResistorColor::Black,
        1 => ResistorColor::Brown,
        2 => ResistorColor::Red,
        3 => ResistorColor::Orange,
        4 => ResistorColor::Yellow,
        5 => ResistorColor::Green,
        6 => ResistorColor::Blue,
        7 => ResistorColor::Violet,
        8 => ResistorColor::Grey,
        9 => ResistorColor::White,
        _ => panic!("Not supposed to happens."),
    }
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    let color = match value {
        0..=9 => value_to_color(value),
        _ => return String::from("value out of range"),
    };
    let color = match color {
        ResistorColor::Black => "Black",
        ResistorColor::Brown => "Brown",
        ResistorColor::Red => "Red",
        ResistorColor::Orange => "Orange",
        ResistorColor::Yellow => "Yellow",
        ResistorColor::Green => "Green",
        ResistorColor::Blue => "Blue",
        ResistorColor::Violet => "Violet",
        ResistorColor::Grey => "Grey",
        ResistorColor::White => "White",
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
