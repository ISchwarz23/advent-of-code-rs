use std::collections::HashSet;
use advent_of_code::rect::Rectangle;
use advent_of_code::vector::{Vector2d, DIRS_MAIN};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_map(input);
    let result = map
        .get_trail_heads()
        .iter()
        .map(|pos| get_reachable_mountain_tops(pos, &map).into_iter().collect::<HashSet<_>>())
        .map(|unique_mountain_tops| unique_mountain_tops.iter().count() as u64)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_map(input);
    let result = map
        .get_trail_heads()
        .iter()
        .map(|pos| get_reachable_mountain_tops(pos, &map).iter().count() as u64)
        .sum();

    Some(result)
}

fn get_reachable_mountain_tops(start_pos: &Vector2d, map: &Map) -> Vec<Vector2d> {
    let mut positions = vec![start_pos.clone()];

    let mut reachable_mountain_tops: Vec<Vector2d> = Vec::new();
    while let Some(pos) = positions.pop() {
        if let Some(9) = map.get_height_at(&pos) {
            reachable_mountain_tops.push(pos);
            continue;
        }
        find_possible_next_positions(&pos, map)
            .into_iter()
            .for_each(|pos| positions.push(pos));
    }
    reachable_mountain_tops
}

fn find_possible_next_positions(current_pos: &Vector2d, map: &Map) -> Vec<Vector2d> {
    let current_height = map.get_height_at(current_pos).unwrap();
    DIRS_MAIN
        .iter()
        .map(|direction| current_pos + direction)
        .filter(|pos| map.bounds.contains(&pos))
        .filter(|pos| map.get_height_at(pos).unwrap() as i8 - current_height as i8 == 1)
        .collect()
}

fn parse_map(input: &str) -> Map {
    let heights = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    Map::new(heights)
}

struct Map {
    heights: Vec<Vec<u8>>,
    bounds: Rectangle,
}

impl Map {
    fn get_height_at(&self, pos: &Vector2d) -> Option<u8> {
        self.heights
            .get(pos.y as usize)
            .map(|row| row[pos.x as usize])
    }

    fn get_trail_heads(&self) -> Vec<Vector2d> {
        self.heights
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, height)| {
                    if height == &0 {
                        Some(Vector2d {
                            x: x as i32,
                            y: y as i32,
                        })
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<Vector2d>>()
    }

    fn new(heights: Vec<Vec<u8>>) -> Map {
        let bounds = Rectangle {
            x_from: 0,
            x_to: heights[0].len() as i32 - 1,
            y_from: 0,
            y_to: heights.len() as i32 - 1,
        };
        Map { heights, bounds }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
