use lazy_static;
use regex::Regex;
use std::str::FromStr;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().map(|line| Game::from_str(line).unwrap());

    // The Elf would first like to know which games would have been possible if the bag contained only
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let max: (u32, u32, u32) = (12, 13, 14);

    let res = games
        .filter_map(|game| {
            for round in game.rounds {
                if round.red > max.0 || round.green > max.1 || round.blue > max.2 {
                    return None;
                }
            }
            Some(game.id)
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.lines().map(|line| Game::from_str(line).unwrap());

    let res = games
        .filter_map(|game| {
            let mut max: (u32, u32, u32) = (0, 0, 0);
            for round in game.rounds {
                if round.red > max.0 {
                    max.0 = round.red;
                }
                if round.green > max.1 {
                    max.1 = round.green;
                }
                if round.blue > max.2 {
                    max.2 = round.blue;
                }
            }
            Some(max.0 * max.1 * max.2)
        })
        .sum();
    Some(res)
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    blue: u32,
    red: u32,
    green: u32,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(": ").ok_or("Invalid line format")?;
        let id = l
            .split_once(" ")
            .ok_or("Invalid game format")?
            .1
            .parse::<u32>()
            .map_err(|_| -> &str { "Invalid game format" })?;
        let rounds = r.split("; ").map(|round| Round::new(round)).collect();
        Ok(Self { id, rounds })
    }
}

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
}

impl Round {
    fn new(input: &str) -> Self {
        // let re = Regex::new(r"(\d+) (\w+)").unwrap();
        // let re = &*RE;
        // 3 blue, 4 red
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for cap in RE.captures_iter(input) {
            let amount = cap[1].parse::<u32>().unwrap();
            match &cap[2] {
                "blue" => blue = amount,
                "red" => red = amount,
                "green" => green = amount,
                _ => panic!("Unknown color"),
            }
        }
        Self { blue, red, green }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }

    #[test]
    fn test_parser() {
        let input = "Game 1: 3 blue, 4 red; 5 green, 2 blue";
        let game = Game::from_str(input).unwrap();
        assert_eq!(game.id, 1);
        assert_eq!(game.rounds.len(), 2);
        assert_eq!(game.rounds[0].blue, 3);
        assert_eq!(game.rounds[0].red, 4);
        assert_eq!(game.rounds[0].green, 0);
        assert_eq!(game.rounds[1].blue, 2);
        assert_eq!(game.rounds[1].red, 0);
        assert_eq!(game.rounds[1].green, 5);
    }
}
