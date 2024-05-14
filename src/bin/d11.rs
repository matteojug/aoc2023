use std::collections::HashSet;

aoc::main!();
aoc::check!("inputs/11.in", 9648398, 618800410814);

fn part_1(input: &Input) -> usize {
    apsp(input, 1)
}
fn part_2(input: &Input) -> usize {
    apsp(input, 1000000 - 1)
}

fn apsp(input: &Input, exp_factor: usize) -> usize {
    let mut s = 0;
    for i in 0..input.galaxies.len() {
        for j in i + 1..input.galaxies.len() {
            let d = (input.galaxies[i].0 + input.galaxies[i].1 * exp_factor)
                .abs_diff(input.galaxies[j].0 + input.galaxies[j].1 * exp_factor)
                + (input.galaxies[i].2 + input.galaxies[i].3 * exp_factor)
                    .abs_diff(input.galaxies[j].2 + input.galaxies[j].3 * exp_factor);
            s += d;
        }
    }
    s
}
struct Input {
    galaxies: Vec<(usize, usize, usize, usize)>,
}

fn load_input(raw: String) -> Input {
    let input = raw
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut galaxies = Vec::new();
    let empty_rows: HashSet<usize> = input
        .iter()
        .enumerate()
        .filter_map(|(i, l)| {
            if l.iter().all(|x| *x == b'.') {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    let empty_cols: HashSet<usize> = (0..input[0].len())
        .filter_map(|i| {
            if input.iter().all(|x| x[i] == b'.') {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    let mut er = 0;
    for i in 0..input.len() {
        if empty_rows.contains(&i) {
            er += 1;
            continue;
        }
        let mut ec = 0;
        for j in 0..input[0].len() {
            if empty_cols.contains(&j) {
                ec += 1;
                continue;
            }
            if input[i][j] == b'#' {
                galaxies.push((i, er, j, ec));
            }
        }
    }
    Input { galaxies }
}
