advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse(input);
    Some(cards.iter().map(|c| c.score1()).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse(input);
    // let mut stack = HashMap::<u32, u32>::new();
    let mut stack = vec![1; cards.len()];

    for card in cards {
        let s = card.matches();
        for n in 1..=s {
            stack[(card.id + n - 1) as usize] += stack[(card.id - 1) as usize];
            // if let Some(v) = stack.get(&(card.id + n)) {
            //     stack.insert(card.id + n, v + stack.get(&card.id).unwrap());
            // }
        }
    }
    Some(stack.iter().sum())
}

impl Card {
    fn score1(&self) -> u32 {
        match self.matches() {
            0 => 0,
            _ => 2_u32.pow(self.matches() - 1),
        }
    }

    fn matches(&self) -> u32 {
        self.left
            .iter()
            .map(|l| self.right.contains(l))
            .filter(|b| *b)
            .count() as u32
    }
}

fn parse(input: &str) -> Vec<Card> {
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    input
        .lines()
        .map(|line| {
            let (id, rhs) = line.split_once(": ").unwrap();
            let id = id.split_whitespace().last().unwrap().parse().unwrap();
            let (left, right) = rhs.split_once(" | ").unwrap();
            let left = left
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let right = right
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            Card { id, left, right }
        })
        .collect()
}

#[allow(dead_code)]
#[derive(Debug)]
struct Card {
    id: u32,
    left: Vec<u32>,
    right: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
