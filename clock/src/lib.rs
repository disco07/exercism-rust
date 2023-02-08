use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const DAY: i32 = 24 * 60;
const HOUR: i32 = 60;

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes%HOUR)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}
