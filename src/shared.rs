#![allow(unused)]

use std::fmt::Display;
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

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.chunks(self.width) {
            for tile in row {
                tile.fmt(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct UFNode {
    parent: usize,
    size: usize,
}

#[derive(Debug, Clone)]
pub struct UnionFind {
    nodes: Vec<UFNode>,
    num_roots: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let nodes = (0..size).map(|parent| UFNode { parent, size: 1 }).collect();
        Self {
            nodes,
            num_roots: size,
        }
    }

    pub fn find(&mut self, mut index: usize) -> usize {
        let mut parent = self.nodes[index].parent;
        while index != parent {
            let grand_parent = self.nodes[parent].parent;
            self.nodes[index].parent = grand_parent;
            index = grand_parent;
            parent = self.nodes[index].parent;
        }
        index
    }

    pub fn union(&mut self, mut index1: usize, mut index2: usize) -> bool {
        index1 = self.find(index1);
        index2 = self.find(index2);
        if index1 == index2 {
            return false;
        }
        if self.nodes[index1].size < self.nodes[index2].size {
            (index1, index2) = (index2, index1);
        }
        self.nodes[index2].parent = index1;
        self.nodes[index1].size += self.nodes[index2].size;
        self.num_roots -= 1;
        true
    }

    pub fn roots(&self) -> impl Iterator<Item = (usize, usize)> {
        self.nodes
            .iter()
            .enumerate()
            .filter_map(|(ix, n)| (n.parent == ix).then_some((ix, n.size)))
    }

    pub const fn num_roots(&self) -> usize {
        self.num_roots
    }
}

impl Display for UnionFind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sizes = self.roots().map(|(_, s)| s).collect::<Vec<_>>();
        sizes.sort_unstable();
        let mut fmt_list = f.debug_list();
        let mut last = 0;
        let mut count = 0;
        for &size in sizes.iter().rev() {
            if size == last {
                count += 1;
            } else {
                if count > 0 {
                    fmt_list.entry(&(last, count));
                }
                last = size;
                count = 1;
            }
        }
        if count > 0 {
            fmt_list.entry(&(last, count));
        }
        fmt_list.finish()
    }
}
