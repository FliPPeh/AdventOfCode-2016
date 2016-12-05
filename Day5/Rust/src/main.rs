extern crate crypto;
extern crate rand;

use std::env::args;
use std::io::{stdout, Write};

use crypto::md5::Md5;
use crypto::digest::Digest;

use rand::Rng;

const PASSLEN: usize = 8;

fn part1(pass_base: &str) -> () {
    let mut digest = Md5::new();
    let mut password = String::new();

    for i in 0.. {
        digest.input_str(&pass_base);
        digest.input_str(&format!("{}", i));

        let hash = digest.result_str();

        if hash.chars().take(5).all(|c| c == '0') {
            let next = hash.chars().nth(5).unwrap();

            password.push(next);

            print!("Part 1: {}\r", password);
            stdout().flush().ok().unwrap();

            if password.len() == PASSLEN {
                break;
            }
        }

        digest.reset();
    }

    println!("");
}

fn part2(pass_base: &str) -> () {
    let mut digest = Md5::new();
    let mut password: [char; PASSLEN] = ['?'; PASSLEN];

    let mut rng = rand::thread_rng();

    let wildcards = &['!', '§', '$', '%', '&', '/', '(', ')', '[', ']', '{', '}', '#', '*'];

    for i in 0.. {
        digest.input_str(&pass_base);
        digest.input_str(&format!("{}", i));

        let hash = digest.result_str();

        if hash.chars().take(5).all(|c| c == '0') {
            if let Some(pos) = hash.chars().nth(5).unwrap().to_digit(8) {
                let next = hash.chars().nth(6).unwrap();

                if password[pos as usize] == '?' {
                    password[pos as usize] = next;
                }

                if password.iter().all(|c| *c != '?') {
                    break;
                }
            }
        }

        if i % 10000 == 0 {
            print!("Part 2: {}\r", password
                .iter()
                .cloned()
                .map(|c| if c == '?' {
                        wildcards[rng.gen::<usize>() % wildcards.len()] 
                    } else {
                        c
                    })
                .collect::<String>());

            stdout().flush().ok().unwrap();
        }

        digest.reset();
    }

    println!("Part 2: {}", password.iter().cloned().collect::<String>());
}

fn main() {
    let pass_base = args().nth(1).expect("Gib base");

    part1(&pass_base);
    part2(&pass_base);
}
