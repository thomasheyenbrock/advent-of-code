use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

type Cache<'a> = HashMap<&'a str, u64>;

fn count_combinations<'a>(cache: &mut Cache<'a>, patterns: &Vec<&str>, design: &'a str) -> u64 {
    if design.len() == 1 {
        return if patterns.contains(&design) { 1 } else { 0 };
    }

    if let Some(result) = cache.get(design) {
        return *result;
    }

    let mut result = 0;

    for pattern in patterns {
        if design == *pattern {
            result += 1;
        } else if design.starts_with(pattern) {
            result += count_combinations(cache, patterns, &design[pattern.len()..]);
        }
    }

    cache.insert(design, result);

    result
}

fn main() {
    let mut iter = INPUT.lines();
    let patterns = iter.next().unwrap().split(", ").collect::<Vec<_>>();

    let mut count = 0;
    let mut sum = 0;

    let mut cache = Cache::default();

    for design in iter.skip(1) {
        // Cache warming
        for i in 1..design.len() {
            let design = &design[0..i];
            count_combinations(&mut cache, &patterns, design);
        }

        let result = count_combinations(&mut cache, &patterns, design);
        println!("{design}: {result}");

        sum += result;
        if result > 0 {
            count += 1;
        }
    }

    println!("{count}");
    println!("{sum}");
}
