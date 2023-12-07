use std::cmp::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
enum CardValue { JOKER, _2, _3, _4, _5, _6, _7, _8, _9, T, J, Q, K, A }
impl CardValue {
    fn from_char(c: char) -> Option<CardValue> {
        match c {
            '*' => Some(CardValue::JOKER),
            '2' => Some(CardValue::_2),
            '3' => Some(CardValue::_3),
            '4' => Some(CardValue::_4),
            '5' => Some(CardValue::_5),
            '6' => Some(CardValue::_6),
            '7' => Some(CardValue::_7),
            '8' => Some(CardValue::_8),
            '9' => Some(CardValue::_9),
            'T' => Some(CardValue::T),
            'J' => Some(CardValue::J),
            'Q' => Some(CardValue::Q),
            'K' => Some(CardValue::K),
            'A' => Some(CardValue::A),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct CardHand {
    cards: [CardValue; 5],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl CardHand {
    fn rank(&self) -> HandRank {
        use HandRank::*;
        let mut counts = HashMap::new();
        let mut jokers_count = 0;
        for &card in self.cards.iter() {
            if card == CardValue::JOKER {
                jokers_count += 1;
            } else {
                *counts.entry(card).or_insert(0) += 1;
            } 
        }
        let mut count_vec: Vec<_> = counts.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        let hand_rank = match (count_vec.get(0).map(|t| t.1), count_vec.get(1).map(|t| t.1)) {
            (Some(&5), _) => FiveOfAKind,
            (Some(&4), _) => match jokers_count {
                1 => FiveOfAKind,
                _ => FourOfAKind,
            }
            (Some(&3), Some(&2)) => FullHouse,
            (Some(&3), _) => match jokers_count {
                1 => FourOfAKind,
                2 => FiveOfAKind,
                _ => ThreeOfAKind,
            }
            (Some(&2), Some(&2)) => match jokers_count {
                1 => FullHouse,
                _ => TwoPair,
            }
            (Some(&2), _) => match jokers_count {
                1 => ThreeOfAKind,
                2 => FourOfAKind,
                3 => FiveOfAKind,
                _ => OnePair,
            },
            _ => match jokers_count{
                5 => FiveOfAKind,
                4 => FiveOfAKind,
                3 => FourOfAKind,
                2 => ThreeOfAKind,
                1 => OnePair,
                _ => HighCard,
            }
        };
        hand_rank
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().cmp(&other.rank()).then_with(|| self.cards.cmp(&other.cards))
    }
}

fn parse_game(input: &str) -> Vec<(CardHand, u64)> {
    input.lines().map(|line| {
        let (card_part, bid_part) = line.split_once(" ").unwrap();
        let bid = bid_part.parse().unwrap();
        let mut cards = [CardValue::A; 5];
        for (i, c) in card_part.chars().enumerate() {
            if i>=5 { 
                eprintln!("More than 5 cards in hand: {}", card_part);
                std::process::exit(1);
            }
            cards[i] = CardValue::from_char(c).unwrap();
        }
        (CardHand { cards }, bid)
    }).collect()
}

pub fn part1(input: String) -> u64 {
    let mut game = parse_game(&input);
    game.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(&hand2));
    let mut total = 0;
    for (i, (_hand, bid)) in game.iter().enumerate() {
        total += bid * (i+1) as u64;
    }
    total as u64
}

pub fn part2(input: String) -> u64 {
    part1(input.replace("J", "*"))
}