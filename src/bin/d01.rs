use aoc;
use std::collections::HashMap;

aoc::main!();
aoc::check!("inputs/1.in", 55538, 54875);

const DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn load_input(raw: String) -> String {
    raw
}

fn part_1(input: &String) -> u32 {
    input
        .lines()
        .map(|line| {
            let n = format!(
                "{}{}",
                line.chars().filter(|c| c.is_ascii_digit()).next().unwrap(),
                line.chars()
                    .filter(|c| c.is_ascii_digit())
                    .next_back()
                    .unwrap()
            );
            n.parse::<u32>().unwrap()
        })
        .sum()
}

fn part_2(input: &String) -> u32 {
    let neddles: HashMap<String, u32> = DIGITS
        .iter()
        .enumerate()
        .map(|(i, d)| (String::from(*d), (i + 1) as u32))
        .chain((0..=9).map(|d| (d.to_string(), d)))
        .collect();

    input
        .lines()
        .map(|line| {
            let first = neddles
                .iter()
                .map(|(d, val)| (line.find(d), val))
                .filter(|x| x.0.is_some())
                .min()
                .unwrap()
                .1;
            let last = neddles
                .iter()
                .map(|(d, val)| (line.rfind(d), val))
                .filter(|x| x.0.is_some())
                .max()
                .unwrap()
                .1;

            first * 10 + last
        })
        .sum()
}
