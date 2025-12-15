const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Area {
    cells: Vec<(usize, usize)>,
}

impl Area {
    fn from(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Self {
        let mut cells = vec![];
        Self::collect_cells(&mut cells, grid[i][j], grid, i, j);
        Self { cells }
    }

    fn collect_cells(
        cells: &mut Vec<(usize, usize)>,
        c: char,
        grid: &Vec<Vec<char>>,
        i: usize,
        j: usize,
    ) {
        if grid[i][j] != c {
            return;
        }
        if cells.iter().any(|cell| cell.0 == i && cell.1 == j) {
            return;
        }
        cells.push((i, j));
        if i > 0 {
            Self::collect_cells(cells, c, grid, i - 1, j);
        }
        if i < grid.len() - 1 {
            Self::collect_cells(cells, c, grid, i + 1, j);
        }
        if j > 0 {
            Self::collect_cells(cells, c, grid, i, j - 1);
        }
        if j < grid[0].len() - 1 {
            Self::collect_cells(cells, c, grid, i, j + 1);
        }
    }

    fn contains(&self, i: usize, j: usize) -> bool {
        self.cells.iter().any(|cell| cell.0 == i && cell.1 == j)
    }

    fn area(&self) -> u64 {
        self.cells.len() as u64
    }

    fn perimiter(&self) -> u64 {
        self.cells
            .iter()
            .map(|(i, j)| {
                let mut count = 0;

                if *i == 0 || !self.contains(i - 1, *j) {
                    count += 1;
                }

                if !self.contains(i + 1, *j) {
                    count += 1;
                }

                if *j == 0 || !self.contains(*i, j - 1) {
                    count += 1;
                }

                if !self.contains(*i, j + 1) {
                    count += 1;
                }

                count
            })
            .sum::<u64>()
    }

    fn sides(&self) -> u64 {
        // Counting sides is the same as counting corners
        self.cells
            .iter()
            .map(|(i, j)| {
                let mut convex_count = 0;

                let is_down_border = *i == 0 || !self.contains(i - 1, *j);
                if is_down_border {
                    convex_count += 1;
                }

                let is_up_border = !self.contains(i + 1, *j);
                if is_up_border {
                    convex_count += 1;
                }

                let is_left_border = *j == 0 || !self.contains(*i, j - 1);
                if is_left_border {
                    convex_count += 1;
                }
                let is_right_border = !self.contains(*i, j + 1);
                if is_right_border {
                    convex_count += 1;
                }

                let convex_count = match convex_count {
                    0 | 1 => 0,
                    2 if is_up_border && is_down_border => 0,
                    2 if is_left_border && is_right_border => 0,
                    2 => 1,
                    3 => 2,
                    4 => 4,
                    _ => unreachable!(),
                };

                let mut concave_count = 0;

                if self.contains(i + 1, *j)
                    && self.contains(*i, j + 1)
                    && !self.contains(i + 1, j + 1)
                {
                    concave_count += 1;
                }

                if *j > 0
                    && self.contains(i + 1, *j)
                    && self.contains(*i, j - 1)
                    && !self.contains(i + 1, j - 1)
                {
                    concave_count += 1;
                }

                if *i > 0
                    && *j > 0
                    && self.contains(i - 1, *j)
                    && self.contains(*i, j - 1)
                    && !self.contains(i - 1, j - 1)
                {
                    concave_count += 1;
                }

                if *i > 0
                    && self.contains(i - 1, *j)
                    && self.contains(*i, j + 1)
                    && !self.contains(i - 1, j + 1)
                {
                    concave_count += 1;
                }

                convex_count + concave_count
            })
            .sum::<u64>()
    }
}

fn main() {
    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut areas: Vec<Area> = vec![];

    while let Some((i, j)) = grid.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(|(j, _)| {
            areas
                .iter()
                .all(|area| !area.contains(i, j))
                .then_some((i, j))
        })
    }) {
        println!("{i},{j}");
        areas.push(Area::from(&grid, i, j));
    }

    // PART 1
    println!(
        "{}",
        areas
            .iter()
            .map(|area| area.area() * area.perimiter())
            .sum::<u64>()
    );

    // PART 2
    println!(
        "{}",
        areas
            .iter()
            .map(|area| area.area() * area.sides())
            .sum::<u64>()
    );
}
