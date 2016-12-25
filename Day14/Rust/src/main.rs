extern crate crypto;

use crypto::digest::Digest;

use crypto::md5::Md5;
use std::env;

fn is_key(input: &str, idx: i32, target: char) -> Option<(i32, String)> {
    let mut digest = Md5::new();

    for i in idx..idx+1000 {
        digest.input_str(&input);
        digest.input_str(&format!("{}", i));
        
        let result: Vec<_> = digest.result_str().chars().collect();

        for j in 0..28 {
            if (result[j] == target)
                && (result[j + 1] == target)
                && (result[j + 2] == target)
                && (result[j + 3] == target)
                && (result[j + 4] == target)
            {
                return Some((i, digest.result_str()));
            }
        }

        digest.reset();
    }

    None
}

fn main() {
    let input = if let Some(input) = env::args().nth(1) {
        input
    } else {
        println!("usage: {} <input>", env::args().nth(0).unwrap());
        return;
    };

    let mut digest = Md5::new();
    let mut keys = Vec::new();

    'outer: for i in 0.. {
        digest.input_str(&input);
        digest.input_str(&format!("{}", i));

        let result: Vec<_> = digest.result_str().chars().collect();

        'inner: for j in 0..30 {
            let target = result[j];

            if (result[j + 1] == target) && (result[j + 2] == target) {
                if let Some((ii, s)) = is_key(&input, i+1, target) {
                    println!("{} -> {} <=> {} <- {}!", i, digest.result_str(), s, ii);
                    keys.push(i);

                    if keys.len() == 64 {
                        break 'outer;
                    }
                }

                break 'inner;
            }
        }

        digest.reset();
    }

    //println!("Keys: {:?}", keys);
}
