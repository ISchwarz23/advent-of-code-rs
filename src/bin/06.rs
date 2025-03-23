use advent_of_code::rect::Rectangle;
use advent_of_code::vector::{Vector2d, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP};
use std::collections::HashSet;

advent_of_code::solution!(6);

const GUARD_DIRECTIONS: [Vector2d; 4] = [DIR_UP, DIR_RIGHT, DIR_DOWN, DIR_LEFT];

pub fn part_one(input: &str) -> Option<u64> {
    let (map_bounds, obstacles, guard_start_position) = parse_input(input)?;

    let mut guard_position = guard_start_position.clone();
    let mut guard_direction_index = 0;

    let mut visited_positions = HashSet::new();
    while map_bounds.contains(&guard_position) {
        visited_positions.insert(guard_position.clone());

        let next_pos = &guard_position + &(GUARD_DIRECTIONS[guard_direction_index]);
        if obstacles.contains(&next_pos) {
            // turn right
            guard_direction_index += 1;
            if guard_direction_index == GUARD_DIRECTIONS.len() {
                guard_direction_index = 0;
            }
        } else {
            // move forward
            guard_position = next_pos;
        }
    }

    Some(visited_positions.len() as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> Option<(Rectangle, HashSet<Vector2d>, Vector2d)> {
    let area = Rectangle {
        x_from: 0,
        x_to: (input.lines().next()?.len() - 1) as i32,
        y_from: 0,
        y_to: (input.lines().count() - 1) as i32,
    };

    let obstacles: HashSet<Vector2d> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Vector2d {
                        x: x as i32,
                        y: y as i32,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    let guard_start_position: Vector2d = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars().enumerate().find_map(|(x, c)| {
                if c == '^' {
                    Some(Vector2d {
                        x: x as i32,
                        y: y as i32,
                    })
                } else {
                    None
                }
            })
        })
        .unwrap();

    Some((area, obstacles, guard_start_position))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
