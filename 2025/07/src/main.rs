use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>,
}

#[derive(PartialEq, Debug)]
enum Cell {
    Empty,
    Beam,
    Splitter,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let cells = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Cell::Empty,
                        'S' | '|' => Cell::Beam,
                        '^' => Cell::Splitter,
                        c => panic!("Unknown cell {c}"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let height = cells.len();
        let width = cells.iter().next().unwrap().len();
        Self {
            height,
            width,
            cells,
        }
    }
}

fn count_paths(
    grid: &Grid,
    (beam_row, beam_col): (usize, usize),
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if beam_row + 1 == grid.height {
        return 1;
    }

    if let Some(count) = cache.get(&(beam_row, beam_col)) {
        return *count;
    }

    let count = match grid.cells[beam_row + 1][beam_col] {
        Cell::Empty => count_paths(grid, (beam_row + 1, beam_col), cache),
        Cell::Splitter => {
            count_paths(grid, (beam_row + 1, beam_col - 1), cache)
                + count_paths(grid, (beam_row + 1, beam_col + 1), cache)
        }
        Cell::Beam => unreachable!(),
    };
    cache.insert((beam_row, beam_col), count);
    count
}

fn main() {
    let mut grid = Grid::from(INPUT);
    let mut count_splits = 0;

    for row in 1..grid.height {
        for col in 0..grid.width {
            if grid.cells[row - 1][col] != Cell::Beam {
                continue;
            }

            if grid.cells[row][col] == Cell::Splitter {
                grid.cells[row][col - 1] = Cell::Beam;
                grid.cells[row][col + 1] = Cell::Beam;
                count_splits += 1;
            } else {
                grid.cells[row][col] = Cell::Beam;
            }
        }
    }

    println!("{count_splits}");

    let grid = Grid::from(INPUT);
    let beam = grid
        .cells
        .iter()
        .enumerate()
        .find_map(|(row, cells)| {
            cells
                .iter()
                .enumerate()
                .find_map(|(col, cell)| (cell == &Cell::Beam).then_some(col))
                .map(|col| (row, col))
        })
        .unwrap();
    let mut cache = HashMap::new();
    let count_paths = count_paths(&grid, beam, &mut cache);

    println!("{count_paths}");
}
