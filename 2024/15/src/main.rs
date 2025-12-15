const INPUT: &str = include_str!("input.txt");

struct Warehouse1 {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    robot: (usize, usize),
}

impl From<&str> for Warehouse1 {
    fn from(s: &str) -> Self {
        let grid = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rows = grid.len();
        let cols = grid[0].len();
        let robot = grid
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(j, c)| (*c == '@').then_some((i, j)))
            })
            .unwrap();
        Self {
            grid,
            rows,
            cols,
            robot,
        }
    }
}

impl Warehouse1 {
    fn r#move(&mut self, direction: char) {
        let mut boxes = 0;
        match direction {
            '^' => {
                while self.robot.0 >= boxes + 1
                    && self.grid[self.robot.0 - boxes - 1][self.robot.1] != '.'
                {
                    if self.grid[self.robot.0 - boxes - 1][self.robot.1] == '#' {
                        return;
                    }
                    boxes += 1;
                }
            }
            'v' => {
                while self.robot.0 + boxes + 1 < self.rows
                    && self.grid[self.robot.0 + boxes + 1][self.robot.1] != '.'
                {
                    if self.grid[self.robot.0 + boxes + 1][self.robot.1] == '#' {
                        return;
                    }
                    boxes += 1;
                }
            }
            '<' => {
                while self.robot.1 >= boxes + 1
                    && self.grid[self.robot.0][self.robot.1 - boxes - 1] != '.'
                {
                    if self.grid[self.robot.0][self.robot.1 - boxes - 1] == '#' {
                        return;
                    }
                    boxes += 1;
                }
            }
            '>' => {
                while self.robot.1 + boxes + 1 < self.cols
                    && self.grid[self.robot.0][self.robot.1 + boxes + 1] != '.'
                {
                    if self.grid[self.robot.0][self.robot.1 + boxes + 1] == '#' {
                        return;
                    }
                    boxes += 1;
                }
            }
            _ => {
                return;
            }
        }

        self.grid[self.robot.0][self.robot.1] = '.';
        match direction {
            '^' => {
                self.grid[self.robot.0 - boxes - 1][self.robot.1] = 'O';
                self.grid[self.robot.0 - 1][self.robot.1] = '@';
                self.robot.0 -= 1;
            }
            'v' => {
                self.grid[self.robot.0 + boxes + 1][self.robot.1] = 'O';
                self.grid[self.robot.0 + 1][self.robot.1] = '@';
                self.robot.0 += 1;
            }
            '<' => {
                self.grid[self.robot.0][self.robot.1 - boxes - 1] = 'O';
                self.grid[self.robot.0][self.robot.1 - 1] = '@';
                self.robot.1 -= 1;
            }
            '>' => {
                self.grid[self.robot.0][self.robot.1 + boxes + 1] = 'O';
                self.grid[self.robot.0][self.robot.1 + 1] = '@';
                self.robot.1 += 1;
            }
            _ => unreachable!(),
        }
    }

    fn gps_sum(&self) -> usize {
        let mut sum = 0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.grid[i][j] == 'O' {
                    sum += i * 100 + j
                }
            }
        }

        sum
    }
}

struct Warehouse2 {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    robot: (usize, usize),
}

impl From<&str> for Warehouse2 {
    fn from(s: &str) -> Self {
        let grid = s
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|c| match c {
                        '#' => ['#', '#'],
                        'O' => ['[', ']'],
                        '.' => ['.', '.'],
                        '@' => ['@', '.'],
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let rows = grid.len();
        let cols = grid[0].len();
        let robot = grid
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(j, c)| (*c == '@').then_some((i, j)))
            })
            .unwrap();
        Self {
            grid,
            rows,
            cols,
            robot,
        }
    }
}

impl Warehouse2 {
    fn find_boxes_vertical(
        &self,
        position: (usize, usize),
        is_initial: bool,
        direction: char,
    ) -> Option<Vec<(usize, usize)>> {
        if direction == '^' && position.0 == 0 {
            return None;
        }
        if direction == 'v' && position.0 == self.rows - 1 {
            return None;
        }

        let row = match direction {
            '^' => position.0 - 1,
            'v' => position.0 + 1,
            _ => unreachable!(),
        };

        if self.grid[row][position.1] == '#'
            || (!is_initial && self.grid[row][position.1 + 1] == '#')
        {
            return None;
        }

        let boxes_left = if self.grid[row][position.1] == '[' {
            self.find_boxes_vertical((row, position.1), false, direction)?
        } else if self.grid[row][position.1] == ']' {
            self.find_boxes_vertical((row, position.1 - 1), false, direction)?
        } else {
            vec![]
        };
        let boxes_right = if !is_initial && self.grid[row][position.1 + 1] == '[' {
            self.find_boxes_vertical((row, position.1 + 1), false, direction)?
        } else {
            vec![]
        };

        let mut boxes = if is_initial { vec![] } else { vec![position] };
        boxes.extend(boxes_left);
        boxes.extend(boxes_right);
        Some(boxes)
    }

