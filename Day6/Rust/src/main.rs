use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();

    let mut cols = Vec::new();
    
    for signal in stdin.lock().lines().map(Result::unwrap).map(|l| l.chars().collect::<Vec<_>>()) {
        for (i, col) in signal.iter().enumerate() {
            if cols.len() < (i + 1) {
                cols.push(Vec::new())
            }

            cols[i].push(*col);
        }
    }

    let mut p1 = String::new();
    let mut p2 = String::new();

    for col in cols.iter_mut() {
        col.sort();

        let mut cln = col.clone();
        cln.dedup();
        cln.sort_by(|&a, &b| {
            let oc_a = col.iter().filter(|&c| *c == a).count();
            let oc_b = col.iter().filter(|&c| *c == b).count();

            oc_b.cmp(&oc_a)
        });

        p1.push(cln[0]);
        p2.push(cln[cln.len() - 1]);
    }

    println!("Part 1: {}, Part 2: {}", p1, p2);
}
