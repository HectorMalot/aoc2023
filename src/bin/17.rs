advent_of_code::solution!(17);

use pathfinding::prelude::astar;

pub fn part_one(input: &str) -> Option<u32> {
    find_path(parse(input), 1, 3).into()
}

pub fn part_two(input: &str) -> Option<u32> {
    find_path(parse(input), 4, 10).into()
}

fn find_path(board: Vec<Vec<u8>>, min_dist: usize, max_dist: usize) -> u32 {
    let goal: Pos = Pos {
        x: board[0].len() - 1,
        y: board.len() - 1,
    };
    let result = astar(
        &State::new(Pos { x: 0, y: 0 }, Direction::Down),
        |s| s.successors(min_dist, max_dist, &board),
        |s| s.distance(&goal),
        |s| s.pos == goal,
    );
    result.unwrap().1
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|x| x as u8 - b'0').collect())
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    facing: Direction,
}

impl State {
    fn new(pos: Pos, facing: Direction) -> Self {
        Self { pos, facing }
    }

    fn distance(&self, other: &Pos) -> u32 {
        (self.pos.x.abs_diff(other.x) + self.pos.y.abs_diff(other.y)) as u32
    }

    // Possible next nodes:
    // Always make a turn left/right
    // Step forward from min..=max steps
    fn successors(&self, min: usize, max: usize, board: &Vec<Vec<u8>>) -> Vec<(Self, u32)> {
        let mut result = Vec::new();
        let mut directions = vec![self.facing.turn_left(), self.facing.turn_right()];
        if self.pos.x == 0 && self.pos.y == 0 {
            // Special case for the start
            directions = vec![Direction::Down, Direction::Right]
        }
        for direction in directions {
            for i in min..=max {
                if let Some(pos) = direction.advance(self.pos, i) {
                    if pos.x >= board[0].len() || pos.y >= board.len() {
                        continue;
                    }
                    let state = Self::new(pos, direction);
                    result.push((state, self.pos.cost(&pos, &board))); // TODO: Cost
                }
            }
        }

        result
    }
}

impl Pos {
    fn step_towards(&self, other: &Pos) -> Self {
        if self.x < other.x {
            Pos {
                x: self.x + 1,
                y: self.y,
            }
        } else if self.x > other.x {
            Pos {
                x: self.x - 1,
                y: self.y,
            }
        } else if self.y < other.y {
            Pos {
                x: self.x,
                y: self.y + 1,
            }
        } else {
            Pos {
                x: self.x,
                y: self.y - 1,
            }
        }
    }

    fn cost(&self, to: &Pos, board: &Vec<Vec<u8>>) -> u32 {
        let mut cost = 0;
        let mut pos = *self;
        while pos != *to {
            pos = pos.step_towards(to);
            cost += board[pos.y][pos.x] as u32;
        }
        cost
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn advance(&self, pos: Pos, steps: usize) -> Option<Pos> {
        Some(match self {
            Self::Up if pos.y >= steps => Pos {
                x: pos.x,
                y: pos.y - steps,
            },
            Self::Down => Pos {
                x: pos.x,
                y: pos.y + steps,
            },
            Self::Left if pos.x >= steps => Pos {
                x: pos.x - steps,
                y: pos.y,
            },
            Self::Right => Pos {
                x: pos.x + steps,
                y: pos.y,
            },
            _ => return None,
        })
    }

    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
