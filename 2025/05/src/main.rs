const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut iter = INPUT.split("\n\n");
    let ranges = iter.next().unwrap();
    let ingredients = iter.next().unwrap();

    let mut ranges = ranges
        .lines()
        .map(|line| {
            let mut iter = line.split("-");
            let start = iter.next().unwrap().parse::<u64>().unwrap();
            let end = iter.next().unwrap().parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<Vec<_>>();

    let mut count = 0;

    for ingredient in ingredients.lines() {
        let ingredient = ingredient.parse::<u64>().unwrap();
        for (start, end) in ranges.iter() {
            if &ingredient >= start && &ingredient <= end {
                count += 1;
                break;
            }
        }
    }

    println!("{count}");

    'overlaps: loop {
        for i in 0..ranges.len() - 1 {
            for j in i + 1..ranges.len() {
                let (a_start, a_end) = ranges[i];
                let (b_start, b_end) = ranges[j];

                if a_start <= b_start && b_start <= a_end {
                    ranges.remove(j);
                    ranges.remove(i);
                    ranges.push((a_start, a_end.max(b_end)));
                    continue 'overlaps;
                }

                if b_start <= a_start && a_start <= b_end {
                    ranges.remove(j);
                    ranges.remove(i);
                    ranges.push((b_start, b_end.max(a_end)));
                    continue 'overlaps;
                }
            }
        }
        break;
    }

    let count = ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum::<u64>();
    println!("{count}");
}
