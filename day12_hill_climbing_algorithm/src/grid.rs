#![allow(dead_code)]

use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

use crate::cell::Cell;

pub struct Grid<T> {
    v: Vec<T>,
    pub w: usize,
    pub h: usize,
}

impl Grid<u8> {
    pub fn parse(lines: &Vec<String>) -> Grid<u8> {
        let mut v: Vec<u8> = Vec::new();
        let mut w = 0usize;
        let mut h = 0usize;

        for line in lines {
            let mut line = line.as_bytes().to_vec();
            w = line.len();
            h += 1;

            v.append(&mut line);
        }

        Grid { v, w, h }
    }
}

impl<T: Default + Clone + Display + Debug + Eq + Copy> Grid<T> {
    pub fn new(w: usize, h: usize) -> Grid<T> {
        Grid {
            v: vec![T::default(); w * h],
            w,
            h,
        }
    }

    fn print(&self) {
        for i in 0..self.h {
            for j in 0..self.w {
                print!("{:?} ", self[(i, j)]);
            }
            println!();
        }
    }

    pub fn find(&self, el: T) -> Option<Cell> {
        for ind in 0..self.h * self.w {
            if self.v[ind] == el {
                let i = (ind / self.w) as isize;
                let j = (ind % self.w) as isize;
                return Some(Cell { i, j });
            }
        }

        None
    }

    pub fn init(&mut self, el: T) {
        for ind in 0..self.w * self.h {
            self.v[ind] = el;
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &T {
        &self.v[i * self.w + j]
    }
}

impl<T> Index<Cell> for Grid<T> {
    type Output = T;

    fn index(&self, Cell { i, j }: Cell) -> &T {
        &self.v[i as usize * self.w + j as usize]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        &mut self.v[i * self.w + j]
    }
}

impl<T> IndexMut<Cell> for Grid<T> {
    fn index_mut(&mut self, Cell { i, j }: Cell) -> &mut T {
        &mut self.v[i as usize * self.w + j as usize]
    }
}
