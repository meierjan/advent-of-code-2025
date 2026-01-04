use std::collections::{HashMap, HashSet, VecDeque};

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
                        let mut path_with_new_step = prev_steps.clone();
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

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_input = parse(input);
    let mut path_map = HashMap::new();
    for (from, to) in parsed_input {
        path_map.insert(from, to);
    }

    let start = "svr";
    let end = "out";

    let fft = "fft";
    let dac = "dac";

    let mut cache: HashMap<(&str, &str), u64> = HashMap::new();

    let fft_2_dac = part2_rek(&path_map, fft, dac, &mut cache);

    let result: u64;
    if fft_2_dac > 0 {
        result = part2_rek(&path_map, start, fft, &mut cache)
            * part2_rek(&path_map, fft, dac, &mut cache)
            * part2_rek(&path_map, dac, end, &mut cache);
    } else {
        result = part2_rek(&path_map, start, fft, &mut cache)
            * part2_rek(&path_map, dac, fft, &mut cache)
            * part2_rek(&path_map, fft, end, &mut cache);
    }
    return Some(result);
}

pub fn part2_rek<'a>(
    input: &HashMap<&'a str, Vec<&'a str>>,
    src: &'a str,
    out: &'a str,
    cache: &mut HashMap<(&'a str, &'a str), u64>,
) -> u64 {
    if src == out {
        1u64
    } else {
        let children = input.get(src);

        match children {
            Some(children) => {
                let sum = children
                    .into_iter()
                    .map(|c| {
                        let result: u64;

                        let c_opt = cache.get(&(*c, out));

                        match c_opt {
                            Some(c) => result = *c,
                            None => {
                                result = part2_rek(input, c, out, cache);
                                cache.insert((c, out), result);
                            }
                        }

                        result
                    })
                    .sum();
                sum
            }
            None => 0u64,
        }
    }
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
        let example_b = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        let result = part_two(example_b);
        assert_eq!(result, Some(2));
    }
}
