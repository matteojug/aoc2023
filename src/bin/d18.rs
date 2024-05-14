aoc::main!();
aoc::check!("inputs/18.in", 36725, 97874103749720);

fn part_1(input: &Input) -> i32 {
    let mut area = 0;
    let mut curr = (0, 0);
    for (dir, step, ..) in &input.ins {
        let next = match dir {
            b'R' => (curr.0, curr.1 + step),
            b'L' => (curr.0, curr.1 - step),
            b'D' => (curr.0 + step, curr.1),
            b'U' => (curr.0 - step, curr.1),
            _ => panic!(),
        };
        area += (curr.1 + next.1) * (next.0 - curr.0) + step;
        curr = next;
    }
    area / 2 + 1
}

fn part_2(input: &Input) -> i64 {
    let mut area = 0i64;
    let mut curr = (0, 0);
    for (.., code) in &input.ins {
        let (step, dir) = code.split_at(5);
        let step = i32::from_str_radix(step, 16).unwrap();
        let next = match dir.bytes().next().unwrap() {
            b'0' => (curr.0, curr.1 + step),
            b'2' => (curr.0, curr.1 - step),
            b'1' => (curr.0 + step, curr.1),
            b'3' => (curr.0 - step, curr.1),
            _ => panic!(),
        };
        area += ((curr.1 + next.1) as i64) * ((next.0 - curr.0) as i64) + (step as i64);
        curr = next;
    }
    area / 2 + 1
}

struct Input {
    ins: Vec<(u8, i32, String)>,
}

fn load_input(raw: String) -> Input {
    Input {
        ins: raw
            .lines()
            .map(|line| {
                let (a, b) = line.split_once("(#").unwrap();
                let (a1, a2) = a.trim().split_once(" ").unwrap();
                (
                    a1.bytes().next().unwrap(),
                    a2.parse::<i32>().unwrap(),
                    String::from(b.strip_suffix(")").unwrap()),
                )
            })
            .collect(),
    }
}
