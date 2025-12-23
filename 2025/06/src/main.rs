const INPUT: &str = include_str!("input.txt");

struct Grid<'a> {
    height: usize,
    width: usize,
    cells: Vec<Vec<&'a str>>,
}

impl<'a> From<&'a str> for Grid<'a> {
    fn from(value: &'a str) -> Self {
        let height = value.lines().count();
        let width = value
            .lines()
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .count();
        let cells = value
            .lines()
            .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            height,
            width,
            cells,
        }
    }
}

enum Operation {
    Addition,
    Multiplication,
}

fn main() {
    let mut sum = 0;

    let grid = Grid::from(INPUT);
    for col in 0..grid.width {
        let (operation, mut result) = match grid.cells[grid.height - 1][col] {
            "+" => (Operation::Addition, 0),
            "*" => (Operation::Multiplication, 1),
            s => panic!("Invalid operation {s}"),
        };

        for row in 0..grid.height - 1 {
            match operation {
                Operation::Addition => result += grid.cells[row][col].parse::<u64>().unwrap(),
                Operation::Multiplication => result *= grid.cells[row][col].parse::<u64>().unwrap(),
            }
        }

        sum += result;
    }

    println!("{sum}");

    let input = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = input.len();
    let width = input[0].len();

    let mut mirrored = String::new();
    for col in 0..width {
        for row in 0..height {
            mirrored.push(input[row][col]);
        }
        mirrored.push('\n');
    }

    let mut problems = vec![];
    let mut current = vec![];
    for line in mirrored.lines() {
        if line.trim().is_empty() {
            problems.push(current);
            current = vec![];
        } else {
            current.push(line);
        }
    }
    problems.push(current);

    let mut sum = 0;

    for problem in problems {
        let mut iter = problem.into_iter();
        let mut first = iter.next().unwrap();

        let mut char_iter = first.chars().rev();
        let operation = match char_iter.next().unwrap() {
            '+' => Operation::Addition,
            '*' => Operation::Multiplication,
            c => panic!("Invalid operation {c}"),
        };

        let mut result = char_iter
            .rev()
            .collect::<String>()
            .trim()
            .parse::<u64>()
            .unwrap();

        for n in iter {
            match operation {
                Operation::Addition => result += n.trim().parse::<u64>().unwrap(),
                Operation::Multiplication => result *= n.trim().parse::<u64>().unwrap(),
            }
        }

        sum += result;
    }

    println!("{sum}");
}
