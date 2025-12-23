const INPUT: &str = include_str!("input.txt");

fn largest_of_size(digits: &[u64], length: usize) -> u64 {
    if length == 0 {
        return 0;
    }

    for d in (1..=9).rev() {
        let a_index = digits
            .iter()
            .enumerate()
            .find_map(|(i, battery)| (battery == &d).then_some(i));
        let a_index = match a_index {
            Some(i) => i,
            None => continue,
        };
        if a_index + length > digits.len() {
            continue;
        }

        let a = digits[a_index];
        let b = largest_of_size(&digits[(a_index + 1)..digits.len()], length - 1);
        return a * 10u64.pow(length as u32 - 1) + b;
    }

    unreachable!()
}

fn main() {
    let mut sum_2 = 0;
    let mut sum_12 = 0;

    for line in INPUT.lines() {
        let batteries = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>();

        sum_2 += largest_of_size(&batteries, 2);
        sum_12 += largest_of_size(&batteries, 12);
    }

    println!("{sum_2}");
    println!("{sum_12}");
}
