use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 75))
}

fn solve(input: &str, number_of_blinks: usize) -> u64 {
    let numbers = parse_numbers(input);
    let mut occurrences = to_occurrence_map(&numbers);
    for _ in 0..number_of_blinks {
        occurrences = blink(occurrences);
    }
    occurrences.values().sum()
}

fn blink(occurrences: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut occurrences_after_blink: HashMap<u64, u64> = HashMap::new();
    occurrences
        .into_iter()
        .flat_map(|(stone, number_of_occurrence)| {
            blink_on_stone(stone)
                .into_iter()
                .map(|stone_after_blink| (stone_after_blink, number_of_occurrence))
                .collect::<Vec<(u64, u64)>>()
        })
        .for_each(|(stone_after_blink, number_of_occurrence)| {
            *occurrences_after_blink
                .entry(stone_after_blink)
                .or_insert(0) += number_of_occurrence
        });
    occurrences_after_blink
}

fn blink_on_stone(stone: u64) -> Vec<u64> {
    // If the stone is engraved with the number 0, it is replaced by a stone engraved with the
    // number 1.
    if stone == 0 {
        return vec![1];
    }

    // If the stone is engraved with a number that has an even number of digits, it is replaced
    // by two stones. The left half of the digits are engraved on the new left stone, and the
    // right half of the digits are engraved on the new right stone.
    let str = format!("{}", stone);
    if str.len() % 2 == 0 {
        let mid = str.len() / 2;
        let (first_half, second_half) = str.split_at(mid);
        return vec![
            first_half.parse::<u64>().unwrap(),
            second_half.parse::<u64>().unwrap(),
        ];
    }

    // If none of the other rules apply, the stone is replaced by a new stone; the old stone's
    // number multiplied by 2024 is engraved on the new stone.
    vec![stone * 2024]
}

fn parse_numbers(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn to_occurrence_map(numbers: &Vec<u64>) -> HashMap<u64, u64> {
    let mut occurrences: HashMap<u64, u64> = HashMap::new();
    for &num in numbers {
        *occurrences.entry(num).or_insert(0) += 1;
    }
    occurrences
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_ne!(result, None);
    }
}
