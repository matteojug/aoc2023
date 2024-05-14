use std::collections::HashMap;

aoc::main!();
aoc::check!("inputs/12.in", 7090, 6792010726878);

fn part_1(input: &Input) -> usize {
    input.cases.iter().map(|c| dp(c)).sum::<usize>()
}
fn part_2(input: &Input) -> usize {
    input
        .cases
        .iter()
        .map(|c| {
            let c = Case {
                line: (0..5)
                    .flat_map(|_| [b'?'].iter().chain(c.line.iter()))
                    .skip(1)
                    .copied()
                    .collect(),
                blocks: (0..5).flat_map(|_| c.blocks.iter()).copied().collect(),
            };
            dp(&c)
        })
        .sum::<usize>()
}

fn dp(input: &Case) -> usize {
    let mut line = vec![b'.'];
    line.append(&mut input.line.clone());
    line.push(b'.');
    let mut dp: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    dp.insert(0, HashMap::from([(0, 1)]));
    for i in 1..line.len() {
        if line[i] == b'#' {
            continue;
        }
        let mut im = HashMap::new();
        if dp.contains_key(&(i - 1)) {
            for (k, v) in &dp[&(i - 1)] {
                im.insert(*k, *v);
            }
        }
        for j in (0..i).rev() {
            if line[j] == b'#' {
                continue;
            }
            if dp.contains_key(&j) {
                for (ig, cnt) in &dp[&j] {
                    if *ig < input.blocks.len() && i - j - 1 == input.blocks[*ig] {
                        *im.entry(*ig + 1).or_default() += cnt
                    }
                }
            }
            if line[j] == b'.' {
                break;
            }
        }
        if !im.is_empty() {
            dp.insert(i, im);
        }
    }
    dp[&(line.len() - 1)][&input.blocks.len()]
}

struct Input {
    cases: Vec<Case>,
}
struct Case {
    line: Vec<u8>,
    blocks: Vec<usize>,
}

fn load_input(raw: String) -> Input {
    Input {
        cases: raw
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(" ").unwrap();
                Case {
                    line: a.bytes().collect(),
                    blocks: b
                        .trim()
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect(),
                }
            })
            .collect(),
    }
}
