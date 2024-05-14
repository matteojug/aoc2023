use std::collections::{HashMap, HashSet};

aoc::main!();
aoc::check!("inputs/8.in", 16343, 15299095336639);

fn part_1(input: &Input) -> usize {
    let mut curr = &str_to_u32("AAA");
    let target = &str_to_u32("ZZZ");
    let mut i = 0;
    loop {
        let ii = input.instr[i % input.instr.len()];
        match ii {
            b'L' => curr = &input.map[curr].0,
            b'R' => curr = &input.map[curr].1,
            _ => unreachable!(),
        }
        i += 1;
        if curr == target {
            return i;
        }
    }
}

fn part_2(input: &Input) -> usize {
    let mut curr: Vec<_> = input.map.keys().filter(|k| *k % 36 == 10).collect();
    let mut i = 0;
    while i < 2 * input.map.len() * input.instr.len() {
        let ii = input.instr[i % input.instr.len()];
        for j in 0..curr.len() {
            curr[j] = match ii {
                b'L' => &input.map[curr[j]].0,
                b'R' => &input.map[curr[j]].1,
                _ => unreachable!(),
            };
        }
        i += 1;
        if curr.iter().all(|c| *c % 36 == 35) {
            return i;
        }
    }
    // Meet at cycle
    let mut cycles = Vec::new();
    for idx in 0..curr.len() {
        let mut ii = i;
        let mut seen: HashSet<(u32, usize)> = HashSet::new();
        let mut curr_ = curr[idx];
        let mut finals = Vec::new();
        loop {
            let k = (*curr_, ii % input.instr.len());
            if seen.contains(&k) {
                break;
            }
            seen.insert(k);
            if curr_ % 36 == 35 {
                finals.push(ii - i);
            }
            curr_ = match input.instr[ii % input.instr.len()] {
                b'L' => &input.map[curr_].0,
                b'R' => &input.map[curr_].1,
                _ => unreachable!(),
            };
            ii += 1;
        }
        assert!(finals.len() == 1);
        cycles.push((ii - i, finals[0]));
    }
    let mut z = cycles[0].1;
    while !curr
        .iter()
        .enumerate()
        .all(|(ii, _)| z % cycles[ii].0 == cycles[ii].1)
    {
        z += cycles[0].0;
    }
    return i + z;
}

struct Input {
    instr: Vec<u8>,
    map: HashMap<u32, (u32, u32)>,
}

fn str_to_u32(s: &str) -> u32 {
    let mut r = 0;
    for c in s.chars() {
        r *= 36;
        r += c.to_digit(36).unwrap();
    }
    r
}

fn load_input(raw: String) -> Input {
    let mut input = raw.lines();
    let instr = String::from(input.next().unwrap());
    input.next();
    Input {
        instr: instr.bytes().collect(),
        map: input
            .map(|line| {
                let (k, mut vs) = line.split_once("=").unwrap();
                vs = vs.trim();
                let (l, r) = vs[1..vs.len() - 1].split_once(", ").unwrap();
                (str_to_u32(k.trim()), (str_to_u32(l), str_to_u32(r)))
            })
            .collect(),
    }
}
