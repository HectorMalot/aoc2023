#![allow(dead_code)]
use regex::Regex;
use std::str::FromStr;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds = seeds
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let maps: Vec<Mapper> = maps.split("\n\n").filter_map(|m| m.parse().ok()).collect();

    seeds
        .iter()
        .map(|s| {
            let mut value = *s;
            for map in maps.iter() {
                value = map.map(value);
            }
            value
        })
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"(\d+) (\d+)").unwrap();

    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let mut seeds = re
        .captures_iter(seeds)
        .map(|c| (c[1].parse::<u64>().unwrap(), c[2].parse::<u64>().unwrap()))
        .collect::<Vec<(u64, u64)>>();
    let maps: Vec<Mapper> = maps.split("\n\n").filter_map(|m| m.parse().ok()).collect();

    let res = maps
        .iter()
        .map(|m| {
            let mut next: Vec<(u64, u64)> = Vec::new();
            for seed in seeds.iter() {
                let mapped = m.map_range(*seed);
                next.extend(mapped);
            }
            // Optionally: merge overlapping ranges here
            seeds = next;
            seeds.clone()
        })
        .last()
        .unwrap();
    res.iter().map(|(a, _)| a).min().copied()
}

impl FromStr for Mapper {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, s) = s.split_once("\n").unwrap();
        let maps = s
            .lines()
            .map(|l| {
                let v = l.split_whitespace().collect::<Vec<&str>>();
                (
                    v[0].parse::<u64>().unwrap(),
                    v[1].parse::<u64>().unwrap(),
                    v[2].parse::<u64>().unwrap(),
                )
            })
            .collect();
        Ok(Mapper {
            header: header.split_ascii_whitespace().nth(0).unwrap().to_string(),
            maps,
        })
    }
}

impl Mapper {
    fn map(&self, value: u64) -> u64 {
        for map in self.maps.iter() {
            if map.1 <= value && value < map.1 + map.2 {
                return map.0 + (value - map.1);
            }
        }
        value
    }

    fn map_range(&self, range: (u64, u64)) -> Vec<(u64, u64)> {
        let mut result = Vec::new();
        // LOGIC HERE
        // if the full range is within a map, then map the whole range
        // if the range is partially within a map, then map the partial range, and then map the remainder
        // if the range is not within a map, then return the range as-is

        let mut start = range.0;
        let mut end = range.0 + range.1 - 1;
        for map in self.maps.iter() {
            if map.1 <= start && end < map.1 + map.2 {
                // Full range is within a map
                result.push((map.0 + (start - map.1), 1 + end - start));
                return result;
            } else if map.1 <= start && start < map.1 + map.2 {
                // Partial range is within a map (start - middle)
                result.push((map.0 + (start - map.1), map.1 + map.2 - start));
                start = map.1 + map.2;
            } else if map.1 <= end && end < map.1 + map.2 {
                // Partial range is within a map (middle - end)
                result.push((map.0, end - map.1 + 1));
                end = map.1 - 1;
            } else if start < map.1 && end >= map.1 + map.2 {
                // Partial range is within a map (start - end)
                result.push((map.0, map.2));
                // start -> map.1 (map.1 - start)
                result.extend(self.map_range((start, map.1 - start)));
                // map.1+map.2 -> end
                result.extend(self.map_range((map.1 + map.2, end - map.1 - map.2 + 1)));
                return result;
            }
        }
        result.push((start, end - start + 1));
        result
    }
}

#[derive(Debug)]
struct Mapper {
    header: String,
    maps: Vec<Map>,
}

type Map = (u64, u64, u64); // Dest range, Src range, range length

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_mapping() {
        let mapper = Mapper {
            header: "map-to-test".to_string(),
            maps: vec![(1, 4, 3), (5, 0, 4)],
        };
        assert_eq!(mapper.map(0), 5);
        assert_eq!(mapper.map(1), 6);
        assert_eq!(mapper.map(2), 7);
        assert_eq!(mapper.map(3), 8);
        assert_eq!(mapper.map(4), 1);
        assert_eq!(mapper.map(5), 2);
        assert_eq!(mapper.map(6), 3);
        assert_eq!(mapper.map(7), 7);
        assert_eq!(mapper.map(8), 8);
    }

    #[test]
    fn test_map_range() {
        let mapper = Mapper {
            header: "map-to-test".to_string(),
            maps: vec![(100, 20, 50)],
        };
        // No overlap
        assert_eq!(mapper.map_range((70, 10)), vec![(70, 10)]);
        // Full overlap (map surrounds range)
        assert_eq!(mapper.map_range((20, 10)), vec![(100, 10)]);
        // Partial overlap (range surrounds map)
        assert_eq!(
            mapper.map_range((0, 100)),
            vec![(100, 50), (0, 20), (70, 30)]
        );
        // Partial overlap (range start until middle)
        assert_eq!(mapper.map_range((60, 20)), vec![(140, 10), (70, 10)]);
        // Partial overlap (middle until range end)
        assert_eq!(mapper.map_range((10, 20)), vec![(100, 10), (10, 10)]);
    }
}
