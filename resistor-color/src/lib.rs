extern crate core;

use std::fmt::{Display, Formatter};
use crate::ResistorColor::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
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

impl Display for ResistorColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color as u32
}

pub fn value_to_color_string(value: u32) -> String {
    if value > colors().len() as u32 {
        return String::from("value out of range");
    }
    colors().get(value as usize).unwrap().to_string()
}

pub fn colors() -> Vec<ResistorColor> {
    let vec: Vec<ResistorColor> = vec![Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White];
    vec
}