extern crate regex;

use std::io::{BufRead};
use regex::Regex;

type Triangle = (i32, i32, i32);

fn part1(triangles: &[Triangle]) -> usize {
    triangles
        .iter()
        .map(|&(a, b, c)| ([(a + b) > c, (c + a) > b, (c + b) > a].iter().all(|&b| b), (a, b, c)))
        .filter(|&(r, _)| r)
        .count()
}

fn part2(triangles: &[Triangle]) -> usize {
    let mut flipped = Vec::new();
    let mut i = triangles.iter();

    while flipped.len() < triangles.len() {
        let &(a1, b1, c1) = i.next().unwrap();
        let &(a2, b2, c2) = i.next().unwrap();
        let &(a3, b3, c3) = i.next().unwrap();

        flipped.push((a1, a2, a3));
        flipped.push((b1, b2, b3));
        flipped.push((c1, c2, c3));
    }

    part1(&flipped)
}

fn main() {
    let stdin = std::io::stdin();
    let split_pattern = Regex::new(r"\s+").unwrap();

    let triangles: Vec<_> = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|s| s.trim().to_string())
        .map(|s| {
            let mut i = split_pattern.split(&s).map(str::parse::<i32>).map(Result::unwrap);
            (i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
        })
        .collect();

        println!("Part 1: {}", part1(&triangles));
        println!("Part 2: {}", part2(&triangles));
}
/*
abc
cab
cba
*/
