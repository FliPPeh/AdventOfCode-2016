extern crate regex;

use regex::Regex;

use std::io::{BufRead, stdin};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TargetType {
    Bot,
    Output   
}

type TargetID = usize;
type Target = (TargetType, TargetID);

type Chip = i32;

type Bot = (Option<Chip>, Option<Chip>);

#[derive(Debug)]
enum Instruction {
    Input(Target, Chip),
    Pass(Target, Target, Target)
}

static mut BOTS: [Bot; 256] = [(None, None); 256];

fn put(bot: Bot, c: Chip) -> Bot {
    match bot {
        (None,        None)        => (Some(c), None),
        (a @ Some(_), None)        => (a,       Some(c)),
        (None,        b @ Some(_)) => (Some(c), b),
        (a @ Some(_), b @ Some(_)) => (a,       b),
    }
}

fn is_full(bot: Bot) -> bool {
    bot.0.is_some() && bot.1.is_some()
}

fn main() {
    let stdin = stdin();

    let match_input = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
    let match_pass = Regex::new(r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$").unwrap();

    let instructions = stdin.lock()
         .lines()
         .map(std::io::Result::unwrap)
         .map(|input| {
            if let Some(cap) = match_input.captures(&input) {
                return Instruction::Input(
                    (TargetType::Bot, cap.at(2).unwrap().parse().unwrap()),
                    cap.at(1).unwrap().parse().unwrap());
            }

            if let Some(cap) = match_pass.captures(&input) {
                let t_lo = if cap.at(2).unwrap() == "bot" { TargetType::Bot } else { TargetType::Output };
                let t_hi = if cap.at(4).unwrap() == "bot" { TargetType::Bot } else { TargetType::Output };

                return Instruction::Pass(
                    (TargetType::Bot, cap.at(1).unwrap().parse().unwrap()),
                    (t_lo, cap.at(3).unwrap().parse().unwrap()),
                    (t_hi, cap.at(5).unwrap().parse().unwrap()));
            }

            panic!("invalid input");
         })
         .collect::<Vec<_>>();

    /* Seed */
    for instr in &instructions {
        if let Instruction::Input((TargetType::Bot, n), c) = *instr {
            unsafe {
                BOTS[n] = put(BOTS[n], c);
            }
        }
    }

    /* Distribute */
    loop {
        let mut handovers = 0;

        for (i, _) in unsafe { BOTS.iter() }.enumerate() {
            unsafe {
                if let (Some(a), Some(b)) = BOTS[i] {
                    for (hi, lo) in instructions
                        .iter()
                        .filter_map(|instr| match *instr {
                            Instruction::Pass((TargetType::Bot, n), lo, hi) => if n == i {
                                    Some((hi, lo))
                                } else {
                                    None
                                },

                            _ => None
                        }) {

                        if let (TargetType::Bot, lo) = lo {
                            if !is_full(BOTS[lo]) {
                                BOTS[lo] = put(BOTS[lo], if a < b { a } else { b });
                                handovers += 1;
                            }
                        }

                        if let (TargetType::Bot, hi) = hi {
                            if !is_full(BOTS[hi]) {
                                BOTS[hi] = put(BOTS[hi], if a < b { b } else { a });
                                handovers += 1;
                            }
                        }
                    }
                }
            }
        }

        if handovers == 0 {
            break;
        }
    }

    for (i, bot) in unsafe { BOTS.iter() }.enumerate() {
        if bot.0.is_some() && bot.1.is_some() {
            println!("Bot {} <{} ! {}>", i, bot.0.unwrap(), bot.1.unwrap());
        }
    }
}
