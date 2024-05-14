aoc::main!();
aoc::check!("inputs/5.in", 462648396, 2520479);

fn part_1(input: &Input) -> u64 {
    input.seeds.iter().map(|x| input.map(*x)).min().unwrap()
}

fn part_2(input: &Input) -> u64 {
    let mut intervals = Vec::new();
    for i in 0..input.seeds.len() / 2 {
        intervals.push((
            input.seeds[i * 2],
            input.seeds[i * 2] + input.seeds[i * 2 + 1],
        ))
    }
    for map in &input.maps {
        let mut new_intervals = Vec::new();
        for (mut a, b) in intervals {
            for tpl in map {
                if tpl.src + tpl.size <= a || tpl.src >= b {
                    continue;
                }
                if a < tpl.src {
                    new_intervals.push((a, tpl.src - 1));
                    a = tpl.src;
                }
                let end = (tpl.src + tpl.size).min(b);
                new_intervals.push((a - tpl.src + tpl.dst, end - 1 - tpl.src + tpl.dst));
                a = end;
            }
            if a < b {
                new_intervals.push((a, b));
            }
        }
        intervals = new_intervals;
    }
    intervals.iter().map(|x| x.0).min().unwrap()
}

struct InputTuple {
    dst: u64,
    src: u64,
    size: u64,
}

struct Input {
    seeds: Vec<u64>,
    maps: Vec<Vec<InputTuple>>,
}

impl Input {
    fn map(&self, mut seed: u64) -> u64 {
        for map in &self.maps {
            for tpl in map {
                if seed >= tpl.src && seed < tpl.src + tpl.size {
                    seed = seed - tpl.src + tpl.dst;
                    break;
                }
            }
        }
        seed
    }
}

fn load_input(raw: String) -> Input {
    let mut lines = raw.lines();
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut maps = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.contains(":") {
            maps.push(Vec::new());
            continue;
        }
        let mut ns = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap());
        maps.last_mut().unwrap().push(InputTuple {
            dst: ns.next().unwrap(),
            src: ns.next().unwrap(),
            size: ns.next().unwrap(),
        })
    }
    for map in &mut maps {
        map.sort_by_key(|t| t.src)
    }
    Input { seeds, maps }
}
