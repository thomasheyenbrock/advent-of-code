const INPUT: &str = include_str!("input.txt");

struct Game {
    id: usize,
    cubes: Vec<Cubes>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut iter = value.split(": ");

        let id = iter.next().unwrap();
        let id = &id[5..id.len()].parse::<usize>().unwrap();

        let cubes = iter
            .next()
            .unwrap()
            .split("; ")
            .map(|str| Cubes::from(str))
            .collect();

        Self { id: *id, cubes }
    }
}

impl Game {
    fn power(&self) -> usize {
        let mut min_cubes = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };
        for cubes in self.cubes.iter() {
            min_cubes.max_mut(cubes);
        }
        min_cubes.red * min_cubes.green * min_cubes.blue
    }
}

struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl From<&str> for Cubes {
    fn from(value: &str) -> Self {
        let mut cubes = Self {
            red: 0,
            green: 0,
            blue: 0,
        };
        for part in value.split(", ") {
            let mut iter = part.split(" ");
            let count = iter.next().unwrap().parse::<usize>().unwrap();
            match iter.next().unwrap() {
                "red" => cubes.red = count,
                "green" => cubes.green = count,
                "blue" => cubes.blue = count,
                color => panic!("Unknown color {color}"),
            }
        }
        cubes
    }
}

impl Cubes {
    fn is_less_or_equal(&self, other: &Cubes) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn max_mut(&mut self, other: &Cubes) {
        self.red = self.red.max(other.red);
        self.green = self.green.max(other.green);
        self.blue = self.blue.max(other.blue);
    }
}

fn main() {
    let games = INPUT
        .lines()
        .map(|line| Game::from(line))
        .collect::<Vec<_>>();

    let cubes = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut sum = 0;
    let mut powers = 0;

    for game in games {
        if game.cubes.iter().all(|c| c.is_less_or_equal(&cubes)) {
            sum += game.id;
        }

        powers += game.power();
    }

    println!("{sum}");
    println!("{powers}");
}
