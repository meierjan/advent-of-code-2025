advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;

    let ranges = input.split(',').map(|range_str| {
        // tuple (from, to)
        let (from_str, to_str) = range_str.trim().split_once('-').unwrap();
        return (
            from_str.parse::<u64>().unwrap(),
            to_str.parse::<u64>().unwrap(),
        );
    });

    for range in ranges {
        let (from, to) = range;
        for canditate in from..=to {
            let canditate_str = canditate.to_string();
            let c_len = canditate_str.len();
            if c_len % 2 == 0 {
                let middle = c_len / 2;
                // 6 -> [0..2, 3..5]
                let a = &canditate_str[0..middle];
                let b = &canditate_str[middle..c_len];

                if a == b {
                    total += canditate;
                }
            }
        }
    }

    return Some(total);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;

    let ranges = input.split(',').map(|range_str| {
        // tuple (from, to)
        let (from_str, to_str) = range_str.trim().split_once('-').unwrap();
        return (
            from_str.parse::<u64>().unwrap(),
            to_str.parse::<u64>().unwrap(),
        );
    });

    for range in ranges {
        let (from, to) = range;

        for canditate in from..=to {
            let candidate_str = canditate.to_string();
            let max_group_len = candidate_str.len() / 2;

            let chars: Vec<char> = candidate_str.chars().collect();

            for chunk_size in 1..=max_group_len {
                let mut chunked_c = chars.chunks(chunk_size);
                let first_chunk = chunked_c.next().unwrap();
                let is_match = chunked_c.all(|chunk| first_chunk == chunk);

                if is_match {
                    total += canditate;
                    break;
                }
            }
        }
    }

    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
