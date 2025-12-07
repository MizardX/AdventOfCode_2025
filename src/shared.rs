#![allow(unused)]

use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

impl Pos {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(data.len(), width * height);
        Self {
            data,
            width,
            height,
        }
    }

    const fn get_index(&self, pos: Pos) -> Option<usize> {
        if pos.row < self.height || pos.col < self.width {
            Some(pos.row * self.width + pos.col)
        } else {
            None
        }
    }

    pub fn all_positions(&self) -> impl Iterator<Item = Pos> {
        (0..self.height).flat_map(|row| (0..self.width).map(move |col| Pos::new(row, col)))
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Pos) -> &Self::Output {
        let index = self.get_index(pos).expect("index out of range");
        &self.data[index]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        let index = self.get_index(pos).expect("index out of range");
        &mut self.data[index]
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<u8>,
{
    type Err = T::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().len();
        let mut data = Vec::with_capacity(height * width);
        for line in s.lines() {
            for ch in line.bytes() {
                data.push(ch.try_into()?);
            }
        }
        Ok(Self {
            data,
            width,
            height,
        })
    }
}
