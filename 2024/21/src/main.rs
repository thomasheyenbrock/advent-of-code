use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

type MoveMap = HashMap<(String, String), String>;

fn numeric_keypad_from_to(from: &str, to: &str) -> Vec<&'static str> {
    match (from, to) {
        ("0", "0") => vec!["A"],
        ("0", "1") => vec!["^<A"],
        ("0", "2") => vec!["^A"],
        ("0", "3") => vec![">^A", "^>A"],
        ("0", "4") => vec!["^<^A", "^^<A"],
        ("0", "5") => vec!["^^A"],
        ("0", "6") => vec![">^^A", "^>^A", "^^>A"],
        ("0", "7") => vec!["^^^<A", "^<^^A", "^^<^A"],
        ("0", "8") => vec!["^^^A"],
        ("0", "9") => vec![">^^^A", "^>^^A", "^^>^A", "^^^>A"],
        ("0", "A") => vec![">A"],
        ("1", "0") => vec![">vA"],
        ("1", "1") => vec!["A"],
        ("1", "2") => vec![">A"],
        ("1", "3") => vec![">>A"],
        ("1", "4") => vec!["^A"],
        ("1", "5") => vec![">^A", "^>A"],
        ("1", "6") => vec![">>^A", ">^>A", "^>>A"],
        ("1", "7") => vec!["^^A"],
        ("1", "8") => vec![">^^A", "^>^A", "^^>A"],
        ("1", "9") => vec![">>^^A", ">^>^A", "^>>^A", ">^^>A", "^>^>A", "^^>>A"],
        ("1", "A") => vec![">>vA", ">v>A"],
        ("2", "0") => vec!["vA"],
        ("2", "1") => vec!["<A"],
        ("2", "2") => vec!["A"],
        ("2", "3") => vec![">A"],
        ("2", "4") => vec!["<^A", "^<A"],
        ("2", "5") => vec!["^A"],
        ("2", "6") => vec![">^A", "^>A"],
        ("2", "7") => vec!["<^^A", "^<^A", "^^<A"],
        ("2", "8") => vec!["^^A"],
        ("2", "9") => vec![">^^A", "^>^A", "^^>A"],
        ("2", "A") => vec![">vA", "v>A"],
        ("3", "0") => vec!["<vA", "v<A"],
        ("3", "1") => vec!["<<A"],
        ("3", "2") => vec!["<A"],
        ("3", "3") => vec!["A"],
        ("3", "4") => vec!["<<^A", "<^<A", "^<<A"],
        ("3", "5") => vec!["<^A", "^<A"],
        ("3", "6") => vec!["^A"],
        ("3", "7") => vec!["<<^^A", "<^<^A", "^<<^A", "<^^<A", "^<^<A", "^^<<A"],
        ("3", "8") => vec!["<^^A", "^<^A", "^^<A"],
        ("3", "9") => vec!["^^A"],
        ("3", "A") => vec!["vA"],
        ("4", "0") => vec![">vvA", "v>vA"],
        ("4", "1") => vec!["vA"],
        ("4", "2") => vec![">vA", "v>A"],
        ("4", "3") => vec![">>vA", ">v>A", "v>>A"],
        ("4", "4") => vec!["A"],
        ("4", "5") => vec![">A"],
        ("4", "6") => vec![">>A"],
        ("4", "7") => vec!["^A"],
        ("4", "8") => vec![">^A", "^>A"],
        ("4", "9") => vec![">>^A", ">^>A", "^>>A"],
        ("4", "A") => vec![">>vvA", ">v>vA", "v>>vA", ">vv>A", "v>v>A"],
        ("5", "0") => vec!["vvA"],
        ("5", "1") => vec!["<vA", "v<A"],
        ("5", "2") => vec!["vA"],
        ("5", "3") => vec![">vA", "v>A"],
        ("5", "4") => vec!["<A"],
        ("5", "5") => vec!["A"],
        ("5", "6") => vec![">A"],
        ("5", "7") => vec!["<^A", "^<A"],
        ("5", "8") => vec!["^A"],
        ("5", "9") => vec![">^A", "^>A"],
        ("5", "A") => vec![">vvA", "v>vA", "vv>A"],
        ("6", "0") => vec!["<vvA", "v<vA", "vv<A"],
        ("6", "1") => vec!["<<vA", "<v<A", "v<<A"],
        ("6", "2") => vec!["<vA", "v<A"],
        ("6", "3") => vec!["vA"],
        ("6", "4") => vec!["<<A"],
        ("6", "5") => vec!["<A"],
        ("6", "6") => vec!["A"],
        ("6", "7") => vec!["<<^A", "<^<A", "^<<A"],
        ("6", "8") => vec!["<^A", "^<A"],
        ("6", "9") => vec!["^A"],
        ("6", "A") => vec!["vvA"],
        ("7", "0") => vec![">vvvA", "v>vvA", "vv>vA"],
        ("7", "1") => vec!["vvA"],
        ("7", "2") => vec![">vvA", "v>vA", "vv>A"],
        ("7", "3") => vec![">>vvA", ">v>vA", "v>>vA", ">vv>A", "v>v>A", "vv>>A"],
        ("7", "4") => vec!["vA"],
        ("7", "5") => vec![">vA", "v>A"],
        ("7", "6") => vec![">>vA", ">v>A", "v>>A"],
        ("7", "7") => vec!["A"],
        ("7", "8") => vec![">A"],
        ("7", "9") => vec![">>A"],
        ("7", "A") => vec![
            ">>vvvA", ">v>vvA", "v>>vvA", ">vv>vA", "v>v>vA", "vv>>vA", ">vvv>A", "v>vv>A",
            "vv>v>A",
        ],
        ("8", "0") => vec!["vvvA"],
        ("8", "1") => vec!["<vvA", "v<vA", "vv<A"],
        ("8", "2") => vec!["vvA"],
        ("8", "3") => vec![">vvA", "v>vA", "vv>A"],
        ("8", "4") => vec!["<vA", "v<A"],
        ("8", "5") => vec!["vA"],
        ("8", "6") => vec![">vA", "v>A"],
        ("8", "7") => vec!["<A"],
        ("8", "8") => vec!["A"],
        ("8", "9") => vec![">A"],
        ("8", "A") => vec![">vvvA", "v>vvA", "vv>vA", "vvv>A"],
        ("9", "0") => vec!["<vvvA", "v<vvA", "vv<vA", "vvv<A"],
        ("9", "1") => vec!["<<vvA", "<v<vA", "v<<vA", "<vv<A", "v<v<A", "vv<<A"],
        ("9", "2") => vec!["<vvA", "v<vA", "vv<A"],
        ("9", "3") => vec!["vvA"],
        ("9", "4") => vec!["<<vA", "<v<A", "v<<A"],
        ("9", "5") => vec!["<vA", "v<A"],
        ("9", "6") => vec!["vA"],
        ("9", "7") => vec!["<<A"],
        ("9", "8") => vec!["<A"],
        ("9", "9") => vec!["A"],
        ("9", "A") => vec!["vvvA"],
        ("A", "0") => vec!["<A"],
        ("A", "1") => vec!["<^<A", "^<<A"],
        ("A", "2") => vec!["<^A", "^<A"],
        ("A", "3") => vec!["^A"],
        ("A", "4") => vec!["<^<^A", "^<<^A", "<^^<A", "^<^<A", "^^<<A"],
        ("A", "5") => vec!["<^^A", "^<^A", "^^<A"],
        ("A", "6") => vec!["^^A"],
        ("A", "7") => vec![
            "<^<^^A", "^<<^^A", "<^^<^A", "^<^<^A", "^^<<^A", "<^^^<A", "^<^^<A", "^^<^<A",
            "^^^<<A",
        ],
        ("A", "8") => vec!["<^^^A", "^<^^A", "^^<^A", "^^^<A"],
        ("A", "9") => vec!["^^^A"],
        ("A", "A") => vec!["A"],
        _ => unreachable!(),
    }
}

