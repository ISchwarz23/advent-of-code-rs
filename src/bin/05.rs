use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let page_comparator = parse_ordering_rules_to_comparator(input);
    let updates: Vec<Vec<u64>> = parse_updates(input);

    let result: u64 = updates
        .into_iter()
        .filter(|pages| check_pages_in_order(pages, &page_comparator))
        .map(|pages| pages[pages.len() / 2])
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let page_comparator = parse_ordering_rules_to_comparator(input);
    let updates: Vec<Vec<u64>> = parse_updates(input);

    let result: u64 = updates
        .into_iter()
        .filter(|pages| !check_pages_in_order(pages, &page_comparator))
        .map(|pages| clone_and_order(&pages, &page_comparator))
        .map(|pages| pages[pages.len() / 2])
        .sum();
    Some(result)
}

fn parse_ordering_rules_to_comparator(input: &str) -> impl Fn(u64, u64) -> Ordering {
    let ordering_rules: HashMap<(u64, u64), Ordering> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once("|").unwrap())
        .flat_map(|(first, second)| {
            let first = first.parse::<u64>().unwrap();
            let then = second.parse::<u64>().unwrap();
            vec![
                ((first, then), Ordering::Less),
                ((then, first), Ordering::Greater),
            ]
        })
        .collect();

    move |first, second| {
        ordering_rules
            .get(&(first, second))
            .map(|ord| *ord)
            .unwrap_or(Ordering::Equal)
    }
}

fn parse_updates(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|page| page.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

fn check_pages_in_order<F: Fn(u64, u64) -> Ordering>(pages: &Vec<u64>, page_comparator: F) -> bool {
    let copy = clone_and_order(pages, page_comparator);
    &copy == pages
}

fn clone_and_order<F: Fn(u64, u64) -> Ordering>(pages: &Vec<u64>, page_comparator: F) -> Vec<u64> {
    let mut copy = pages.clone();
    copy.sort_by(|arg0: &u64, arg1: &u64| page_comparator(*arg0, *arg1));
    copy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
