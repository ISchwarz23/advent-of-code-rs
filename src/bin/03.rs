use regex::Regex;

advent_of_code::solution!(3);

const MUL_PATTERN: &str = r"mul\((\d{1,3}),(\d{1,3})\)";

pub fn part_one(input: &str) -> Option<u64> {
    let result: i32 = Regex::new(MUL_PATTERN).unwrap()
        .captures_iter(input)
        .map(|mul| {
            (
                mul[1].parse::<i32>().unwrap(),
                mul[2].parse::<i32>().unwrap(),
            )
        })
        .map(|(f1, f2)| f1 * f2)
        .sum();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .split("do()")
        .map(|s| s.split_once("don't()").map_or(s, |(before, _)| before))
        .map(|s| part_one(s).unwrap())
        .sum();
    Some(result)
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
        assert_eq!(result, Some(48));
    }
}
