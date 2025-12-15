use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

const INPUT: &str = include_str!("input.txt");

struct Graph {
    vertices: HashSet<Vertex>,
    edges: HashSet<Edge>,
}

impl Graph {
    fn new() -> Self {
        let mut vertices = HashSet::new();
        let mut edges = HashSet::new();

        for line in INPUT.lines() {
            let mut iter = line.split("-");
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();

            vertices.insert(Vertex(a.to_string()));
            vertices.insert(Vertex(b.to_string()));

            edges.insert(Edge(Vertex(a.to_string()), Vertex(b.to_string())));
        }

        Self { vertices, edges }
    }

    fn neighbors(&self, vertex: &Vertex) -> Vec<Vertex> {
        self.edges
            .iter()
            .filter_map(|edge| edge.contains(vertex))
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Vertex(String);

#[derive(Eq, Hash)]
struct Edge(Vertex, Vertex);

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Edge {
    fn contains(&self, other: &Vertex) -> Option<Vertex> {
        if &self.0 == other {
            Some(self.1.clone())
        } else if &self.1 == other {
            Some(self.0.clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Eq)]
struct Clique(Vec<Vertex>);

impl From<HashSet<Vertex>> for Clique {
    fn from(value: HashSet<Vertex>) -> Self {
        Self(value.into_iter().collect())
    }
}

impl ToString for Clique {
    fn to_string(&self) -> String {
        let mut list = self.0.iter().map(|v| v.0.clone()).collect::<Vec<_>>();
        list.sort();
        list.join(",")
    }
}

impl PartialEq for Clique {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Hash for Clique {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

fn bron_kerbosch(
    max_cliques: &mut HashSet<Clique>,
    graph: &Graph,
    r: HashSet<Vertex>,
    mut p: HashSet<Vertex>,
    mut x: HashSet<Vertex>,
) {
    if p.is_empty() && x.is_empty() {
        max_cliques.insert(r.clone().into());
    }
    for v in p.clone() {
        {
            let mut r = r.clone();
            r.insert(v.clone());

            let n = graph.neighbors(&v).into_iter().collect::<HashSet<_>>();

            let p = p
                .clone()
                .intersection(&n)
                .map(ToOwned::to_owned)
                .collect::<HashSet<_>>();

            let x = x
                .clone()
                .intersection(&n)
                .map(ToOwned::to_owned)
                .collect::<HashSet<_>>();

            bron_kerbosch(max_cliques, graph, r, p, x);
        }

        p.remove(&v);
        x.insert(v);
    }
}

fn main() {
    let graph = Graph::new();

    // PART 1
    let mut cliques = HashSet::new();

    for a in graph.vertices.iter() {
        let neighbors_a = graph.neighbors(&a);
        for b in neighbors_a.iter() {
            for c in graph.neighbors(&b) {
                if &c != a && neighbors_a.contains(&c) {
                    cliques.insert(Clique(vec![a.clone(), b.clone(), c.clone()]));
                }
            }
        }
    }

    println!(
        "{}",
        cliques
            .iter()
            .filter(|c| c.0.iter().any(|v| v.0.starts_with("t")))
            .count()
    );

    // PART 2
    let r = HashSet::new();
    let p = graph.vertices.clone();
    let x = HashSet::new();
    let mut max_cliques = HashSet::new();
    bron_kerbosch(&mut max_cliques, &graph, r, p, x);

    println!(
        "{}",
        max_cliques
            .into_iter()
            .max_by_key(|c| c.0.len())
            .unwrap()
            .to_string()
    );
}
