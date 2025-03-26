use std::collections::HashSet;

advent_of_code::solution!(7);


struct Operations;
impl Operations {
    fn add(first: &u64, second: &u64) -> u64 {
        first + second
    }
    fn multiply(first: &u64, second: &u64) -> u64 {
        first * second
    }
    fn concat(first: &u64, second: &u64) -> u64 {
        format!("{}{}", first, second).parse::<u64>().unwrap()
    }
}


pub fn part_one(input: &str) -> Option<u64> {
    let operations = [Operations::add, Operations::multiply];
    Some(solve(input, &operations))
}

pub fn part_two(input: &str) -> Option<u64> {
    let operations = [Operations::add, Operations::multiply, Operations::concat];
    Some(solve(input, &operations))
}

fn solve(input: &str, operations: &[fn(&u64, &u64) -> u64]) -> u64 {
    input.lines()
        .map(|line| parse(line))
        .filter(|(test_value, numbers)| can_be_solved(test_value, numbers, operations))
        .map(|(test_value, _)| test_value)
        .sum()
}


fn parse(line: &str) -> (u64, Vec<u64>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let test_value: u64 = parts[0].parse::<u64>().unwrap();
    let remaining_numbers: Vec<u64> = parts[1].split_whitespace().map(|nbr| nbr.parse::<u64>().unwrap()).collect();
    (test_value, remaining_numbers)
}

fn can_be_solved(test_value: &u64, numbers: &Vec<u64>, operations: &[fn(&u64, &u64) -> u64]) -> bool {
    let mut results = HashSet::from([numbers.first().unwrap().clone()]);

    for next_number in numbers.iter().skip(1) {
        results = results.iter()
            .flat_map(|intermediate_result| {
                operations.iter().map(|operation| operation(intermediate_result, next_number)).collect::<Vec<u64>>()
            })
            .filter(|result| result <= test_value)
            .collect::<HashSet<u64>>();
    }

    results.contains(test_value)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
