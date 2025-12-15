const INPUT: &str = include_str!("input.txt");

fn main() {
    // PART 1
    let mut disk = vec![];

    let mut is_free = false;
    let mut id = 0;
    for char in INPUT.trim().chars() {
        let num = char.to_string().parse::<u32>().unwrap();

        for _ in 0..num {
            disk.push(if is_free { None } else { Some(id) });
        }

        if !is_free {
            id += 1;
        }

        is_free = !is_free;
    }

    while let Some(first_free) = disk
        .iter()
        .enumerate()
        .find_map(|(i, id)| id.is_none().then_some(i))
    {
        match disk.pop().unwrap() {
            Some(id) => {
                disk[first_free].replace(id);
            }
            None => {}
        }
    }

    let sum = disk
        .iter()
        .enumerate()
        .fold(0, |sum, (i, id)| sum + i * id.unwrap());

    println!("{sum}");

    // PART 2
    let mut disk = vec![];

    let mut is_free = false;
    let mut id = 0;
    for char in INPUT.trim().chars() {
        let num = char.to_string().parse::<u32>().unwrap();

        for _ in 0..num {
            disk.push(if is_free { None } else { Some(id) });
        }

        if !is_free {
            id += 1;
        }

        is_free = !is_free;
    }

    while id > 0 {
        id -= 1;

        let file_len = disk.iter().filter(|id2| id2 == &&Some(id)).count();
        let current_index = disk
            .iter()
            .enumerate()
            .find_map(|(i, id2)| (id2 == &Some(id)).then_some(i))
            .unwrap();

        let new_index = (0..current_index).find(|i| {
            for j in *i..(i + file_len) {
                if disk[j].is_some() {
                    return false;
                }
            }
            true
        });
        if let Some(new_index) = new_index {
            for j in 0..file_len {
                disk[current_index + j].take();
                disk[new_index + j].replace(id);
            }
        }
    }

    let sum = disk
        .iter()
        .enumerate()
        .fold(0, |sum, (i, id)| sum + i * id.unwrap_or(0));

    println!("{sum}");
}
