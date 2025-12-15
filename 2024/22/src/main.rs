use std::collections::{hash_map::Entry, HashMap};

const INPUT: &str = include_str!("input.txt");

fn mix(num: u64, other: u64) -> u64 {
    num ^ other
}

fn prune(num: u64) -> u64 {
    num % 16777216
}

fn generate(num: u64) -> u64 {
    let num = prune(mix(num, num * 64));
    let num = prune(mix(num, num / 32));
    let num = prune(mix(num, num * 2048));
    num
}

fn pad(num: i8) -> String {
    if num < 0 {
        num.to_string()
    } else {
        format!("0{num}")
    }
}

#[derive(Debug)]
struct Buyer {
    nums: Vec<u64>,
    price_for_seq: HashMap<String, u64>,
}

impl Buyer {
    fn new(initial_secret_num: u64) -> Self {
        let mut nums = vec![initial_secret_num];
        for _ in 0..2000 {
            nums.push(generate(*nums.last().unwrap()));
        }

        let prices = nums.iter().map(|n| n % 10).collect::<Vec<_>>();

        let mut price_for_seq = HashMap::new();

        for i in 1..(prices.len() - 3) {
            let d1 = (prices[i] as i64) - (prices[i - 1] as i64);
            let d2 = (prices[i + 1] as i64) - (prices[i] as i64);
            let d3 = (prices[i + 2] as i64) - (prices[i + 1] as i64);
            let d4 = (prices[i + 3] as i64) - (prices[i + 2] as i64);
            let seq = format!(
                "{}{}{}{}",
                pad(d1 as i8),
                pad(d2 as i8),
                pad(d3 as i8),
                pad(d4 as i8)
            );
            match price_for_seq.entry(seq) {
                Entry::Occupied(_) => {}
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(prices[i + 3]);
                }
            }
        }

        Self {
            nums,
            price_for_seq,
        }
    }
}

fn main() {
    let buyers = INPUT
        .lines()
        .map(|initial_secret_num| Buyer::new(initial_secret_num.parse::<u64>().unwrap()))
        .collect::<Vec<_>>();

    // PART 1
    println!(
        "{}",
        buyers
            .iter()
            .map(|buyer| buyer.nums.last().unwrap())
            .sum::<u64>()
    );

    // PART 2
    let mut max = 0;

    for a in -9..10 {
        for b in -9..10 {
            for c in -9..10 {
                for d in -9..10 {
                    let seq = format!("{}{}{}{}", pad(a), pad(b), pad(c), pad(d));

                    let mut total = 0;
                    for buyer in buyers.iter() {
                        total += *buyer.price_for_seq.get(&seq).unwrap_or(&0);
                    }
                    if total > max {
                        max = total;
                    }
                }
            }
        }
    }

    println!("{max}");
}
