#![allow(dead_code)]

use regex::Regex;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse(input);
    races.iter().map(|r| r.win_options()).reduce(|a, b| a * b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums: Vec<u64> = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect();
    Race {
        time: nums[0],
        distance: nums[1],
    }
    .win_options()
    .into()
}

struct Race {
    time: u64,
    distance: u64,
}

fn parse(s: &str) -> Vec<Race> {
    let (t, d) = s.trim().split_once("\n").unwrap();
    let mut times = Vec::new();
    let mut distances = Vec::new();

    let re = Regex::new(r"\d+").unwrap();
    for cap in re.captures_iter(t) {
        times.push(cap[0].parse::<u64>().unwrap());
    }
    for cap in re.captures_iter(d) {
        distances.push(cap[0].parse::<u64>().unwrap());
    }
    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect()
}

impl Race {
    fn win_options(&self) -> u64 {
        let t = self.time as f64;
        let d = self.distance as f64 + 1.0;

        let start = ((-t + (t.powf(2.0) - 4.0 * d).sqrt()) / -2.0).ceil() as u64;
        self.time + 1 - (2 * start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse() {
        let input = "Time:      7  15   30
                           Distance:  9  40  200";
        let result = parse(input);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].time, 7);
        assert_eq!(result[0].distance, 9);
        assert_eq!(result[1].time, 15);
        assert_eq!(result[1].distance, 40);
    }
}
