extern crate regex;

use regex::Regex;

use std::io::{BufRead, stdin};
use std::collections::HashMap;
use std::cmp::{min, max};
use std::env::args;

type Chip = i32;

#[derive(Debug, Clone, Copy)]
enum Target {
    Bot(usize),
    Output(usize)
}

#[derive(Debug, Copy, Clone)]
struct Bot(Option<Chip>, Option<Chip>);

impl Bot {
    fn put(&mut self, c: Chip) {
        *self = match *self {
            Bot(None,        None)        => Bot(Some(c), None),
            Bot(a @ Some(_), None)        => Bot(a,       Some(c)),
            Bot(None,        b @ Some(_)) => Bot(Some(c), b),
            Bot(a @ Some(_), b @ Some(_)) => Bot(a,       b),
        }
    }

    fn full(&self) -> bool {
        self.0.is_some() && self.1.is_some()
    }
}

const NUMBOTS: usize = 256;
const NUMOUTS: usize = 256;

fn main() {
    let a: Chip = args().nth(1).and_then(|s| s.parse().ok()).expect("invalid arguments");
    let b: Chip = args().nth(2).and_then(|s| s.parse().ok()).expect("invalid arguments");

    let mut bots = [(false, Bot(None, None)); NUMBOTS];
    let mut outputs = [None; NUMOUTS];
    let mut pairs = HashMap::<usize, (Target, Target)>::new();

    let match_input = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
    let match_pass = Regex::new(r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$").unwrap();

    let stdin = stdin();

    for line in stdin.lock().lines().map(std::io::Result::unwrap) {
        if let Some(cap) = match_input.captures(&line) {
            let i_self: usize = cap.at(2).unwrap().parse().unwrap();
            let c: Chip = cap.at(1).unwrap().parse().unwrap();

            bots[i_self].1.put(c);
        }

        if let Some(cap) = match_pass.captures(&line) {
            let i_lo = cap.at(3).unwrap().parse().unwrap();
            let i_hi = cap.at(5).unwrap().parse().unwrap();

            let i_self = cap.at(1).unwrap().parse().unwrap();
            let t_lo = if cap.at(2).unwrap() == "bot" {Target::Bot} else {Target::Output}(i_lo);
            let t_hi = if cap.at(4).unwrap() == "bot" {Target::Bot} else {Target::Output}(i_hi);

            pairs.entry(i_self).or_insert((t_lo, t_hi));
        }
    }

    loop {
        let mut happening = false;

        for i in 0 .. bots.len() {
            if bots[i].1.full() && !bots[i].0 {
                let lo = min((bots[i].1).0.unwrap(), (bots[i].1).1.unwrap());
                let hi = max((bots[i].1).0.unwrap(), (bots[i].1).1.unwrap());

                if let Some(&(lo_target, hi_target)) = pairs.get(&i) {
                    macro_rules! put {
                        ($target:expr, $val:expr) => {
                            match $target {
                                Target::Output(j) => outputs[j] = Some($val),
                                Target::Bot(j) => (bots[j].1).put($val)
                            }
                        }
                    }

                    put!(lo_target, lo);
                    put!(hi_target, hi);

                    bots[i].0 = true;
                    happening = true;
                }
            }
        }

        if !happening {
            break;
        }
    }

    for (i, bot) in bots.iter().enumerate() {
        println!("Bot {} <{} ! {}>", i, (bot.1).0.unwrap_or(-1), (bot.1).1.unwrap_or(-1));
    }

    println!("Part 1: {:?}", bots
                .iter()
                .enumerate()
                .filter(|&(_, &(_, Bot(ba, _ )))| ba.iter().any(|&ba| ba == a || ba == b))
                .filter(|&(_, &(_, Bot(_,  bb)))| bb.iter().any(|&bb| bb == a || bb == b))
                .nth(0)
                .expect("I failed :(")
                .0);
    println!("Part 2: {}", (0..3).map(|i| outputs[i].unwrap()).product::<i32>());
}