    fn r#move(&mut self, direction: char) {
        let mut boxes = match dbg!(direction) {
            '^' | 'v' => match self.find_boxes_vertical(self.robot, true, direction) {
                Some(boxes) => boxes,
                None => {
                    dbg!("BLOCK");
                    return;
                }
            },
            '<' => {
                let mut boxes = vec![];
                while self.robot.1 >= 2 * boxes.len() + 1
                    && self.grid[self.robot.0][self.robot.1 - 2 * boxes.len() - 1] != '.'
                {
                    if self.grid[self.robot.0][self.robot.1 - 2 * boxes.len() - 1] == '#' {
                        dbg!("BLOCK");
                        return;
                    }
                    boxes.push((self.robot.0, self.robot.1 - 2 * boxes.len() - 2));
                }
                boxes
            }
            '>' => {
                let mut boxes = vec![];
                while self.robot.1 + 2 * boxes.len() + 1 < self.cols
                    && self.grid[self.robot.0][self.robot.1 + 2 * boxes.len() + 1] != '.'
                {
                    if self.grid[self.robot.0][self.robot.1 + 2 * boxes.len() + 1] == '#' {
                        dbg!("BLOCK");
                        return;
                    }
                    boxes.push((self.robot.0, self.robot.1 + 2 * boxes.len() + 1));
                }
                boxes
            }
            _ => return,
        };

        boxes.sort();
        if direction == '^' {
            boxes.reverse();
        }
        dbg!(&boxes);

        while let Some((i, j)) = boxes.pop() {
            match direction {
                '^' => {
                    self.grid[i - 1][j] = '[';
                    self.grid[i - 1][j + 1] = ']';
                    self.grid[i][j] = '.';
                    self.grid[i][j + 1] = '.';
                }
                'v' => {
                    self.grid[i + 1][j] = '[';
                    self.grid[i + 1][j + 1] = ']';
                    self.grid[i][j] = '.';
                    self.grid[i][j + 1] = '.';
                }
                '<' => {
                    self.grid[i][j - 1] = '[';
                    self.grid[i][j] = ']';
                }
                '>' => {
                    self.grid[i][j + 2] = ']';
                    self.grid[i][j + 1] = '[';
                    self.grid[i][j] = '.';
                }
                _ => unreachable!(),
            }
        }

        match direction {
            '^' => {
                self.grid[self.robot.0 - 1][self.robot.1] = '@';
                self.grid[self.robot.0][self.robot.1] = '.';
                self.robot.0 -= 1;
            }
            'v' => {
                self.grid[self.robot.0 + 1][self.robot.1] = '@';
                self.grid[self.robot.0][self.robot.1] = '.';
                self.robot.0 += 1;
            }
            '<' => {
                self.grid[self.robot.0][self.robot.1 - 1] = '@';
                self.grid[self.robot.0][self.robot.1] = '.';
                self.robot.1 -= 1;
            }
            '>' => {
                self.grid[self.robot.0][self.robot.1 + 1] = '@';
                self.grid[self.robot.0][self.robot.1] = '.';
                self.robot.1 += 1;
            }
            _ => unreachable!(),
        }
    }

    fn gps_sum(&self) -> usize {
        let mut sum = 0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.grid[i][j] == '[' {
                    sum += i * 100 + j
                }
            }
        }

        sum
    }

    fn _print(&self) {
        println!(
            "{}",
            self.grid
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

fn main() {
    // PART 1
    let mut iter = INPUT.split("\n\n");
    let mut warehouse = Warehouse1::from(iter.next().unwrap());
    let sequence = iter.next().unwrap();

    for c in sequence.chars() {
        warehouse.r#move(c);
    }

    println!("{}", warehouse.gps_sum());

    // PART 2
    let mut iter = INPUT.split("\n\n");
    let mut warehouse = Warehouse2::from(iter.next().unwrap());
    let sequence = iter.next().unwrap();

    for c in sequence.chars() {
        warehouse.r#move(c);
    }

    println!("{}", warehouse.gps_sum());
}
