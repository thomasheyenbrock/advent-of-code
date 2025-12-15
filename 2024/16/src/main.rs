use std::{
    collections::{HashMap, HashSet},
    u64,
};

const INPUT: &str = include_str!("input.txt");

type Node = (usize, usize);

struct Maze {
    grid: Vec<Vec<char>>,
    start: Node,
    end: Node,
    rows: usize,
    cols: usize,
}

impl Maze {
    fn new() -> Self {
        let mut start = None;
        let mut end = None;

        let grid = INPUT
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        if c == 'S' {
                            start = Some((i, j))
                        } else if c == 'E' {
                            end = Some((i, j))
                        }
                        c
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let rows = grid.len();
        let cols = grid[0].len();

        Self {
            grid,
            start: start.unwrap(),
            end: end.unwrap(),
            rows,
            cols,
        }
    }

    fn get(&self, node: &Node) -> char {
        self.grid[node.0][node.1]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Graph {
    vertices: Vec<Vertex>,
    edges: HashSet<Edge>,
}

impl Graph {
    fn neighbors(&self, v: &Vertex) -> Vec<Vertex> {
        self.edges
            .iter()
            .filter_map(|edge| (&edge.from == v).then_some(edge.to))
            .collect::<Vec<_>>()
    }

    fn edge_cost(&self, from: &Vertex, to: &Vertex) -> u64 {
        self.edges
            .iter()
            .find(|edge| &edge.from == from && &edge.to == to)
            .unwrap()
            .cost
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vertex {
    row: usize,
    col: usize,
    dir: Direction,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Edge {
    from: Vertex,
    to: Vertex,
    cost: u64,
}

impl Edge {
    fn new(from: Vertex, to: Vertex, cost: u64) -> Self {
        Self { from, to, cost }
    }
}

fn get_paths(prev: &HashMap<Vertex, Vec<Vertex>>, v: &Vertex) -> Vec<Vec<Vertex>> {
    let p = prev.get(v).unwrap();

    if p.is_empty() {
        return vec![vec![v.clone()]];
    }

    p.into_iter()
        .flat_map(|p| {
            get_paths(prev, p)
                .into_iter()
                .map(|mut path| {
                    path.push(v.clone());
                    path
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let maze = Maze::new();

    // PART 1
    let mut graph = Graph {
        vertices: vec![],
        edges: HashSet::new(),
    };
    for row in 1..(maze.rows - 1) {
        for col in 1..(maze.cols - 1) {
            if maze.get(&(row, col)) == '#' {
                continue;
            }

            let up = Vertex {
                row,
                col,
                dir: Direction::Up,
            };
            let right = Vertex {
                row,
                col,
                dir: Direction::Right,
            };
            let down = Vertex {
                row,
                col,
                dir: Direction::Down,
            };
            let left = Vertex {
                row,
                col,
                dir: Direction::Left,
            };

            graph.vertices.push(up.clone());
            graph.vertices.push(right.clone());
            graph.vertices.push(down.clone());
            graph.vertices.push(left.clone());

            graph
                .edges
                .insert(Edge::new(up.clone(), right.clone(), 1000));
            graph
                .edges
                .insert(Edge::new(right.clone(), up.clone(), 1000));

            graph
                .edges
                .insert(Edge::new(right.clone(), down.clone(), 1000));
            graph
                .edges
                .insert(Edge::new(down.clone(), right.clone(), 1000));

            graph
                .edges
                .insert(Edge::new(down.clone(), left.clone(), 1000));
            graph
                .edges
                .insert(Edge::new(left.clone(), down.clone(), 1000));

            graph
                .edges
                .insert(Edge::new(left.clone(), up.clone(), 1000));
            graph
                .edges
                .insert(Edge::new(up.clone(), left.clone(), 1000));

            if maze.get(&(row - 1, col)) != '#' {
                graph.edges.insert(Edge::new(
                    up.clone(),
                    Vertex {
                        row: row - 1,
                        col,
                        dir: Direction::Up,
                    },
                    1,
                ));
            }
            if maze.get(&(row, col + 1)) != '#' {
                graph.edges.insert(Edge::new(
                    right.clone(),
                    Vertex {
                        row,
                        col: col + 1,
                        dir: Direction::Right,
                    },
                    1,
                ));
            }
            if maze.get(&(row + 1, col)) != '#' {
                graph.edges.insert(Edge::new(
                    down.clone(),
                    Vertex {
                        row: row + 1,
                        col,
                        dir: Direction::Down,
                    },
                    1,
                ));
            }
            if maze.get(&(row, col - 1)) != '#' {
                graph.edges.insert(Edge::new(
                    left.clone(),
                    Vertex {
                        row,
                        col: col - 1,
                        dir: Direction::Left,
                    },
                    1,
                ));
            }
        }
    }

    let source = Vertex {
        row: maze.start.0,
        col: maze.start.1,
        dir: Direction::Right,
    };
    let mut dist = HashMap::<Vertex, u64>::new();
    let mut prev = HashMap::<Vertex, Vec<Vertex>>::new();
    let mut q = HashSet::<Vertex>::new();

    for v in graph.vertices.iter() {
        dist.insert(v.clone(), u64::MAX);
        prev.insert(v.clone(), vec![]);
        q.insert(v.clone());
    }
    dist.insert(source.clone(), 0);

    while !q.is_empty() {
        println!("{}", q.len());
        let mut u = None;
        for v in q.iter() {
            match u {
                Some(u) if dist.get(&u) <= dist.get(v) => {}
                _ => u = Some(v.clone()),
            }
        }
        let u = u.unwrap();
        q.remove(&u);

        for v in graph.neighbors(&u) {
            if !q.contains(&v) {
                continue;
            }

            let alt = dist.get(&u).unwrap() + graph.edge_cost(&u, &v);

            if &alt <= dist.get(&v).unwrap() {
                dist.insert(v, alt);
                let p = prev.get_mut(&v).unwrap();
                p.push(u);
            }
        }
    }

    let up = dist
        .get(&Vertex {
            row: maze.end.0,
            col: maze.end.1,
            dir: Direction::Up,
        })
        .unwrap();
    let right = dist
        .get(&Vertex {
            row: maze.end.0,
            col: maze.end.1,
            dir: Direction::Right,
        })
        .unwrap();
    let down = dist
        .get(&Vertex {
            row: maze.end.0,
            col: maze.end.1,
            dir: Direction::Down,
        })
        .unwrap();
    let left = dist
        .get(&Vertex {
            row: maze.end.0,
            col: maze.end.1,
            dir: Direction::Left,
        })
        .unwrap();

    println!("up: {up}");
    println!("right: {right}");
    println!("down: {down}");
    println!("left: {left}");

    let min = up.min(right).min(down).min(left);
    println!("min: {min}");

    // PART 2
    let mut paths = vec![];

    if up == min {
        paths.extend(get_paths(
            &prev,
            &Vertex {
                row: maze.end.0,
                col: maze.end.1,
                dir: Direction::Up,
            },
        ))
    }
    if right == min {
        paths.extend(get_paths(
            &prev,
            &Vertex {
                row: maze.end.0,
                col: maze.end.1,
                dir: Direction::Right,
            },
        ))
    }
    if down == min {
        paths.extend(get_paths(
            &prev,
            &Vertex {
                row: maze.end.0,
                col: maze.end.1,
                dir: Direction::Down,
            },
        ))
    }
    if left == min {
        paths.extend(get_paths(
            &prev,
            &Vertex {
                row: maze.end.0,
                col: maze.end.1,
                dir: Direction::Left,
            },
        ))
    }

    let mut tiles = HashSet::new();
    for path in paths {
        for node in path {
            tiles.insert((node.row, node.col));
        }
    }

    println!("{}", tiles.len())
}
