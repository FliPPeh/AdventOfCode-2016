extern crate regex;

use regex::Regex;

use std::borrow::Borrow;
use std::io::{stdin, BufRead};
use std::fmt;

const SCREEN_HEIGHT: usize = 6;
const SCREEN_WIDTH: usize = 50;

struct Screen {
    pixbuf: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]
}

impl Screen {
    fn new() -> Screen {
        Screen {
            pixbuf: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT]
        }
    }

    fn lit(&self) -> usize {
        self.pixbuf.iter().flat_map(|row| row.iter()).filter(|&&p| p).count()
    }

    fn call(&mut self, op: Operation) {
        match op {
            Operation::Rect{ w, h }      => self.fill_rect(w, h),
            Operation::RotateRow{ row, n } => self.rotate_row(row, n),
            Operation::RotateCol{ col, n } => self.rotate_col(col, n)
        }
    }

    fn fill_rect(&mut self, w: usize, h: usize) {
        for i in 0 .. h {
            for j in 0 .. w {
                self.pixbuf[i][j] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, n: usize) {
        let old = self.pixbuf[row];

        for i in 0 .. SCREEN_WIDTH {
            self.pixbuf[row][(i + n) % SCREEN_WIDTH] = old[i];
        }
    }

    fn rotate_col(&mut self, col: usize, n: usize) {
        let old: Vec<_> = self.pixbuf.iter().map(|r| r[col]).rev().collect();

        for i in 0 .. SCREEN_HEIGHT {
            self.pixbuf[SCREEN_HEIGHT - 1 - i][col] = old[(i + n) % SCREEN_HEIGHT];
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.pixbuf.iter() {
            for col in row.iter() {
                write!(f, "{}", if *col { '#' } else { '.' })?
            }

            write!(f, "\n")?
        }

        Result::Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Rect{ w: usize, h: usize },
    RotateRow{ row: usize, n: usize },
    RotateCol{ col: usize, n: usize }
}

fn parse_operation<T: Borrow<str>>(p: T) -> Operation {
    let pattern_rect: Regex = Regex::new(r"^rect (\d+)x(\d+)").unwrap();
    let pattern_rotr: Regex = Regex::new(r"^rotate row y=(\d+) by (\d+)").unwrap();
    let pattern_rotc: Regex = Regex::new(r"^rotate column x=(\d+) by (\d+)").unwrap();

    let l = p.borrow();

    if let Some(c) = pattern_rect.captures(l) {
        return Operation::Rect{
            w: c.at(1).unwrap().parse().unwrap(),
            h: c.at(2).unwrap().parse().unwrap()
        }
    }

    if let Some(c) = pattern_rotr.captures(l) {
        return Operation::RotateRow{
            row: c.at(1).unwrap().parse().unwrap(),
            n: c.at(2).unwrap().parse().unwrap()
        }
    }

    if let Some(c) = pattern_rotc.captures(l) {
        return Operation::RotateCol{
            col: c.at(1).unwrap().parse().unwrap(),
            n: c.at(2).unwrap().parse().unwrap()
        }
    }

    panic!("invalid command: {}", l);
}

fn main() {
    let mut s = Screen::new();

    let stdin = stdin();

    for command in stdin.lock().lines().map(std::io::Result::unwrap).map(parse_operation) {
        println!("=> {:?}", command);

        s.call(command);
        println!("{}", s);
    }

    println!("{}", s.lit());
}
