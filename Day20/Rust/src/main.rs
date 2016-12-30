use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut blocks = stdin
        .lock()
        .lines()
        .map(std::io::Result::unwrap)
        .map(|l| {
            let mut range = l.split('-');

            (range.next().unwrap().parse::<usize>().unwrap(),
             range.next().unwrap().parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    blocks.sort();

    let mut low: usize = 0;
    let mut total: usize = 0;
    let (mut tlo, mut thi) = blocks[0];

    for &(lo, hi) in &blocks {
        if low >= lo {
            low = hi + 1;
        }

        if lo > thi + 1 {
            total += thi - tlo + 1;

            tlo = lo;
            thi = hi;
        } else {
            thi = std::cmp::max(thi, hi);
        }
    }

    println!("Part 1: {}", low);
    println!("Part 2: {}", 2usize.pow(32) - total - (thi - tlo + 1));
}
