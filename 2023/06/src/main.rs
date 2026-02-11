const INPUT: &str = include_str!("input.txt");

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn wins(&self) -> usize {
        let i = ((self.time as f32 - ((self.time.pow(2) - 4 * self.distance) as f32).sqrt()) / 2f32)
            .ceil() as usize;
        self.time + 1 - 2 * i
    }
}

fn main() {
    let mut iter = INPUT.lines();
    let times = iter
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .filter(|s| !s.is_empty());
    let distances = iter
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .filter(|s| !s.is_empty());

    let races = times
        .zip(distances)
        .map(|(time, distance)| Race {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap(),
        })
        .collect::<Vec<_>>();

    let mut prod = 1;
    for race in races.iter() {
        prod *= race.wins();
    }
    println!("{prod}");

    let mut time = String::new();
    let mut distance = String::new();
    for race in races.iter() {
        time += &race.time.to_string();
        distance += &race.distance.to_string();
    }

    let race = Race {
        time: time.parse().unwrap(),
        distance: distance.parse().unwrap(),
    };
    println!("{}", race.wins());
}
