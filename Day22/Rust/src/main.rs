#![feature(conservative_impl_trait)]

extern crate regex;

use std::io::{stdin, BufRead, Result as IoResult};
use regex::Regex;

#[derive(Debug)]
struct Node {
    file: String,
    size_tb: usize,
    used_tb: usize
}

fn combinations(n: usize) -> impl Iterator<Item=(usize, usize)> + 'static {
    (0..n).flat_map(move |e| std::iter::repeat(e).zip(0..n))
}

fn main() {
    let pattern = Regex::new(r"(?x)
        (?P<fs>[/a-z0-9-]+)\s+
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
}
