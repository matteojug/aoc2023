use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;

aoc::main!();
aoc::check!("inputs/7.in", 250254244, 250087440);

fn part_1(input: &Input) -> u32 {
    let card_value: HashMap<_, _> = "AKQJT98765432"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect();
    let mut h: Vec<usize> = (0..input.hands.len()).collect();
    h.sort_by(|a, b| {
        let a = &input.hands[*a];
        let b = &input.hands[*b];
        if a.1.value() > b.1.value() {
            return Ordering::Greater;
        } else if a.1.value() < b.1.value() {
            return Ordering::Less;
        }
        for (aa, bb) in a.0.chars().zip(b.0.chars()) {
            if card_value[&aa] < card_value[&bb] {
                return Ordering::Greater;
            } else if card_value[&aa] > card_value[&bb] {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    });
    h.iter()
        .enumerate()
        .map(|(i, idx)| (i + 1) as u32 * input.hands[*idx].3)
        .sum::<u32>()
}

fn part_2(input: &Input) -> u32 {
    let card_value: HashMap<_, _> = "AKQT98765432J"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect();
    let mut h: Vec<usize> = (0..input.hands.len()).collect();
    h.sort_by(|a, b| {
        let a = &input.hands[*a];
        let b = &input.hands[*b];
        if a.2.value() > b.2.value() {
            return Ordering::Greater;
        } else if a.2.value() < b.2.value() {
            return Ordering::Less;
        }
        for (aa, bb) in a.0.chars().zip(b.0.chars()) {
            if card_value[&aa] < card_value[&bb] {
                return Ordering::Greater;
            } else if card_value[&aa] > card_value[&bb] {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    });
    h.iter()
        .enumerate()
        .map(|(i, idx)| (i + 1) as u32 * input.hands[*idx].3)
        .sum::<u32>()
}

enum HandKind {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    OnePair,
    High,
}

impl HandKind {
    fn value(&self) -> u32 {
        match self {
            Self::High => 0,
            Self::OnePair => 1,
            Self::TwoPair => 2,
            Self::Three => 3,
            Self::Full => 4,
            Self::Four => 5,
            Self::Five => 6,
        }
    }

    fn from(s: &str) -> Self {
        let mut h = HashMap::new();
        for c in s.chars() {
            *h.entry(c).or_insert(0) += 1
        }
        let mut v: Vec<u32> = h.values().copied().collect();
        v.sort_by_key(|x| Reverse(*x));
        if v[0] == 5 {
            return Self::Five;
        } else if v[0] == 4 {
            return Self::Four;
        } else if v[0] == 1 {
            return Self::High;
        }
        if v.len() > 1 {
            if v[0] == 3 && v[1] == 2 {
                return Self::Full;
            } else if v[0] == 2 && v[1] == 2 {
                return Self::TwoPair;
            }
        }
        if v[0] == 3 {
            Self::Three
        } else if v[0] == 2 {
            Self::OnePair
        } else {
            panic!();
        }
    }

    fn fromj(s: &str) -> Self {
        let mut h = HashMap::new();
        for c in s.chars() {
            *h.entry(c).or_insert(0) += 1
        }
        let j = h.remove(&'J').unwrap_or(0);
        let mut v: Vec<u32> = h.values().copied().collect();
        v.push(0);
        v.sort_by_key(|x| Reverse(*x));
        v[0] += j;
        if v[0] == 5 {
            return Self::Five;
        } else if v[0] == 4 {
            return Self::Four;
        } else if v[0] == 1 {
            return Self::High;
        }
        if v.len() > 1 {
            if v[0] == 3 && v[1] == 2 {
                return Self::Full;
            } else if v[0] == 2 && v[1] == 2 {
                return Self::TwoPair;
            }
        }
        if v[0] == 3 {
            Self::Three
        } else if v[0] == 2 {
            Self::OnePair
        } else {
            panic!();
        }
    }
}

struct Input {
    hands: Vec<(String, HandKind, HandKind, u32)>,
}

fn load_input(raw: String) -> Input {
    Input {
        hands: raw
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();
                (
                    String::from(hand),
                    HandKind::from(hand),
                    HandKind::fromj(hand),
                    bid.parse().unwrap(),
                )
            })
            .collect(),
    }
}
