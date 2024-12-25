#[macro_use]
extern crate num_derive;

use num_traits::FromPrimitive;
use strum_macros::EnumIter; // 0.17.1
use std::fmt::Display;
use core::fmt;
//
#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, FromPrimitive, Hash)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter)]
pub enum Diag {
    NE,
    SE,
    SW,
    NW,
}

impl Diag {
    pub const fn get_dir(self) -> (i64, i64) {
        match self {
            Self::NE => (1, -1),
            Self::SE => (1, 1),
            Self::SW => (-1, 1),
            Self::NW => (-1, -1),
        }
    }
}


impl Display for Dir{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dir::N => write!(f, "^"),
            Dir::S => write!(f, "v"),
            Dir::W => write!(f, "<"),
            Dir::E => write!(f, ">"),
        }
    }
}

impl Dir{
    pub fn new(c: char) -> Self {
        match c {
            '^' => Dir::N,
            'v' => Dir::S,
            '<' => Dir::W,
            '>' => Dir::E,
            _ => panic!("Unknown character"),
        }
    }
    pub const fn get_dir(self) -> (i64, i64) {
        match self {
            Self::N => (0, -1),
            Self::E => (1, 0),
            Self::S => (0, 1),
            Self::W => (-1, 0),
        }
    }

    pub const fn get_char(self) -> char {
        match self {
            Self::N => '^',
            Self::E => '>',
            Self::S => 'v',
            Self::W => '<',
        }
    }

    pub fn opposite(self) -> Self {
        FromPrimitive::from_u8((self as u8 + 2) % 4).unwrap()
    }

    pub fn cw(self) -> Self {
        FromPrimitive::from_u8((self as u8 + 1) % 4).unwrap()
    }
    pub fn ccw(self) -> Self {
        FromPrimitive::from_u8((self as u8 + 3) % 4).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Dir::N.ccw(), Dir::W);
    }
}
