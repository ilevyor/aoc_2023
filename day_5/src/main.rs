use itertools::Itertools;

fn main() {
    let sample = include_str!("../input/sample.txt");
    let input = include_str!("../input/input.txt");
    println!("SAMPL1: {}", seed_map(sample));
    println!("INPUT1: {}", seed_map(input));
    println!("SAMPL2: {}", seed_range_map(sample));
    println!("INPUT2: {}", seed_range_map(input));
}

// todo make utils crate:
fn parse_list(s: &str) -> Vec<usize> {
    s.split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn drain_range(
    source: &mut Vec<(usize, usize)>,
    dest: &mut Vec<(usize, usize)>,
    index: usize,
    i: usize,
    l: usize,
    d: usize,
) -> bool {
    let (i1, l1) = source[index];
    if i <= i1 && (i1 + l1) <= (i + l) {
        dest.push((i1 + d - i, l1));
        source.remove(index);
        true
    } else if i1 < i && (i + l) < (i1 + l1) {
        source[index].1 = i - i1;
        source.insert(index + 1, (i + l, (i1 + l1) - (i + l)));
        dest.push((d, l));
        false
    } else if i <= i1 && (i + l) < (i1 + l1) {
        source[index] = (i + l, l1 - (i + l - i1));
        dest.push((i1 + d - i, i + l - i1));
        false
    } else if i1 < i && (i1 + l1) <= (i + l) {
        source[index].1 = i - i1;
        dest.push((d, i1 + l1 - i));
        false
    } else {
        panic!("UNREACHABLE?")
    }
}

fn drain_ranges(
    source: &mut Vec<(usize, usize)>,
    dest: &mut Vec<(usize, usize)>,
    index: usize,
    length: usize,
    new_index: usize,
) {
    let mut range_finder = source
        .iter()
        .enumerate()
        .skip_while(|(_, seed)| seed.0 + seed.1 < index)
        .peekable();
    let mut l = if let Some((l, _)) = range_finder.peek() {
        *l
    } else {
        return;
    };
    let mut r = if let Some((r, _s)) = range_finder
        .skip_while(|(_, seed)| seed.0 <= index + length)
        .next()
    {
        if l == r {
            return;
        }
        r - 1
    } else {
        source.len() - 1
    };
    if l == r {
        drain_range(source, dest, l, index, length, new_index);
    } else {
        drain_range(source, dest, r, index, length, new_index);
        if drain_range(source, dest, l, index, length, new_index) {
            r -= 1
        } else {
            l += 1
        }
    }
    dest.extend(source.drain(l..r).map(|(a, b)| (a + new_index - index, b)));
}

fn seed_range_map(map: &str) -> usize {
    let mut lines = map.lines();
    let seeds = parse_list(lines.next().unwrap().split(":").nth(1).unwrap());
    let mut seeds: Vec<(usize, usize)> = seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut i| (*i.next().unwrap(), *i.next().unwrap()))
        .collect_vec();
    seeds.sort_unstable_by_key(|t| t.0);
    let len = seeds.len();
    let steps = lines
        .filter(|l| !l.is_empty())
        .group_by(|l| l.ends_with(":"));
    steps
        .into_iter()
        .skip(1)
        .step_by(2)
        .fold(seeds, |cur, ranges| {
            let (old, mut new) =
                ranges
                    .1
                    .fold((cur, Vec::with_capacity(len)), |(mut init, mut res), s| {
                        let s = parse_list(s);
                        // I think this is less performant, would need to test
                        //let i = init.binary_search(&s[1]).unwrap_or_else(|e| e);
                        drain_ranges(&mut init, &mut res, s[1], s[2], s[0]);
                        (init, res)
                    });
            new.extend(old);
            new.sort_unstable_by_key(|t| t.0);

            let mut cur = 0;
            let mut rhs = new[0].0 + new[0].1;
            for i in 1..new.len() {
                if rhs >= new[i].0 {
                    rhs = usize::max(new[i].0 + new[i].1, rhs);
                } else {
                    new[cur].1 = rhs - new[cur].0;
                    rhs = new[i].0 + new[i].1;
                    cur += 1;
                    new[cur] = new[i]
                }
            }
            new[cur].1 = rhs - new[cur].0;
            new.drain(cur + 1..);
            new
        })[0]
        .0
}

fn seed_map(map: &str) -> usize {
    let mut lines = map.lines();
    let mut seeds = parse_list(lines.next().unwrap().split(":").nth(1).unwrap());
    seeds.sort_unstable();
    let len = seeds.len();
    let steps = lines
        .filter(|l| !l.is_empty())
        .group_by(|l| l.ends_with(":"));
    steps
        .into_iter()
        .skip(1)
        .step_by(2)
        .fold(seeds, |cur, ranges| {
            let (old, mut new) =
                ranges
                    .1
                    .fold((cur, Vec::with_capacity(len)), |(mut init, mut res), s| {
                        let s = parse_list(s);
                        // I think this is less performant, would need to test
                        //let i = init.binary_search(&s[1]).unwrap_or_else(|e| e);
                        let mut range_finder = init
                            .iter()
                            .enumerate()
                            .skip_while(|(_, seed)| **seed < s[1])
                            .peekable();
                        let l = if let Some((l, _)) = range_finder.peek() {
                            *l
                        } else {
                            return (init, res);
                        };
                        let r = if let Some((r, _)) = range_finder
                            .skip_while(|(_, seed)| **seed <= s[1] + s[2])
                            .next()
                        {
                            r
                        } else {
                            init.len()
                        };
                        res.extend(init.drain(l..r).map(|e| e + s[0] - s[1]));
                        (init, res)
                    });
            new.extend(old);
            new.sort_unstable();
            new
        })[0]
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::drain_range;

    #[test]
    fn drain_entire() {
        let mut source = vec![(5, 7), (10, 6), (20, 5)];
        let mut dest = vec![];
        drain_range(&mut source, &mut dest, 2, 19, 6, 10);
        drain_range(&mut source, &mut dest, 1, 9, 7, 5);
        drain_range(&mut source, &mut dest, 0, 5, 7, 0);
        assert_eq!(source, vec![]);
        assert_eq!(dest, vec![(11, 5), (6, 6), (0, 7)]);
    }

    #[test]
    fn drain_middle() {
        let mut source = vec![(10, 6)];
        let mut dest = vec![];
        drain_range(&mut source, &mut dest, 0, 13, 2, 10);
        assert_eq!(source, vec![(10, 3), (15, 1)]);
        assert_eq!(dest, vec![(10, 2)]);
    }

    #[test]
    fn drain_left() {
        let mut source = vec![(7, 10)];
        let mut dest = vec![];
        drain_range(&mut source, &mut dest, 0, 5, 2, 10);
        assert_eq!(source, vec![(7, 10)]);
        assert_eq!(dest, vec![]);
        drain_range(&mut source, &mut dest, 0, 5, 7, 11);
        assert_eq!(source, vec![(12, 5)]);
        assert_eq!(dest, vec![(13, 5)]);
    }

    #[test]
    fn drain_right() {
        let mut source = vec![(7, 10)];
        let mut dest = vec![];
        drain_range(&mut source, &mut dest, 0, 9, 13, 10);
        assert_eq!(source, vec![(7, 2)]);
        assert_eq!(dest, vec![(10, 8)]);
    }
}
