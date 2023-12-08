use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Debug,
};

fn main() {
    let sample = include_str!("../input/sample.txt");
    let sample1 = include_str!("../input/sample1.txt");
    let sample2 = include_str!("../input/sample2.txt");
    let input = include_str!("../input/input.txt");
    println!("SAMPL1: {}", follow_map(sample));
    println!("SAMPL2: {}", follow_map(sample1));
    println!("INPUT1: {}", follow_map(input));
    println!("SAMPL3: {}", follow_map_crt(sample2));
    println!("INPUT2: {}", follow_map_crt(input));
}

fn three_digit(b: &mut impl Iterator<Item = u8>) -> u32 {
    b.take(3).fold(0, |i, b| i << 8 | b as u32)
}

fn parse_direction(s: &str) -> (u32, [u32; 2]) {
    let mut b = s.bytes();
    let n = three_digit(&mut b);
    let l = three_digit(&mut (&mut b).skip(4));
    let r = three_digit(&mut b.skip(2));
    (n, [l, r])
}

fn debug_int(i: u32) -> String {
    let mut s = String::new();
    i.to_be_bytes()[1..].iter().for_each(|b| s.push(*b as char));
    s
}

#[derive(Clone)]
struct Cycle {
    index: u32,
    encountered: usize,
    len: usize,
    goals: Vec<(usize, u32)>,
}
impl Debug for Cycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cycle")
            .field("index", &debug_int(self.index))
            .field("encountered", &self.encountered)
            .field("len", &self.len)
            .field(
                "goals",
                &self
                    .goals
                    .iter()
                    .map(|(a, b)| (*a, debug_int(*b)))
                    .collect::<Vec<(usize, String)>>(),
            )
            .finish()
    }
}

impl Cycle {
    fn new(index: u32, encountered: usize, len: usize) -> Self {
        Self {
            index,
            encountered,
            len,
            goals: vec![],
        }
    }
}

fn follow_map_crt(map: &str) -> usize {
    let mut lines = map.lines();
    let dir_str = lines.next().unwrap();
    let directions = dir_str.bytes().enumerate().cycle().enumerate();
    let init = b'A' as u32;
    let (map, init) = lines
        .skip(1)
        .fold((HashMap::new(), vec![]), |(mut m, mut v), s| {
            let (k, val) = parse_direction(s);
            if (k & 0x0000FF) ^ init == 0 {
                v.push(k)
            }
            m.insert(k, val);
            (m, v)
        });
    let mut cur: Vec<Option<u32>> = init.iter().map(|i| Some(*i)).collect();
    let mut vistied: Vec<HashMap<(usize, u32), usize>> = vec![HashMap::new(); cur.len()];
    let mut cycles: Vec<Cycle> = vec![Cycle::new(0, 0, 0); cur.len()];
    for (i, (step, b)) in directions {
        let direction = if b == b'L' { 0 } else { 1 };
        for (j, u) in cur.iter_mut().enumerate() {
            if let Some(state) = u {
                match vistied[j].entry((step, *state)) {
                    Entry::Vacant(e) => {
                        e.insert(i);
                        *u = Some(map[state][direction]);
                    }
                    Entry::Occupied(e) => {
                        let encountered = *e.get();
                        let cycle = Cycle::new(*state, encountered, i - encountered);
                        cycles[j] = cycle;
                        *u = None;
                    }
                }
            }
        }
        if cur.iter().all(|o| o.is_none()) {
            break;
        }
    }
    let directions = dir_str
        .bytes()
        .cycle()
        .enumerate()
        .take(cycles.iter().map(|c| c.encountered + c.len).max().unwrap());
    let goal = b'Z' as u32;
    let mut cur = init.clone();
    for (i, b) in directions {
        let direction = if b == b'L' { 0 } else { 1 };
        for (j, c) in cur.iter_mut().enumerate() {
            if (*c & 0x0000FF) ^ goal == 0 {
                // subtract encountered from goal?
                cycles[j].goals.push((i, *c));
            }
            *c = map[c][direction];
        }
    }
    cycles
        .iter()
        .map(|c| c.goals[0])
        .fold(1, |a, b| num::integer::lcm(a, b.0))
}

fn follow_map(map: &str) -> usize {
    let mut lines = map.lines();
    let directions = lines.next().unwrap().bytes().cycle().enumerate();
    let map = lines.skip(1).fold(HashMap::new(), |mut m, s| {
        let (k, v) = parse_direction(s);
        m.insert(k, v);
        m
    });
    let mut cur = three_digit(&mut "AAA".bytes());
    let goal = three_digit(&mut "ZZZ".bytes());
    for (i, b) in directions {
        if cur == goal {
            return i;
        } else if b == b'L' {
            cur = map[&cur][0];
        } else {
            cur = map[&cur][1];
        }
    }
    panic!("UNREACHABLE");
}

#[cfg(test)]
mod test {
    use crate::three_digit;

    #[test]
    fn terminates_correctly() {
        let goal = b'Z' as u32;
        let init = b'A' as u32;
        let mut count_z = 0;
        let mut count_a = 0;
        for x in 'A'..='Z' {
            for y in 'A'..='Z' {
                for z in 'A'..='Z' {
                    let mut s = "".to_owned();
                    s.push(x);
                    s.push(y);
                    s.push(z);
                    let u = three_digit(&mut s.bytes());
                    if (u & 0x0000FF) ^ goal == 0 {
                        count_z += 1;
                    }
                    if (u & 0x0000FF) ^ init == 0 {
                        count_a += 1;
                    }
                    assert_eq!((u & 0x0000FF) ^ goal == 0, z == 'Z');
                    assert_eq!((u & 0x0000FF) ^ init == 0, z == 'A');
                }
            }
        }
        assert_eq!(count_z, 26 * 26);
        assert_eq!(count_a, 26 * 26);
    }
}
