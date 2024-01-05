use std::{collections::HashSet, hash::Hash, ops::Not};

fn main() {
    let sample = include_str!("../input/sample.txt");
    let sample2 = include_str!("../input/sample2.txt");
    let input = include_str!("../input/input.txt");
    println!("SAMPL1: {}", pipe_len(sample));
    println!("SAMPL2: {}", pipe_len(sample2));
    println!("INPUT1: {}", pipe_len(input));
    // println!("SAMPL2: {}", analyze_report_front(sample2));
    // println!("INPUT2: {}", analyze_report_front(input));
}

// |, -, L, J, 7, F
#[derive(Clone, Copy, Debug)]
enum D {
    N,
    E,
    S,
    W,
}

// I | O
// and
// I
// -
// O
#[derive(Clone, Copy)]
enum Side {
    Inside,
    Outside,
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Inside => Side::Outside,
            Side::Outside => Side::Inside,
        }
    }
}

fn next_side(side: Side, b: u8, nb: u8) -> Side {
    match (b, nb) {
        (b'L', b'J')
        | (b'J', b'L')
        | (b'L', b'F')
        | (b'F', b'L')
        | (b'7', b'J')
        | (b'J', b'7')
        | (b'7', b'F')
        | (b'F', b'7')
        | (b'|', b'|')
        | (b'-', b'-')
        | (b'|', b'7')
        | (b'7', b'|')
        | (b'|', b'J')
        | (b'J', b'|')
        | (b'-', b'L')
        | (b'L', b'-')
        | (b'-', b'J')
        | (b'J', b'-') => side,
        (b'7', b'L')
        | (b'L', b'7')
        | (b'J', b'F')
        | (b'F', b'J')
        | (b'|', b'L')
        | (b'L', b'|')
        | (b'|', b'F')
        | (b'F', b'|')
        | (b'-', b'F')
        | (b'F', b'-')
        | (b'-', b'7')
        | (b'7', b'-') => !side,
        (_, b'S') => side,
        _ => panic!("INVALID CONNECTION"),
    }
}

// remember to sub 1 from rows and cols
fn add_to_interior(
    byte: u8,
    side: Side,
    interior: &mut HashSet<(usize, usize)>,
    (i, j): (usize, usize),
    rows: usize,
    cols: usize,
) {
    match (byte, side) {
        (b'|', Side::Inside) => {
            if j > 0 {
                interior.insert((i, j - 1));
                if i > 0 {
                    interior.insert((i - 1, j - 1));
                }
                if i < rows {
                    interior.insert((i + 1, j - 1));
                }
            }
        }
        (b'|', Side::Outside) => {
            if j < cols {
                interior.insert((i, j + 1));
                if i > 0 {
                    interior.insert((i - 1, j + 1));
                }
                if i < rows {
                    interior.insert((i + 1, j + 1));
                }
            }
        }
        (b'-', Side::Inside) => {
            if i > 0 {
                interior.insert((i - 1, j));
                if j > 0 {
                    interior.insert((i - 1, j - 1));
                }
                if j < cols {
                    interior.insert((i - 1, j + 1));
                }
            }
        }
        (b'-', Side::Outside) => {
            if i < rows {
                interior.insert((i + 1, j));
                if j > 0 {
                    interior.insert((i - 1, j - 1));
                }
                if j < cols {
                    interior.insert((i - 1, j + 1));
                }
            }
        }
        (b'7', Side::Inside) => {
            if i < rows && j > 0 {
                interior.insert((i + 1, j - 1));
            }
        }
        (b'7', Side::Outside) => {
            if i > 0 {
                interior.insert((i - 1, j));
            }
            if j < cols {
                interior.insert((i, j + 1));
            }
            if i > 0 && j < cols {
                interior.insert((i - 1, j + 1));
            }
        }
        (b'F', Side::Inside) => {
            if i < rows && j < cols {
                interior.insert((i + 1, j + 1));
            }
        }
        (b'F', Side::Outside) => {
            if i > 0 {
                interior.insert((i - 1, j));
            }
            if j > 0 {
                interior.insert((i, j - 1));
            }
            if i > 0 && j > 0 {
                interior.insert((i - 1, j - 1));
            }
        }
        (b'J', Side::Inside) => {
            if i > 0 && j > 0 {
                interior.insert((i - 1, j - 1));
            }
        }
        (b'J', Side::Outside) => {
            if i < rows {
                interior.insert((i + 1, j));
            }
            if j < cols {
                interior.insert((i, j + 1));
            }
            if i < rows && j < cols {
                interior.insert((i + 1, j + 1));
            }
        }
        (b'L', Side::Inside) => {
            if i > 0 && j < rows {
                interior.insert((i - 1, j + 1));
            }
        }
        (b'L', Side::Outside) => {
            if i < rows {
                interior.insert((i + 1, j));
            }
            if j > 0 {
                interior.insert((i, j - 1));
            }
            if i < rows && j > 0 {
                interior.insert((i + 1, j - 1));
            }
        }
        _ => panic!("INVALID PIPE"),
    }
}

