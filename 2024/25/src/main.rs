const INPUT: &str = include_str!("input.txt");

fn fits(key: &Vec<u8>, lock: &Vec<u8>) -> bool {
    key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5)
}

fn main() {
    let mut keys = vec![];
    let mut locks = vec![];

    for thing in INPUT.trim().split("\n\n") {
        if thing.starts_with("#") {
            let mut lock = vec![];
            for i in 0..5 {
                if &thing[(6 + i)..(6 + i + 1)] == "." {
                    lock.push(0)
                } else if &thing[(12 + i)..(12 + i + 1)] == "." {
                    lock.push(1)
                } else if &thing[(18 + i)..(18 + i + 1)] == "." {
                    lock.push(2)
                } else if &thing[(24 + i)..(24 + i + 1)] == "." {
                    lock.push(3)
                } else if &thing[(30 + i)..(30 + i + 1)] == "." {
                    lock.push(4)
                } else {
                    lock.push(5)
                }
            }
            locks.push(lock);
        } else {
            let mut key = vec![];
            for i in 0..5 {
                if &thing[(30 + i)..(30 + i + 1)] == "." {
                    key.push(0)
                } else if &thing[(24 + i)..(24 + i + 1)] == "." {
                    key.push(1)
                } else if &thing[(18 + i)..(18 + i + 1)] == "." {
                    key.push(2)
                } else if &thing[(12 + i)..(12 + i + 1)] == "." {
                    key.push(3)
                } else if &thing[(6 + i)..(6 + i + 1)] == "." {
                    key.push(4)
                } else {
                    key.push(5)
                }
            }
            keys.push(key);
        }
    }

    let mut count = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if fits(key, lock) {
                count += 1
            }
        }
    }

    println!("{count}");
}
