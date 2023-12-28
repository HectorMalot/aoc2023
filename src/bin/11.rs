advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u128> {
    let m = Map::new(input, 2);
    m.galaxies
        .iter()
        .enumerate()
        .map(|(i, pos)| {
            let mut distance = 0;
            for other in &m.galaxies[i..] {
                distance += m.distance(pos, other);
            }
            distance
        })
        .sum::<u128>()
        .into()
}

pub fn part_two(input: &str) -> Option<u128> {
    let m = Map::new(input, 1000000);
    m.galaxies
        .iter()
        .enumerate()
        .map(|(i, pos)| {
            let mut distance = 0;
            for other in &m.galaxies[i..] {
                distance += m.distance(pos, other);
            }
            distance
        })
        .sum::<u128>()
        .into()
}

#[derive(Debug)]
struct Map {
    galaxies: Vec<Pos>,
    y_weights: Vec<u128>,
    x_weights: Vec<u128>,
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Map {
    fn new(input: &str, weight: u128) -> Self {
        let mut galaxies = Vec::new();
        let mut y_weights = Vec::new();
        let mut x_weights = vec![0; input.split_once('\n').unwrap().0.len()];

        for (y, line) in input.lines().enumerate() {
            if line.contains('#') {
                y_weights.push(1);
            } else {
                y_weights.push(weight);
            }

            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push(Pos {
                        x: x as i32,
                        y: y as i32,
                    });
                    x_weights[x] += 1;
                }
            }
        }
        let x_weights = x_weights
            .into_iter()
            .map(|x| if x == 0 { weight } else { 1 })
            .collect();

        Self {
            galaxies,
            y_weights,
            x_weights,
        }
    }

    fn distance(&self, pos: &Pos, other: &Pos) -> u128 {
        let mut distance = 0;
        for x in pos.x.min(other.x)..=pos.x.max(other.x) {
            distance += self.x_weights[x as usize];
        }
        for y in pos.y.min(other.y)..=pos.y.max(other.y) {
            distance += self.y_weights[y as usize];
        }
        distance - 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let m = Map::new(input, 100);
        let result: Option<u128> = m
            .galaxies
            .iter()
            .enumerate()
            .map(|(i, pos)| {
                let mut distance = 0;
                for other in &m.galaxies[i..] {
                    distance += m.distance(pos, other);
                }
                distance
            })
            .sum::<u128>()
            .into();
        assert_eq!(result, Some(8410));
    }
}
