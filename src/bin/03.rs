advent_of_code::solution!(3);

pub fn part_one(_input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let result: u64 = input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

            let mut w_s = 0;
            let mut result: Vec<u32> = vec![];

            for i in 0..12 {
                let w_e = numbers.len() - 11 + i;
                let w_slize = &numbers[w_s..w_e];

                // NOTE: This doesn't work, as max_by_key gives last match if multiple maxes are found
                // let (pos, max_value) = w_slize.iter().enumerate().max_by_key(|v| v.1).unwrap();
                // So, we need to do this

                let max_value = w_slize.iter().max().unwrap();
                let pos = w_slize.iter().position(|c| c == max_value).unwrap();

                w_s = w_s + pos + 1;
                result.push(*max_value);
            }

            result.iter().fold(0u64, |acc, d| acc * 10 + (*d as u64))
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
