#[macro_export]
macro_rules! main {
    () => {
        use std::time::Instant;
        use std::{env, fs};

        fn main() {
            let now = Instant::now();
            let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
            let input = load_input(input);
            println!("part_1: {:?}", part_1(&input));
            println!("part_2: {:?}", part_2(&input));
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
        }
    };
}

#[macro_export]
macro_rules! check {
    ( $input: expr, $sol1: expr, $sol2: expr) => {
        #[cfg(test)]
        mod tests {
            use std::{fs};
            #[test]
            fn test() {
                let input = super::load_input(fs::read_to_string($input).unwrap());
                assert_eq!(super::part_1(&input), $sol1);
                assert_eq!(super::part_2(&input), $sol2);
            }
        }
    };
}
