use std::collections::HashSet;

aoc::main!();
aoc::check!("inputs/21.in", 3729, 0);

fn part_1(input: &Input) -> usize {
    let mut p = HashSet::new();
    let h = input.map.len();
    let w = input.map[0].len();
    p.insert(input.start.clone());
    for _ in 0..64 {
        let mut pp = HashSet::new();
        for (i, j) in &p {
            if *i > 0 && input.map[i - 1][*j] == b'.' {
                pp.insert((i - 1, *j));
            }
            if *i < h - 1 && input.map[i + 1][*j] == b'.' {
                pp.insert((i + 1, *j));
            }
            if *j > 0 && input.map[*i][j - 1] == b'.' {
                pp.insert((*i, j - 1));
            }
            if *j < w - 1 && input.map[*i][j + 1] == b'.' {
                pp.insert((*i, j + 1));
            }
        }
        p = pp;
    }
    p.len()
}

fn part_2(input: &Input) -> usize {
    1
}

struct Input {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
}

fn load_input(raw: String) -> Input {
    let mut map: Vec<Vec<_>> = raw.lines().map(|line| line.bytes().collect()).collect();
    let start = map
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, x)| **x == b'S')
                .map(|(j, _)| (i, j))
                .next()
        })
        .next()
        .unwrap();
    map[start.0][start.1] = b'.';
    Input { map, start }
}
