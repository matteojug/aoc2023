use aoc;

aoc::main!();
aoc::check!("inputs/2.in", 2486, 87984);

fn solvable(line: &str, red: i32, green: i32, blue: i32) -> Option<i32> {
    let mut s1 = line.split(":");
    let game = s1.next()?;
    for ball in s1.next()?.split(&[';', ',']) {
        let mut ball = ball.trim().split(' ');
        let n: i32 = ball.next()?.parse().unwrap();
        if n > match ball.next().unwrap() {
            "red" => red,
            "green" => green,
            "blue" => blue,
            _ => panic!("wtf?"),
        } {
            return None;
        }
    }

    Some(game.strip_prefix("Game ")?.parse::<i32>().unwrap())
}

fn power(line: &str) -> i32 {
    let mut s1 = line.split(":");
    s1.next();
    let mut min = (0, 0, 0);
    for ball in s1.next().unwrap().split(&[';', ',']) {
        let mut ball = ball.trim().split(' ');
        let n: i32 = ball.next().unwrap().parse().unwrap();
        let nr: &mut i32 = match ball.next().unwrap() {
            "red" => &mut min.0,
            "green" => &mut min.1,
            "blue" => &mut min.2,
            _ => panic!("wtf?"),
        };
        *nr = (*nr).max(n);
    }
    min.0 * min.1 * min.2
}

fn load_input(raw: String) -> String {
    raw
}

fn part_1(input: &String) -> i32 {
    input.lines().filter_map(|x| solvable(x, 12, 13, 14)).sum()
}

fn part_2(input: &String) -> i32 {
    input.lines().map(power).sum()
}
