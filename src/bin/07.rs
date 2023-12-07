use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut cards = input.lines().map(Hand::new).collect::<Vec<_>>();
    cards.sort_unstable();
    cards
        .iter()
        .enumerate()
        .map(|(i, c)| (i + 1) as u32 * c.bet)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.replace("J", "X");
    let mut cards = input.lines().map(Hand::new).collect::<Vec<_>>();
    cards.sort_unstable();
    cards
        .iter()
        .enumerate()
        .map(|(i, c)| (i + 1) as u32 * c.bet)
        .sum::<u32>()
        .into()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
enum Rank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: Vec<Card>,
    rank: Rank,
    bet: u32,
}

impl Hand {
    fn new(input: &str) -> Hand {
        let (cards, bet) = input.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(Card::new)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();
        Hand {
            rank: determine_rank(&cards),
            cards,
            bet: bet.parse().unwrap(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.rank.cmp(&other.rank) {
            std::cmp::Ordering::Equal => {
                let compares = self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .collect::<Vec<(&Card, &Card)>>();
                for c in compares {
                    match c.0.cmp(c.1) {
                        std::cmp::Ordering::Equal => continue,
                        other => return Some(other),
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
            other => Some(other),
        }
    }
}

fn determine_rank(cards: &Vec<Card>) -> Rank {
    let mut uniques = HashMap::new();
    for card in cards {
        if card == &Card::Joker {
            continue;
        }
        *uniques.entry(card).or_insert(0) += 1;
    }
    match uniques.len() {
        0 | 1 => Rank::FiveOfAKind,
        2 => {
            let mut counts = uniques.values().collect::<Vec<_>>();
            counts.sort();
            match counts.as_slice() {
                [1, _] => Rank::FourOfAKind,
                [2, _] => Rank::FullHouse,
                _ => panic!("Invalid hand"),
            }
        }
        3 => {
            let mut counts = uniques.values().collect::<Vec<_>>();
            counts.sort();
            match counts.as_slice() {
                [1, 1, _] => Rank::ThreeOfAKind,
                [1, 2, _] => Rank::TwoPair,
                _ => panic!("Invalid hand"),
            }
        }
        4 => Rank::Pair,
        5 => Rank::HighCard,
        _ => panic!("Invalid hand"),
    }
}

impl Card {
    fn new(s: char) -> Result<Card, &'static str> {
        match s {
            'X' => Ok(Card::Joker),
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err("Invalid card"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_ord() {
        assert!(Card::Two < Card::Three);
        assert!(Card::Three < Card::Four);
        assert!(Card::Four < Card::Five);
        assert!(Card::Five < Card::Six);
        assert!(Card::Six < Card::Seven);
        assert!(Card::Seven < Card::Eight);
        assert!(Card::Eight < Card::Nine);
        assert!(Card::Nine < Card::Ten);
        assert!(Card::Ten < Card::Jack);
        assert!(Card::Jack < Card::Queen);
        assert!(Card::Queen < Card::King);
        assert!(Card::King < Card::Ace);
    }
}
