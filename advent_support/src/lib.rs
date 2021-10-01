extern crate lazy_static;

pub use lazy_static::lazy_static;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt::{Display};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines2<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
        .map(|l| l.unwrap())
        .collect()
}

pub fn lines_arg1() -> Vec<String> {
    read_lines2(&argv()[1])
}

pub fn contents_arg1() -> String {
    std::fs::read_to_string(&argv()[1]).unwrap()
}

pub fn argv() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

#[derive(Debug)]
pub struct Array2d<T>
where T: Copy + Display {
    buf: Vec<T>,
    cols: usize,
    rows: usize,
}

fn index_of(col: usize, row: usize, cols: usize, rows: usize) -> usize {
    assert!(col < cols);
    assert!(row < rows);
    row * cols + col
}

impl<T> Array2d<T>
where T: Copy + Display {
    pub fn new(cols: usize, rows: usize, value: T) -> Array2d<T> {
        assert!(cols > 0);
        assert!(rows > 0);
        let mut buf: Vec<T> = Vec::new();
        let size = (cols*rows) as usize;
        buf.resize(size, value);
        Array2d::<T> {
            buf: buf,
            cols: cols,
            rows: rows,
        }
    }

    pub fn get(&self, col: usize, row: usize) -> T {
        self.buf[index_of(col, row, self.cols, self.rows)]
    }

    pub fn get_wrapped(&self, col: usize, row: usize) -> T {
        let wrapped_col = col % self.cols;
        let wrapped_row = row % self.rows;
        self.buf[index_of(wrapped_col, wrapped_row, self.cols, self.rows)]
    }

    pub fn set(&mut self, col: usize, row: usize, value: T) {
        self.buf[index_of(col, row, self.cols, self.rows)] = value
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn map_to<T2>(&self, transform: fn(&T) -> T2) -> Array2d<T2>
    where T2: Copy + Display {
        Array2d::<T2> {
            buf: self.buf.iter().map(transform).collect(),
            cols: self.cols,
            rows: self.rows,
        }
    }

    pub fn dump(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{}", self.get(col, row));
            }
            println!()
        }
    }
}