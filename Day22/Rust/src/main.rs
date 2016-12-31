#![feature(conservative_impl_trait)]

extern crate regex;

use std::io::{stdin, BufRead, Result as IoResult};
use regex::Regex;

#[derive(Debug)]
struct Node {
    coord: (usize, usize),
    file: String,
    size_tb: usize,
    used_tb: usize
}

fn combinations(n: usize) -> impl Iterator<Item=(usize, usize)> + 'static {
    (0..n).flat_map(move |e| std::iter::repeat(e).zip(0..n))
}

fn main() {
    let pattern = Regex::new(r"(?x)
        (?P<fs>/dev/grid/node-x(?P<x>\d+)-y(?P<y>\d+))\s+
        (?P<size>\d+)T\s+
        (?P<used>\d+)T\s+
        (?P<avail>\d+)T\s+
        (?P<useperc>\d+)%").unwrap();

    let stdin = stdin();
    let nodes = stdin
        .lock()
        .lines()
        .skip(2)
        .map(IoResult::unwrap)
        .flat_map(|l| pattern.captures(&l).map(|cap| Node {
                coord: (
                    cap.name("x").and_then(|n| n.parse().ok()).unwrap(),
                    cap.name("y").and_then(|n| n.parse().ok()).unwrap()),
                file: cap.name("fs").unwrap().to_string(),
                size_tb: cap.name("size").and_then(|n| n.parse().ok()).unwrap(),
                used_tb: cap.name("used").and_then(|n| n.parse().ok()).unwrap()
            }))
        .collect::<Vec<_>>();

    let mut viable = Vec::new();

    for (a, b) in combinations(nodes.len()).filter(|&(a, b)| a != b) {
        let node_a = &nodes[a];
        let node_b = &nodes[b];

        if node_a.used_tb > 0 && (node_b.size_tb - node_b.used_tb) >= node_a.used_tb {
            viable.push((node_a, node_b))
        }
    }

    println!("Part 1: {}", viable.len());

    let (mx, my) = nodes.iter().fold((0, 0), |(mx, my), &Node { coord: (x, y), ..}| {
        (if x > mx {x} else {mx}, if y > my {y} else {my})
    });

    println!("Grid size: {}x{}", mx, my);

    for y in 0..my+1 {
        for x in 0..mx+1 {
            let node = nodes.iter().find(|n| n.coord == (x, y)).expect("what");

            let (fg, bg) = if (x, y) == (0, 0) {
                (2, 0)
            } else if (x, y) == (mx, 0) {
                (1, 0)
            } else if viable.iter().find(|&&(a, b)| a.coord == (x, y) || b.coord == (x, y)).is_some() {
                (4, 0)
            } else {
                (0, 0)
            };

            print!("\x1b[3{};1m\x1b[4{};1m{:3}/{:3}\x1b[0m ", fg, bg, node.used_tb, node.size_tb);
        }

        println!("");
    }
}

// 23 to y1
// 24 to y0
// 30 to y0 x30 (1 bef. target)
// 35 to target @ y0 x30
// 40 to target @ y0 x29
// ...
// 180 to target @ y0 x1
// 181 to target @ y0 x0!
