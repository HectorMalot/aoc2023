use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i128> {
    let s = input.lines().map(|l| l.into()).collect::<Vec<Series>>();
    (s.iter()
        .map(|s| s.forecast(s.numbers.len() as i128))
        .sum::<i128>() as i128)
        .into()
}

pub fn part_two(input: &str) -> Option<i128> {
    let s = input.lines().map(|l| l.into()).collect::<Vec<Series>>();
    (s.iter().map(|s| s.forecast(-1_i128)).sum::<i128>() as i128).into()
}

#[derive(Debug)]
struct Series {
    numbers: Vec<i128>,
    factors: Vec<i128>,
}

impl From<&str> for Series {
    fn from(input: &str) -> Self {
        let numbers = input
            .split(' ')
            .map(|line| line.parse::<i128>().unwrap() as i128)
            .collect::<Vec<i128>>();
        let mut next = numbers.clone();
        let mut factors = Vec::new();
        loop {
            factors.push(next[0]);
            next = next
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<i128>>();
            if next.iter().all(|n| *n == 0) {
                break;
            }
        }
        Self { factors, numbers }
    }
}

impl Series {
    fn forecast(&self, x: i128) -> i128 {
        self.factors
            .iter()
            .enumerate()
            .map(|(i, f)| (f * x.fact_partial(i as i128) / (i as i128).fact()))
            .sum()
    }
}

trait Fact {
    type T;

    fn fact(&self) -> Self::T;
    fn fact_partial(&self, steps: Self::T) -> Self::T;
}

impl Fact for i128 {
    type T = i128;

    fn fact(&self) -> Self::T {
        (1..=*self).product()
    }
    fn fact_partial(&self, steps: Self::T) -> Self::T {
        ((*self - steps + 1)..=*self).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_factorials() {
        assert_eq!(0.fact(), 1);
        assert_eq!(1.fact(), 1);
        assert_eq!(2.fact(), 2);
        assert_eq!(3.fact(), 6);
        assert_eq!(4.fact(), 24);
        assert_eq!(5.fact(), 120);
    }

    #[test]
    fn test_partial_factorial() {
        assert_eq!(5.fact_partial(0), 1);
        assert_eq!(5.fact_partial(1), 5);
        assert_eq!(5.fact_partial(2), 20);
        assert_eq!(5.fact_partial(3), 60);
        assert_eq!(5.fact_partial(4), 120);
        assert_eq!(5.fact_partial(5), 120);
    }
}
