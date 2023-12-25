use std::str::FromStr;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut m = input.parse::<Maze>().unwrap();
    while m.step() {}
    Some(m.steps / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut m = input.parse::<Maze>().unwrap();
    let mut classified_map = vec![vec![Classification::Unvisited; m.pipes[0].len()]; m.pipes.len()];
    classified_map[m.current.1][m.current.0] = Classification::Pipe;

    // 1 round to classify all pipes
    while m.step() {
        classified_map[m.current.1][m.current.0] = Classification::Pipe;
    }

    // 2nd round to find the areas to the left and the right of the walking path
    m.direction = Direction::West;
    while m.step() {
        let (x, y) = (m.current.0, m.current.1);
        match (&m.direction, &m.pipes[m.current.1][m.current.0]) {
            // Straight pipes
            (Direction::North, Pipe::NS) => {
                if x > 0 && (classified_map[y][x - 1] == Classification::Unvisited) {
                    classified_map[y][x - 1] = Classification::Left;
                }
                if x < classified_map[y].len() - 1
                    && classified_map[y][x + 1] == Classification::Unvisited
                {
                    classified_map[y][x + 1] = Classification::Right;
                }
            }
            (Direction::South, Pipe::NS) => {
                if x > 0 && classified_map[y][x - 1] == Classification::Unvisited {
                    classified_map[y][x - 1] = Classification::Right;
                }
                if x < classified_map[y].len() - 1
                    && classified_map[y][x + 1] == Classification::Unvisited
                {
                    classified_map[y][x + 1] = Classification::Left;
                }
            }
            (Direction::East, Pipe::EW) => {
                if y > 0 && classified_map[y - 1][x] == Classification::Unvisited {
                    classified_map[y - 1][x] = Classification::Left;
                }
                if y < classified_map.len() - 1
                    && classified_map[y + 1][x] == Classification::Unvisited
                {
                    classified_map[y + 1][x] = Classification::Right;
                }
            }
            (Direction::West, Pipe::EW) => {
                if y > 0 && classified_map[y - 1][x] == Classification::Unvisited {
                    classified_map[y - 1][x] = Classification::Right;
                }
                if y < classified_map.len() - 1
                    && classified_map[y + 1][x] == Classification::Unvisited
                {
                    classified_map[y + 1][x] = Classification::Left;
                }
            }
            // Corner pipes
            (Direction::North, Pipe::NE) => {
                if x > 0 && classified_map[y][x - 1] == Classification::Unvisited {
                    classified_map[y][x - 1] = Classification::Left;
                }
                if y < classified_map.len() - 1
                    && classified_map[y + 1][x] == Classification::Unvisited
                {
                    classified_map[y + 1][x] = Classification::Left;
                }
            }
            (Direction::East, Pipe::NE) => {
                if x > 0 && classified_map[y][x - 1] == Classification::Unvisited {
                    classified_map[y][x - 1] = Classification::Right;
                }
                if y < classified_map.len() - 1
                    && classified_map[y + 1][x] == Classification::Unvisited
                {
                    classified_map[y + 1][x] = Classification::Right;
                }
            }
            (Direction::North, Pipe::NW) => {
                if x < classified_map[y].len() - 1
                    && classified_map[y][x + 1] == Classification::Unvisited
                {
                    classified_map[y][x + 1] = Classification::Right;
                }
                if y < classified_map.len() - 1
                    && classified_map[y + 1][x] == Classification::Unvisited
                {
                    classified_map[y + 1][x] = Classification::Right;
                }
            }
            (Direction::West, Pipe::NW) => {
                if x < classified_map[y].len() - 1
                    && classified_map[y][x + 1] == Classification::Unvisited
                {
                    classified_map[y][x + 1] = Classification::Left;
                }
                if y < classified_map.len() - 1
                    && classified_map[y + 1][x] == Classification::Unvisited
                {
                    classified_map[y + 1][x] = Classification::Left;
                }
            }
            (Direction::East, Pipe::SE) => {
                if x > 0 && classified_map[y][x - 1] == Classification::Unvisited {
                    classified_map[y][x - 1] = Classification::Left;
                }
                if y > 0 && classified_map[y - 1][x] == Classification::Unvisited {
                    classified_map[y - 1][x] = Classification::Left;
                }
            }
            (Direction::South, Pipe::SE) => {
                if x > 0 && classified_map[y][x - 1] == Classification::Unvisited {
                    classified_map[y][x - 1] = Classification::Right;
                }
                if y > 0 && classified_map[y - 1][x] == Classification::Unvisited {
                    classified_map[y - 1][x] = Classification::Right;
                }
            }
            // new
            (Direction::South, Pipe::SW) => {
                if x < classified_map[y].len() - 1
                    && classified_map[y][x + 1] == Classification::Unvisited
                {
                    classified_map[y][x + 1] = Classification::Left;
                }
                if y > 0 && classified_map[y - 1][x] == Classification::Unvisited {
                    classified_map[y - 1][x] = Classification::Left;
                }
            }
            (Direction::West, Pipe::SW) => {
                if x < classified_map[y].len() - 1
                    && classified_map[y][x + 1] == Classification::Unvisited
                {
                    classified_map[y][x + 1] = Classification::Right;
                }
                if y > 0 && classified_map[y - 1][x] == Classification::Unvisited {
                    classified_map[y - 1][x] = Classification::Right;
                }
            }
            _ => panic!("Invalid pipe at {:?}", m.current),
        }
    }

    // This is inefficient (multiple loops over the map) but I don't feel like
    // fighting the borrow checker right now
    let mut unclassified = true;
    while unclassified {
        unclassified = false;
        for y in 0..classified_map.len() {
            for x in 0..classified_map[y].len() {
                match classified_map[y][x] {
                    Classification::Left | Classification::Right => {
                        let mut n = vec![];
                        if x > 0 {
                            n.push((x - 1, y));
                        }
                        if x < classified_map[y].len() - 1 {
                            n.push((x + 1, y));
                        }
                        if y > 0 {
                            n.push((x, y - 1));
                        }
                        if y < classified_map.len() - 1 {
                            n.push((x, y + 1));
                        }
                        let mut n = n
                            .into_iter()
                            .filter(|(x, y)| {
                                *x < classified_map[0].len()
                                    && *y < classified_map.len()
                                    && x >= &0
                                    && y >= &0
                                    && classified_map[*y][*x] == Classification::Unvisited
                            })
                            .collect::<Vec<(usize, usize)>>();

                        while let Some((x2, y2)) = n.pop() {
                            classified_map[y2][x2] = classified_map[y][x].clone();
                        }
                    }
                    Classification::Unvisited => unclassified = true,
                    _ => {}
                }
            }
        }
    }

    // count all pipe classifications
    let counts = classified_map
        .iter()
        .map(|row| {
            row.iter().fold(
                (0, 0),
                |(left, right), classification| match classification {
                    Classification::Left => (left + 1, right),
                    Classification::Right => (left, right + 1),
                    _ => (left, right),
                },
            )
        })
        .reduce(|(left, right), (left2, right2)| (left + left2, right + right2))
        .unwrap();
    if classified_map[0][0] == Classification::Left {
        Some(counts.1)
    } else {
        Some(counts.0)
    }
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
            Pipe::Start => return false,
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
        return true;
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

#[derive(Debug, Clone, PartialEq)]
enum Classification {
    Pipe,
    Left,
    Right,
    Unvisited,
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
