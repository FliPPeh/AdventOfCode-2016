extern crate crypto;

use crypto::digest::Digest;

use crypto::md5::Md5;
use std::env;

fn find<F>(hash: F)
    where F: Fn(i32) -> String
{
    let mut keys = Vec::new();
    let mut tbc: Vec<(i32, char)> = Vec::new();

    'outer: for i in 0.. {
        let result_str = hash(i);
        let result: Vec<_> = result_str.chars().collect();

        // Check for trips
        for j in 0..30 {
            let target = result[j];

            if (result[j + 1] == target) && (result[j + 2] == target) {
                tbc.push((i, target));

                break;
            }
        }

        // Check for quads
        for j in 0..28 {
            let target = result[j];

            if (result[j + 1] == target)
                && (result[j + 2] == target)
                && (result[j + 3] == target)
                && (result[j + 4] == target)
            {
                for &(ti, t) in tbc.iter().filter(|&&(ti, t)|
                        (t == target)
                            && i <= (ti + 1000)
                            && ti < i)
                {
                    // Don't double confirm
                    if keys.iter().find(|&&ti2| ti2 == ti).is_some() {
                        continue;
                    }

                    println!("{} -> {} <=> {} <- {}!", ti, hash(ti), result_str, i);

                    keys.push(ti);

                    if keys.len() == 64 {
                        break 'outer;
                    }
                }

                // Prune a bit, so memory doesn't explode
                tbc.retain(|&(ti, _)|
                    keys.iter().find(|&&ti2| ti2 == ti).is_none() && ((ti + 1000) > i));
            }
        }
    }

    keys.sort();
    println!("{:?}", keys);
}

fn main() {
    let input = if let Some(input) = env::args().nth(1) {
        input
    } else {
        println!("usage: {} <input>", env::args().nth(0).unwrap());
        return;
    };

    println!("Part 1");
    find(|idx| {
        let mut digest = Md5::new();

        digest.input_str(&input);
        digest.input_str(&format!("{}", idx));

        digest.result_str()
    });

    println!("Part 2");
    find(|idx| {
        let mut digest = Md5::new();
        let mut res = format!("{}{}", input, idx);

        for _ in 0..2017 {
            digest.input_str(&res);

            res = digest.result_str();
            digest.reset();
        }

        res
    });

    //println!("Keys: {:?}", keys);
}
