use std::{cell::RefCell, collections::HashMap};

aoc::main!();
aoc::check!("inputs/14.in", 106517, 79723);

fn part_1(input: &Input) -> usize {
    let map = input.map.borrow();
    let h = map.len();
    let mut score = 0;
    for i in 0..map[0].len() {
        let mut next = 0;
        for j in 0..h {
            match map[j][i] {
                b'.' => {}
                b'#' => {
                    next = j + 1;
                }
                b'O' => {
                    score += h - next;
                    next += 1;
                }
                _ => panic!(),
            }
        }
    }
    score
}

fn tilt<T: Fn(usize, usize) -> (usize, usize)>(
    map: &mut Vec<Vec<u8>>,
    (w, h): (usize, usize),
    f: T,
) {
    for c in 0..w {
        let mut next = 0;
        for r in 0..h {
            let (row, col) = f(r, c);
            let ch = map[row][col];
            match ch {
                b'.' => {}
                b'#' => {
                    next = r + 1;
                }
                b'O' => {
                    map[row][col] = b'.';
                    let (row, col) = f(next, c);
                    map[row][col] = b'O';
                    next += 1;
                }
                _ => panic!(),
            }
        }
    }
}

fn cycle(map: &mut Vec<Vec<u8>>) {
    let w = map[0].len();
    let h = map.len();
    tilt(map, (w, h), |r, c| (r, c));
    tilt(map, (h, w), |r, c| (c, r));
    tilt(map, (w, h), |r, c| (h - r - 1, c));
    tilt(map, (h, w), |r, c| (c, w - r - 1));
}

fn hmap(map: &Vec<Vec<u8>>) -> u64 {
    let mut h: u64 = 0;
    for line in map {
        for c in line {
            h = h.wrapping_mul(3);
            h = h.wrapping_add(match c {
                b'.' => 0,
                b'#' => 1,
                b'O' => 2,
                _ => panic!(),
            })
        }
    }
    h
}
fn part_2(input: &Input) -> usize {
    let mut seen = HashMap::new();
    let mut i = 0;
    let mut wrapped = false;
    while i < 1_000_000_000 {
        i += 1;
        cycle(&mut *input.map.borrow_mut());
        if wrapped {
            continue;
        }
        let h = hmap(&*input.map.borrow());
        if seen.contains_key(&h) {
            let delta = i - seen[&h];
            let cnt = (1_000_000_000 - i) / delta;
            i += cnt * delta;
            wrapped = true;
        } else {
            seen.insert(h, i);
        }
    }
    let h = input.map.borrow().len();
    input
        .map
        .borrow()
        .iter()
        .enumerate()
        .map(|(i, line)| line.iter().filter(|c| **c == b'O').count() * (h - i))
        .sum::<usize>()
}

struct Input {
    map: RefCell<Vec<Vec<u8>>>,
}

fn load_input(raw: String) -> Input {
    Input {
        map: RefCell::new(raw.lines().map(|line| line.bytes().collect()).collect()),
    }
}
