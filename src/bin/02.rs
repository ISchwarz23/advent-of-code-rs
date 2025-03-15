advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| parse_line_to_sequence(line))
            .filter(|sequence| is_safe(sequence))
            .count() as u64,
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn parse_line_to_sequence(line: &str) -> Vec<i32> {
    line.split(" ")
        .map(|nr_str| nr_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn is_safe(sequence: &Vec<i32>) -> bool {
    let safe_deltas = if is_ascending(sequence) { 1..=3 } else { -3..=-1 };
    sequence
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .all(|delta| safe_deltas.contains(&delta))
}

fn is_ascending(sequence: &Vec<i32>) -> bool {
    sequence.iter().take(3).sum::<i32>() / 3 < sequence.iter().rev().take(3).sum::<i32>() / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
