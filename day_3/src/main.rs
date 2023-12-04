use itertools::{izip, Itertools};
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;

fn main() {
    let sample = include_str!("../input/sample.txt");
    let input = include_str!("../input/input.txt");
    println!("SAMPL1: {}", gear_parts(sample));
    println!("INPUT1: {}", gear_parts(input));
    println!("SAMPL2: {}", gear_ratio(sample));
    println!("INPUT2: {}", gear_ratio(input));
}

fn is_digit(b: u8) -> bool {
    b'0' <= b && b <= b'9'
}

fn is_symbol(b: u8) -> bool {
    !is_digit(b) && b != b'.'
}

fn adj_part_num(
    (sum, cur, adj): (u32, u32, bool),
    ((a, b, c), (d, e, f), (g, h, i)): ((u8, u8, u8), (u8, u8, u8), (u8, u8, u8)),
) -> (u32, u32, bool) {
    if b'0' <= e && e <= b'9' {
        let cur = cur * 10 + (e - b'0') as u32;
        let adj = adj
            || is_symbol(a)
            || is_symbol(b)
            || is_symbol(c)
            || is_symbol(d)
            || is_symbol(f)
            || is_symbol(g)
            || is_symbol(h)
            || is_symbol(i);
        (sum, cur, adj)
    } else {
        let sum = if adj { sum + cur } else { sum };
        (sum, 0, false)
    }
}

fn update_row(v: &mut Vec<u32>, b: u8) {
    if is_digit(b) {
        let t = v.last_mut().unwrap();
        *t *= 10;
        *t += (b - b'0') as u32;
    } else if *v.last().unwrap() != 0 {
        v.push(0)
    }
}

fn update_ratios(
    ratios: &mut Vec<(usize, usize)>,
    row: usize,
    len: usize,
    (a, b, c): (u8, u8, u8),
) {
    if is_digit(b) {
        ratios.push((row, len - 1))
    } else {
        if is_digit(a) {
            ratios.push((row, len - 2))
        }
        if is_digit(c) {
            ratios.push((row, len - 1))
        }
    }
}

fn adj_gear_ratio(
    top: &mut Vec<u32>,
    mid: &mut Vec<u32>,
    bot: &mut Vec<u32>,
    gear: &mut Vec<((usize, usize), (usize, usize))>,
    ((a, b, c), (d, e, f), (g, h, i)): ((u8, u8, u8), (u8, u8, u8), (u8, u8, u8)),
) {
    update_row(top, d);
    update_row(mid, e);
    update_row(bot, f);
    if e == b'*' {
        let mut ratios = vec![];
        update_ratios(&mut ratios, 0, top.len(), (a, d, g));
        update_ratios(&mut ratios, 1, mid.len(), (b, e, h));
        update_ratios(&mut ratios, 2, bot.len(), (c, f, i));
        if ratios.len() == 2 {
            gear.push((ratios[0], ratios[1]))
        }
    }
}

fn gear_ratio(schematic: &str) -> u32 {
    // is it possible to get the types for this to work?
    // Some(std::iter::repeat(b'.')).iter()
    let line_len = schematic.lines().next().unwrap().len();
    let line = (0..line_len).map(|_| '.').collect::<String>();
    let lines = [line.as_str()]
        .into_iter()
        .chain(schematic.lines())
        .chain([line.as_str()].into_iter());
    lines
        .map(|l| {
            [b'.']
                .into_iter()
                .chain(l.bytes())
                .chain([b'.', b'.'].into_iter())
        })
        .tuple_windows::<(_, _, _)>()
        .par_bridge()
        .map(|(a, b, c)| {
            let mut top = vec![0];
            let mut mid = vec![0];
            let mut bot = vec![0];
            let mut gear = vec![];
            izip!(a, b, c)
                .tuple_windows::<(_, _, _)>()
                .for_each(|t| adj_gear_ratio(&mut top, &mut mid, &mut bot, &mut gear, t));
            let nums = [top, mid, bot];
            gear.into_iter()
                .map(|((i1, j1), (i2, j2))| nums[i1][j1] * nums[i2][j2])
                .sum::<u32>()
        })
        .sum()
}

fn gear_parts(schematic: &str) -> u32 {
    // is it possible to get the types for this to work?
    // Some(std::iter::repeat(b'.')).iter()
    let line_len = schematic.lines().next().unwrap().len();
    let line = (0..line_len).map(|_| '.').collect::<String>();
    let lines = [line.as_str()]
        .into_iter()
        .chain(schematic.lines())
        .chain([line.as_str()].into_iter());
    lines
        .map(|l| {
            [b'.']
                .into_iter()
                .chain(l.bytes())
                .chain([b'.', b'.'].into_iter())
        })
        .tuple_windows::<(_, _, _)>()
        .par_bridge()
        .map(|(a, b, c)| {
            izip!(a, b, c)
                .tuple_windows::<(_, _, _)>()
                .fold((0, 0, false), adj_part_num)
        })
        .map(|(s, _, _)| s)
        .sum()
}
