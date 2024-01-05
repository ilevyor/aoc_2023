use std::collections::VecDeque;

fn main() {
    let sample = include_str!("../input/sample.txt");
    let sample2 = include_str!("../input/sample2.txt");
    let input = include_str!("../input/input.txt");
    println!("SAMPL1: {}", analyze_report(sample));
    println!("INPUT1: {}", analyze_report(input));
    println!("SAMPL2: {}", analyze_report_front(sample2));
    println!("INPUT2: {}", analyze_report_front(input));
}

fn discrete_derivative(numbers: impl Iterator<Item = i32>) -> i32 {
    let mut l = VecDeque::new();
    let mut l1 = VecDeque::new();
    let mut r = Vec::new();
    for n in numbers {
        let mut n = n;
        l1.push_back(n);
        for d in l.drain(..) {
            n = d - n;
            l1.push_back(n);
        }
        std::mem::swap(&mut l, &mut l1);
        r.push(n);
    }
    r.iter().sum()
}

fn analyze_report(report: &str) -> i32 {
    report
        .lines()
        .map(|l| {
            discrete_derivative(
                l.split_whitespace()
                    .rev()
                    .map(|n| n.parse::<i32>().unwrap()),
            )
        })
        .sum()
}

fn analyze_report_front(report: &str) -> i32 {
    report
        .lines()
        .map(|l| discrete_derivative(l.split_whitespace().map(|n| n.parse::<i32>().unwrap())))
        .sum()
}
