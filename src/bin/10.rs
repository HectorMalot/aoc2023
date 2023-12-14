use std::str::FromStr;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut m = input.parse::<Maze>().unwrap();
    while !m.step() {}
    Some(m.steps / 2)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[derive(Debug)]
struct Maze {
    pipes: Vec<Vec<Pipe>>,
    current: (usize, usize),
    direction: Direction,
    steps: u32,
}

impl Maze {
    fn step(&mut self) -> bool {
        // move in the current direction
        match self.direction {
            Direction::North => {
                self.current.1 -= 1;
            }
            Direction::East => {
                self.current.0 += 1;
            }
            Direction::South => {
                self.current.1 += 1;
            }
            Direction::West => {
                self.current.0 -= 1;
            }
        }
        self.steps += 1;

        // check if we need to change direction
        match self.pipes[self.current.1][self.current.0] {
            Pipe::NS | Pipe::EW => {}
            Pipe::Start => return true,
            Pipe::NE => match self.direction {
                Direction::South => self.direction = Direction::East,
                Direction::West => self.direction = Direction::North,
                _ => panic!("Invalid direction for NE pipe"),
            },
            Pipe::SE => match self.direction {
                Direction::West => self.direction = Direction::South,
                Direction::North => self.direction = Direction::East,
                _ => panic!("Invalid direction for SE pipe"),
            },
            Pipe::NW => match self.direction {
                Direction::South => self.direction = Direction::West,
                Direction::East => self.direction = Direction::North,
                _ => panic!("Invalid direction for NW pipe"),
            },
            Pipe::SW => match self.direction {
                Direction::North => self.direction = Direction::West,
                Direction::East => self.direction = Direction::South,
                _ => panic!("Invalid direction for SW pipe"),
            },
            Pipe::Ground => panic!("Fell off the maze at {:?}", self.current),
        }
        return false;
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Pipe {
    NS,
    EW,
    NE,
    SE,
    NW,
    SW,
    Start,
    Ground,
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            'S' => Pipe::Start,
            '.' => Pipe::Ground,
            _ => panic!("Invalid pipe character: {}", c),
        }
    }
}

impl FromStr for Maze {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0, 0);
        let pipes = s
            .lines()
            .map(|line| line.chars().map(Pipe::from).collect())
            .collect::<Vec<Vec<Pipe>>>();
        pipes.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, pipe)| {
                if let Pipe::Start = pipe {
                    start = (x, y);
                }
            })
        });
        Ok(Maze {
            pipes,
            current: start,
            direction: Direction::West,
            steps: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
