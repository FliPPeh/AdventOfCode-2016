extern crate regex;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::io::{stdin, BufRead, Result as IoResult};

use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Operation {
    SwapPos(usize, usize),
    SwapChar(char, char),
    Rotate(isize),
    RotatePos(char),
    Reverse(usize, usize),
    Move(usize, usize)
}

lazy_static! {
    static ref RE_SWAP: Regex =
        Regex::new(r"^swap (letter|position) (\d+|[a-zA-Z]) with (letter|position) (\d+|[a-zA-Z])$")
            .unwrap();

    static ref RE_ROTATE: Regex = Regex::new(r"^rotate (left|right) (\d+) steps?$").unwrap();
    static ref RE_ROTATE_LETTER: Regex = Regex::new(r"^rotate based on position of letter ([a-zA-Z])$").unwrap();
    static ref RE_REVERSE_RANGE: Regex = Regex::new(r"^reverse positions (\d+) through (\d+)$").unwrap();
    static ref RE_MOVE: Regex = Regex::new(r"^move position (\d+) to position (\d+)$").unwrap();
}

fn parse_operation(input: &str) -> Option<Operation> {
    if let Some(cap) = RE_SWAP.captures(&input) {
        let typ = cap.at(1).unwrap();
        let a = cap.at(2).unwrap();
        let b = cap.at(4).unwrap();

        return if typ == "letter" {
            Some(Operation::SwapChar(a.as_bytes()[0] as char, b.as_bytes()[0] as char))
        } else {
            Some(Operation::SwapPos(a.parse().unwrap(), b.parse().unwrap()))
        }
    }

    if let Some(cap) = RE_ROTATE.captures(&input) {
        let dir = cap.at(1).unwrap();
        let n = cap.at(2).unwrap().parse::<isize>().unwrap();

        return Some(Operation::Rotate(if dir == "left" {
            n
        } else {
            -n
        }));
    }

    if let Some(cap) = RE_ROTATE_LETTER.captures(&input) {
        return Some(Operation::RotatePos(cap.at(1).unwrap().as_bytes()[0] as char));
    }

    if let Some(cap) = RE_REVERSE_RANGE.captures(&input) {
        return Some(Operation::Reverse(
            cap.at(1).unwrap().parse().unwrap(),
            cap.at(2).unwrap().parse::<usize>().unwrap() + 1));
    }

    if let Some(cap) = RE_MOVE.captures(&input) {
        return Some(Operation::Move(
            cap.at(1).unwrap().parse().unwrap(),
            cap.at(2).unwrap().parse().unwrap()));
    }

    debug_assert!(false, "Whoa there");

    None
}

fn rotate_string(string: &str, n: isize) -> String {
    let old = string.as_bytes();

    String::from_utf8((n..n+old.len() as isize)
          .map(|i| old[if i < 0 {
              let d = (-i as usize % old.len()) as isize;

              (old.len() as isize - d) as usize % old.len()
          } else {
              i as usize % old.len()
          }])
          .collect()).unwrap()
}

fn swap_chars(string: String, a: usize, b: usize) -> String {
    let mut old = string.into_bytes();
    let tmp = old[b];

    old[b] = old[a];
    old[a] = tmp;

    String::from_utf8(old).unwrap()
}

fn scramble(input: &str, ops: &[Operation], inverse: bool) -> String {
    let mut input = input.to_string();

    let iter: Box<Iterator<Item=&Operation>> = if inverse {
        Box::new(ops.iter().rev())
    } else {
        Box::new(ops.iter())
    };

    for op in iter {
        //println!("=> {:?}", op);
        //print!("{} -> ", input);

        input = match *op {
            Operation::SwapPos(a, b) => {
                swap_chars(input, a, b)
            },

            Operation::SwapChar(a, b) => {
                let apos = input.find(a).unwrap();
                let bpos = input.find(b).unwrap();

                swap_chars(input, apos, bpos)
            },

            Operation::Rotate(n) => {
                rotate_string(&input, if inverse { -n } else { n })
            },

            /*  Pos | f | NPos 
             *  ----+---+------
             *   0  | 1 | 1    
             *   1  | 2 | 3    
             *   2  | 3 | 5    
             *   3  | 4 | 7    
             *   4  | 6 | 2    
             *   5  | 7 | 4    
             *   6  | 8 | 6    
             *   7  | 9 | 0    
             *
             * 0 => -9   4 => -7
             * 1 => -1   5 => -3
             * 2 => -6   6 => 0
             * 3 => -2   7 => -4
             */
            Operation::RotatePos(c) => {
                let pos = input.find(c).unwrap();
                let n = if !inverse {
                    -(1 + pos as isize + if pos >= 4 {1} else {0})
                } else {
                    match pos {
                        0 => 9,
                        1 => 1,
                        2 => 6,
                        3 => 2,
                        4 => 7,
                        5 => 3,
                        6 => 0,
                        7 => 4,
                        _ => unreachable!()
                    }
                };
                            

                rotate_string(&input, n)
            },

            Operation::Reverse(s, e) => {
                String::from_utf8(
                               input[0..s].bytes()
                        .chain(input[s..e].bytes().rev())
                        .chain(input[e.. ].bytes())
                        .collect())
                    .unwrap()
            },

            Operation::Move(a, b) => {
                let (a_, b_) = if !inverse { (a, b) } else { (b, a) };
                let c = input.remove(a_);

                input.insert(b_, c);
                input
            }
        };

        //println!("{}", input);
    }

    input
}

fn main() {
    let input = env::args().nth(1).unwrap_or("abcde".to_string());
    let stdin = stdin();

    let ops = stdin
        .lock()
        .lines()
        .map(IoResult::unwrap)
        .flat_map(|l| parse_operation(&l))
        .collect::<Vec<_>>();

    println!("Part 1: {} -> {}", input, scramble(&input, &ops, false));
    println!("Part 2: {} -> {}", input, scramble(&input, &ops, true));
}
