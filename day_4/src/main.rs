use std::{collections::VecDeque, iter::repeat};

use rayon::prelude::{ParallelBridge, ParallelIterator};

fn main() {
    let sample = include_str!("../input/sample.txt");
    let input = include_str!("../input/input.txt");
    println!("SAMPL1: {}", lottery_winnings(sample));
    println!("INPUT1: {}", lottery_winnings(input));
    println!("SAMPL2: {}", card_copies(sample));
    println!("INPUT2: {}", card_copies(input));
}

struct OrderedTraverse<T: Ord + Copy> {
    unique: Vec<T>,
    repeated: Vec<T>,
    i: usize,
    j: usize,
}

impl<T: Ord + Copy> Iterator for OrderedTraverse<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.unique.len() || self.j >= self.repeated.len() {
            None
        } else {
            let (a, b) = (self.unique[self.i], self.repeated[self.j]);
            if a < b {
                self.i += 1;
            } else {
                self.j += 1;
            }
            Some((a, b))
        }
    }
}

impl<T: Ord + Copy> OrderedTraverse<T> {
    fn new(mut unique: Vec<T>, mut repeated: Vec<T>) -> Self {
        unique.sort_unstable();
        repeated.sort_unstable();
        Self {
            unique,
            repeated,
            i: 0,
            j: 0,
        }
    }
}

fn parse_list(s: &str) -> Vec<u32> {
    let mut v = s
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    v.sort_unstable();
    v
}

fn push_copies(v: &mut VecDeque<u32>, copies: u32, count: usize) {
    v.extend(repeat(0).take(count.checked_sub(v.len()).unwrap_or_default()));
    v.iter_mut().take(count).for_each(|e| *e += copies);
}

fn card_copies(cards: &str) -> u32 {
    cards
        .lines()
        .map(|l| {
            let mut sides = l.split(':').skip(1).next().unwrap().split('|');
            let win = parse_list(sides.next().unwrap());
            let nums = parse_list(sides.next().unwrap());
            OrderedTraverse::new(win, nums)
                .filter(|(a, b)| a == b)
                .count()
        })
        .fold((VecDeque::<u32>::new(), 0), |(mut d, t), ct| {
            let copies = d.pop_front().unwrap_or_default() + 1;
            push_copies(&mut d, copies, ct);
            (d, t + copies)
        })
        .1
}

fn lottery_winnings(cards: &str) -> u32 {
    cards
        .lines()
        .par_bridge()
        .map(|l| {
            let mut sides = l.split(':').skip(1).next().unwrap().split('|');
            let win = parse_list(sides.next().unwrap());
            let nums = parse_list(sides.next().unwrap());
            OrderedTraverse::new(win, nums)
                .filter(|(a, b)| a == b)
                .fold(1, |e, _| e * 2)
                / 2
        })
        .sum()
}
