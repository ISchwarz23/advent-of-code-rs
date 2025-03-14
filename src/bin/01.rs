use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left_sequence, mut right_sequence) = parse_input(input);
    left_sequence.sort_unstable();
    right_sequence.sort_unstable();

    let mut result: u64 = 0;
    for (index, left_value) in left_sequence.iter().enumerate() {
        let right_value = right_sequence[index];
        result += left_value.abs_diff(right_value) as u64;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left_sequence, right_sequence) = parse_input(input);
    let left_sequence_no_of_occurrence = to_occurrence_map(left_sequence);
    let right_sequence_no_of_occurrence = to_occurrence_map(right_sequence);

    let mut result: u64 = 0;
    for (number, no_of_occurrence_left) in left_sequence_no_of_occurrence.iter() {
        let no_of_occurrence_right = right_sequence_no_of_occurrence.get(number).unwrap_or(&(0));
        result += (number * no_of_occurrence_left * no_of_occurrence_right) as u64;
    }
    Some(result)
}

fn to_occurrence_map(numbers: Vec<u32>) -> HashMap<u32, u32> {
    let mut occurrences: HashMap<u32, u32> = HashMap::new();
    for &num in &numbers {
        *occurrences.entry(num).or_insert(0) += 1;
    }
    occurrences
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    }
    (left, right)
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
        assert_eq!(result, Some(31));
    }
}
