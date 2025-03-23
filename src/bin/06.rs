use advent_of_code::rect::Rectangle;
use advent_of_code::vector::{Vector2d, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP};
use std::collections::HashSet;

advent_of_code::solution!(6);

const GUARD_DIRECTIONS: [Vector2d; 4] = [DIR_UP, DIR_RIGHT, DIR_DOWN, DIR_LEFT];

pub fn part_one(input: &str) -> Option<u64> {
    let (map_bounds, obstacles, guard_start_position) = parse_input(input)?;
    calc_guard_path(&map_bounds, &obstacles, &guard_start_position).map(|path| path.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (map_bounds, obstacles, guard_start_position) = parse_input(input)?;

    let mut locations_to_check = calc_guard_path(&map_bounds, &obstacles, &guard_start_position)?;
    locations_to_check.remove(&guard_start_position);

    let mut no_of_obstacles_leading_to_circle: u64 = 0;
    for location_to_check in locations_to_check {
        let mut adapted_obstacles = obstacles.clone(); // TODO: avoid cloning
        adapted_obstacles.insert(location_to_check);
        if calc_guard_path(&map_bounds, &adapted_obstacles, &guard_start_position).is_none() {
            no_of_obstacles_leading_to_circle += 1;
        }
    }
    Some(no_of_obstacles_leading_to_circle)
}

fn calc_guard_path(
    map_bounds: &Rectangle,
    obstacles: &HashSet<Vector2d>,
    guard_start_position: &Vector2d,
) -> Option<HashSet<Vector2d>> {
    let mut guard_position = guard_start_position.clone();
    let mut guard_direction_index = 0;

    // TODO: avoid calls to ".clone()"
    let mut path: HashSet<GuardState> = HashSet::new();
    while map_bounds.contains(&guard_position) {
        let current_direction = &(GUARD_DIRECTIONS[guard_direction_index]);
        let old_path_length = path.len();
        path.insert(GuardState {
            position: guard_position.clone(),
            direction: current_direction.clone(),
        });
        if old_path_length == path.len() {
            return None
        }

        let next_pos = &guard_position + current_direction;
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

    Some(path.iter().map(|state| state.position.clone()).collect())
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct GuardState {
    position: Vector2d,
    direction: Vector2d,
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

    // TODO: avoid second iteration
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
        assert_eq!(result, Some(6));
    }
}
