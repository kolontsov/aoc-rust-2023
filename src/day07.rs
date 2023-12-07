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
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl CardHand {
    fn _rank(&self) -> HandRank {
        use HandRank::*;

        let mut counts = HashMap::new();
        for &card in self.cards.iter() {
            *counts.entry(card).or_insert(0) += 1;
        }
        let mut count_vec: Vec<_> = counts.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1).then_with(|| b.0.cmp(a.0)));

        let hand_rank = match (count_vec[0].1, count_vec.get(1).map(|t| t.1)) {
            (5, _) => FiveOfAKind,
            (4, _) => FourOfAKind,
            (3, Some(&2)) => FullHouse,
            (3, _) => ThreeOfAKind,
            (2, Some(&2)) => TwoPair,
            (2, _) => OnePair,
            _ => HighCard,
        };

        hand_rank
    }
    fn rank(&self) -> HandRank {
        let mut best_rank = self._rank();
        let jokers_count = self.cards.iter().filter(|&&card| card == CardValue::JOKER).count();

        if jokers_count > 0 {
            let card_values = vec![
                CardValue::_2, CardValue::_3, CardValue::_4, CardValue::_5, 
                CardValue::_6, CardValue::_7, CardValue::_8, CardValue::_9, 
                CardValue::T, CardValue::Q, CardValue::K, CardValue::A
            ];

            // Generate all combinations of card_values taken jokers_count at a time
            let combinations = generate_combinations(&card_values, jokers_count);

            for combo in combinations {
                let mut hand_clone = self.cards.clone();
                
                let mut combo_iter = combo.iter();
                for card in hand_clone.iter_mut() {
                    if *card == CardValue::JOKER {
                        *card = *combo_iter.next().unwrap_or(&CardValue::_2); // Replace with any value if none left
                    }
                }

                let combo_hand = CardHand{ cards: hand_clone };
                let combo_rank = combo_hand.rank();

                if combo_rank > best_rank {
                    best_rank = combo_rank;
                }
            }
        }

        best_rank
    }
}

fn generate_combinations(card_values: &[CardValue], count: usize) -> Vec<Vec<CardValue>> {
    if count == 0 {
        return vec![vec![]];
    }

    let mut combos: Vec<Vec<CardValue>> = Vec::new();
    for &value in card_values {
        // For each card_value, get all combinations for count - 1
        let sub_combos = generate_combinations(card_values, count - 1);
        for mut combo in sub_combos {
            // For each combination, add the current card_value to the front
            combo.insert(0, value);
            combos.push(combo);
        }
    }
    combos
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
    input.lines()
        .map(|line| {
            // println!("Line: {}", line);
            let (card_part, bid_part) = line.split_once(" ").unwrap();
            let bid = bid_part.parse().unwrap();
            let mut cards = [CardValue::A; 5];
            // println!("- card part: {}", card_part);
            for (i, c) in card_part.chars().enumerate() {
                if i>=5 { 
                    eprintln!("Warning: more than 5 cards in hand: {}", card_part);
                    std::process::exit(1);
                }
                // println!("  - card: {}", c);
                cards[i] = CardValue::from_char(c).unwrap();
            }
            (CardHand { cards }, bid)
    }).collect()
}

pub fn part1(input: String) -> u64 {
    let mut game = parse_game(&input);
    game.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(&hand2));
    let mut total = 0;
    for (i, (hand, bid)) in game.iter().enumerate() {
        println!("Hand {}: {:?}, bid: {}, rank: {:?}", i, hand.cards, bid, hand.rank());
        total += bid * (i+1) as u64;
    }
    total as u64
}

pub fn part2(input: String) -> u64 {
    let mut game = parse_game(&input.replace("J", "*"));
    game.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(&hand2));
    let mut total = 0;
    for (i, (hand, bid)) in game.iter().enumerate() {
        println!("Hand {}: {:?}, bid: {}, rank: {:?}", i, hand.cards, bid, hand.rank());
        total += bid * (i+1) as u64;
    }
    total as u64
}