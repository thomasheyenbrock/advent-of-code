const INPUT: &str = include_str!("input.txt");

struct Grid {
    height: usize,
    width: usize,
    cells: Vec<Vec<bool>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let height = value.lines().count();
        let width = value.lines().next().unwrap().len();
        let cells = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '@' => true,
                        c => panic!("Unexpected char {c}"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            height,
            width,
            cells,
        }
    }
}

impl Grid {
    fn count_adjacent(&self, row: usize, col: usize) -> u32 {
        let mut count = 0;

        if row > 0 && col > 0 && self.cells[row - 1][col - 1] {
            count += 1;
        }
        if row > 0 && self.cells[row - 1][col] {
            count += 1;
        }
        if row > 0 && col < self.width - 1 && self.cells[row - 1][col + 1] {
            count += 1;
        }
        if col > 0 && self.cells[row][col - 1] {
            count += 1;
        }
        if col < self.width - 1 && self.cells[row][col + 1] {
            count += 1;
        }
        if row < self.height - 1 && col > 0 && self.cells[row + 1][col - 1] {
            count += 1;
        }
        if row < self.height - 1 && self.cells[row + 1][col] {
            count += 1;
        }
        if row < self.height - 1 && col < self.width - 1 && self.cells[row + 1][col + 1] {
            count += 1;
        }

        count
    }
}

fn main() {
    let mut grid = Grid::from(INPUT);

    let mut initial_count = 0;
    let mut total_count = 0;
    let mut is_first_loop = true;

    loop {
        let mut remove = vec![];

        for row in 0..grid.height {
            for col in 0..grid.width {
                if grid.cells[row][col] && grid.count_adjacent(row, col) < 4 {
                    remove.push((row, col));
                }
            }
        }

        if remove.len() == 0 {
            break;
        }

        total_count += remove.len();
        if is_first_loop {
            is_first_loop = false;
            initial_count = remove.len();
        }

        for (row, col) in remove {
            grid.cells[row][col] = false;
        }
    }

    println!("{initial_count}");
    println!("{total_count}");
}
