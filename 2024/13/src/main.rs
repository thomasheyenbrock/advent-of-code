const INPUT: &str = include_str!("input.txt");

struct Machine {
    prize: (u64, u64),
    a: (u64, u64),
    b: (u64, u64),
}

fn main() {
    let mut lines = INPUT.lines().peekable();

    let mut machines = vec![];
    while lines.peek().is_some() {
        let a = lines.next().unwrap();
        let b = lines.next().unwrap();
        let prize = lines.next().unwrap();
        lines.next();

        let mut iter_a = a
            .split("Button A: X+")
            .skip(1)
            .next()
            .unwrap()
            .split(", Y+")
            .map(|str| str.parse::<u64>().unwrap());

        let mut iter_b = b
            .split("Button B: X+")
            .skip(1)
            .next()
            .unwrap()
            .split(", Y+")
            .map(|str| str.parse::<u64>().unwrap());

        let mut iter_prize = prize
            .split("Prize: X=")
            .skip(1)
            .next()
            .unwrap()
            .split(", Y=")
            .map(|str| str.parse::<u64>().unwrap());

        machines.push(Machine {
            a: (iter_a.next().unwrap(), iter_a.next().unwrap()),
            b: (iter_b.next().unwrap(), iter_b.next().unwrap()),
            prize: (iter_prize.next().unwrap(), iter_prize.next().unwrap()),
        });
    }

    // PART 1
    let mut sum = 0;

    'machine_loop: for machine in machines.iter() {
        let mut a = 0;
        while a * machine.a.0 <= machine.prize.0 {
            let mut b = 0;
            while a * machine.a.0 + b * machine.b.0 <= machine.prize.0 {
                if a * machine.a.0 + b * machine.b.0 == machine.prize.0
                    && a * machine.a.1 + b * machine.b.1 == machine.prize.1
                {
                    sum += a * 3 + b;
                    continue 'machine_loop;
                }

                b += 1;
            }

            a += 1;
        }
    }

    println!("{sum}");

    // PART 2
    for machine in machines.iter_mut() {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
    }

    let mut sum = 0;

    for machine in machines.iter() {
        let b = (machine.prize.1 as f64
            - machine.prize.0 as f64 / machine.a.0 as f64 * machine.a.1 as f64)
            / (machine.b.1 as f64 - machine.b.0 as f64 / machine.a.0 as f64 * machine.a.1 as f64);
        let a = machine.prize.0 as f64 / machine.a.0 as f64
            - b * machine.b.0 as f64 / machine.a.0 as f64;

        let a = a.round() as i64;
        let b = b.round() as i64;

        if a >= 0
            && b >= 0
            && a as u64 * machine.a.0 + b as u64 * machine.b.0 == machine.prize.0
            && a as u64 * machine.a.1 + b as u64 * machine.b.1 == machine.prize.1
        {
            sum += a * 3 + b;
        }
    }

    println!("{sum}");
}
