#![cfg_attr(feature = "multithread", feature(step_by))]

extern crate crypto;
extern crate rand;

#[cfg(feature = "multithread")]
use std::time;

#[cfg(feature = "multithread")]
use std::thread;

#[cfg(feature = "multithread")]
use std::sync::{mpsc, Arc, Mutex};

#[cfg(feature = "multithread")]
use std::iter::repeat;

use std::env::args;
use std::io::{stdout, Write};

use crypto::md5::Md5;
use crypto::digest::Digest;

use rand::Rng;

const PASSLEN: usize = 8;

#[cfg(feature = "multithread")]
const NUMTHREADS: usize = 4;

const WILDCARDS: &'static [char] =
    &['!', '§', '$', '%', '&', '/', '(', ')', '[', ']', '{', '}', '#', '*'];

fn matrixify(c: char) -> char {
    let mut rng = rand::thread_rng();

    if c == '?' {
        WILDCARDS[rng.gen::<usize>() % WILDCARDS.len()] 
    } else {
        c
    }
}


#[cfg(not(feature = "multithread"))]
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

#[cfg(not(feature = "multithread"))]
fn part2(pass_base: &str) -> () {
    let mut digest = Md5::new();
    let mut password: [char; PASSLEN] = ['?'; PASSLEN];

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
            print!("Part 2: {}\r", password.iter().cloned().map(matrixify).collect::<String>());

            stdout().flush().ok().unwrap();
        }

        digest.reset();
    }

    println!("Part 2: {}", password.iter().cloned().collect::<String>());
}

#[cfg(feature = "multithread")]
fn hack(base_pass: &str) -> () {
    let mut workers = Vec::new();

    let (p1_tx, p1_rx) = mpsc::channel::<(usize, char)>();
    let (p2_tx, p2_rx) = mpsc::channel::<(usize, usize, char)>();

    let kill = Arc::new(Mutex::new(false));

    for i in 0..NUMTHREADS {
        let base_pass_cln = base_pass.to_string();
        let tx1 = p1_tx.clone();
        let tx2 = p2_tx.clone();

        let kill_cln = kill.clone();

        workers.push(thread::spawn(move || {
            let mut digest = Md5::new();

            for j in (i..).step_by(NUMTHREADS) {
                if *kill_cln.lock().unwrap() {
                    return;
                }

                digest.input_str(&base_pass_cln);
                digest.input_str(&format!("{}", j));

                let hash = digest.result_str();

                if hash.chars().take(5).all(|c| c == '0') {
                    let sixth = hash.chars().nth(5).unwrap();

                    tx1.send((j, sixth)).unwrap();

                    if let Some(pos) = sixth.to_digit(8) {
                        let next = hash.chars().nth(6).unwrap();

                        tx2.send((j, pos as usize, next)).unwrap();
                    }
                }

                digest.reset();
            }
        }));
    }

    let mut p1_idx = 0;
    let mut p1 = String::new();

    let mut p2_idx = 0;
    let mut p2 = ['?'; PASSLEN];

    let mut blink = 0;

    loop {
        match p1_rx.try_recv() {
            Ok((idx, chr)) => if idx >= p1_idx && p1.len() < PASSLEN {
                p1.push(chr);
                p1_idx = idx;
            },

            _ => {}
        }

        match p2_rx.try_recv() {
            Ok((idx, pos, chr)) =>
                match p2[pos] {
                    '?'               => { p2[pos] = chr; p2_idx = idx; }
                    c if idx < p2_idx => { p2[pos] = chr; p2_idx = idx; }
                    _                 => {}
                },

            _ => {}
        }

        let p1_str = p1.chars().chain(repeat('_')).take(PASSLEN).collect::<String>();
        let p2_str = p2.iter().cloned().map(matrixify).collect::<String>();

        if p1.len() >= PASSLEN && !p2.iter().any(|c| *c == '?') {
            println!("[ DONE! ]  {} - {}", p1_str, p2_str);

            *kill.lock().unwrap() = true;   
            break;
        } else {
            if blink < 25 {
                print!("[HACKING]  ");
            } else if blink >= 25 {
                print!("           ");

                if blink == 50 {
                    blink = 0;
                }
            }

            blink += 1;

            print!("{} - {}\r", p1_str, p2_str);
            stdout().flush().ok().unwrap();
        }

        thread::sleep(time::Duration::from_millis(25));
    }

    for t in workers {
        t.join().unwrap();
    }
}

#[cfg(feature = "multithread")]
fn main() {
    let pass_base = args().nth(1).expect("Gib base");

    println!("Hacking with MAXIMUM EFFORT...");
    hack(&pass_base);
}

#[cfg(not(feature = "multithread"))]
fn main() {
    let pass_base = args().nth(1).expect("Gib base");

    println!("Hacking like it's the 90s...");
    part1(&pass_base);
    part2(&pass_base);
}
