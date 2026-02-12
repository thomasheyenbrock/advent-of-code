use crate::puzzles::Solver;

pub fn get_solver() -> Solver {
    solver
}

const INPUT: &str = include_str!("input.txt");

struct Grid {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let cells = value
            .lines()
            .map(|line| line.chars().map(Cell::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            height: cells.len(),
            width: cells[0].len(),
            cells,
        }
    }
}

impl Grid {
    fn cell(&self, row: usize, col: usize) -> &Cell {
        &self.cells[row][col]
    }

    fn read_num(&self, row: usize, col: usize) -> Option<usize> {
        let mut col = col;

        let mut digits = vec![];
        while col < self.width {
            match self.cell(row, col) {
                Cell::Digit(d) => {
                    digits.push(*d);
                    col += 1;
                }
                _ => break,
            }
        }

        if digits.is_empty() {
            None
        } else {
            let mut num = 0;
            let mut pow = 1;
            while let Some(d) = digits.pop() {
                num += d * pow;
                pow *= 10;
            }

            Some(num)
        }
    }

    fn is_adjacent_to_symbol(&self, row: usize, start_col: usize, end_col: usize) -> bool {
        if start_col > 0 && self.cell(row, start_col - 1).is_symbol() {
            true
        } else if end_col < self.width && self.cell(row, end_col).is_symbol() {
            true
        } else if row > 0 && (start_col..end_col).any(|col| self.cell(row - 1, col).is_symbol()) {
            true
        } else if row > 0 && start_col > 0 && self.cell(row - 1, start_col - 1).is_symbol() {
            true
        } else if row > 0 && end_col < self.width && self.cell(row - 1, end_col).is_symbol() {
            true
        } else if row + 1 < self.height
            && (start_col..end_col).any(|col| self.cell(row + 1, col).is_symbol())
        {
            true
        } else if row + 1 < self.height
            && start_col > 0
            && self.cell(row + 1, start_col - 1).is_symbol()
        {
            true
        } else if row + 1 < self.height
            && end_col < self.width
            && self.cell(row + 1, end_col).is_symbol()
        {
            true
        } else {
            false
        }
    }
}

enum Cell {
    Empty,
    Symbol,
    Gear,
    Digit(usize),
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '0' => Self::Digit(0),
            '1' => Self::Digit(1),
            '2' => Self::Digit(2),
            '3' => Self::Digit(3),
            '4' => Self::Digit(4),
            '5' => Self::Digit(5),
            '6' => Self::Digit(6),
            '7' => Self::Digit(7),
            '8' => Self::Digit(8),
            '9' => Self::Digit(9),
            '*' => Self::Gear,
            _ => Self::Symbol,
        }
    }
}

impl Cell {
    fn is_symbol(&self) -> bool {
        matches!(self, Self::Gear | Self::Symbol)
    }

    fn is_gear(&self) -> bool {
        matches!(self, Self::Gear)
    }

    fn is_digit(&self) -> bool {
        matches!(self, Self::Digit(_))
    }
}

fn solver() -> (String, String) {
    let grid = Grid::from(INPUT);

    let mut sum_1 = 0;

    for row in 0..grid.height {
        let mut col = 0;

        while col < grid.width {
            if let Some(num) = grid.read_num(row, col) {
                let start_col = col;
                let end_col = col + num.to_string().len();

                if grid.is_adjacent_to_symbol(row, start_col, end_col) {
                    sum_1 += num;
                }

                col = end_col;
            } else {
                col += 1;
            }
        }
    }

    let mut sum_2 = 0;

    for row in 0..grid.height {
        for col in 0..grid.width {
            if !grid.cell(row, col).is_gear() {
                continue;
            }

            let mut adjacent = vec![];

            // Num in the same row before
            let mut start_col = col;
            while start_col > 0 && grid.cell(row, start_col - 1).is_digit() {
                start_col -= 1;
            }
            if start_col < col {
                adjacent.push(grid.read_num(row, start_col).unwrap());
            }

            // Num in the same row after
            if col + 1 < grid.width && grid.cell(row, col + 1).is_digit() {
                adjacent.push(grid.read_num(row, col + 1).unwrap());
            }

            // Check row above
            if row > 0 {
                if grid.cell(row - 1, col).is_digit() {
                    // If the cell right above is a digit there can at most be one adjacent number here
                    let mut start_col = col;
                    while start_col > 0 && grid.cell(row - 1, start_col - 1).is_digit() {
                        start_col -= 1;
                    }
                    adjacent.push(grid.read_num(row - 1, start_col).unwrap());
                } else {
                    // Check for adjacent number in top left
                    if col > 0 && grid.cell(row - 1, col - 1).is_digit() {
                        let mut start_col = col - 1;
                        while start_col > 0 && grid.cell(row - 1, start_col - 1).is_digit() {
                            start_col -= 1;
                        }
                        adjacent.push(grid.read_num(row - 1, start_col).unwrap());
                    }

                    // Check adjacent number in top right
                    if col + 1 < grid.width && grid.cell(row - 1, col + 1).is_digit() {
                        adjacent.push(grid.read_num(row - 1, col + 1).unwrap());
                    }
                }
            }

            // Check row below
            if row + 1 < grid.height {
                if grid.cell(row + 1, col).is_digit() {
                    // If the cell right below is a digit there can at most be one adjacent number here
                    let mut start_col = col;
                    while start_col > 0 && grid.cell(row + 1, start_col - 1).is_digit() {
                        start_col -= 1;
                    }
                    adjacent.push(grid.read_num(row + 1, start_col).unwrap());
                } else {
                    // Check for adjacent number in bottom left
                    if col > 0 && grid.cell(row + 1, col - 1).is_digit() {
                        let mut start_col = col - 1;
                        while start_col > 0 && grid.cell(row + 1, start_col - 1).is_digit() {
                            start_col -= 1;
                        }
                        adjacent.push(grid.read_num(row + 1, start_col).unwrap());
                    }

                    // Check adjacent number in bottom right
                    if col + 1 < grid.width && grid.cell(row + 1, col + 1).is_digit() {
                        adjacent.push(grid.read_num(row + 1, col + 1).unwrap());
                    }
                }
            }

            if adjacent.len() == 2 {
                sum_2 += adjacent[0] * adjacent[1];
            }
        }
    }

    (sum_1.to_string(), sum_2.to_string())
}
