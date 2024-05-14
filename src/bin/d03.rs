use std::collections::{HashMap, HashSet};

aoc::main!();
aoc::check!("inputs/3.in", 537832, 81939900);

fn load_input(raw: String) -> String {
    raw
}

fn part_1(input: &String) -> i32 {
    let map: Vec<&[u8]> = input.lines().map(|x| x.as_bytes()).collect();
    let n = map.len() as i32;
    let m = map[0].len() as i32;

    let has_sym = |i: i32, j: i32| {
        for ii in -1..=1 {
            for jj in -1..=1 {
                if ii == 0 && jj == 0 {
                    continue;
                }
                let i = i + ii;
                let j = j + jj;
                if i >= 0
                    && i < m
                    && j >= 0
                    && j < n
                    && map[i as usize][j as usize] != b'.'
                    && !map[i as usize][j as usize].is_ascii_digit()
                {
                    return true;
                }
            }
        }
        return false;
    };

    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut sum: i32 = 0;
            let mut curr: i32 = 0;
            let mut curr_sym: bool = false;
            for (j, c) in line.bytes().chain(".".bytes()).enumerate() {
                if !c.is_ascii_digit() {
                    if curr != 0 && curr_sym {
                        sum += curr
                    }
                    curr = 0;
                    curr_sym = false;
                } else {
                    curr *= 10;
                    curr += (c - b'0') as i32;
                    curr_sym |= has_sym(i as i32, j as i32);
                }
            }
            sum
        })
        .sum::<i32>()
}

fn part_2(input: &String) -> i32 {
    let map: Vec<&[u8]> = input.lines().map(|x| x.as_bytes()).collect();
    let n = map.len() as i32;
    let m = map[0].len() as i32;

    let mut gears: HashMap<(i32, i32), HashSet<usize>> = HashMap::new();
    let mut nums: Vec<i32> = vec![];

    let near_gear = |i: i32, j: i32| {
        let mut ret = Vec::new();
        for ii in -1..=1 {
            for jj in -1..=1 {
                if ii == 0 && jj == 0 {
                    continue;
                }
                let i = i + ii;
                let j = j + jj;
                if i >= 0 && i < m && j >= 0 && j < n && map[i as usize][j as usize] == b'*' {
                    ret.push((i, j))
                }
            }
        }
        return ret;
    };

    input.lines().enumerate().for_each(|(i, line)| {
        let mut curr: i32 = 0;
        let mut curr_gear: HashSet<(i32, i32)> = HashSet::new();
        for (j, c) in line.bytes().chain(".".bytes()).enumerate() {
            if !c.is_ascii_digit() {
                if curr != 0 && !curr_gear.is_empty() {
                    let curr_idx = nums.len();
                    nums.push(curr);
                    for gear in curr_gear {
                        gears.entry(gear).or_insert(HashSet::new()).insert(curr_idx);
                    }
                    curr_gear = HashSet::new();
                } else {
                    curr_gear.clear();
                }
                curr = 0;
            } else {
                curr *= 10;
                curr += (c - b'0') as i32;
                for gear in near_gear(i as i32, j as i32) {
                    curr_gear.insert(gear);
                }
            }
        }
    });
    gears
        .iter()
        .filter_map(|(_, gn)| {
            if gn.len() != 2 {
                None
            } else {
                let mut a = gn.iter();
                Some(nums[*a.next().unwrap()] * nums[*a.next().unwrap()])
            }
        })
        .sum::<i32>()
}
