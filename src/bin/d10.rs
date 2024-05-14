use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
};

aoc::main!();
aoc::check!("inputs/10.in", 6931, 357);

fn part_1(input: &Input) -> usize {
    let mut q = VecDeque::new();
    q.push_back(input.start);
    let mut vis: HashMap<_, _> = HashMap::from([(input.start, 0)]);
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        let d = vis[&(x, y)];
        let c = map_pipe(input.map[x][y]);
        if c & NORTH != 0 && !vis.contains_key(&(x - 1, y)) {
            vis.insert((x - 1, y), d + 1);
            q.push_back((x - 1, y));
        }
        if c & SOUTH != 0 && !vis.contains_key(&(x + 1, y)) {
            vis.insert((x + 1, y), d + 1);
            q.push_back((x + 1, y));
        }
        if c & EAST != 0 && !vis.contains_key(&(x, y - 1)) {
            vis.insert((x, y - 1), d + 1);
            q.push_back((x, y - 1));
        }
        if c & WEST != 0 && !vis.contains_key(&(x, y + 1)) {
            vis.insert((x, y + 1), d + 1);
            q.push_back((x, y + 1));
        }
    }
    *input.loop_idx.borrow_mut() = vis.keys().copied().collect();
    *vis.values().max().unwrap()
}

fn part_2(input: &Input) -> usize {
    let mut cnt = 0;
    for i in 0..input.map.len() {
        for j in 0..input.map[i].len() {
            if input.loop_idx.borrow().contains(&(i, j)) {
                continue;
            }
            cnt += (0..j)
                .filter(|jj| {
                    input.loop_idx.borrow().contains(&(i, *jj))
                        && map_pipe(input.map[i][*jj]) & NORTH != 0
                })
                .count()
                % 2;
        }
    }
    cnt
}

struct Input {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    loop_idx: RefCell<HashSet<(usize, usize)>>,
}

const NORTH: u8 = 0x1;
const SOUTH: u8 = 0x2;
const EAST: u8 = 0x4;
const WEST: u8 = 0x8;

fn map_pipe(s: u8) -> u8 {
    match s {
        b'|' => NORTH | SOUTH,
        b'-' => EAST | WEST,
        b'L' => NORTH | WEST,
        b'J' => NORTH | EAST,
        b'7' => EAST | SOUTH,
        b'F' => WEST | SOUTH,
        b'.' => 0,
        _ => panic!(),
    }
}

fn load_input(raw: String) -> Input {
    let mut map: Vec<Vec<u8>> = raw.lines().map(|line| line.bytes().collect()).collect();
    let (x, y) = map
        .iter()
        .enumerate()
        .filter_map(|(i, v)| v.iter().position(|x| *x == b'S').and_then(|x| Some((i, x))))
        .next()
        .unwrap();
    let mut f = 0;
    if x > 0 && map_pipe(map[x - 1][y]) & SOUTH != 0 {
        f |= NORTH;
    }
    if x < map.len() - 1 && map_pipe(map[x + 1][y]) & NORTH != 0 {
        f |= SOUTH;
    }
    if y > 0 && map_pipe(map[x][y - 1]) & WEST != 0 {
        f |= EAST;
    }
    if y < map[x].len() - 1 && map_pipe(map[x][y + 1]) & EAST != 0 {
        f |= WEST;
    }
    map[x][y] = "|-LJ7F".bytes().find(|x| map_pipe(*x) == f).unwrap();
    Input {
        map,
        start: (x, y),
        loop_idx: RefCell::new(HashSet::new()),
    }
}
