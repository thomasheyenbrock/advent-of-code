const INPUT: &str = include_str!("input.txt");

type Position = (usize, usize);

struct Track {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    start: Position,
    end: Position,
}

impl Track {
    fn new() -> Self {
        let mut start = None;
        let mut end = None;

        let grid = INPUT
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        if c == 'S' {
                            start = Some((i, j));
                        } else if c == 'E' {
                            end = Some((i, j));
                        }
                        c
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rows = grid.len();
        let cols = grid[0].len();

        Self {
            grid,
            rows,
            cols,
            start: start.unwrap().into(),
            end: end.unwrap().into(),
        }
    }

    fn get(&self, position: &Position) -> char {
        self.grid[position.0][position.1]
    }

    fn path(&self) -> Vec<Position> {
        let mut path = vec![self.start.clone()];

        while path.last().unwrap() != &self.end {
            self.find_next(&mut path);
        }

        path
    }

    fn find_next(&self, p: &mut Vec<Position>) {
        let last = p.last().unwrap();

        let up = self.up(&last).unwrap();
        let down = self.down(&last).unwrap();
        let left = self.left(&last).unwrap();
        let right = self.right(&last).unwrap();

        let second_last = if p.len() == 1 {
            None
        } else {
            Some(p[p.len() - 2].clone())
        };

        match (
            second_last,
            self.get(&up) != '#',
            self.get(&down) != '#',
            self.get(&left) != '#',
            self.get(&right) != '#',
        ) {
            (None, true, false, false, false) => p.push(up),
            (None, false, true, false, false) => p.push(down),
            (None, false, false, true, false) => p.push(left),
            (None, false, false, false, true) => p.push(right),
            (Some(second_last), true, true, false, false) if second_last == up => p.push(down),
            (Some(second_last), true, true, false, false) if second_last == down => p.push(up),
            (Some(second_last), true, false, true, false) if second_last == up => p.push(left),
            (Some(second_last), true, false, true, false) if second_last == left => p.push(up),
            (Some(second_last), true, false, false, true) if second_last == up => p.push(right),
            (Some(second_last), true, false, false, true) if second_last == right => p.push(up),
            (Some(second_last), false, true, true, false) if second_last == down => p.push(left),
            (Some(second_last), false, true, true, false) if second_last == left => p.push(down),
            (Some(second_last), false, true, false, true) if second_last == down => p.push(right),
            (Some(second_last), false, true, false, true) if second_last == right => p.push(down),
            (Some(second_last), false, false, true, true) if second_last == left => p.push(right),
            (Some(second_last), false, false, true, true) if second_last == right => p.push(left),
            _ => unreachable!(),
        }
    }

    fn up(&self, position: &Position) -> Option<Position> {
        (position.0 > 0).then(|| (position.0 - 1, position.1))
    }

    fn down(&self, position: &Position) -> Option<Position> {
        (position.0 < self.rows - 1).then(|| (position.0 + 1, position.1))
    }

    fn left(&self, position: &Position) -> Option<Position> {
        (position.1 > 0).then(|| (position.0, position.1 - 1))
    }

    fn right(&self, position: &Position) -> Option<Position> {
        (position.1 < self.cols - 1).then(|| (position.0, position.1 + 1))
    }
}

fn dist(a: &Position, b: &Position) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn main() {
    let track = Track::new();
    let path = track.path();

    // PART 1
    let mut cheats = vec![];

    for (i, position) in path.iter().enumerate() {
        let up = track.up(position).unwrap();
        if track.get(&up) == '#' {
            if let Some(upup) = track.up(&up) {
                match path.iter().position(|p| p == &upup) {
                    Some(j) if j > i => cheats.push(j - i - 2),
                    _ => {}
                }
            }
        }

        let down = track.down(position).unwrap();
        if track.get(&down) == '#' {
            if let Some(downdown) = track.down(&down) {
                match path.iter().position(|p| p == &downdown) {
                    Some(j) if j > i => cheats.push(j - i - 2),
                    _ => {}
                }
            }
        }

        let left = track.left(position).unwrap();
        if track.get(&left) == '#' {
            if let Some(leftleft) = track.left(&left) {
                match path.iter().position(|p| p == &leftleft) {
                    Some(j) if j > i => cheats.push(j - i - 2),
                    _ => {}
                }
            }
        }

        let right = track.right(position).unwrap();
        if track.get(&right) == '#' {
            if let Some(rightright) = track.right(&right) {
                match path.iter().position(|p| p == &rightright) {
                    Some(j) if j > i => cheats.push(j - i - 2),
                    _ => {}
                }
            }
        }
    }

    println!("{}", cheats.iter().filter(|cheat| **cheat >= 100).count());

    // PART 2
    let mut cheats = vec![];

    for (i, position) in path.iter().enumerate() {
        for p1 in 0..track.rows {
            for p2 in 0..track.cols {
                let p = (p1, p2);
                let dist = dist(position, &p);
                if dist > 20 {
                    continue;
                }
                if track.get(&p) == '#' {
                    continue;
                }

                match path.iter().position(|pos| pos == &p) {
                    Some(j) if j > i => cheats.push(j - i - dist),
                    _ => {}
                }
            }
        }
    }

    println!("{}", cheats.iter().filter(|cheat| **cheat >= 100).count());
}
