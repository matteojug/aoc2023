use std::collections::{BinaryHeap, HashSet};

aoc::main!();
aoc::check!("inputs/17.in", 742, 918);

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn part_1(input: &Input) -> i32 {
    let mut q = BinaryHeap::new();
    let mut seen = HashSet::new();
    q.push((0i32, 0usize, 0usize, 0, Dir::RIGHT));
    seen.insert((0usize, 0usize, 0, Dir::RIGHT));
    let h = input.map.len();
    let w = input.map[0].len();
    while let Some((cost, i, j, cnt, dir)) = q.pop() {
        if i == h - 1 && j == w - 1 {
            return -cost;
        }
        if dir != Dir::DOWN && i > 0 && (dir != Dir::UP || cnt < 3) {
            let s = (i - 1, j, 1 + if dir == Dir::UP { cnt } else { 0 }, Dir::UP);
            if !seen.contains(&s) {
                seen.insert(s.clone());
                q.push((cost - (input.map[s.0][s.1] as i32), s.0, s.1, s.2, s.3));
            }
        }
        if dir != Dir::UP && i < h - 1 && (dir != Dir::DOWN || cnt < 3) {
            let s = (
                i + 1,
                j,
                1 + if dir == Dir::DOWN { cnt } else { 0 },
                Dir::DOWN,
            );
            if !seen.contains(&s) {
                seen.insert(s.clone());
                q.push((cost - (input.map[s.0][s.1] as i32), s.0, s.1, s.2, s.3));
            }
        }
        if dir != Dir::RIGHT && j > 0 && (dir != Dir::LEFT || cnt < 3) {
            let s = (
                i,
                j - 1,
                1 + if dir == Dir::LEFT { cnt } else { 0 },
                Dir::LEFT,
            );
            if !seen.contains(&s) {
                seen.insert(s.clone());
                q.push((cost - (input.map[s.0][s.1] as i32), s.0, s.1, s.2, s.3));
            }
        }
        if dir != Dir::LEFT && j < w - 1 && (dir != Dir::RIGHT || cnt < 3) {
            let s = (
                i,
                j + 1,
                1 + if dir == Dir::RIGHT { cnt } else { 0 },
                Dir::RIGHT,
            );
            if !seen.contains(&s) {
                seen.insert(s.clone());
                q.push((cost - (input.map[s.0][s.1] as i32), s.0, s.1, s.2, s.3));
            }
        }
    }
    panic!()
}

fn part_2(input: &Input) -> i32 {
    let mut q = BinaryHeap::new();
    let mut seen = HashSet::new();
    q.push((0i32, 0usize, 0usize, 0, Dir::RIGHT));
    seen.insert((0usize, 0usize, 0, Dir::RIGHT));
    q.push((0i32, 0usize, 0usize, 0, Dir::DOWN));
    seen.insert((0usize, 0usize, 0, Dir::DOWN));
    let h = input.map.len();
    let w = input.map[0].len();
    while let Some((cost, i, j, cnt, dir)) = q.pop() {
        if i == h - 1 && j == w - 1 && cnt >= 4 {
            return -cost;
        }
        if dir != Dir::DOWN && i > 0 && (dir != Dir::UP && cnt >= 4 || dir == Dir::UP && cnt < 10) {
            let s = (i - 1, j, 1 + if dir == Dir::UP { cnt } else { 0 }, Dir::UP);
            if !seen.contains(&s) {
                seen.insert(s.clone());
                q.push((cost - (input.map[s.0][s.1] as i32), s.0, s.1, s.2, s.3));
            }
        }
        if dir != Dir::UP
            && i < h - 1
            && (dir != Dir::DOWN && cnt >= 4 || dir == Dir::DOWN && cnt < 10)
        {
            let s = (
                i + 1,
                j,
                1 + if dir == Dir::DOWN { cnt } else { 0 },
                Dir::DOWN,
            );
            if !seen.contains(&s) {
                seen.insert(s.clone());
                q.push((cost - (input.map[s.0][s.1] as i32), s.0, s.1, s.2, s.3));
            }
        }
        if dir != Dir::RIGHT
            && j > 0
            && (dir != Dir::LEFT && cnt >= 4 || dir == Dir::LEFT && cnt < 10)
        {
            let s = (
                i,
                j - 1,
                1 + if dir == Dir::LEFT { cnt } else { 0 },
                Dir::LEFT,
            );
            if !seen.contains(&s) {
                seen.insert(s.clone());
                q.push((cost - (input.map[s.0][s.1] as i32), s.0, s.1, s.2, s.3));
            }
        }
        if dir != Dir::LEFT
            && j < w - 1
            && (dir != Dir::RIGHT && cnt >= 4 || dir == Dir::RIGHT && cnt < 10)
        {
            let s = (
                i,
                j + 1,
                1 + if dir == Dir::RIGHT { cnt } else { 0 },
                Dir::RIGHT,
            );
            if !seen.contains(&s) {
                seen.insert(s.clone());
                q.push((cost - (input.map[s.0][s.1] as i32), s.0, s.1, s.2, s.3));
            }
        }
    }
    panic!()
}

struct Input {
    map: Vec<Vec<u8>>,
}

fn load_input(raw: String) -> Input {
    Input {
        map: raw
            .lines()
            .map(|line| line.bytes().map(|x| x - b'0').collect())
            .collect(),
    }
}
