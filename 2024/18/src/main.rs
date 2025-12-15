use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

const SIZE: usize = 71;

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
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Edge {
    from: Vertex,
    to: Vertex,
    cost: u64,
}

impl Edge {
    fn new(from: Vertex, to: Vertex) -> Self {
        Self { from, to, cost: 1 }
    }
}

fn shortest_path(coordinates: &Vec<(usize, usize)>, count: usize, log: bool) -> Option<u64> {
    let mut grid = (0..SIZE)
        .map(|_| (0..SIZE).map(|_| '.').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (x, y) in coordinates.iter().take(count) {
        grid[*x][*y] = '#';
    }

    let mut graph = Graph {
        vertices: vec![],
        edges: HashSet::new(),
    };

    for x in 0..SIZE {
        for y in 0..SIZE {
            if grid[x][y] == '#' {
                continue;
            }

            graph.vertices.push(Vertex { x, y });

            if x > 0 && grid[x - 1][y] != '#' {
                graph.edges.insert(Edge {
                    from: Vertex { x, y },
                    to: Vertex { x: x - 1, y },
                    cost: 1,
                });
                graph.edges.insert(Edge {
                    from: Vertex { x: x - 1, y },
                    to: Vertex { x, y },
                    cost: 1,
                });
            }

            if x < SIZE - 1 && grid[x + 1][y] != '#' {
                graph
                    .edges
                    .insert(Edge::new(Vertex { x, y }, Vertex { x: x + 1, y }));
                graph
                    .edges
                    .insert(Edge::new(Vertex { x: x + 1, y }, Vertex { x, y }));
            }

            if y > 0 && grid[x][y - 1] != '#' {
                graph
                    .edges
                    .insert(Edge::new(Vertex { x, y }, Vertex { x, y: y - 1 }));
                graph
                    .edges
                    .insert(Edge::new(Vertex { x, y: y - 1 }, Vertex { x, y }));
            }

            if y < SIZE - 1 && grid[x][y + 1] != '#' {
                graph
                    .edges
                    .insert(Edge::new(Vertex { x, y }, Vertex { x, y: y + 1 }));
                graph
                    .edges
                    .insert(Edge::new(Vertex { x, y: y + 1 }, Vertex { x, y }));
            }
        }
    }

    let source = Vertex { x: 0, y: 0 };
    let mut dist = HashMap::<Vertex, u64>::new();
    let mut prev = HashMap::<Vertex, Vec<Vertex>>::new();
    let mut q = HashSet::<Vertex>::new();

    for v in graph.vertices.iter() {
        prev.insert(v.clone(), vec![]);
        q.insert(v.clone());
    }
    dist.insert(source.clone(), 0);

    while !q.is_empty() {
        if log {
            println!("{}", q.len());
        }

        let mut u = None;
        for v in q.iter() {
            let dist_v = match dist.get(v) {
                Some(d) => d,
                None => continue,
            };

            match u {
                Some(uu) => {
                    let dist_u = dist.get(&uu).unwrap();
                    if dist_v < dist_u {
                        u = Some(v.clone());
                    }
                }
                None => u = Some(v.clone()),
            }
        }
        let u = match u {
            Some(u) => u,
            None => break,
        };
        q.remove(&u);

        for v in graph.neighbors(&u) {
            if !q.contains(&v) {
                continue;
            }

            match dist.get(&u) {
                Some(dist_u) => {
                    let alt = dist_u + graph.edge_cost(&u, &v);
                    if dist.get(&v).map_or(true, |dist_v| &alt <= dist_v) {
                        dist.insert(v, alt);
                        let p = prev.get_mut(&v).unwrap();
                        p.push(u);
                    }
                }
                None => {}
            };
        }
    }

    dist.get(&Vertex {
        x: SIZE - 1,
        y: SIZE - 1,
    })
    .map(|n| *n)
}

fn main() {
    let coordinates = INPUT
        .lines()
        .map(|line| {
            let mut iter = line.split(",").map(|str| str.parse::<usize>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect::<Vec<_>>();

    // PART 1
    println!("{}", shortest_path(&coordinates, 1024, true).unwrap());

    // PART 2
    for i in 1025..=INPUT.lines().count() {
        match shortest_path(&coordinates, i, false) {
            Some(result) => {
                println!("{i}: {result}");
            }
            None => {
                println!("{i}: NO PATH!");
                println!("{:?}", coordinates[i - 1]);
                break;
            }
        }
    }
}
