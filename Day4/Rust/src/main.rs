extern crate regex;

use regex::Regex;
use std::io::{stdin, BufRead, Result as IoResult};

fn calc_checksum(crypt: &str) -> String {
    let mut chars = crypt.chars().filter(|&c| c != '-').collect::<Vec<_>>();
    chars.sort();
    chars.dedup();
    chars.sort_by(|a, b| {
        let oc_a = crypt.chars().filter(|c| *c == *a).count();
        let oc_b = crypt.chars().filter(|c| *c == *b).count();

        oc_b.cmp(&oc_a)
    });

    chars.iter().cloned().take(5).collect()
}

fn shift_cypher(crypt: &str, shift_num: i32) -> String {
    let alph = "abcdefghijklmnopqrstuvwxyz";

    crypt.chars().map(|c| {
        match c {
            '-'       => ' ',
            'a'...'z' =>
                alph
                    .chars()
                    .nth((alph.find(c).unwrap() + shift_num as usize) % alph.len())
                    .unwrap(),
            _ => unreachable!()
        }
    }).collect()
}

fn main() {
    let stdin = stdin();
    let room_pattern = Regex::new(r"(?x)
        ^
        (?P<crypt>(?:[:alpha:]+-?)+)-
        (?P<sectorid>[:digit:]+)
        \[(?P<checksum>[:alpha:]+)\]").unwrap();

    println!("{:#?}",
        stdin
            .lock()
            .lines()
            .map(IoResult::unwrap)
            .map(|s| {
                let caps = room_pattern.captures(&s).unwrap();

                (caps.name("crypt").unwrap().to_owned(),
                 caps.name("sectorid").unwrap().parse::<i32>().unwrap(),
                 caps.name("checksum").unwrap().to_owned())
            })
            .map(|(c, s, q)| (c.clone(), s, q, calc_checksum(&c), shift_cypher(&c, s)))
            .inspect(|r| println!("{:?}", r))
            .filter(|&(_, _, ref claim_sum, ref real_sum, _)| claim_sum == real_sum)
            .fold(0, |acc, (_, id, _, _, _)| acc + id));
}
