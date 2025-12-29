advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges_vec, ingredients_vec) = input.split_once("\n\n").unwrap();

    let ingredients = ingredients_vec
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let ranges = ranges_vec
        .lines()
        .map(|line| {
            let (from, to) = line.split_once("-").unwrap();
            return (from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap());
        })
        .collect::<Vec<(u64, u64)>>();

    let in_range = |ing: &u64, ranges: &Vec<(u64, u64)>| -> bool {
        return ranges.iter().any(|(from, to)| from <= ing && ing <= to);
    };

    let count = ingredients
        .into_iter()
        .filter(|ing| in_range(&ing, &ranges))
        .count();

    return Some(count as u64);
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges_vec, _) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges_vec
        .lines()
        .map(|line| {
            let (from, to) = line.split_once("-").unwrap();
            return (from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap());
        })
        .collect::<Vec<(u64, u64)>>();

    ranges.sort_by_key(|r| r.0);

    // contraint is a.0 <= b.0
    let intersect = |a: &(u64, u64), b: &(u64, u64)| -> bool {
        return a.1 >= b.0;
    };
    let merge = |a: &(u64, u64), b: &(u64, u64)| -> (u64, u64) {
        return (a.0.min(b.0), a.1.max(b.1));
    };

    let mut result: Vec<(u64, u64)> = vec![];

    let mut i = 0;

    while i < ranges.len() {
        let r_i = ranges[i];

        let mut j = i + 1;

        let mut r_result = r_i;

        while j < ranges.len() {
            let r_j = ranges[j];

            if intersect(&r_result, &r_j) {
                r_result = merge(&r_result, &r_j);

                i += 1;
                j += 1;
            } else {
                break;
            }
        }

        result.push(r_result);
        i += 1;
    }

    let range_length = result.into_iter().map(|(from, to)| to - from + 1).sum();

    Some(range_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
