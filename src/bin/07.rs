use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

const START: char = 'S';
const SPLITTER: char = '^';

pub fn find_splitter(
    s: &(usize, usize),
    splitters: &Vec<(usize, usize)>,
) -> Option<(usize, usize)> {
    let (y, x) = s;
    for (s_y, s_x) in splitters {
        if s_y > y && x == s_x {
            return Some((*s_y, *s_x));
        }
    }
    return None;
}

fn positions(input: &str) -> Vec<(usize, usize)> {
    return input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .into_iter()
                .filter(|(_, char)| *char == SPLITTER)
                .map(move |(x, _)| (y, x))
        })
        .flatten()
        .collect();
}

pub fn part_one(input: &str) -> Option<u64> {
    let splitters = positions(input);
    let start: (usize, usize) = (0, input.find(START).unwrap());

    let mut seen_set = HashSet::new();

    let mut current_lookup = vec![start];
    current_lookup.sort_by_key(|s| s.0);

    while !current_lookup.is_empty() {
        let current = current_lookup.pop().unwrap();
        let h_s_o = find_splitter(&current, &splitters);
        match h_s_o {
            Some(h_s) => {
                if !seen_set.contains(&h_s) {
                    // move?
                    seen_set.insert(h_s);
                    current_lookup.push((h_s.0, h_s.1 - 1));
                    current_lookup.push((h_s.0, h_s.1 + 1));
                }
            }
            None => continue,
        }
    }

    Some(seen_set.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count: u64 = 0;

    let splitters = positions(input);
    let start: (usize, usize) = (0, input.find(START).unwrap());

    let mut current_level: HashMap<(usize, usize), u64> = HashMap::new();
    current_level.entry(start).insert_entry(1);

    while !current_level.is_empty() {
        let mut next_level = HashMap::new();

        for (position, timeline_count) in &current_level {
            match find_splitter(&position, &splitters) {
                Some((hs_y, hs_x)) => {
                    *next_level.entry((hs_y, hs_x - 1)).or_insert(0) += timeline_count;
                    *next_level.entry((hs_y, hs_x + 1)).or_insert(0) += timeline_count;
                }
                None => {
                    count += timeline_count;
                    continue;
                }
            }
        }
        current_level = next_level;
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
