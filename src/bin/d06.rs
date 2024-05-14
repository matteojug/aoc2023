aoc::main!();
aoc::check!("inputs/6.in", 131376, 34123437);

fn part_1(input: &Input) -> u32 {
    // x * (t-x) = d -> xx - xt + d = 0
    input
        .time
        .iter()
        .zip(input.dist.iter())
        .map(|(t, d)| {
            let mut z = ((*t as f64 - ((t * t - 4 * d) as f64).sqrt()) / 2.0) as u32;
            if z * (t - z) <= *d {
                z += 1;
            }
            assert!(z * (t - z) > *d);
            t - z - z + 1
        })
        .fold(1, |acc, x| acc * x)
}

fn part_2(input: &Input) -> u64 {
    let t = input.time_all;
    let d = input.dist_all;
    let mut z = ((t as f64 - ((t * t - 4 * d) as f64).sqrt()) / 2.0) as u64;
    if z * (t - z) <= d {
        z += 1;
    }
    assert!(z * (t - z) > d);
    t - z - z + 1
}

struct Input {
    time: Vec<u32>,
    dist: Vec<u32>,
    time_all: u64,
    dist_all: u64,
}

fn load_input(raw: String) -> Input {
    let mut input = raw.lines();
    let mut parse = || {
        input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .map(|x| x.parse::<u32>().unwrap())
            .collect()
    };
    let mut input2 = raw.lines();
    let mut parse2 = || {
        input2
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("")
            .parse::<u64>()
            .unwrap()
    };
    Input {
        time: parse(),
        dist: parse(),
        time_all: parse2(),
        dist_all: parse2(),
    }
}
