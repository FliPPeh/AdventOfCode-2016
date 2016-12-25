extern crate regex;

use regex::Regex;

use std::io::{stdin, BufRead};

// Disc Num, npos, T0 pos
#[derive(Debug)]
struct Disc(i32, i32, i32);

impl Disc {
    fn position(&self, time: i32) -> i32 {
        (self.2 + time) % self.1
    }
}

fn find_droptime(discs: &[Disc]) -> i32 {
    for t0 in 0_i32.. {
        if (t0 + 1..t0 + discs.len() as i32 + 1).zip(discs.iter()).all(|(t, d)| d.position(t) == 0) {
            return t0;
        }
    }

    unreachable!();
}

fn main() {
    let pattern = Regex::new(r"^Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).$").unwrap();
    let stdin = stdin();

    let mut discs = stdin
        .lock()
        .lines()
        .map(std::io::Result::unwrap)
        .map(|line| {
            let cap = pattern.captures(&line).unwrap();

            Disc(
                cap.at(1).unwrap().parse().unwrap(),
                cap.at(2).unwrap().parse().unwrap(),
                cap.at(3).unwrap().parse().unwrap())
        })
        .collect::<Vec<_>>();

    discs.sort_by(|d1, d2| d1.0.cmp(&d2.0));

    println!("{:?}", discs);
    println!("part1: {}", find_droptime(&discs));

    let l = discs.len() as i32;
    discs.push(Disc(l, 11, 0));

    println!("{:?}", discs);
    println!("part2: {}", find_droptime(&discs));
}
