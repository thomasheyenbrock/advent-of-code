const INPUT: &str = include_str!("input.txt");

struct Card {
    winning: Vec<usize>,
    actual: Vec<usize>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let mut iter = value.split(": ");
        iter.next().unwrap();
        let nums = iter.next().unwrap();

        let mut iter = nums.split(" | ");
        let winning = iter
            .next()
            .unwrap()
            .split(" ")
            .filter(|str| !str.trim().is_empty())
            .map(|str| str.trim().parse::<usize>().unwrap())
            .collect();
        let actual = iter
            .next()
            .unwrap()
            .split(" ")
            .filter(|str| !str.trim().is_empty())
            .map(|str| str.trim().parse::<usize>().unwrap())
            .collect();

        Self { winning, actual }
    }
}

fn main() {
    let cards = INPUT.lines().map(Card::from).collect::<Vec<_>>();
    let mut counts = vec![1; cards.len()];

    let mut sum = 0;

    for i in 0..cards.len() {
        let card = &cards[i];
        let matches = card
            .actual
            .iter()
            .filter(|n| card.winning.contains(n))
            .count();
        if matches > 0 {
            sum += 2u32.pow(matches as u32 - 1);

            let max = (cards.len() - 1).min(i + matches);
            for j in i + 1..=max {
                counts[j] += counts[i];
            }
        }
    }

    println!("{sum}");
    println!("{}", counts.iter().sum::<u32>());
}
