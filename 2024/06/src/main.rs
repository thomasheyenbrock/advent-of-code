const INPUT: &str = include_str!("input.txt");

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(PartialEq)]
enum State {
    Moving,
    LeftBoard,
    Loop,
}

struct Board {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    direction: Direction,
    position: (usize, usize),
    marker_count: u32,
    steps: Vec<(usize, usize, Direction)>,
}

impl Board {
    fn new() -> Self {
        let grid = INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rows = grid.len();
        let cols = grid[0].len();

        let mut i = 0;
        let mut j = 0;
        'outer: while i < rows {
            j = 0;
            while j < cols {
                if grid[i][j] == '^' {
                    break 'outer;
                }
                j += 1;
            }
            i += 1;
        }

        Self {
            grid,
            rows,
            cols,
            direction: Direction::Up,
            position: (i, j),
            marker_count: 0,
            steps: vec![],
        }
    }

    fn mark(&mut self) {
        if self.grid[self.position.0][self.position.1] != 'X' {
            self.marker_count += 1;
        }
        self.grid[self.position.0][self.position.1] = 'X';
    }

    fn is_in_front_of_obstacle(&self) -> bool {
        match self.direction {
            Direction::Up => {
                self.position.0 >= 1 && self.grid[self.position.0 - 1][self.position.1] == '#'
            }
            Direction::Right => {
                self.position.1 + 1 < self.cols
                    && self.grid[self.position.0][self.position.1 + 1] == '#'
            }
            Direction::Down => {
                self.position.0 + 1 < self.rows
                    && self.grid[self.position.0 + 1][self.position.1] == '#'
            }
            Direction::Left => {
                self.position.1 >= 1 && self.grid[self.position.0][self.position.1 - 1] == '#'
            }
        }
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn step(&mut self) -> State {
        while self.is_in_front_of_obstacle() {
            self.turn_right();
        }

        let step = (self.position.0, self.position.1, self.direction.clone());
        if self
            .steps
            .iter()
            .any(|s| s.0 == step.0 && s.1 == step.1 && s.2 == step.2)
        {
            return State::Loop;
        }
        self.steps.push(step);

        match self.direction {
            Direction::Up if self.position.0 >= 1 => {
                self.position = (self.position.0 - 1, self.position.1);
                State::Moving
            }
            Direction::Right if self.position.1 + 1 < self.cols => {
                self.position = (self.position.0, self.position.1 + 1);
                State::Moving
            }
            Direction::Down if self.position.0 + 1 < self.rows => {
                self.position = (self.position.0 + 1, self.position.1);
                State::Moving
            }
            Direction::Left if self.position.1 >= 1 => {
                self.position = (self.position.0, self.position.1 - 1);
                State::Moving
            }
            _ => State::LeftBoard,
        }
    }
}

fn main() {
    // PART 1
    let mut board = Board::new();
    let mut state = State::Moving;
    while state == State::Moving {
        board.mark();
        state = board.step();
    }
    println!("{}", board.marker_count);

    // PART 2
    let mut count = 0;

    let board = Board::new();
    let rows = board.rows;
    let cols = board.cols;

    for i in 0..rows {
        for j in 0..cols {
            // Print to see progress because this is not particularly fast
            println!("({i},{j})");

            let mut board = Board::new();
            if board.grid[i][j] != '.' {
                continue;
            }

            board.grid[i][j] = '#';

            let mut state = State::Moving;
            while state == State::Moving {
                board.mark();
                state = board.step();
            }

            if state == State::Loop {
                count += 1
            }
        }
    }

    println!("{count}");
}
