const INPUT: &str = include_str!("input.txt");

const DIGIT_STR: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let mut digit_sum = 0;
    let mut word_sum = 0;

    for line in INPUT.lines() {
        let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
        let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
        digit_sum += first * 10 + last;

        let mut first = None;
        let mut last = 0;
        let mut i = 0;
        while i < line.len() {
            let char = &line[i..i + 1];
            if let Ok(digit) = char.parse::<u32>() {
                if first.is_none() {
                    first = Some(digit)
                }
                last = digit;
            } else {
                for (digit, digit_str) in DIGIT_STR.iter().enumerate() {
                    if i + digit_str.len() > line.len() {
                        continue;
                    }
                    if &line[i..i + digit_str.len()] == *digit_str {
                        if first.is_none() {
                            first = Some(digit as u32)
                        }
                        last = digit as u32;
                        break;
                    }
                }
            }

            i += 1;
        }
        word_sum += first.unwrap() * 10 + last;
    }

    println!("{digit_sum}");
    println!("{word_sum}");
}
