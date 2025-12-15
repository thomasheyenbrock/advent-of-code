use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

const SEPARATOR: &str = "   ";

fn main() {
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

    let mut sum = 0;
    for (a, b) in list_a.iter().zip(list_b.iter()) {
        sum += a.abs_diff(*b)
    }

    println!("{sum}");

    // PART 2
    let mut map_b = HashMap::<u128, u128>::new();
    for b in list_b {
        let existing = map_b.get(&b).unwrap_or(&0);
        map_b.insert(b, existing + 1);
    }

    let mut sum = 0;
    for a in list_a {
        sum += a * map_b.get(&a).unwrap_or(&0)
    }
    println!("{sum}");
}
