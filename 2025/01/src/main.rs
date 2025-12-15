const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut current = 50;

    let mut exact_zero = 0;
    let mut passing_zero = 0;

    for line in INPUT.lines() {
        let mut iter = line.chars();

        let direction = match iter.next() {
            Some('L') => -1,
            Some('R') => 1,
            Some(c) => panic!("Invalid direction {c}"),
            None => panic!("Empty line"),
        };

        let clicks = iter
            .collect::<String>()
            .parse::<i32>()
            .expect("Invalid number of clicks");

        for _ in 0..clicks {
            current = (current + direction + 100) % 100;

            if current == 0 {
                passing_zero += 1;
            }
        }

        if current == 0 {
            exact_zero += 1;
        }
    }

    println!("{exact_zero}");
    println!("{passing_zero}");
}
