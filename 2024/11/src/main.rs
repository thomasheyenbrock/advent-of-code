use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

struct Program {
    cache: HashMap<u8, HashMap<u64, u64>>,
}

impl Program {
    fn new() -> Self {
        Self {
            cache: HashMap::default(),
        }
    }

    fn prime_cache(&mut self, iterations: u8) {
        for i in 1..=iterations {
            self.cache.insert(i, HashMap::default());
        }
    }

    fn compute_len(&mut self, num: u64, iterations: u8) -> u64 {
        if iterations == 0 {
            return 1;
        }

        if let Some(count) = self.cache.get(&iterations).and_then(|map| map.get(&num)) {
            return *count;
        };

        let count = self.uncached_compute(num, iterations);

        if let Some(cache) = self.cache.get_mut(&iterations) {
            cache.insert(num, count);
        }

        count
    }

    fn uncached_compute(&mut self, num: u64, iterations: u8) -> u64 {
        if num == 0 {
            return self.compute_len(1, iterations - 1);
        }

        let str = num.to_string();
        let len = str.len();
        if len % 2 == 0 {
            let first = *&str[0..(len / 2)].parse::<u64>().unwrap();
            let second = *&str[(len / 2)..].parse::<u64>().unwrap();
            return self.compute_len(first, iterations - 1)
                + self.compute_len(second, iterations - 1);
        }

        self.compute_len(num * 2024, iterations - 1)
    }
}

fn main() {
    // PART 1
    let mut stones = INPUT
        .trim()
        .split(" ")
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        let mut i = 0;
        while i < stones.len() {
            if stones[i] == 0 {
                stones[i] = 1;
                i += 1;
                continue;
            }

            let str = stones[i].to_string();
            let len = str.len();
            if len % 2 == 0 {
                stones[i] = *&str[0..(len / 2)].parse::<u64>().unwrap();
                stones.insert(i + 1, *&str[(len / 2)..].parse::<u64>().unwrap());
                i += 2;
                continue;
            }

            stones[i] = stones[i] * 2024;
            i += 1;
        }
    }

    println!("{}", stones.len());

    // PART 2
    let mut stones = HashMap::<u64, u64>::new();
    for stone in INPUT.trim().split(" ") {
        let key = stone.parse::<u64>().unwrap();
        let val = stones.get(&key).unwrap_or(&0);
        stones.insert(key, val + 1);
    }

    for _ in 0..75 {
        let mut new_stones = HashMap::<u64, u64>::new();

        for (stone, count) in stones {
            if stone == 0 {
                let val = *new_stones.get(&1).unwrap_or(&0);
                new_stones.insert(1, val + count);
                continue;
            }

            let str = stone.to_string();
            let len = str.len();
            if len % 2 == 0 {
                let first = *&str[0..(len / 2)].parse::<u64>().unwrap();
                let val = *new_stones.get(&first).unwrap_or(&0);
                new_stones.insert(first, val + count);

                let second = *&str[(len / 2)..].parse::<u64>().unwrap();
                let val = *new_stones.get(&second).unwrap_or(&0);
                new_stones.insert(second, val + count);

                continue;
            }

            let mult = stone * 2024;
            let val = *new_stones.get(&mult).unwrap_or(&0);
            new_stones.insert(mult, val + count);
        }

        stones = new_stones;
    }

    println!("{}", stones.values().sum::<u64>());

    // PART 2 (did it again because I fucked up the first try because I used
    // u32 which caused an overflow that didn't crash the program because I
    // ran it in --release mode)
    let mut program = Program::new();
    program.prime_cache(75);
    println!(
        "{}",
        INPUT
            .trim()
            .split(" ")
            .map(|str| str.parse::<u64>().unwrap())
            .map(|num| program.compute_len(num, 75))
            .sum::<u64>()
    );
}
