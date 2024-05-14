use std::cell::RefCell;

aoc::main!();
aoc::check!("inputs/13.in", 37025, 32854);

fn find_mirror(v: &Vec<u64>) -> Option<usize> {
    for i in 1..v.len() {
        let mut eq = true;
        for j in 0..i.min(v.len() - i) {
            if v[i + j] != v[i - j - 1] {
                eq = false;
                break;
            }
        }
        if eq {
            return Some(i);
        }
    }
    None
}

fn find_quasi_mirror(v: &Vec<u64>, skip: usize) -> Option<usize> {
    for i in 1..v.len() {
        if i == skip {
            continue;
        }
        let mut dist = 0;
        for j in 0..i.min(v.len() - i) {
            dist += (v[i + j] ^ v[i - j - 1]).count_ones();
        }
        if dist == 1 {
            return Some(i);
        }
    }
    None
}

fn part_1(input: &Input) -> usize {
    input
        .maps
        .iter()
        .enumerate()
        .map(|(map_i, map)| {
            if let Some(m) = find_mirror(map) {
                input.mirrors.borrow_mut().push((m, 0));
                return 100 * m;
            }
            let cols: Vec<_> = (0..input.maps_w[map_i])
                .rev()
                .map(|i| {
                    map.iter()
                        .map(|line| (line >> i) & 1)
                        .reduce(|acc, e| (acc << 1) | e)
                        .unwrap()
                })
                .collect();
            if let Some(m) = find_mirror(&cols) {
                input.mirrors.borrow_mut().push((0, m));
                return m;
            }
            panic!();
        })
        .sum::<usize>()
}

fn part_2(input: &Input) -> usize {
    input
        .maps
        .iter()
        .enumerate()
        .map(|(map_i, map)| {
            if let Some(m) = find_quasi_mirror(map, (*input.mirrors.borrow())[map_i].0) {
                return 100 * m;
            }
            let cols: Vec<_> = (0..input.maps_w[map_i])
                .rev()
                .map(|i| {
                    map.iter()
                        .map(|line| (line >> i) & 1)
                        .reduce(|acc, e| (acc << 1) | e)
                        .unwrap()
                })
                .collect();
            if let Some(m) = find_quasi_mirror(&cols, (*input.mirrors.borrow())[map_i].1) {
                return m;
            }
            panic!();
        })
        .sum::<usize>()
}

struct Input {
    maps: Vec<Vec<u64>>,
    maps_w: Vec<usize>,
    mirrors: RefCell<Vec<(usize, usize)>>,
}

fn load_input(raw: String) -> Input {
    let mut maps = vec![];
    let mut maps_w = vec![];
    maps.push(vec![]);
    maps_w.push(0);
    for line in raw.lines() {
        if line.is_empty() {
            maps.push(vec![]);
            maps_w.push(0);
            continue;
        }
        *maps_w.last_mut().unwrap() = line.len();
        maps.last_mut().unwrap().push(
            line.bytes()
                .map(|c| if c == b'#' { 1 } else { 0 })
                .reduce(|acc, e| (acc << 1) | e)
                .unwrap(),
        );
    }
    Input {
        maps,
        maps_w,
        mirrors: RefCell::new(Vec::new()),
    }
}
