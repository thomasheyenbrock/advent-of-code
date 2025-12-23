const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut sum_exactly_twice = 0;
    let mut sum_more_than_twice = 0;

    for range in INPUT.split(",") {
        let mut iter = range.split("-");
        let start = iter.next().unwrap().parse::<u64>().unwrap();
        let end = iter.next().unwrap().parse::<u64>().unwrap();

        'numbers: for i in start..=end {
            let str = i.to_string();
            let len = str.len();

            'divisors: for d in (1..=len / 2).rev() {
                if len % d != 0 {
                    continue;
                }

                let expected = &str[0..d];
                for chunk in 1..len / d {
                    if &str[chunk * d..(chunk + 1) * d] != expected {
                        continue 'divisors;
                    }
                }

                sum_more_than_twice += i;
                if d * 2 == len {
                    sum_exactly_twice += i;
                }

                continue 'numbers;
            }
        }
    }

    println!("{sum_exactly_twice}");
    println!("{sum_more_than_twice}");
}
