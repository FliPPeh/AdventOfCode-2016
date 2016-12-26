use std::env;

fn step(a: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(a.len() * 2 + 1);

    res.extend_from_slice(a);
    res.push(0);

    for c in a.iter().rev().map(|&c| if c == 1 {0} else {1}) {
        res.push(c);
    }

    res
}

fn checksum(a: &[u8]) -> Vec<u8> {
    if a.len() % 2 == 1 {
        Vec::from(a)
    } else {
        let mut cs = Vec::with_capacity(a.len() / 2);

        for i in 0..a.len() / 2 {
            cs.push(!((a[i*2] > 0) ^ (a[i*2+1] > 0)) as u8);
        }

        checksum(&cs)
    }
}

fn main() {
    let (rinput, rlen) = if env::args().count() >= 2 {
        let a = env::args().nth(1);
        let b = env::args().nth(2);

        (a.unwrap(), b.unwrap())
    } else {
        println!("usage: {} <input> <len>", env::args().nth(0).unwrap());
        return;
    };

    let len = rlen.parse().unwrap();

    let mut input = if !rinput.chars().all(|c| c == '1' || c == '0') {
        println!("error: malformed input");
        return;
    } else {
        rinput.chars().map(|c| if c == '1' {1} else {0}).collect::<Vec<u8>>()
    };

    while input.len() < len {
        input = step(&input);
    }

    input.truncate(len);

    println!("Data: {}", input.iter().map(|i| format!("{}", i)).collect::<String>());
    println!("Checksum: {}", checksum(&input).iter().map(|i| format!("{}", i)).collect::<String>());
}
