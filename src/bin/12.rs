advent_of_code::solution!(12);

fn parse(input: &str) -> (Vec<usize>, Vec<(u64, Vec<u64>)>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let piece_sizes: Vec<usize> = parts[0..parts.len() - 1]
        .iter()
        .map(|piece| piece.chars().filter(|x| *x == '#').count())
        .collect();

    let challenges_raw = parts.last().unwrap();
    let challenges: Vec<(u64, Vec<u64>)> = challenges_raw
        .lines()
        .map(|line| {
            let (area, part_count) = line.split_once(":").unwrap();
            let (x, y) = area.split_once("x").unwrap();
            let size = x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap();

            let part_counts: Vec<u64> = part_count
                .trim()
                .split(" ")
                .map(|number| number.parse::<u64>().unwrap())
                .collect();

            (size, part_counts)
        })
        .collect();

    (piece_sizes, challenges)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (pieces, challenges) = parse(input);

    let matching_fields = challenges
        .iter()
        .filter(|(size, piece_config)| {
            let required_size: u64 = piece_config
                .iter()
                .enumerate()
                .map(|(i, count)| *count * pieces[i] as u64)
                .sum();

            return (1.2 * required_size as f64) < *size as f64;
        })
        .count();

    Some(matching_fields as u64)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
