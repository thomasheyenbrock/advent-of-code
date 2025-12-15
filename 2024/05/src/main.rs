const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut iter = INPUT.split("\n\n");
    let rules = iter.next().unwrap().lines().collect::<Vec<_>>();
    let updates = iter.next().unwrap();

    // PART 1
    let mut sum = 0;

    'validate: for update in updates.lines() {
        let numbers = update.split(",").collect::<Vec<_>>();

        for i in 0..(numbers.len() - 1) {
            for j in (i + 1)..numbers.len() {
                let rule = format!("{}|{}", numbers[j], numbers[i]);
                if rules.contains(&rule.as_str()) {
                    continue 'validate;
                }
            }
        }

        sum += numbers[(numbers.len() - 1) / 2]
            .parse::<u32>()
            .expect("Not a valid number")
    }

    println!("{sum}");

    // PART 2
    let mut sum = 0;

    for update in updates.lines() {
        let mut numbers = update.split(",").collect::<Vec<_>>();
        let mut is_valid = true;

        'outer: for i in 0..(numbers.len() - 1) {
            for j in (i + 1)..numbers.len() {
                let rule = format!("{}|{}", numbers[j], numbers[i]);
                if rules.contains(&rule.as_str()) {
                    is_valid = false;
                    break 'outer;
                }
            }
        }

        if is_valid {
            continue;
        }

        let mut ordered = Vec::with_capacity(numbers.len());
        while numbers.len() > 0 {
            let (index, num) = numbers
                .clone()
                .into_iter()
                .enumerate()
                .find(|(index, num)| {
                    for (i, n) in numbers.iter().enumerate() {
                        if &i == index {
                            continue;
                        }

                        let rule = format!("{}|{}", n, num);
                        if rules.contains(&rule.as_str()) {
                            return false;
                        }
                    }

                    return true;
                })
                .expect("No number that can be put in front");
            numbers.remove(index);
            ordered.push(num);
        }

        sum += ordered[(ordered.len() - 1) / 2]
            .parse::<u32>()
            .expect("Not a valid number")
    }

    println!("{sum}");
}
