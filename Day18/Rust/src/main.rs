use std::io::{Write, stdout};
use std::env;

fn main() {
    let input = if let Some(inp) = env::args().nth(1) {
        inp.chars().filter_map(|c| match c {
            '^' => Some(true),
            '.' => Some(false),
             _  => None
        }).collect::<Vec<_>>()
    } else {
        writeln!(stdout(), "usage: {} <^|.>... <rows>", env::args().nth(0).unwrap());
        return;
    };

    let len: usize = if let Some(inp) = env::args().nth(2) {
        match inp.parse() {
            Ok(l) => l,
            _ => {
                writeln!(stdout(), "error: bad numeric");
                return;
            }
        }
    } else {
        writeln!(stdout(), "usage: {} <^|.>... <rows>", env::args().nth(0).unwrap());
        return;
    };

    let rowlen = input.len();
    let mut res = vec![input];

    while res.len() < len {
        let mut new = Vec::with_capacity(rowlen);

        {
            let last = res.last().unwrap();

            for i in 0..rowlen {
                let l = if i == 0 { false } else { last[i - 1] };
                let c = last[i];
                let r = if i == (rowlen - 1) { false } else { last[i + 1] };

                new.push(match (l, c, r) {
                    (true, true, false) => true,
                    (false, true, true) => true,
                    (true, false, false) => true,
                    (false, false, true) => true,
                    _ => false
                });
            }
        }

        res.push(new);
    }

    for row in &res {
        println!("{}", row.iter().map(|b| if *b {'^'} else {'.'}).collect::<String>());
    }

    println!("Safe tiles: {}", res.iter().flat_map(|r| r).filter(|&t| !*t).count());
}
