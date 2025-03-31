advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|report_str| parse_report(report_str))
        .filter(|report| is_safe(report))
        .count();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|report_str| parse_report(report_str))
        .filter(|report| is_safe_when_skipping(report))
        .count();
    Some(result as u64)
}

fn parse_report(report_str: &str) -> Vec<i32> {
    report_str.split(" ")
        .map(|level_str| level_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let safe_deltas = if is_ascending(report) { 1..=3 } else { -3..=-1 };
    report
        .windows(2)
        .map(|level_pair| level_pair[1] - level_pair[0])
        .all(|level_delta| safe_deltas.contains(&level_delta))
}

fn is_safe_when_skipping(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        // could this be improved? (avoid cloning)
        let mut report_with_skipped_level = report.clone();
        report_with_skipped_level.remove(i);
        if is_safe(&report_with_skipped_level) {
            return true;
        }
    }
    false
}

fn is_ascending(report: &Vec<i32>) -> bool {
    report.first().unwrap() < report.last().unwrap()
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
        assert_eq!(result, Some(4));
    }
}
