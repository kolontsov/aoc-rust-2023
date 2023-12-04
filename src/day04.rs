struct Card {
    //    num: u32,
    count: u32,
    winning: Vec<u32>,
    mine: Vec<u32>,
}

fn parse_card(input: &str) -> Card {
    let (_, card_part) = input.split_once(": ").unwrap();
    //let num = num_part.split_whitespace().last().unwrap().parse::<u32>().unwrap();
    let (winning_part, mine_part) = card_part.split_once("|").unwrap();
    let winning = winning_part
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let mine = mine_part
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let count = 1;
    Card {
        /*num,*/ winning,
        mine,
        count,
    }
}

fn parse_all(input: &str) -> Vec<Card> {
    input.lines().map(parse_card).collect()
}

pub fn part1(input: String) -> u64 {
    let cards = parse_all(&input);
    let mut total = 0;
    for card in cards.iter() {
        let numbers_won = card.mine.iter()
            .filter(|num| card.winning.contains(num))
            .count();
        if numbers_won > 0 {
            total += 1<<(numbers_won-1);
        }
    }
    total
}

pub fn part2(input: String) -> u64 {
    let mut cards = parse_all(&input);
    let len = cards.len();
    for idx in 0..len {
        let card = &cards[idx];
        let numbers_won = card.mine.iter()
            .filter(|num| card.winning.contains(num))
            .count();
        let n = card.count;
        for i in 0..numbers_won {
            cards[idx + i + 1].count += n
        }
    }
    cards.iter().map(|card| card.count as u64).sum()
}
