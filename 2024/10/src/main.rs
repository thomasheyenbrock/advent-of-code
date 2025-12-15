use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

struct Map {
    grid: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn new() -> Self {
        let grid = INPUT
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let rows = grid.len();
        let cols = grid[0].len();

        Self { grid, rows, cols }
    }
}

fn find_paths(map: &Map, i: isize, j: isize, height: u8) -> Vec<(isize, isize)> {
    if i < 0 || i >= map.rows as isize || j < 0 || j >= map.cols as isize {
        vec![]
    } else if map.grid[i as usize][j as usize] != height {
        vec![]
    } else if height == 9 {
        vec![(i, j)]
    } else {
        let mut list = vec![];
        list.extend(find_paths(map, i + 1, j, height + 1));
        list.extend(find_paths(map, i - 1, j, height + 1));
        list.extend(find_paths(map, i, j + 1, height + 1));
        list.extend(find_paths(map, i, j - 1, height + 1));
        list
    }
}

fn main() {
    let map = Map::new();

    let mut score = 0;
    let mut rating = 0;

    for i in 0..map.rows {
        for j in 0..map.cols {
            let paths = find_paths(&map, i as isize, j as isize, 0);
            rating += paths.len();
            score += HashSet::<(isize, isize)>::from_iter(paths.into_iter()).len();
        }
    }

    println!("{score}");
    println!("{rating}");
}
