use crate::puzzles::Solver;

pub fn get_solver() -> Solver {
    solver
}

use std::cmp::Ordering;

const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq)]
struct Box {
    x: i64,
    y: i64,
    z: i64,
}

fn distance(a: &Box, b: &Box) -> i64 {
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)
}

fn solver() -> (String, String) {
    let boxes = INPUT
        .lines()
        .map(|line| {
            let mut iter = line.split(",");
            let x = iter.next().unwrap().parse::<i64>().unwrap();
            let y = iter.next().unwrap().parse::<i64>().unwrap();
            let z = iter.next().unwrap().parse::<i64>().unwrap();
            Box { x, y, z }
        })
        .collect::<Vec<_>>();

    let mut distances = Vec::with_capacity(boxes.len() * (boxes.len() - 1) / 2);
    for a in 0..boxes.len() - 1 {
        for b in a + 1..boxes.len() {
            distances.push((a, b, distance(&boxes[a], &boxes[b])))
        }
    }
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits = boxes.iter().map(|b| vec![b]).collect::<Vec<_>>();

    let mut part_1 = 0;
    let mut part_2 = 0;

    for (i, (a, b, _)) in distances.iter().enumerate() {
        if i == 1000 {
            circuits.sort_by(|a, b| {
                if a.len() == b.len() {
                    Ordering::Equal
                } else if a.len() > b.len() {
                    Ordering::Greater
                } else if a.len() < b.len() {
                    Ordering::Less
                } else {
                    unreachable!()
                }
            });
            circuits.reverse();

            part_1 = circuits[0].len() * circuits[1].len() * circuits[2].len();
        }

        let a = &boxes[*a];
        let b = &boxes[*b];

        let circuit_a_index = circuits.iter().position(|c| c.contains(&a)).unwrap();
        let mut circuit_a = circuits.remove(circuit_a_index);

        if let Some(circuit_b_index) = circuits.iter().position(|c| c.contains(&b)) {
            let circuit_b = circuits.remove(circuit_b_index);
            circuit_a.extend(circuit_b);
        }

        circuits.push(circuit_a);

        if circuits.len() == 1 {
            part_2 = a.x * b.x;
            break;
        }
    }

    (part_1.to_string(), part_2.to_string())
}
