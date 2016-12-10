extern crate regex;

use std::env::args;
use regex::Regex;

fn decompress_v1(input_str: &str) -> String {
    let compression_marker = Regex::new(r"\((?P<len>\d+)x(?P<reps>\d+)\)").unwrap();

    let mut input_offset = 0usize;
    let mut decompressed = String::new();

    while let Some(cap) = compression_marker.captures(&input_str[input_offset ..]) {
        let match_span = cap.pos(0).unwrap();
        let len: usize = cap.name("len").unwrap().parse().unwrap();
        let reps: usize = cap.name("reps").unwrap().parse().unwrap();

        if input_offset < (input_offset + match_span.0) {
            decompressed.push_str(&input_str[input_offset .. input_offset + match_span.0]);
        }

        for _ in 0 .. reps {
            decompressed.push_str(&input_str[
                (input_offset + match_span.1) .. (input_offset + match_span.1 + len)]);
        }

        input_offset += match_span.1 + len;
    }

    decompressed.push_str(&input_str[input_offset .. input_str.len()]);
    decompressed
}

fn decompress_v2(input_str: &str) -> usize {
    let compression_marker = Regex::new(r"\((?P<len>\d+)x(?P<reps>\d+)\)").unwrap();

    let mut input_offset = 0usize;
    let mut siz = 0;

    while let Some(cap) = compression_marker.captures(&input_str[input_offset ..]) {
        let (ms, me) = cap.pos(0).unwrap();
        let len: usize = cap.name("len").unwrap().parse().unwrap();
        let reps: usize = cap.name("reps").unwrap().parse().unwrap();

        siz += ms + (reps * decompress_v2(&input_str[input_offset + me .. input_offset + me + len]));

        input_offset += me + len;
    }

    siz + input_str.len() - input_offset
}

#[test]
fn test_examples_v1() {
    assert!(decompress_v1("ADVENT") == "ADVENT");
    assert!(decompress_v1("A(1x5)BC") == "ABBBBBC");
    assert!(decompress_v1("(3x3)XYZ") == "XYZXYZXYZ");
    assert!(decompress_v1("A(2x2)BCD(2x2)EFG") == "ABCBCDEFEFG");
    assert!(decompress_v1("(6x1)(1x3)A") == "(1x3)A");
    assert!(decompress_v1("X(8x2)(3x3)ABCY") == "X(3x3)ABC(3x3)ABCY");
}

#[test]
fn test_examples_v2() {
    assert!(decompress_v2("(3x3)XYZ") == 9);
    assert!(decompress_v2("X(8x2)(3x3)ABCY") == 20);
    assert!(decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A") == 241920);
    assert!(decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN") == 445);
}

fn main() {
    let compressed = args().nth(1).expect("No argument given");

    println!("{:?}", decompress_v2(&compressed));
}
