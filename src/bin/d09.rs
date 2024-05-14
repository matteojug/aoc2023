aoc::main!();
aoc::check!("inputs/9.in", 1584748274, 1026);

fn part_1(input: &Input) -> i32 {
    input.vals.iter().map(|h| next(h)).sum::<i32>()
}
fn part_2(input: &Input) -> i32 {
    input.vals.iter().map(|h| prev(h)).sum::<i32>()
}

fn next(v: &Vec<i32>) -> i32 {
    if v.iter().all(|x| *x == v[0]) {
        return v[0];
    }
    let mut vd = Vec::new();
    for i in 1..v.len() {
        vd.push(v[i] - v[i - 1]);
    }
    v.last().unwrap() + next(&vd)
}
fn prev(v: &Vec<i32>) -> i32 {
    if v.iter().all(|x| *x == v[0]) {
        return v[0];
    }
    let mut vd = Vec::new();
    for i in 1..v.len() {
        vd.push(v[i] - v[i - 1]);
    }
    v.first().unwrap() - prev(&vd)
}
struct Input {
    vals: Vec<Vec<i32>>,
}

fn load_input(raw: String) -> Input {
    let vals = raw
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    Input { vals }
}
