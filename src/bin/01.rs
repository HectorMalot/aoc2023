advent_of_code::solution!(1);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| line.chars().filter(|&c| c.is_numeric()).collect::<String>())
        .filter(|line| line.len() >= 1)
        .map(|line| {
            line.chars().nth(0).unwrap().to_digit(10).unwrap() * 10
                + line.chars().last().unwrap().to_digit(10).unwrap()
        })
        .sum();
    Some(result)
}

// list of written out numbers
const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .lines()
        // recursively replace all written out numbers with their digits
        .map(|line| {
            let mut line = line.to_string();
            for (i, number) in NUMBERS.iter().enumerate() {
                line = line.replace(number, format!("{}{}{}", number, i, number).as_str());
            }
            line
        })
        .join("\n");
    part_one(&res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_filepath("examples", "01-b"));
        assert_eq!(result, Some(281));
    }
}
