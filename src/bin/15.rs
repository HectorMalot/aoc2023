use std::str::FromStr;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split(',').map(hash).sum())
}

// rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
pub fn part_two(input: &str) -> Option<u32> {
    let input = input
        .trim()
        .split(',')
        .map(Instruction::from_str)
        .filter(Result::is_ok)
        .map(Result::unwrap);

    let mut state = vec![Vec::<Lens>::new(); 256];
    for ins in input {
        match ins {
            Instruction::Add(lbl, val) => {
                let idx = hash(&lbl);
                if state[idx as usize].iter().any(|lens| lens.label == lbl) {
                    state[idx as usize].iter_mut().for_each(|lens| {
                        if lens.label == lbl {
                            lens.value = val;
                        }
                    });
                    continue;
                }
                state[idx as usize].push(Lens {
                    label: lbl,
                    value: val,
                });
            }
            Instruction::Remove(lbl) => {
                let idx = hash(&lbl);
                state[idx as usize].retain(|lens| lens.label != lbl);
            }
        }
    }

    state
        .iter()
        .enumerate()
        .map(|(n_box, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(n_slot, lens)| lens.value * (n_box + 1) as u32 * (n_slot + 1) as u32)
                .sum::<u32>()
        })
        .sum::<u32>()
        .into()
}

fn hash(input: &str) -> u32 {
    input
        .chars()
        .map(|c| c as u32)
        .fold(0, |acc, c| ((acc + c) * 17) % 256)
}

enum Instruction {
    Add(String, u32),
    Remove(String),
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    value: u32,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('=') {
            if let Some((lbl, val)) = s.split_once('=') {
                Ok(Instruction::Add(lbl.to_string(), val.parse().unwrap()))
            } else {
                Err("Invalid instruction")
            }
        } else {
            let lbl = s.replace('-', "");
            Ok(Instruction::Remove(lbl))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        println!("{:?}", hash("qp"));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
