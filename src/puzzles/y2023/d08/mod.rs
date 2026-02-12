use crate::puzzles::Solver;

pub fn get_solver() -> Solver {
    solver
}

use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

const START: &str = "AAA";
const STOP: &str = "ZZZ";

enum Direction {
    Left,
    Right,
}

struct Node {
    left: &'static str,
    right: &'static str,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a == b {
        a
    } else if a > b {
        gcd(a - b, b)
    } else {
        gcd(a, b - a)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn solver() -> (String, String) {
    let mut iter = INPUT.lines();

    let directions = iter
        .next()
        .unwrap()
        .chars()
        .map(Direction::from)
        .collect::<Vec<_>>();
    iter.next();

    let nodes = HashMap::<&'static str, Node>::from_iter(iter.map(|row| {
        (
            &row[0..3],
            Node {
                left: &row[7..10],
                right: &row[12..15],
            },
        )
    }));

    let mut steps = 0;
    let mut current = START;

    while current != STOP {
        for direction in directions.iter() {
            let node = nodes.get(current).unwrap();
            current = if matches!(direction, Direction::Left) {
                node.left
            } else {
                node.right
            };
        }
        steps += 1;
    }

    let part_1 = steps * directions.len();

    let count = nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .fold(1, |count, start| {
            let mut steps = 0;
            let mut current = *start;

            while !current.ends_with("Z") {
                for direction in directions.iter() {
                    let node = nodes.get(current).unwrap();
                    current = if matches!(direction, Direction::Left) {
                        node.left
                    } else {
                        node.right
                    };
                }
                steps += 1;
            }

            lcm(count, steps)
        });

    let part_2 = count * directions.len();

    (part_1.to_string(), part_2.to_string())
}
