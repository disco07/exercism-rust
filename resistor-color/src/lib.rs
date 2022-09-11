extern crate core;

use crate::ResistorColor::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    unimplemented!("convert a color into a numerical representation")
}

pub fn value_to_color_string(value: u32) -> String {
    let color = colors().get((value as usize));
    match color {
        Some(x) => x.to_string(),
        None => panic!("Index not found"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut vec: Vec<ResistorColor> = vec![Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White];
    vec
}

impl ToString for ResistorColor {
    fn to_string(&self) -> String {
        match self {
            Black => "Black".to_string(),
            Brown => "Brown".to_string(),
            Red => "Red".to_string(),
            Orange => "Orange".to_string(),
            Yellow => "Yellow".to_string(),
            Green => "Green".to_string(),
            Blue => "Blue".to_string(),
            Violet => "Violet".to_string(),
            Grey => "Grey".to_string(),
            White => "White".to_string(),
            _ => panic!("enum not find"),
        }
    }
}