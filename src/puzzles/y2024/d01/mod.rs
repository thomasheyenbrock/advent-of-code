use std::collections::HashMap;

use crate::puzzles::Solver;

pub fn get_solver() -> Solver {
    solver
}

const INPUT: &str = include_str!("input.txt");

const SEPARATOR: &str = "   ";

fn solver() -> (String, String) {
    // PART 1
    let lines = INPUT.lines().count();

    let mut list_a = Vec::with_capacity(lines);
    let mut list_b = Vec::with_capacity(lines);

    for line in INPUT.lines() {
        let mut iter = line.split(SEPARATOR);
        list_a.push(
            iter.next()
                .expect("First number to exist")
                .parse::<u128>()
                .expect("First number is not an integer"),
        );
        list_b.push(
            iter.next()
                .expect("Second number to exist")
                .parse::<u128>()
                .expect("Second number is not an integer"),
        );
    }

    list_a.sort();
    list_b.sort();

    let mut sum_1 = 0;
    for (a, b) in list_a.iter().zip(list_b.iter()) {
        sum_1 += a.abs_diff(*b)
    }

    // PART 2
    let mut map_b = HashMap::<u128, u128>::new();
    for b in list_b {
        let existing = map_b.get(&b).unwrap_or(&0);
        map_b.insert(b, existing + 1);
    }

    let mut sum_2 = 0;
    for a in list_a {
        sum_2 += a * map_b.get(&a).unwrap_or(&0)
    }

    (sum_1.to_string(), sum_2.to_string())
}
