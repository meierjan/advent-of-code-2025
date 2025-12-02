const LOCK_SIZE: i32 = 100;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut counter: u64 = 0;

    let numbers = input.split("\n").into_iter().map(|line| {
        let sign = line.chars().next().unwrap();
        let number = line.get(1..).unwrap().parse::<i32>().unwrap();
        let num = if sign == 'R' { number } else { -number };
        return num;
    });

    numbers.into_iter().fold(50, |current_lock_pos, rotation| {
        let new_lock_pos = (((current_lock_pos + rotation) % LOCK_SIZE) + LOCK_SIZE) % LOCK_SIZE;
        if new_lock_pos == 0 {
            counter += 1
        }
        return new_lock_pos;
    });

    println!("result: {}", counter);

    return Some(counter);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut click_counter: u64 = 0;

    let numbers = input.split("\n").into_iter().map(|line| {
        let sign = line.chars().next().unwrap();
        let number = line.get(1..).unwrap().parse::<i32>().unwrap();
        let num = if sign == 'R' { number } else { -number };
        return num;
    });

    numbers.into_iter().fold(50, |current_lock_pos, rotation| {
        let clicks_as_zero = if rotation.is_positive() {
            // rotate right
            (current_lock_pos + rotation) / LOCK_SIZE
        } else {
            // rotate left
            let val = ((LOCK_SIZE - current_lock_pos) + rotation.abs()) / LOCK_SIZE;
            if current_lock_pos == 0 { val - 1 } else { val }
        };

        click_counter += clicks_as_zero.unsigned_abs() as u64;

        return (((current_lock_pos + rotation) % LOCK_SIZE) + LOCK_SIZE) % LOCK_SIZE;
    });

    return Some(click_counter);
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
        assert_eq!(result, Some(6));
    }
}
