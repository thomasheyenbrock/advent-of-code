use crate::puzzles::Solver;

pub fn get_solver() -> Solver {
    solver
}

use std::collections::{HashMap, hash_map::Entry};

const INPUT: &str = include_str!("input.txt");

struct Game {
    players: Vec<Player>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        Self {
            players: value.lines().map(Player::from).collect(),
        }
    }
}

#[derive(Eq, PartialEq)]
struct Player {
    hand: Hand,
    bet: usize,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl From<&str> for Player {
    fn from(value: &str) -> Self {
        let mut iter = value.split(" ");
        Self {
            hand: Hand::from(iter.next().unwrap()),
            bet: iter.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum Hand {
    FiveOfAKind(Cards),
    FourOfAKind(Cards),
    FullHouse(Cards),
    ThreeOfAKind(Cards),
    TwoPair(Cards),
    OnePair(Cards),
    HighCard(Cards),
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let cards = Cards::from(value);

        let mut counts = HashMap::new();

        match counts.entry(&cards._1) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(occupied_entry.get() + 1);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
        match counts.entry(&cards._2) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(occupied_entry.get() + 1);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
        match counts.entry(&cards._3) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(occupied_entry.get() + 1);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
        match counts.entry(&cards._4) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(occupied_entry.get() + 1);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
        match counts.entry(&cards._5) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(occupied_entry.get() + 1);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }

        let mut counts = counts.values().map(|i| *i).collect::<Vec<_>>();
        counts.sort();

        if counts == vec![1, 1, 1, 1, 1] {
            Self::HighCard(cards)
        } else if counts == vec![1, 1, 1, 2] {
            Self::OnePair(cards)
        } else if counts == vec![1, 2, 2] {
            Self::TwoPair(cards)
        } else if counts == vec![1, 1, 3] {
            Self::ThreeOfAKind(cards)
        } else if counts == vec![2, 3] {
            Self::FullHouse(cards)
        } else if counts == vec![1, 4] {
            Self::FourOfAKind(cards)
        } else if counts == vec![5] {
            Self::FiveOfAKind(cards)
        } else {
            unreachable!()
        }
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Cards {
    _1: Card,
    _2: Card,
    _3: Card,
    _4: Card,
    _5: Card,
}

impl From<&str> for Cards {
    fn from(value: &str) -> Self {
        let mut iter = value.chars();
        Self {
            _1: Card::from(iter.next().unwrap()),
            _2: Card::from(iter.next().unwrap()),
            _3: Card::from(iter.next().unwrap()),
            _4: Card::from(iter.next().unwrap()),
            _5: Card::from(iter.next().unwrap()),
        }
    }
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::N9,
            '8' => Card::N8,
            '7' => Card::N7,
            '6' => Card::N6,
            '5' => Card::N5,
            '4' => Card::N4,
            '3' => Card::N3,
            '2' => Card::N2,
            _ => unreachable!(),
        }
    }
}

struct JokerGame {
    players: Vec<JokerPlayer>,
}

impl From<Game> for JokerGame {
    fn from(value: Game) -> Self {
        Self {
            players: value.players.into_iter().map(JokerPlayer::from).collect(),
        }
    }
}

#[derive(Eq, PartialEq)]
struct JokerPlayer {
    hand: JokerHand,
    bet: usize,
}

impl Ord for JokerPlayer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for JokerPlayer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl From<Player> for JokerPlayer {
    fn from(value: Player) -> Self {
        Self {
            hand: JokerHand::from(value.hand),
            bet: value.bet,
        }
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum JokerHand {
    FiveOfAKind(JokerCards),
    FourOfAKind(JokerCards),
    FullHouse(JokerCards),
    ThreeOfAKind(JokerCards),
    TwoPair(JokerCards),
    OnePair(JokerCards),
    HighCard(JokerCards),
}

impl From<Hand> for JokerHand {
    fn from(value: Hand) -> Self {
        let cards = match value {
            Hand::FiveOfAKind(cards)
            | Hand::FourOfAKind(cards)
            | Hand::FullHouse(cards)
            | Hand::ThreeOfAKind(cards)
            | Hand::TwoPair(cards)
            | Hand::OnePair(cards)
            | Hand::HighCard(cards) => cards,
        };
        let cards = JokerCards::from(cards);

        let mut counts = HashMap::new();
        match counts.entry(&cards._1) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(occupied_entry.get() + 1);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
        match counts.entry(&cards._2) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(occupied_entry.get() + 1);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
        match counts.entry(&cards._3) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(occupied_entry.get() + 1);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
        match counts.entry(&cards._4) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(occupied_entry.get() + 1);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
        match counts.entry(&cards._5) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(occupied_entry.get() + 1);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }

        let joker_count = counts.remove(&JokerCard::J).unwrap_or(0);
        let mut counts = counts.values().map(|i| *i).collect::<Vec<_>>();
        counts.sort();

        if counts == vec![1, 1, 1, 1, 1] {
            Self::HighCard(cards)
        } else if counts == vec![1, 1, 1, 2] {
            Self::OnePair(cards)
        } else if counts == vec![1, 2, 2] {
            Self::TwoPair(cards)
        } else if counts == vec![1, 1, 3] {
            Self::ThreeOfAKind(cards)
        } else if counts == vec![2, 3] {
            Self::FullHouse(cards)
        } else if counts == vec![1, 4] {
            Self::FourOfAKind(cards)
        } else if counts == vec![5] {
            Self::FiveOfAKind(cards)
        } else if counts == vec![1, 1, 1, 1] && joker_count == 1 {
            Self::OnePair(cards)
        } else if counts == vec![1, 1, 2] && joker_count == 1 {
            Self::ThreeOfAKind(cards)
        } else if counts == vec![2, 2] && joker_count == 1 {
            Self::FullHouse(cards)
        } else if counts == vec![1, 3] && joker_count == 1 {
            Self::FourOfAKind(cards)
        } else if counts == vec![4] && joker_count == 1 {
            Self::FiveOfAKind(cards)
        } else if counts == vec![1, 1, 1] && joker_count == 2 {
            Self::ThreeOfAKind(cards)
        } else if counts == vec![1, 2] && joker_count == 2 {
            Self::FourOfAKind(cards)
        } else if counts == vec![3] && joker_count == 2 {
            Self::FiveOfAKind(cards)
        } else if counts == vec![1, 1] && joker_count == 3 {
            Self::FourOfAKind(cards)
        } else if counts == vec![2] && joker_count == 3 {
            Self::FiveOfAKind(cards)
        } else if counts == vec![1] && joker_count == 4 {
            Self::FiveOfAKind(cards)
        } else if counts == vec![] && joker_count == 5 {
            Self::FiveOfAKind(cards)
        } else {
            unreachable!()
        }
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct JokerCards {
    _1: JokerCard,
    _2: JokerCard,
    _3: JokerCard,
    _4: JokerCard,
    _5: JokerCard,
}

impl From<Cards> for JokerCards {
    fn from(value: Cards) -> Self {
        Self {
            _1: JokerCard::from(value._1),
            _2: JokerCard::from(value._2),
            _3: JokerCard::from(value._3),
            _4: JokerCard::from(value._4),
            _5: JokerCard::from(value._5),
        }
    }
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
enum JokerCard {
    A,
    K,
    Q,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
    J,
}

impl From<Card> for JokerCard {
    fn from(value: Card) -> Self {
        match value {
            Card::A => Self::A,
            Card::K => Self::K,
            Card::Q => Self::Q,
            Card::J => Self::J,
            Card::T => Self::T,
            Card::N9 => Self::N9,
            Card::N8 => Self::N8,
            Card::N7 => Self::N7,
            Card::N6 => Self::N6,
            Card::N5 => Self::N5,
            Card::N4 => Self::N4,
            Card::N3 => Self::N3,
            Card::N2 => Self::N2,
        }
    }
}

fn solver() -> (String, String) {
    let mut game = Game::from(INPUT);
    game.players.sort();

    let mut sum_1 = 0;
    for (i, player) in game.players.iter().rev().enumerate() {
        sum_1 += (i + 1) * player.bet;
    }

    let mut game = JokerGame::from(game);
    game.players.sort();

    let mut sum_2 = 0;
    for (i, player) in game.players.iter().rev().enumerate() {
        sum_2 += (i + 1) * player.bet;
    }

    (sum_1.to_string(), sum_2.to_string())
}
