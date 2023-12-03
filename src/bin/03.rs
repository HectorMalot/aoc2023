use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let (partlist, linklist) = parse_input(input);
    let res = partlist
        .iter()
        .filter_map(|(_pos, part)| {
            let neighbours = part.neighbours();
            if neighbours.iter().any(|pos| linklist.get(pos).is_some()) {
                Some(part.val)
            } else {
                None
            }
        })
        .sum::<i32>();
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (partlist, mut linklist) = parse_input(input);
    for (pos, part) in partlist.iter() {
        let neighbours = part.neighbours();
        for neighbour in neighbours {
            if let Some(link) = linklist.get_mut(&neighbour) {
                link.neighbours.push(*pos);
            }
        }
    }
    let res = linklist
        .iter()
        .filter(|(_pos, link)| link.val == '*' && link.neighbours.len() == 2)
        .map(|(_pos, link)| {
            partlist.get(&link.neighbours[0]).unwrap().val
                * partlist.get(&link.neighbours[1]).unwrap().val
        })
        .sum::<i32>();
    Some(res as u32)
}

#[derive(Debug, Clone, Copy)]
struct Part {
    val: i32,
    x: i32,
    y: i32,
    width: i32,
}

#[derive(Debug, Clone)]
struct Link {
    val: char,
    neighbours: Vec<(i32, i32)>,
}

impl Part {
    fn neighbours(&self) -> Vec<(i32, i32)> {
        let mut neighbours = Vec::new();
        for x in (self.x - 1)..=self.x + self.width {
            for y in vec![self.y - 1, self.y + 1] {
                neighbours.push((x, y));
            }
        }
        neighbours.push((self.x - 1, self.y));
        neighbours.push((self.x + self.width, self.y));
        neighbours
    }
}

fn parse_input(input: &str) -> (HashMap<(i32, i32), Part>, HashMap<(i32, i32), Link>) {
    let mut partlist: HashMap<(i32, i32), Part> = HashMap::new();
    let mut linklist: HashMap<(i32, i32), Link> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let mut active_part = Option::<Part>::None;
        line.chars().enumerate().for_each(|(x, c)| {
            if c.is_numeric() {
                if let Some(ref mut part) = active_part {
                    // Extend the current part
                    part.val = part.val * 10 + c.to_digit(10).unwrap() as i32;
                    part.width += 1;
                } else {
                    // Start a new part
                    active_part = Some(Part {
                        val: c.to_digit(10).unwrap() as i32,
                        x: x as i32,
                        y: y as i32,
                        width: 1,
                    });
                }
            } else {
                if let Some(part) = active_part {
                    // End the current part
                    partlist.insert((part.x, part.y), part);
                    active_part = None;
                }
                if c != '.' {
                    // Add the current character
                    linklist.insert(
                        (x as i32, y as i32),
                        Link {
                            val: c,
                            neighbours: vec![],
                        },
                    );
                }
            }
        });
        // End the current part (end of line edge case)
        if let Some(part) = active_part {
            partlist.insert((part.x, part.y), part);
        }
    });
    (partlist, linklist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }

    #[test]
    fn test_neighbours() {
        let part = Part {
            val: 1,
            x: 1,
            y: 1,
            width: 3,
        };
        let neighbours = part.neighbours();
        assert!(neighbours.contains(&(0, 0)));
        assert!(neighbours.contains(&(1, 0)));
        assert!(neighbours.contains(&(2, 0)));
        assert!(neighbours.contains(&(3, 0)));
        assert!(neighbours.contains(&(4, 0)));
        assert!(neighbours.contains(&(0, 1)));
        assert!(neighbours.contains(&(4, 1)));
        assert!(neighbours.contains(&(0, 2)));
        assert!(neighbours.contains(&(1, 2)));
        assert!(neighbours.contains(&(2, 2)));
        assert!(neighbours.contains(&(3, 2)));
        assert!(neighbours.contains(&(4, 2)));
        assert_eq!(neighbours.len(), 12);
    }
}
