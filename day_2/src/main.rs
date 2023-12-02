use rayon::prelude::{ParallelBridge, ParallelIterator};


fn main() {
    let sample = include_str!("../input/sample.txt");
    let input = include_str!("../input/input.txt");
    let _cubes = RGB{r: 12, g: 13, b: 14};
    println!("SAMPLE: {}", min_games(sample));
    println!("INPUT: {}", min_games(input));
}

struct RGB {
    r: u32,
    g: u32,
    b: u32,
}

fn min_games(s: &str) -> u32 {
    s.lines().par_bridge().map(|l| {
        let l = l.split(": ").skip(1).next().unwrap();
        let mut a = RGB{r: 0, g: 0, b: 0 };
        l.split("; ").flat_map(|g| g.split(", ")).for_each(|snip| {
            let mut snip = snip.split(' '); 
            let ct = snip.next().unwrap().parse::<u32>().unwrap();
            match snip.next().unwrap().bytes().next().unwrap() {
                b'r' => a.r = u32::max(a.r, ct),
                b'g' => a.g = u32::max(a.g, ct),
                b'b' => a.b = u32::max(a.b, ct),
                _ => panic!("NOT A COLOR")
            };
        });
        a.r * a.g * a.b
    }).sum()
}

fn _possible_games(cubes: &RGB, s: &str) -> usize {
    s.lines().enumerate().par_bridge().filter_map(|(i, l)| {
        let l = l.split(": ").skip(1).next().unwrap();
        if l.split("; ").flat_map(|g| g.split(", ")).all(|snip| {
            let mut snip = snip.split(' '); 
            let ct = snip.next().unwrap().parse::<u32>().unwrap();
            match snip.next().unwrap().bytes().next().unwrap() {
                b'r' => ct <= cubes.r,
                b'g' => ct <= cubes.g,
                b'b' => ct <= cubes.b,
                _ => panic!("NOT A COLOR")
            }
        }) {
            Some(i + 1)
        } else {
            None
        }
    }).sum()
}