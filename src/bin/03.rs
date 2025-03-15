use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result: i32 = mul_pattern.captures_iter(input)
        .map(|mul| (mul[1].parse::<i32>().unwrap(), mul[2].parse::<i32>().unwrap()))
        .map(|(f1, f2)| f1 * f2)
        .sum();
    Some(result as u64)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
