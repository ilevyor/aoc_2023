use rayon::prelude::{ParallelBridge, ParallelIterator};
use regex::Regex;

fn main() {
    let sample = include_str!("../input/sample.txt");
    // let sample2 = include_str!("../input/sample2.txt");
    let input = include_str!("../input/input.txt");
    println!("SAMPLE: {}", digit_or_spelled(sample));
    // println!("SAMPLE2: {}", digit_or_spelled(sample2));
    println!("INPUT: {}", digit_or_spelled(input));
}

fn string_to_digit(s: &str) -> u32 {
    match s {
        "1" | "one" | "eno" => 1,
        "2" | "two" | "owt" => 2,
        "3" | "three" | "eerht" => 3,
        "4" | "four" | "ruof" => 4,
        "5" | "five" | "evif" => 5,
        "6" | "six" | "xis" => 6,
        "7" | "seven" | "neves" => 7,
        "8" | "eight" | "thgie" => 8,
        "9" | "nine" | "enin" => 9,
        _ => panic!("invalid string"),
    }
}

fn digit_or_spelled(input: &str) -> u32 {
    let regex = Regex::new("one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9").unwrap();
    let revex = Regex::new("eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|1|2|3|4|5|6|7|8|9").unwrap();
    // let regex = Regex::new("1|2|3|4|5|6|7|8|9").unwrap();
    input.lines().par_bridge().map(|s| {
        let rev = s.chars().rev().collect::<String>();
        let first = string_to_digit(regex.find(s).unwrap().into());
        let last = string_to_digit(revex.find(&rev).unwrap().into());
        (first * 10 + last) as u32
    }).sum::<u32>()
}

fn _add_digits(input: &str) -> u32 {
    input.lines().par_bridge().map(|s| {
        let first = s.bytes().find(|b| *b > b'0' &&  *b <= b'9').unwrap() - b'0';
        let last = s.bytes().find(|b| *b > b'0' &&  *b <= b'9').unwrap() - b'0';
        (first * 10 + last) as u32
    }).sum::<u32>()
}