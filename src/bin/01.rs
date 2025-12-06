advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zeros: u64 = 0;
    for line in input.lines() {
        let direction_multiplier = if line.starts_with("R") { 1 } else { -1 };
        let steps: i32 = line[1..].parse().unwrap();
        dial = (dial + (direction_multiplier * steps)) % 100;
        if dial == 0 {
            zeros += 1
        }
        if dial < 0 {
            dial += 100
        }
        // println!("The dial is rotated `{line}` to point at `{dial}`.")
    }
    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zeros: u64 = 0;
    for line in input.lines() {
        let direction_multiplier = if line.starts_with("R") { 1 } else { -1 };
        let steps: i32 = line[1..].parse().unwrap();
        let motion = dial + (direction_multiplier * steps);

        let mut z_plus = 0;
        if motion >= 100 {
            z_plus = motion / 100;
        } else if motion <= 0 {
            z_plus = motion / -100;
            if dial != 0 {
                z_plus += 1;
            }
        }
        zeros += z_plus as u64;
        dial = motion % 100;
        if dial < 0 {
            dial += 100
        }
        // println!("The dial is rotated `{line}` to point at `{dial}`; during this rotation, it points at `0` *{z_plus} times*.")
    }
    Some(zeros)
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