fn numeric_keypad_map() -> MoveMap {
    let mut map = MoveMap::new();
    for i in 0..=10 {
        for j in 0..=10 {
            let from = if i == 10 {
                "A".to_string()
            } else {
                i.to_string()
            };
            let to = if j == 10 {
                "A".to_string()
            } else {
                j.to_string()
            };
            let r#move = numeric_keypad_from_to(&from, &to)
                .first()
                .unwrap()
                .to_string();
            map.insert((from, to), r#move);
        }
    }
    map
}

fn numeric_keypad(move_map: &MoveMap, code: &str) -> String {
    let mut new_code = String::new();

    for i in 0..code.len() {
        let from = if i == 0 { "A" } else { &code[i - 1..i] };
        let to = &code[i..i + 1];
        new_code.push_str(move_map.get(&(from.to_string(), to.to_string())).unwrap());
    }

    new_code
}

fn directional_keypad_from_to(from: &str, to: &str) -> Vec<&'static str> {
    match (from, to) {
        ("^", "^") => vec!["A"],
        ("^", "v") => vec!["vA"],
        ("^", "<") => vec!["v<A"],
        ("^", ">") => vec!["v>A", ">vA"],
        ("^", "A") => vec![">A"],
        ("v", "^") => vec!["^A"],
        ("v", "v") => vec!["A"],
        ("v", "<") => vec!["<A"],
        ("v", ">") => vec![">A"],
        ("v", "A") => vec![">^A", "^>A"],
        ("<", "^") => vec![">^A"],
        ("<", "v") => vec![">A"],
        ("<", "<") => vec!["A"],
        ("<", ">") => vec![">>A"],
        ("<", "A") => vec![">>^A", ">^>A"],
        (">", "^") => vec!["<^A", "^<A"],
        (">", "v") => vec!["<A"],
        (">", "<") => vec!["<<A"],
        (">", ">") => vec!["A"],
        (">", "A") => vec!["^A"],
        ("A", "^") => vec!["<A"],
        ("A", "v") => vec!["<vA", "v<A"],
        ("A", "<") => vec!["v<<A", "<v<A"],
        ("A", ">") => vec!["vA"],
        ("A", "A") => vec!["A"],
        _ => unreachable!(),
    }
}

