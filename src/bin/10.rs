use std::collections::VecDeque;

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

fn is_sol(sel: &Vec<usize>, btns: &Vec<Vec<usize>>, sol: &Vec<bool>) -> bool {
    let mut current = vec![false; sol.len()];

    for s in sel {
        let bts = &btns[*s];

        for b in bts {
            current[*b] = !current[*b];
        }
    }

    current == *sol
}

pub fn part_one(input: &str) -> Option<u64> {
    let result: u64 = input
        .lines()
        .map(Machine::from)
        .into_iter()
        .map(|machine| {
            let buttons_def = machine.buttons;
            let button_count = buttons_def.len();

            let solution: Vec<bool> = machine.lights.into_iter().collect();

            let initial: Vec<Vec<usize>> = (0..button_count).map(|b| vec![b]).collect();
            let mut queue = VecDeque::from(initial);

            loop {
                let current_btn_config = &queue.pop_front().unwrap();

                if is_sol(current_btn_config, &buttons_def, &solution) {
                    return current_btn_config.len() as u64;
                }

                for b_i in 0..button_count {
                    let mut new_btn_config = current_btn_config.clone();
                    new_btn_config.push(b_i);

                    queue.push_back(new_btn_config);
                }
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