fn next(
    (byte, rows, cols): (u8, usize, usize),
    (i, j): (&mut usize, &mut usize),
    p: &mut D,
    ct: &mut usize,
    visited: &mut HashSet<(usize, usize)>,
    interior: &mut HashSet<(usize, usize)>,
    map: &[&[u8]],
    side: &mut Side,
) -> bool {
    visited.insert((*i, *j));
    if byte == b'S' {
        return false;
    }
    add_to_interior(byte, *side, interior, (*i, *j), rows - 1, cols - 1);
    match (*p, byte) {
        (D::N, b'|') | (D::W, b'7') | (D::E, b'F') => {
            *p = D::N;
            *i += 1;
            if *i >= rows {
                return false;
            }
        }
        (D::E, b'-') | (D::S, b'7') | (D::N, b'J') => {
            *p = D::E;
            if *j == 0 {
                return false;
            }
            *j -= 1;
        }
        (D::S, b'|') | (D::E, b'L') | (D::W, b'J') => {
            *p = D::S;
            if *i == 0 {
                return false;
            }
            *i -= 1
        }
        (D::W, b'-') | (D::N, b'L') | (D::S, b'F') => {
            *p = D::W;
            *j += 1;
            if *j >= cols {
                return false;
            }
        }
        _ => return false,
    }
    *ct += 1;
    *side = next_side(*side, byte, map[*i][*j]);
    true
}

fn try_path(
    init_side: Side,
    from: D,
    b: u8,
    (si, sj): (usize, usize),
    rows: usize,
    cols: usize,
    map: &[&[u8]],
) -> Option<(usize, D, u8, HashSet<(usize, usize)>)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut interior: HashSet<(usize, usize)> = HashSet::new();
    let mut side = init_side;
    let mut p = from;
    let mut count = 0;
    let mut i = si;
    let mut j = sj;
    if !next(
        (b, rows, cols),
        (&mut i, &mut j),
        &mut p,
        &mut count,
        &mut visited,
        &mut interior,
        map,
        &mut side,
    ) {
        return None;
    }
    while next(
        (map[i][j], rows, cols),
        (&mut i, &mut j),
        &mut p,
        &mut count,
        &mut visited,
        &mut interior,
        map,
        &mut side,
    ) {}
    if map[i][j] == b'S' {
        let b = match (from, p) {
            (D::N, D::N) | (D::S, D::S) => b'|',
            (D::E, D::E) | (D::W, D::W) => b'-',
            (D::N, D::E) | (D::W, D::S) => b'F',
            (D::S, D::W) | (D::E, D::N) => b'J',
            (D::N, D::W) | (D::E, D::S) => b'7',
            (D::S, D::E) | (D::W, D::N) => b'L',
            _ => panic!("INVALID PIPE"),
        };
        Some((count, from, b, visited))
    } else {
        None
    }
}

fn dfs(
    mut start: Vec<(usize, usize)>,
    pipes: &HashSet<(usize, usize)>,
    max_row: usize,
    max_col: usize,
) -> HashSet<(usize, usize)> {
    let visited: HashSet<(usize, usize)> = HashSet::new();
    while !start.is_empty() {
        let (r, c) = start.pop().unwrap();
        visited.insert((r, c));
        if r > 0 {}
        if c > 0 {}
        if r < max_row {}
        if c < max_col {}
        start.extend([(r + 1, c), (r, c + 1)])
    }
    todo!();
}

fn find_path(
    map: &[&[u8]],
    (si, sj): (usize, usize),
    init_side: Side,
) -> (usize, D, u8, HashSet<(usize, usize)>) {
    let rows = map.len();
    let cols = map[0].len();
    for (b, from) in [(b'|', D::N), (b'-', D::E), (b'|', D::S), (b'-', D::W)] {
        if let Some(res) = try_path(init_side, from, b, (si, sj), rows, cols, map) {
            return res;
        }
    }
    panic!("NO PATH")
}

fn start(map: &[&[u8]]) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(r, l)| l.iter().position(|b| *b == b'S').map(|c| (r, c)))
        .unwrap()
}

fn pipe_len(map: &str) -> usize {
    let map: Vec<&[u8]> = map.lines().map(|s| s.as_bytes()).collect();
    let (ct, d, b, v) = find_path(&map, start(&map), Side::Inside);
    println!("V: {}", v.len() / 2);
    println!("{:?}, {}", d, b as char);
    ct / 2
}

fn pipe_area(map: &str) -> usize {
    let map: Vec<&[u8]> = map.lines().map(|s| s.as_bytes()).collect();
    let (si, sj) = start(&map);
    let (ct, _, sb, v) = find_path(&map, (si, sj), Side::Inside);
    // let v = interior_edge(&map, sb, v);
    ct / 2
}