const DIRECTIONALS: &str = "^v<>A";

fn directional_keypad_map() -> MoveMap {
    let mut map = MoveMap::new();
    for from in DIRECTIONALS.chars() {
        for to in DIRECTIONALS.chars() {
            let r#move =
                directional_keypad_from_to(from.to_string().as_str(), to.to_string().as_str())
                    .first()
                    .unwrap()
                    .to_string();
            map.insert((from.to_string(), to.to_string()), r#move);
        }
    }
    map
}

type Cache = HashMap<(u8, String), usize>;

fn len(cache: &mut Cache, directional_move_map: &MoveMap, code: String, depth: u8) -> usize {
    if depth == 0 {
        return code.len();
    }

    if let Some(result) = cache.get(&(depth, code.clone())) {
        return *result;
    }

    let mut result = 0;
    for i in 0..code.len() {
        let from = if i == 0 { "A" } else { &code[i - 1..i] };
        let to = &code[i..i + 1];
        let code = directional_move_map
            .get(&(from.to_string(), to.to_string()))
            .unwrap();
        result += len(cache, directional_move_map, code.to_string(), depth - 1)
    }

    cache.insert((depth, code), result);
    result
}

fn sum(numeric_move_map: &MoveMap, directional_move_map: &MoveMap, depth: u8) -> usize {
    let mut sum = 0;
    let mut cache = Cache::new();
    for code in INPUT.lines() {
        let num = code
            .chars()
            .into_iter()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let code = numeric_keypad(&numeric_move_map, code);
        let result = len(&mut cache, directional_move_map, code, depth);

        sum += num * result
    }

    sum
}

fn main() {
    // PRELUDE: Optimize move maps
    let mut numeric_move_map = numeric_keypad_map();
    let mut directional_move_map = directional_keypad_map();
    let mut current = sum(&numeric_move_map, &directional_move_map, 25);

    for key in numeric_move_map.clone().keys() {
        let options = numeric_keypad_from_to(&key.0, &key.1);
        let mut iter = options.iter();
        let mut best = iter.next().unwrap();

        for alternative in iter {
            numeric_move_map.insert(key.clone(), alternative.to_string());

            let new_sum = sum(&numeric_move_map, &directional_move_map, 25);
            if new_sum < current {
                current = new_sum;
                best = alternative;
            }
        }

        numeric_move_map.insert(key.clone(), best.to_string());
    }

    let mut current = sum(&numeric_move_map, &directional_move_map, 25);
    for key in directional_move_map.clone().keys() {
        let options = directional_keypad_from_to(&key.0, &key.1);
        let mut iter = options.iter();
        let mut best = iter.next().unwrap();

        for alternative in iter {
            directional_move_map.insert(key.clone(), alternative.to_string());

            let new_sum = sum(&numeric_move_map, &directional_move_map, 25);
            if new_sum < current {
                current = new_sum;
                best = alternative;
            }
        }

        directional_move_map.insert(key.clone(), best.to_string());
    }

    println!("{}", sum(&numeric_move_map, &directional_move_map, 2));
    println!("{}", sum(&numeric_move_map, &directional_move_map, 25));
}
