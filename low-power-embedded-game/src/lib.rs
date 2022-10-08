// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend/divisor, dividend%divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&mut self) -> i16 {
        if self.0 < 0 {
            self.0 *= -1;
        }
        if self.1 < 0 {
            self.1 *= -1;
        }
        self.0 + self.1
    }
}
