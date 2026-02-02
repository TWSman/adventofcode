#[macro_use]
extern crate num_derive;

use num_traits::FromPrimitive;
use strum_macros::EnumIter; // 0.17.1
use std::fmt::Display;
use core::fmt;
use std::ops::{Mul, Add};
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter)]
pub enum AllDir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl AllDir {
    pub const fn get_dir(self) -> (i64, i64) {
        match self {
            Self::NW => (-1, -1),
            Self::N => (0, -1),
            Self::NE => (1, -1),
            Self::E => (1, 0),
            Self::SE => (1, 1),
            Self::S => (0, 1),
            Self::SW => (-1, 1),
            Self::W => (-1, 0),
        }
    }
    pub const fn get_dir_true(self) -> (i64, i64) {
        match self {
            Self::NW => (-1, 1),
            Self::N => (0, 1),
            Self::NE => (1, 1),
            Self::E => (1, 0),
            Self::SE => (1, -1),
            Self::S => (0, -1),
            Self::SW => (-1, -1),
            Self::W => (-1, 0),
        }
    }
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

    pub const fn get_dir_true(self) -> (i64, i64) {
        // Alternate version with y increasing upwards (North is +y)
        match self {
            Self::NE => (1, 1),
            Self::SE => (1, -1),
            Self::SW => (-1, -1),
            Self::NW => (-1, 1),
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

    pub const fn get_dir_true(self) -> (i64, i64) {
        // Alternate version with y increasing upwards (North is +y)
        match self {
            Self::N => (0, 1),
            Self::E => (1, 0),
            Self::S => (0, -1),
            Self::W => (-1, 0),
        }
    }

    pub const fn get_dir_true_vec(self) -> Vec2D {
        match self {
            Self::N => Vec2D { x: 0, y: 1 },
            Self::E => Vec2D { x: 1, y: 0 },
            Self::S => Vec2D { x: 0, y: -1},
            Self::W => Vec2D { x: -1, y: 0},
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec2D {
    pub x: i64,
    pub y: i64,
}

impl Vec2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self {x,y}
    }

    pub fn manhattan(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn distance_to_y(&self, y: i64) -> i64 {
        (self.y - y).abs()
    }
}

impl Mul<i64> for Vec2D {
    type Output = Vec2D;
    fn mul(self, rhs: i64) -> Self::Output {
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}


impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Vec2D) -> Vec2D {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Dir::N.ccw(), Dir::W);
    }

    #[test]
    fn vec2d() {
        let a = Vec2D{x: 10, y: 10};
        let b = Vec2D{x: 20, y: 20};
        assert_eq!(a.manhattan(&b),20);
        assert_eq!(a.manhattan(&a), 0);
        assert_eq!(b.manhattan(&a),20);

        assert_eq!(a.distance_to_y(20),10);
        assert_eq!(a.distance_to_y(0),10);
        assert_eq!(a.distance_to_y(10),0);

        let a = Vec2D{x: -10, y: -10};
        let b = Vec2D{x: -20, y: -20};
        assert_eq!(a.manhattan(&b),20);
        assert_eq!(a.manhattan(&a), 0);
        assert_eq!(b.manhattan(&a),20);

        assert_eq!(a +b , Vec2D{x: -30, y: -30});
        assert_eq!(a * 2 , Vec2D{x: -20, y: -20});
    }
}
