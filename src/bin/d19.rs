use std::{collections::HashMap, ops::RangeInclusive};

aoc::main!();
aoc::check!("inputs/19.in", 368964, 127675188176682);

fn part_1(input: &Input) -> u64 {
    let mut ans = 0;
    for p in &input.parts {
        let mut curr = &String::from("in");
        while input.wf.contains_key(curr) {
            for (c, label) in &input.wf[curr] {
                if let Some(c) = c {
                    let val = p[&c.att];
                    if match c.sym {
                        b'<' => val < c.thr,
                        b'>' => val > c.thr,
                        _ => unreachable!(),
                    } {
                        curr = label;
                        break;
                    }
                } else {
                    curr = label;
                    break;
                }
            }
        }
        if curr == "A" {
            ans += p.values().map(|x| *x as u64).sum::<u64>()
        }
    }
    ans
}

fn part_2(input: &Input) -> u64 {
    count(
        input,
        &String::from("in"),
        1..=4000,
        1..=4000,
        1..=4000,
        1..=4000,
    )
}

fn count(
    input: &Input,
    label: &String,
    mut x: RangeInclusive<u16>,
    mut m: RangeInclusive<u16>,
    mut a: RangeInclusive<u16>,
    mut s: RangeInclusive<u16>,
) -> u64 {
    if label == "R" || x.is_empty() || m.is_empty() || a.is_empty() || s.is_empty() {
        return 0;
    } else if label == "A" {
        return (x.len() as u64) * (m.len() as u64) * (a.len() as u64) * (s.len() as u64);
    }
    let mut ans = 0;
    for (c, label) in &input.wf[label] {
        if let Some(c) = c {
            ans += match (c.att, c.sym) {
                (b'x', b'<') => {
                    let r = count(
                        input,
                        label,
                        (*x.start()).min(c.thr - 1 as u16)..=(*x.end()).min(c.thr - 1 as u16),
                        m.clone(),
                        a.clone(),
                        s.clone(),
                    );
                    x = (*x.start()).max(c.thr as u16)..=(*x.end()).max(c.thr as u16);
                    r
                }
                (b'x', b'>') => {
                    let r = count(
                        input,
                        label,
                        (*x.start()).max(c.thr + 1 as u16)..=(*x.end()).max(c.thr + 1 as u16),
                        m.clone(),
                        a.clone(),
                        s.clone(),
                    );
                    x = (*x.start()).min(c.thr as u16)..=(*x.end()).min(c.thr as u16);
                    r
                }
                (b'm', b'<') => {
                    let r = count(
                        input,
                        label,
                        x.clone(),
                        (*m.start()).min(c.thr - 1 as u16)..=(*m.end()).min(c.thr - 1 as u16),
                        a.clone(),
                        s.clone(),
                    );
                    m = (*m.start()).max(c.thr as u16)..=(*m.end()).max(c.thr as u16);
                    r
                }
                (b'm', b'>') => {
                    let r = count(
                        input,
                        label,
                        x.clone(),
                        (*m.start()).max(c.thr + 1 as u16)..=(*m.end()).max(c.thr + 1 as u16),
                        a.clone(),
                        s.clone(),
                    );
                    m = (*m.start()).min(c.thr as u16)..=(*m.end()).min(c.thr as u16);
                    r
                }
                (b'a', b'<') => {
                    let r = count(
                        input,
                        label,
                        x.clone(),
                        m.clone(),
                        (*a.start()).min(c.thr - 1 as u16)..=(*a.end()).min(c.thr - 1 as u16),
                        s.clone(),
                    );
                    a = (*a.start()).max(c.thr as u16)..=(*a.end()).max(c.thr as u16);
                    r
                }
                (b'a', b'>') => {
                    let r = count(
                        input,
                        label,
                        x.clone(),
                        m.clone(),
                        (*a.start()).max(c.thr + 1 as u16)..=(*a.end()).max(c.thr + 1 as u16),
                        s.clone(),
                    );
                    a = (*a.start()).min(c.thr as u16)..=(*a.end()).min(c.thr as u16);
                    r
                }
                (b's', b'<') => {
                    let r = count(
                        input,
                        label,
                        x.clone(),
                        m.clone(),
                        a.clone(),
                        (*s.start()).min(c.thr - 1 as u16)..=(*s.end()).min(c.thr - 1 as u16),
                    );
                    s = (*s.start()).max(c.thr as u16)..=(*s.end()).max(c.thr as u16);
                    r
                }
                (b's', b'>') => {
                    let r = count(
                        input,
                        label,
                        x.clone(),
                        m.clone(),
                        a.clone(),
                        (*s.start()).max(c.thr + 1 as u16)..=(*s.end()).max(c.thr + 1 as u16),
                    );
                    s = (*s.start()).min(c.thr as u16)..=(*s.end()).min(c.thr as u16);
                    r
                }
                _ => unreachable!(),
            }
        } else {
            ans += count(input, label, x.clone(), m.clone(), a.clone(), s.clone());
        }
    }
    ans
}

struct Cond {
    att: u8,
    sym: u8,
    thr: u16,
}
struct Input {
    wf: HashMap<String, Vec<(Option<Cond>, String)>>,
    parts: Vec<HashMap<u8, u16>>,
}

fn load_input(raw: String) -> Input {
    let mut lines = raw.lines();
    let mut wf = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (a, b) = line.split_once("{").unwrap();
        let b = &b[..b.len() - 1];
        wf.insert(
            String::from(a),
            b.split(",")
                .map(|p| {
                    if let Some((a, b)) = p.split_once(":") {
                        (
                            Some(Cond {
                                att: a.bytes().nth(0).unwrap(),
                                sym: a.bytes().nth(1).unwrap(),
                                thr: a[2..].parse::<u16>().unwrap(),
                            }),
                            String::from(b),
                        )
                    } else {
                        (None, String::from(p))
                    }
                })
                .collect(),
        );
    }
    Input {
        wf: wf,
        parts: lines
            .map(|line| {
                let line = &line[1..line.len() - 1];
                line.split(",")
                    .map(|p| {
                        let (a, b) = p.split_once("=").unwrap();
                        (a.bytes().next().unwrap(), b.parse::<u16>().unwrap())
                    })
                    .collect()
            })
            .collect(),
    }
}
