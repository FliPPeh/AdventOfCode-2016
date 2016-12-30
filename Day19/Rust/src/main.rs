use std::env;

fn main() {
    let num_elfs: usize = env::args().nth(1).map(|r| r.parse().unwrap()).unwrap_or(5);

    let np2 = 2_usize.pow((num_elfs as f64).log2() as u32);
    let diff = num_elfs - np2;

    println!("Part 1: {}", diff * 2 + 1);

    let log3 = (num_elfs as f64).log(3.0) as u32;
    let np3 = 3_usize.pow(log3);
    let np32 = 3_usize.pow(log3 + 1);
    let diff = num_elfs - np3;

    let part2 = if diff == 0 {
        num_elfs
    } else if num_elfs > (np32 - np3) {
        diff + (num_elfs - (np32 - np3))
    } else {
        diff
    };

    println!("Part 2: {}", part2);
}

