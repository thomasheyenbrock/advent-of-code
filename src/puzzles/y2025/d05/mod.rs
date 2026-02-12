use crate::puzzles::Solver;

pub fn get_solver() -> Solver {
    solver
}

const INPUT: &str = include_str!("input.txt");

fn solver() -> (String, String) {
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

    let mut count_1 = 0;

    for ingredient in ingredients.lines() {
        let ingredient = ingredient.parse::<u64>().unwrap();
        for (start, end) in ranges.iter() {
            if &ingredient >= start && &ingredient <= end {
                count_1 += 1;
                break;
            }
        }
    }

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

    let count_2 = ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum::<u64>();

    (count_1.to_string(), count_2.to_string())
}
