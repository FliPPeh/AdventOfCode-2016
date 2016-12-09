extern crate regex;

use std::io::{stdin, BufRead};
use std::borrow::{Borrow};
use regex::Regex;

fn has_abba<T: Borrow<str>>(p: &T) -> bool {
    let sec = p.borrow();

    for i in 0 .. sec.len() - 3 {
        let a = sec.chars().nth(i).unwrap();
        let b = sec.chars().nth(i + 1).unwrap();

        if a != b && sec.chars().nth(i + 2).unwrap() == b && sec.chars().nth(i + 3).unwrap() == a {
            return true;
        }
    }

    false
}

fn find_babs<T: Borrow<str>>(p: &T) -> Vec<String> {
    let sec = p.borrow();

    let mut res = Vec::new();

    for i in 0 .. sec.len() - 2 {
        let a = sec.chars().nth(i).unwrap();
        let b = sec.chars().nth(i + 1).unwrap();
        let c = sec.chars().nth(i + 2).unwrap();

        if a != b && a == c {
            res.push([b, a, b].iter().cloned().collect());
        }                     
    }

    res
}

fn main() {
    let stdin = stdin();
    let pattern = Regex::new(r"(?P<supernet>[:alpha:]+)|(?:\[(?P<hypernet>[:alpha:]+)\])").unwrap();

    let addresses: Vec<_> = 
        stdin.lock()
             .lines()
             .map(std::io::Result::unwrap)
             .map(|l| {
                let mut supernet = Vec::new();
                let mut hypernet = Vec::new();

                for m in pattern.captures_iter(&l) {
                    if let Some(r) = m.name("supernet") {
                        supernet.push(r.to_string());
                    }

                    if let Some(r) = m.name("hypernet") {
                        hypernet.push(r.to_string());
                    }
                }

                (supernet, hypernet)
             })
             .collect();

    println!("transport-layer snooping: {}",
        &addresses
            .iter()
            .filter(|&&(ref r, ref h)| 
                r.iter().any(has_abba) && !h.iter().any(has_abba))
            .count());

    println!("super-secret listening: {}",
        &addresses
            .iter()
            .filter(|&&(ref r, ref h)|
                r.iter()
                 .flat_map(find_babs)
                 .any(|bab| h
                     .iter()
                     .any(|hay| hay
                          .find(&bab)
                          .is_some())))
            .count());
}
