fn main() {
    let sample = include_str!("../input/sample.txt");
    let input = include_str!("../input/input.txt");
    println!("SAMPL1: {}", boat_race_wins(sample));
    println!("INPUT1: {}", boat_race_wins(input));
    println!("SAMPL2: {}", boat_race_win(sample));
    println!("INPUT2: {}", boat_race_win(input));
}

fn distance(n: u64, t: u64) -> u64 {
    n * t - n * n
}

fn count_seconds((t, d): (u64, u64)) -> u64 {
    let disc_2 = (t * t - 4 * d) as f64;
    let disc = disc_2.sqrt();
    let mut l = (((t as f64) - disc) / 2.0) as u64;
    let mut r = (((t as f64) + disc) / 2.0) as u64;
    if distance(l, t) <= d {
        l += 1
    }
    if distance(l, t) <= d {
        l += 1
    }

    if distance(r, t) <= d {
        r -= 1
    }
    (r + 1).checked_sub(l).unwrap_or_default()
}

fn boat_race_win(races: &str) -> u64 {
    let mut races = races.lines();
    let time = races
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = races
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    count_seconds((time, distance))
}

fn boat_race_wins(races: &str) -> u64 {
    let mut races = races.lines();
    let times = races
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap());
    let distances = races
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap());
    times.zip(distances).map(count_seconds).product()
}
