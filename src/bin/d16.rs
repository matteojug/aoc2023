use std::collections::{HashSet, VecDeque};

aoc::main!();
aoc::check!("inputs/16.in", 6740, 7041);

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
type CellPos = (i32, i32, Dir);

fn next_cell(input: &Input, (i, j, dir): CellPos) -> (Option<CellPos>, Option<CellPos>) {
    let mut r = (None, None);
    let mut c = input.map[i as usize][j as usize];
    if c == b'-' && (dir == Dir::LEFT || dir == Dir::RIGHT)
        || c == b'|' && (dir == Dir::UP || dir == Dir::DOWN)
    {
        c = b'.';
    }
    match c {
        b'.' => {
            r.0 = Some(match dir {
                Dir::LEFT => (i, j + 1, dir),
                Dir::RIGHT => (i, j - 1, dir),
                Dir::UP => (i + 1, j, dir),
                Dir::DOWN => (i - 1, j, dir),
            });
        }
        b'-' => {
            r.0 = Some((i, j - 1, Dir::RIGHT));
            r.1 = Some((i, j + 1, Dir::LEFT));
        }
        b'|' => {
            r.0 = Some((i - 1, j, Dir::DOWN));
            r.1 = Some((i + 1, j, Dir::UP));
        }
        b'/' => {
            r.0 = Some(match dir {
                Dir::LEFT => (i - 1, j, Dir::DOWN),
                Dir::RIGHT => (i + 1, j, Dir::UP),
                Dir::DOWN => (i, j + 1, Dir::LEFT),
                Dir::UP => (i, j - 1, Dir::RIGHT),
            });
        }
        b'\\' => {
            r.0 = Some(match dir {
                Dir::RIGHT => (i - 1, j, Dir::DOWN),
                Dir::LEFT => (i + 1, j, Dir::UP),
                Dir::UP => (i, j + 1, Dir::LEFT),
                Dir::DOWN => (i, j - 1, Dir::RIGHT),
            });
        }
        _ => panic!(),
    }
    r
}
fn shoot(input: &Input, s: CellPos) -> usize {
    let mut seen = HashSet::new();
    let mut energized = HashSet::new();
    let mut q = VecDeque::new();
    let w = input.map[0].len() as i32;
    let h = input.map.len() as i32;
    q.push_back(s.clone());
    energized.insert((s.0, s.1));
    seen.insert(s);
    while !q.is_empty() {
        let curr = q.pop_front().unwrap();
        let (a, b) = next_cell(&input, curr);
        for c in [a, b] {
            if let Some(c) = c {
                if c.0 < 0 || c.0 >= h || c.1 < 0 || c.1 >= w {
                    continue;
                }
                if !seen.contains(&c) {
                    q.push_back(c.clone());
                    energized.insert((c.0, c.1));
                    seen.insert(c);
                }
            }
        }
    }
    energized.len()
}

fn part_1(input: &Input) -> usize {
    shoot(input, (0, 0, Dir::LEFT))
}

fn part_2(input: &Input) -> usize {
    let mut best = 0;
    let w = input.map[0].len() as i32;
    let h = input.map.len() as i32;
    for i in 0..h {
        best = best
            .max(shoot(input, (i, 0, Dir::LEFT)))
            .max(shoot(input, (i, w - 1, Dir::RIGHT)));
    }
    for i in 0..w {
        best = best
            .max(shoot(input, (0, i, Dir::UP)))
            .max(shoot(input, (h - 1, i, Dir::DOWN)));
    }
    best
}

struct Input {
    map: Vec<Vec<u8>>,
}

fn load_input(raw: String) -> Input {
    Input {
        map: raw.lines().map(|line| line.bytes().collect()).collect(),
    }
}
