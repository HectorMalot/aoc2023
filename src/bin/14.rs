advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut mirror = Mirror::new(input);
    mirror.tilt();
    mirror.load()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut mirror = Mirror::new(input);

    // Do ±150 cycles to get a stable state
    for _ in 1..=150 {
        mirror.cycle();
    }

    // Find a repeating pattern
    let mut loads = Vec::new();
    for _ in 1..=100 {
        mirror.cycle();
        loads.push(mirror.load().unwrap());
    }
    let pattern_length = find_pattern_length(&loads, 5).unwrap();
    let remainder = (1000000000 - 150) % pattern_length;
    loads.get(remainder - 1).copied()
}

fn find_pattern_length(input: &Vec<u32>, min_length: u32) -> Option<usize> {
    let mut pattern = Vec::new();
    for i in 0..input.len() {
        pattern.push(input[i]);
        if pattern.len() >= min_length as usize {
            if (0..pattern.len())
                .map(|j| pattern[j] == input[pattern.len() + j])
                .all(|x| x)
            {
                return Some(pattern.len());
            }
        }
    }
    None
}

struct Mirror {
    data: Vec<Vec<char>>,
}

impl Mirror {
    fn new(input: &str) -> Self {
        Self {
            data: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn load(&self) -> Option<u32> {
        self.data
            .iter()
            .rev()
            .enumerate()
            .map(|(n, line)| line.iter().filter(|c| **c == 'O').count() as u32 * (n + 1) as u32)
            .sum::<u32>()
            .into()
    }

    fn tilt(&mut self) {
        let input = &mut self.data;
        for i in 1..input.len() {
            for y in (1..=i).rev() {
                for x in 0..input[y].len() {
                    if input[y][x] == 'O' && input[y - 1][x] == '.' {
                        input[y - 1][x] = 'O';
                        input[y][x] = '.';
                    }
                }
            }
        }
    }

    fn rotate(&mut self) {
        let input = &mut self.data;
        let mut output = vec![vec!['.'; input.len()]; input.len()];
        for y in 0..input.len() {
            for x in 0..input.len() {
                output[x][input.len() - y - 1] = input[y][x];
            }
        }
        self.data = output;
    }

    fn cycle(&mut self) {
        for _ in 0..=3 {
            self.tilt();
            self.rotate();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
