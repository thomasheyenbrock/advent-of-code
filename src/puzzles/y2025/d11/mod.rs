use crate::puzzles::Solver;

pub fn get_solver() -> Solver {
    solver
}

use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

struct Graph {
    edges: Vec<(&'static str, &'static str)>,
}

impl From<&'static str> for Graph {
    fn from(value: &'static str) -> Self {
        let mut edges = vec![];

        for line in value.lines() {
            let mut iter = line.split(" ");

            let node = iter.next().unwrap();
            let node = &node[0..node.len() - 1];

            edges.extend(iter.map(|to| (node, to)));
        }

        Self { edges }
    }
}

impl Graph {
    fn count_paths(
        &self,
        from: &'static str,
        to: &'static str,
        cache: &mut HashMap<&'static str, usize>,
    ) -> usize {
        if from == to {
            1
        } else if let Some(count) = cache.get(from) {
            *count
        } else {
            let count = self.edges.iter().fold(0, |sum, edge| {
                if edge.0 == from {
                    sum + self.count_paths(edge.1, to, cache)
                } else {
                    sum
                }
            });
            cache.insert(from, count);
            count
        }
    }
}

fn solver() -> (String, String) {
    let graph = Graph::from(INPUT);
    let part_1 = graph.count_paths("you", "out", &mut HashMap::new());

    let p1 = graph.count_paths("svr", "dac", &mut HashMap::new());
    let p2 = graph.count_paths("dac", "fft", &mut HashMap::new());
    let p3 = graph.count_paths("fft", "out", &mut HashMap::new());

    let q1 = graph.count_paths("svr", "fft", &mut HashMap::new());
    let q2 = graph.count_paths("fft", "dac", &mut HashMap::new());
    let q3 = graph.count_paths("dac", "out", &mut HashMap::new());

    let part_2 = p1 * p2 * p3 + q1 * q2 * q3;

    (part_1.to_string(), part_2.to_string())
}
