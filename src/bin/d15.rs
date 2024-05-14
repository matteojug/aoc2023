use std::collections::HashMap;

aoc::main!();
aoc::check!("inputs/15.in", 511343, 294474);

fn part_1(input: &String) -> u32 {
    let mut sum: u32 = 0;
    let mut h: u32 = 0;
    for c in input.bytes() {
        match c {
            b',' => {
                sum += h;
                h = 0
            }
            b'\n' => {}
            c => h = (h + (c as u32)) * 17 % 256,
        }
    }
    sum += h;
    sum
}

fn hash(s: &str) -> usize {
    let mut h: usize = 0;
    for c in s.bytes() {
        match c {
            b'\n' => {}
            c => h = (h + (c as usize)) * 17 % 256,
        }
    }
    h
}

fn part_2(input: &String) -> usize {
    let mut boxes: Vec<HashMap<String, (usize, usize)>> = vec![];
    for _ in 0..256 {
        boxes.push(HashMap::new());
    }
    for (i, cmd) in input.split(",").enumerate() {
        if cmd.contains("-") {
            let label = String::from(&cmd[..cmd.len() - 1]);
            let b = hash(&label);
            boxes[b].remove(&label);
        } else if cmd.contains("=") {
            let cmd = cmd.split_once("=").unwrap();
            let label = String::from(cmd.0);
            let arg = cmd.1.parse::<usize>().unwrap();
            let b = hash(&label);
            boxes[b]
                .entry(label)
                .and_modify(|(f, _)| *f = arg)
                .or_insert((arg, i));
        }
    }
    let mut sum: usize = 0;
    for (i, bx) in boxes.iter().enumerate() {
        let mut vals: Vec<&(usize, usize)> = bx.values().collect();
        vals.sort_by_key(|(_, t)| t);
        sum += vals
            .iter()
            .enumerate()
            .map(|(j, (f, _))| (i + 1) * (j + 1) * f)
            .sum::<usize>();
    }
    sum
}

fn load_input(raw: String) -> String {
    String::from(raw.trim())
}
