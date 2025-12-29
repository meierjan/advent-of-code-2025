use std::collections::HashSet;

advent_of_code::solution!(4);

const ROLL_SIGN: char = '@';

pub fn can_access(
    x: usize,
    y: usize,
    positions: &Vec<Vec<char>>,
    removed: &HashSet<(usize, usize)>,
) -> bool {
    let max_y = positions.len() - 1;
    let max_x = positions[0].len() - 1;

    if y > max_y {
        panic!("Y out of bounds")
    }

    if x > max_x {
        panic!("X out of bounds")
    }

    let adj_pos: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let adj_roll_count = adj_pos
        .iter()
        .filter(|offset| {
            let (y_n, x_n) = (offset.0 + (y as i32), offset.1 + (x as i32));

            if y_n < 0 || y_n > (max_y as i32) {
                return false;
            }

            if x_n < 0 || x_n > (max_x as i32) {
                return false;
            }

            let y = y_n as usize;
            let x = x_n as usize;

            let p = (y, x);

            if removed.contains(&p) {
                return false;
            }

            return positions[y][x] == ROLL_SIGN;
        })
        .count();

    return adj_roll_count < 4;
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let empty_removed_list = HashSet::new();

    let mut amount = 0;

    for (y, column) in lines.iter().enumerate() {
        for (x, content) in column.iter().enumerate() {
            if content == &ROLL_SIGN && can_access(x, y, &lines, &empty_removed_list) {
                amount += 1;
            }
        }
    }

    return Some(amount);
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut amount = 0;
    let mut removed_rolls = HashSet::new();

    loop {
        let mut remove_in_turn = 0;
        for (y, column) in lines.iter().enumerate() {
            for (x, content) in column.iter().enumerate() {
                let candidate = (y, x);
                if removed_rolls.contains(&candidate) {
                    continue;
                } else if content == &ROLL_SIGN && can_access(x, y, &lines, &removed_rolls) {
                    removed_rolls.insert(candidate);
                    remove_in_turn += 1;
                }
            }
        }

        if remove_in_turn == 0 {
            break;
        } else {
            amount += remove_in_turn;
        }
    }

    return Some(amount);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
