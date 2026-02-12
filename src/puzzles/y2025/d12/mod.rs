use crate::puzzles::Solver;

pub fn get_solver() -> Solver {
    solver
}

const INPUT: &str = include_str!("input.txt");

struct Tree {
    width: usize,
    height: usize,
    present_counts: Vec<usize>,
}

const FACTORS: &[usize] = &[7, 7, 5, 7, 6, 7];

fn solver() -> (String, String) {
    let mut trees = vec![];

    let mut iter = INPUT.split("\n\n").peekable();
    while let Some(item) = iter.next() {
        if iter.peek().is_none() {
            for line in item.lines() {
                let mut iter = line.split(" ");

                let mut dim = iter.next().unwrap().split("x");
                let height = dim.next().unwrap().parse::<usize>().unwrap();
                let width_str = dim.next().unwrap();
                let width_str = &width_str[0..width_str.len() - 1];
                let width = width_str.parse::<usize>().unwrap();

                trees.push(Tree {
                    height,
                    width,
                    present_counts: iter.map(|s| s.parse::<usize>().unwrap()).collect(),
                })
            }
        }
    }

    let mut count = 0;

    for tree in trees {
        let tiled_size = (tree.width / 3) * (tree.height / 3);
        let tile_sum = tree.present_counts.iter().sum::<usize>();
        if tile_sum <= tiled_size {
            count += 1;
            continue;
        }

        let spot_sum = tree
            .present_counts
            .iter()
            .zip(FACTORS.iter())
            .fold(0, |sum, (count, factor)| sum + count * factor);
        let size = tree.width * tree.height;
        if spot_sum > size {
            continue;
        }

        panic!();
    }

    (count.to_string(), String::from("-"))
}
