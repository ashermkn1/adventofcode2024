#![allow(dead_code)]
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
pub const DIAGONAL: [Point; 4] = [
    Point::new(-1, -1),
    Point::new(1, -1),
    Point::new(-1, 1),
    Point::new(1, 1),
];
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    #[inline]
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub const fn from_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }
    #[inline]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }
    #[inline]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }
    #[inline]
    pub fn manhattan(self, other: Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<u8> for Point {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            b'^' | b'U' => UP,
            b'>' | b'R' => RIGHT,
            b'v' | b'D' => DOWN,
            b'<' | b'L' => LEFT,
            _ => unreachable!("malformed input"),
        }
    }
}

impl Mul<i64> for Point {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i64> for Point {
    #[inline]
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Add for Point {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
