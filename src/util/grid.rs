use crate::util::point::*;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
        Grid {
            width: lines[0].len(),
            height: lines.len(),
            bytes: lines.into_iter().flatten().copied().collect(),
        }
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .bytes
            .chunks(self.width)
            .map(|row| String::from_utf8(row.to_vec()).unwrap())
            .join("\n");
        write!(f, "{}", s)
    }
}
impl<T: Copy + PartialEq> Grid<T> {
    #[allow(dead_code)]
    pub fn find(&self, needle: T) -> Option<Point> {
        self.bytes.iter().position(|&h| h == needle).map(|index| {
            let x = index % self.width;
            let y = index / self.width;
            Point::new(x as i32, y as i32)
        })
    }
}

impl<T> Grid<T> {
    
    pub fn same_size_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid {
            height: self.height,
            width: self.width,
            bytes: vec![value; self.height * self.width]
        }
    }
    #[allow(dead_code)]
    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        let w = self.width as i32;
        let h = self.height as i32;
        point.x >= 0 && point.x < w && point.y >= 0 && point.y < h
    }
    pub fn get(&self, point: Point) -> Option<&T> {
        if self.contains(point) {
            Some(&self[point])
        } else {
            None
        }
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(row, col)| Point::from_usize(col, row))
    }
}
impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, point: Point) -> &Self::Output {
        &self.bytes[((self.width as i32) * point.y + point.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.bytes[((self.width as i32) * point.y + point.x) as usize]
    }
}
