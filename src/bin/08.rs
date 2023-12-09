use advent_of_code::aoc_lib::math::lcm;
use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, nodes) = parse(input);

    let mut steps = 0;
    let mut current_node = &nodes["AAA"];
    let mut d = directions.iter().cycle();
    let res = loop {
        steps += 1;
        match d.next().unwrap() {
            Direction::Left => {
                current_node = &nodes[&current_node.left];
            }
            Direction::Right => {
                current_node = &nodes[&current_node.right];
            }
        }
        if current_node.name == "ZZZ" {
            break steps;
        }
    };

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, nodes) = parse(input);

    let start_nodes = nodes
        .iter()
        .filter(|(k, _)| k.ends_with("A"))
        .map(|(_, v)| (v))
        .collect::<Vec<&Node>>();

    let steps = start_nodes
        .iter()
        .map(|n| {
            let mut steps = 0;
            let mut current_node = *n;
            let mut d = directions.iter().cycle();
            loop {
                steps += 1;
                match d.next().unwrap() {
                    Direction::Left => {
                        current_node = &nodes[&current_node.left];
                    }
                    Direction::Right => {
                        current_node = &nodes[&current_node.right];
                    }
                }
                if current_node.name.ends_with("Z") {
                    break steps;
                }
            }
        })
        .collect::<Vec<u64>>();

    // reduce with least common multiple
    steps.into_iter().reduce(lcm)
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<String, Node>) {
    let mut nodes = HashMap::new();
    let mut directions = Vec::new();

    // directions: LRLRLRRRLLRL
    let (d, n) = input.split_once("\n\n").unwrap();
    d.chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .for_each(|c| directions.push(c));

    // nodes: AAA = (BBB, CCC)
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    re.captures_iter(n)
        .map(|cap| Node {
            name: cap[1].to_string(),
            left: cap[2].to_string(),
            right: cap[3].to_string(),
        })
        .for_each(|n| {
            nodes.insert(n.name.clone(), n);
        });

    (directions, nodes)
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_filepath("examples", "08-b"));
        assert_eq!(result, Some(6));
    }
}
