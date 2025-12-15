const INPUT: &str = include_str!("input.txt");

const X: isize = 101;
const Y: isize = 103;

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn r#move(&mut self, times: isize) {
        self.position.0 = (self.position.0 + self.velocity.0 * times + X * times) % X;
        self.position.1 = (self.position.1 + self.velocity.1 * times + Y * times) % Y;
    }
}

fn print(robots: &Vec<Robot>) {
    for y in 0..Y {
        for x in 0..X {
            let count = robots
                .iter()
                .filter(|robot| robot.position.0 == x && robot.position.1 == y)
                .count();
            print!(
                "{}",
                if count == 0 {
                    ".".to_string()
                } else {
                    count.to_string()
                }
            );
        }
        println!("");
    }
}

fn main() {
    // PART 1
    let mut robots = INPUT
        .lines()
        .map(|line| {
            let mut iter = line.split(" ");
            let mut p = (&iter.next().unwrap()[2..])
                .split(",")
                .map(|str| str.parse::<isize>().unwrap());
            let mut v = (&iter.next().unwrap()[2..])
                .split(",")
                .map(|str| str.parse::<isize>().unwrap());
            Robot {
                position: (p.next().unwrap(), p.next().unwrap()),
                velocity: (v.next().unwrap(), v.next().unwrap()),
            }
        })
        .collect::<Vec<_>>();

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for robot in robots.iter_mut() {
        robot.r#move(100);
        if robot.position.0 < (X - 1) / 2 {
            if robot.position.1 < (Y - 1) / 2 {
                q1 += 1;
            } else if robot.position.1 > (Y - 1) / 2 {
                q2 += 1;
            }
        } else if robot.position.0 > (X - 1) / 2 {
            if robot.position.1 < (Y - 1) / 2 {
                q3 += 1;
            } else if robot.position.1 > (Y - 1) / 2 {
                q4 += 1;
            }
        }
    }

    println!("{}", q1 * q2 * q3 * q4);

    // PART 2
    let mut robots = INPUT
        .lines()
        .map(|line| {
            let mut iter = line.split(" ");
            let mut p = (&iter.next().unwrap()[2..])
                .split(",")
                .map(|str| str.parse::<isize>().unwrap());
            let mut v = (&iter.next().unwrap()[2..])
                .split(",")
                .map(|str| str.parse::<isize>().unwrap());
            Robot {
                position: (p.next().unwrap(), p.next().unwrap()),
                velocity: (v.next().unwrap(), v.next().unwrap()),
            }
        })
        .collect::<Vec<_>>();

    for i in 0..1_000_000 {
        let mut count = 0;
        for robot in robots.iter_mut() {
            robot.r#move(1);
            if robot.position.0 == 0
                || robot.position.0 == X - 1
                || robot.position.1 == 0
                || robot.position.1 == Y - 1
            {
                count += 1;
            }
        }

        if count < 5 {
            print(&robots);
            println!("{i}");
            break;
        }
    }
}
