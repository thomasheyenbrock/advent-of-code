const INPUT: &str = include_str!("input.txt");

const THRESHOLD: u8 = 3;

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

impl Direction {
    fn valid(&self, first: u8, second: u8) -> bool {
        match self {
            Direction::Increasing => first < second && second - first <= THRESHOLD,
            Direction::Decreasing => first > second && first - second <= THRESHOLD,
        }
    }
}

fn is_safe(levels: &Vec<u8>) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let direction = if levels[0] < levels[1] {
        Direction::Increasing
    } else if levels[0] > levels[1] {
        Direction::Decreasing
    } else {
        return false;
    };

    let mut is_safe = true;
    for index in 0..(levels.len() - 1) {
        let first = levels[index];
        let second = levels[index + 1];

        if !direction.valid(first, second) {
            is_safe = false;
            break;
        }
    }

    is_safe
}

fn main() {
    // PART 1
    let mut count = 0;

    for report in INPUT.lines() {
        let levels = report
            .split(" ")
            .map(|str| str.parse::<u8>().expect("Not a valid number"))
            .collect::<Vec<_>>();

        if is_safe(&levels) {
            count += 1
        }
    }

    println!("{count}");

    // PART 2
    let mut count = 0;

    for report in INPUT.lines() {
        let levels = report
            .split(" ")
            .map(|str| str.parse::<u8>().expect("Not a valid number"))
            .collect::<Vec<_>>();

        if is_safe(&levels) {
            count += 1;
            continue;
        }

        let is_safe = (0..levels.len()).into_iter().any(|index| {
            let mut sub_levels = levels.clone();
            sub_levels.remove(index);
            is_safe(&sub_levels)
        });
        if is_safe {
            count += 1
        }
    }

    println!("{count}");
}
