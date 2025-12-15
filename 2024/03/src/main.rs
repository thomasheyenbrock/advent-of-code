use std::{iter::Peekable, str::Chars};

const INPUT: &str = include_str!("input.txt");

fn parse_string<'a>(chars: &mut Peekable<Chars<'a>>, s: &str) -> bool {
    for c in s.chars() {
        if chars.next() != Some(c) {
            return false;
        }
    }

    true
}

fn parse_number<'a>(chars: &mut Peekable<Chars<'a>>) -> u32 {
    let mut a = "".to_string();
    while chars.peek().map_or(false, |c| c.is_digit(10)) {
        a.push(chars.next().unwrap())
    }
    a.parse::<u32>().unwrap()
}

fn main() {
    // PART 1
    let mut sum = 0;

    let mut chars = INPUT.chars().peekable();
    while chars.peek().is_some() {
        if !parse_string(&mut chars, "mul(") {
            continue;
        }

        let a = parse_number(&mut chars);

        if !parse_string(&mut chars, ",") {
            continue;
        }

        let b = parse_number(&mut chars);

        if !parse_string(&mut chars, ")") {
            continue;
        }

        sum += a * b
    }

    println!("{sum}");

    // PART 2
    let mut sum = 0;
    let mut is_enabled = true;

    let mut chars = INPUT.chars().peekable();
    while chars.peek().is_some() {
        if chars.peek() == Some(&'d') {
            if is_enabled && parse_string(&mut chars, "don't()") {
                is_enabled = false
            } else if !is_enabled && parse_string(&mut chars, "do()") {
                is_enabled = true
            }
            continue;
        }

        if !is_enabled {
            chars.next();
            continue;
        }

        if !parse_string(&mut chars, "mul(") {
            continue;
        }

        let a = parse_number(&mut chars);

        if !parse_string(&mut chars, ",") {
            continue;
        }

        let b = parse_number(&mut chars);

        if !parse_string(&mut chars, ")") {
            continue;
        }

        sum += a * b
    }

    println!("{sum}");
}
