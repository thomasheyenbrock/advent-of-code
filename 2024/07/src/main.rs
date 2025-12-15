const INPUT: &str = include_str!("input.txt");

#[derive(Clone, PartialEq)]
enum Operator {
    Add,
    Concatenate,
    Multiply,
}

struct OperatorIter {
    operators: Vec<Operator>,
    is_done: bool,
}

impl OperatorIter {
    pub fn new(length: usize) -> Self {
        OperatorIter {
            operators: vec![Operator::Add; length],
            is_done: false,
        }
    }
}

impl Iterator for OperatorIter {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let result = self.operators.clone();

        if self.operators.iter().all(|op| op == &Operator::Multiply) {
            self.is_done = true;
        } else {
            let mut index = 0;
            while index < self.operators.len() {
                match self.operators[index] {
                    Operator::Add => {
                        self.operators[index] = Operator::Concatenate;
                        break;
                    }
                    Operator::Concatenate => {
                        self.operators[index] = Operator::Multiply;
                        break;
                    }
                    Operator::Multiply => {
                        self.operators[index] = Operator::Add;
                        index += 1;
                    }
                }
            }
        }

        Some(result)
    }
}

fn main() {
    // PART 1
    let mut sum = 0;

    for line in INPUT.lines() {
        let mut iter = line.split(": ");
        let total = iter.next().unwrap().parse::<u64>().unwrap();
        let numbers = iter
            .next()
            .unwrap()
            .split(" ")
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        for combination in 0..(2usize.pow(numbers.len() as u32 - 1)) {
            let operators = (0..(numbers.len() - 1)).map(|i| {
                if 2usize.pow(i as u32) & combination == 0 {
                    Operator::Add
                } else {
                    Operator::Multiply
                }
            });

            let mut iter = numbers.iter();
            let mut result = *iter.next().unwrap();

            for (n, op) in iter.zip(operators) {
                match op {
                    Operator::Add => result += n,
                    Operator::Multiply => result *= n,
                    Operator::Concatenate => unreachable!(),
                }
            }

            if result == total {
                sum += total;
                break;
            }
        }
    }

    println!("{sum}");

    // PART 2
    let mut sum = 0;

    for line in INPUT.lines() {
        let mut iter = line.split(": ");
        let total = iter.next().unwrap().parse::<u64>().unwrap();
        let numbers = iter
            .next()
            .unwrap()
            .split(" ")
            .map(|str| str.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        for operators in OperatorIter::new(numbers.len() - 1) {
            let mut iter = numbers.iter();
            let mut result = *iter.next().unwrap();

            for (n, op) in iter.zip(operators.iter()) {
                match op {
                    Operator::Add => result += *n,
                    Operator::Concatenate => {
                        result = format!("{result}{n}").parse::<u64>().unwrap()
                    }
                    Operator::Multiply => result *= *n,
                }
            }

            if result == total {
                sum += total;
                break;
            }
        }
    }

    println!("{sum}");
}
