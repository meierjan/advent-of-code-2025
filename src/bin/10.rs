use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    #[allow(dead_code)]
    joltages: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split(" ").collect();

        let lights: Vec<bool> = parts[0]
            .trim_matches(['[', ']'])
            .chars()
            .map(|c| c == '#')
            .collect();

        let buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1]
            .into_iter()
            .map(|tuple| {
                tuple
                    .trim_matches(['(', ')'])
                    .split(",")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let joltage: Vec<usize> = parts[parts.len() - 1]
            .trim_matches(['{', '}'])
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        return Self {
            lights,
            buttons: buttons,
            joltages: joltage,
        };
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let result: u64 = input
        .lines()
        .map(Machine::from)
        .into_iter()
        .map(|machine| {
            let buttons_def = machine.buttons;

            let solution: Vec<bool> = machine.lights.into_iter().collect();

            let initial: Vec<(Vec<bool>, u64)> = vec![(vec![false; solution.len()], 0u64)];
            let mut queue = VecDeque::from(initial);

            let mut seen: HashSet<Vec<bool>> = HashSet::new();

            loop {
                let (current, d) = &queue.pop_front().unwrap();

                if seen.contains(current) {
                    continue;
                }

                for button in buttons_def.iter() {
                    let mut new_config = current.clone();
                    for b in button {
                        new_config[*b] = !new_config[*b];
                    }

                    if new_config == solution {
                        return *d + 1;
                    }

                    queue.push_back((new_config, d + 1));
                }

                seen.insert(current.clone());
            }
        })
        .sum();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
