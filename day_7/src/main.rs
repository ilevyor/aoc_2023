fn main() {
    let sample = include_str!("../input/sample.txt");
    let input = include_str!("../input/input.txt");
    println!("SAMPL1: {}", camel_cards_score(sample));
    println!("INPUT1: {}", camel_cards_score(input));
    println!("SAMPL2: {}", camel_cards_joker_score(sample));
    println!("INPUT2: {}", camel_cards_joker_score(input));
}



fn process_joker_byte(b: u8, counts: &mut[u32], max: &mut (u32, u32, u32), hand: &mut u32) {
    let i = match b {
        b'A' => 12,
        b'K' => 11,
        b'Q' => 10,
        b'T' => 9,
        b'9' => 8,
        b'8' => 7,
        b'7' => 6,
        b'6' => 5,
        b'5' => 4,
        b'4' => 3,
        b'3' => 2,
        b'2' => 1,
        b'J' => 0,
        _ => panic!("not in the cards")
    };
    if i != 0 {
        counts[i] += 1;
        if max.0 < counts[i] {
            max.0 = counts[i];
        } else {
            max.1 = u32::max(max.1, counts[i]);
        }
    } else {
        max.2 += 1
    }
    *hand <<= 4;
    *hand += i as u32;
}


fn process_byte(b: u8, counts: &mut[u32], max: &mut (u32, u32), hand: &mut u32) {
    let i = match b {
        b'A' => 12,
        b'K' => 11,
        b'Q' => 10,
        b'J' => 9,
        b'T' => 8,
        b'9' => 7,
        b'8' => 6,
        b'7' => 5,
        b'6' => 4,
        b'5' => 3,
        b'4' => 2,
        b'3' => 1,
        b'2' => 0,
        _ => panic!("not in the cards")
    };
    counts[i] += 1;
    if max.0 < counts[i] {
        max.0 = counts[i];
    } else {
        max.1 = u32::max(max.1, counts[i]);
    }
    *hand <<= 4;
    *hand += i as u32;
}

fn hand_to_u32(hand: &str) -> u32 {
    let mut counts = [0; 13];
    let mut max = (0, 0);
    let mut res = 0;
    for b in hand.bytes() {
        process_byte(b, &mut counts, &mut max, &mut res);
    }
    let rank = if max.0 > 3 {
        max.0 + 2
    } else if max.0 == 3 {
        max.0 + max.1
    } else {
        max.0 + max.1 - 1
    };
    res += rank << 20;
    res
}

fn joker_hand_to_u32(hand: &str) -> u32 {
    let mut counts = [0; 13];
    let mut max = (0, 0, 0);
    let mut res = 0;
    for b in hand.bytes() {
        process_joker_byte(b, &mut counts, &mut max, &mut res);
    }
    max.0 += max.2;
    let rank = if max.0 > 3 {
        max.0 + 2
    } else if max.0 == 3 {
        max.0 + max.1
    } else {
        max.0 + max.1 - 1
    };
    res += rank << 20;
    res
}

fn camel_cards_score(hands: &str) -> u32 {
    let mut hands: Vec<(u32, u32)> = hands.lines().map(|l| {
        let mut l = l.split_whitespace();
        let hand = hand_to_u32(l.next().unwrap());
        let score = l.next().unwrap().parse::<u32>().unwrap();
        (hand, score)
    }).collect();
    hands.sort_unstable_by_key(|k| k.0);
    hands.iter().enumerate().map(|(e, (_, s))| (e + 1) as u32 * s).sum()
}

fn camel_cards_joker_score(hands: &str) -> u32 {
    let mut hands: Vec<(u32, u32)> = hands.lines().map(|l| {
        let mut l = l.split_whitespace();
        let hand = joker_hand_to_u32(l.next().unwrap());
        let score = l.next().unwrap().parse::<u32>().unwrap();
        (hand, score)
    }).collect();
    hands.sort_unstable_by_key(|k| k.0);
    hands.iter().enumerate().map(|(e, (_, s))| (e + 1) as u32 * s).sum()
}