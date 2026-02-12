use crate::puzzles::Solver;

pub fn get_solver() -> Solver {
    solver
}

const INPUT: &str = include_str!("input.txt");

struct Sequence {
    nums: Vec<Vec<isize>>,
}

impl From<&str> for Sequence {
    fn from(value: &str) -> Self {
        Self {
            nums: vec![value.split(" ").map(|s| s.parse().unwrap()).collect()],
        }
    }
}

impl Sequence {
    fn extend_next(&mut self) -> isize {
        self.calculate_diffs();

        let last = self.nums.last_mut().unwrap();
        last.push(0);

        for i in (0..self.nums.len() - 1).rev() {
            let diff = *self.nums.get(i + 1).unwrap().last().unwrap();
            let row = self.nums.get_mut(i).unwrap();
            row.push(row[row.len() - 1] + diff);
        }

        *self.nums[0].last().unwrap()
    }

    fn extend_prev(&mut self) -> isize {
        self.calculate_diffs();

        let last = self.nums.last_mut().unwrap();
        last.insert(0, 0);

        for i in (0..self.nums.len() - 1).rev() {
            let diff = *self.nums.get(i + 1).unwrap().first().unwrap();
            let row = self.nums.get_mut(i).unwrap();
            row.insert(0, row[0] - diff);
        }

        *self.nums[0].first().unwrap()
    }

    fn calculate_diffs(&mut self) {
        while !self.has_all_diffs() {
            let last = self.nums.last().unwrap();
            let diffs = (0..last.len() - 1).map(|i| last[i + 1] - last[i]).collect();
            self.nums.push(diffs);
        }
    }

    fn has_all_diffs(&self) -> bool {
        self.nums.last().unwrap().iter().all(|n| n == &0)
    }
}

fn solver() -> (String, String) {
    let sequences = INPUT.lines().map(Sequence::from).collect::<Vec<_>>();

    let mut sum_next = 0;
    let mut sum_prev = 0;
    for mut sequence in sequences {
        sum_next += sequence.extend_next();
        sum_prev += sequence.extend_prev();
    }

    (sum_next.to_string(), sum_prev.to_string())
}
