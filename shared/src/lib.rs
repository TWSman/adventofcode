#[macro_use]
extern crate num_derive;

use num_traits::FromPrimitive;
use strum_macros::EnumIter; // 0.17.1
use std::fmt::Display;
use core::fmt;
use std::ops::{Mul, Add, Sub};
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter)]
pub enum Dir3D {
    N, // Positive y
    E, // Positive x
    S,
    W,
    U, // Positive z
    D,
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

    pub const fn get_dir_true_vec(self) -> Vec2D {
        match self {
            Self::N => Vec2D { x: 0, y: 1 },
            Self::NE => Vec2D { x: 1, y: 1 },
            Self::E => Vec2D { x: 1, y: 0 },
            Self::SE => Vec2D { x: 1, y: -1},
            Self::S => Vec2D { x: 0, y: -1},
            Self::SW => Vec2D { x: -1, y: -1},
            Self::W => Vec2D { x: -1, y: 0},
            Self::NW => Vec2D { x: -1, y: 1},
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


impl Dir3D{
    pub fn new(c: char) -> Self {
        match c {
            '^' => Self::N,
            'v' => Self::S,
            '<' => Self::W,
            '>' => Self::E,
            'U' => Self::U,
            'D' => Self::D,
            _ => panic!("Unknown character"),
        }
    }

    pub const fn get_dir_true(self) -> (i64, i64, i64) {
        // Alternate version with y increasing upwards (North is +y)
        match self {
            Self::E => (1, 0, 0),
            Self::W => (-1, 0, 0),
            Self::N => (0, 1, 0),
            Self::S => (0, -1, 0),
            Self::U => (0, 0, 1),
            Self::D => (0, 0, -1),
        }
    }

    pub const fn get_dir_true_vec(self) -> Vec3D {
        match self {
            Self::N => Vec3D { x: 0, y: 1, z: 0 },
            Self::E => Vec3D { x: 1, y: 0, z: 0 },
            Self::S => Vec3D { x: 0, y: -1, z: 0},
            Self::W => Vec3D { x: -1, y: 0, z: 0},
            Self::U => Vec3D { x: 0, y: 0, z: 1},
            Self::D => Vec3D { x: 0, y: 0, z: -1},
        }
    }

    pub const fn get_char(self) -> char {
        match self {
            Self::N => '^',
            Self::E => '>',
            Self::S => 'v',
            Self::W => '<',
            Self::U => 'U',
            Self::D => 'D',
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Self::N => Self::S,
            Self::S => Self::N,
            Self::E => Self::W,
            Self::W => Self::E,
            Self::U => Self::D,
            Self::D => Self::U,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

impl Add<(i64,i64)> for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: (i64, i64)) -> Vec2D {
        Vec2D {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;
    fn sub(self, rhs: Vec2D) -> Vec2D {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl PartialOrd for Vec2D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec2D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.y.cmp(&other.y) {
            std::cmp::Ordering::Equal => self.x.cmp(&other.x),
            ord => ord,
        }
    }
}

impl PartialOrd for Vec3D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec3D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y)).then(self.z.cmp(&other.z))
    }
}

impl fmt::Display for Vec2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vec3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self {x,y,z}
    }

    pub fn manhattan(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    pub fn dot(&self, other: &Self) -> i64 {
        // Dot product of two vectors
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<i64> for Vec3D {
    type Output = Vec3D;
    fn mul(self, rhs: i64) -> Self::Output {
        Vec3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


impl Add for Vec3D {
    type Output = Vec3D;
    fn add(self, rhs: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;
    fn sub(self, rhs: Vec3D) -> Vec3D {
        Vec3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl fmt::Display for Vec3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vec4D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub t: i64,
}

impl Vec4D {
    pub fn new(x: i64, y: i64, z: i64, t: i64) -> Self {
        Self {x,y,z,t}
    }

    pub fn manhattan(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs() + (self.t - other.t).abs()
    }

    pub fn dot(&self, other: &Self) -> i64 {
        // Dot product of two vectors
        self.x * other.x + self.y * other.y + self.z * other.z + self.t * other.t
    }
}

impl Add for Vec4D {
    type Output = Vec4D;
    fn add(self, rhs: Vec4D) -> Vec4D {
        Vec4D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            t: self.t + rhs.t,
        }
    }
}

impl PartialOrd for Vec4D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec4D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y)).then(self.z.cmp(&other.z)).then(self.t.cmp(&other.t))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Dir::N.ccw(), Dir::W);
        assert_eq!(Dir::W.ccw(), Dir::S);
        assert_eq!(Dir::S.ccw(), Dir::E);
        assert_eq!(Dir::E.ccw(), Dir::N);

        assert_eq!(Dir::W.cw(), Dir::N);
        assert_eq!(Dir::N.cw(), Dir::E);
        assert_eq!(Dir::E.cw(), Dir::S);
        assert_eq!(Dir::S.cw(), Dir::W);

        assert_eq!(Dir::W.opposite(), Dir::E);
        assert_eq!(Dir::E.opposite(), Dir::W);
        assert_eq!(Dir::N.opposite(), Dir::S);
        assert_eq!(Dir::S.opposite(), Dir::N);
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
        assert_eq!(a - b , Vec2D{x: 10, y: 10});
        assert_eq!(a * 2 , Vec2D{x: -20, y: -20});
    }

    #[test]
    fn vec4d() {
        let a = Vec4D{x: 10, y: 10, z:10, t: 10};
        let b = Vec4D{x: 20, y: 20, z:20, t: 20};
        let c = Vec4D{x: 10, y: 10, z:10, t: 20};
        assert_eq!(a.manhattan(&b),40);
        assert_eq!(a.manhattan(&a), 0);
        assert_eq!(b.manhattan(&a),40);

        assert_eq!(a.manhattan(&c),10);

        assert_eq!(a.dot(&b), 800);
    }
}
