use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<(&str, Vec<&str>)> {
    return input
        .lines()
        .map(|line| {
            let (from, to_raw) = line.split_once(":").unwrap();
            let to: Vec<&str> = to_raw.trim().split(" ").collect();

            (from, to)
        })
        .collect();
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_input = parse(input);

    let mut path_map = HashMap::new();
    for (from, to) in parsed_input {
        path_map.insert(from, to);
    }

    let start = "you";
    let end = "out";

    let mut to_visit: VecDeque<(Vec<&str>, u64)> = VecDeque::new();

    to_visit.push_back((vec![start], 0u64));

    let mut exits = 0;
    loop {
        match &to_visit.pop_front() {
            Some((prev_steps, distance)) => {
                let last_step = prev_steps.last().unwrap();

                let next_steps = &path_map[last_step];

                for next_step in next_steps {
                    if *next_step == end {
                        exits += 1;
                    } else {
                        let mut path_with_new_step = next_steps.clone();
                        path_with_new_step.push(next_step);

                        to_visit.push_back((path_with_new_step, distance + 1));
                    }
                }
            }
            None => break,
        }
    }

    Some(exits)
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
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
