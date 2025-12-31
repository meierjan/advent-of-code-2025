advent_of_code::solution!(6);

fn operation(a: u64, b: u64, op_char: char) -> u64 {
    return match op_char {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("Unknown operator"),
    };
}
fn init_value(op_char: char) -> u64 {
    match op_char {
        '+' => 0,
        '*' => 1,
        _ => panic!("Unknown operator"),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let numbers = &lines[..lines.len() - 1]
        .iter()
        .map(|entry| {
            entry
                .iter()
                .map(|value| value.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let functions: Vec<char> = lines
        .last()
        .unwrap()
        .iter()
        .map(|str| str.chars().next().unwrap())
        .collect();

    let mut final_result = 0;
    for x in 0..numbers[0].len() {
        let op = functions[x];
        let mut result: u64 = init_value(op);
        for y in 0..numbers.len() {
            result = operation(result, numbers[y][x], op);
        }

        final_result += result;
    }

    Some(final_result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().rev().collect())
        .collect();

    let operators: Vec<char> = lines
        .pop()
        .into_iter()
        .flatten()
        .filter(|item| *item != ' ')
        .collect();

    let x_len = lines[0].len();
    let y_len = lines.len();

    let mut lines_t = vec![vec![' '; y_len]; x_len];

    for (y, line) in lines.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            lines_t[x][y] = char;
        }
    }

    let exercises: Vec<u64> = lines_t
        .into_iter()
        .filter(|l| !l.iter().all(|c| *c == ' '))
        .map(|line| line.iter().collect::<String>())
        .map(|num_raw| num_raw.trim().parse::<u64>().unwrap())
        .collect();

    let chunked_exercises: Vec<&[u64]> = exercises.chunks(4).collect();

    let result = chunked_exercises
        .into_iter()
        .enumerate()
        .map(|(i, exercise)| {
            let op = operators[i];
            let init_value = init_value(op);

            exercise
                .into_iter()
                .fold(init_value, |acc, num| operation(acc, *num, op))
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
