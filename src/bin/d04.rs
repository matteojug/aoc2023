use std::collections::HashSet;

aoc::main!();
aoc::check!("inputs/4.in", 23678, 15455663);

fn parse_line(input: &str) -> HashSet<u32> {
    input
        .split(" ")
        .filter_map(|x| x.trim().parse().ok())
        .collect()
}

fn load_input(raw: String) -> String {
    raw
}

fn part_1(input: &String) -> i32 {
    input
        .lines()
        .map(|line| {
            let (winning, ours) = line.split_once(":").unwrap().1.split_once("|").unwrap();
            let winning = parse_line(winning);
            let ours = parse_line(ours);
            1 << winning.intersection(&ours).count() >> 1
        })
        .sum::<i32>()
}

fn part_2(input: &String) -> usize {
    let scores: Vec<usize> = input
        .lines()
        .map(|line| {
            let (winning, ours) = line.split_once(":").unwrap().1.split_once("|").unwrap();
            let winning = parse_line(winning);
            let ours = parse_line(ours);
            winning.intersection(&ours).count()
        })
        .collect();
    let mut count: Vec<usize> = scores.iter().map(|_| 1).collect();
    scores
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != 0)
        .for_each(|(i, x)| (1..=*x).for_each(|j| count[i + j] += count[i]));
    count.iter().sum::<usize>()
}
