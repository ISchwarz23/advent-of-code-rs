advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    }
    left.sort_unstable();
    right.sort_unstable();

    let mut result: u64 = 0;
    for (index, left_value) in left.iter().enumerate() {
        let right_value = right[index];
        result += left_value.abs_diff(right_value) as u64;
    }

    Some(result)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
