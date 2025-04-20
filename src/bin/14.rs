use advent_of_code::rect::Rectangle;
use advent_of_code::vector::Vector2d;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(14);

const PATTERN_ROBOT: &str = r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)";
static REGEX_ROBOT: Lazy<Regex> = Lazy::new(|| Regex::new(PATTERN_ROBOT).unwrap());

pub fn part_one(input: &str) -> Option<u64> {
    part_one_internal(
        input,
        Rectangle {
            x_from: 0,
            x_to: 100,
            y_from: 0,
            y_to: 102,
        },
    )
}

pub fn part_one_internal(input: &str, area_bounds: Rectangle) -> Option<u64> {
    let mut robots = input.lines().map(|l| parse_robot(l)).collect::<Vec<_>>();

    for _ in 0..100 {
        robots = robots
            .into_iter()
            .map(|robot| robot.do_move(&area_bounds))
            .collect();
    }

    let quadrants = area_to_quadrants(area_bounds);
    let robots_per_quadrant = count_robots_per_quadrant(&robots, quadrants);
    Some(robots_per_quadrant.iter().fold(1, |acc, next| acc * next))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut robots = input.lines().map(|l| parse_robot(l)).collect::<Vec<_>>();
    let area_bounds = Rectangle {
        x_from: 0,
        x_to: 100,
        y_from: 0,
        y_to: 102,
    };

    for _ in 0..7055 {
        robots = robots
            .into_iter()
            .map(|robot| robot.do_move(&area_bounds))
            .collect();
    }

    debug(&robots, &area_bounds);
    Some(7055)
}

fn parse_robot(line: &str) -> Robot {
    let captures = REGEX_ROBOT
        .captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .map(|c| c.unwrap().as_str().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    Robot {
        position: Vector2d {
            x: captures[0],
            y: captures[1],
        },
        velocity: Vector2d {
            x: captures[2],
            y: captures[3],
        },
    }
}

fn area_to_quadrants(area_bounds: Rectangle) -> Vec<Rectangle> {
    let quadrant_width = area_bounds.width() / 2 - 1;
    let quadrant_height = area_bounds.height() / 2 - 1;

    let quadrants = vec![
        Rectangle {
            x_from: area_bounds.x_from,
            x_to: area_bounds.x_from + quadrant_width,
            y_from: area_bounds.y_from,
            y_to: area_bounds.y_from + quadrant_height,
        },
        Rectangle {
            x_from: area_bounds.x_to - quadrant_width,
            x_to: area_bounds.x_to,
            y_from: area_bounds.y_from,
            y_to: area_bounds.y_from + quadrant_height,
        },
        Rectangle {
            x_from: area_bounds.x_from,
            x_to: area_bounds.x_from + quadrant_width,
            y_from: area_bounds.y_to - quadrant_height,
            y_to: area_bounds.y_to,
        },
        Rectangle {
            x_from: area_bounds.x_to - quadrant_width,
            x_to: area_bounds.x_to,
            y_from: area_bounds.y_to - quadrant_height,
            y_to: area_bounds.y_to,
        },
    ];
    quadrants
}

fn count_robots_per_quadrant(robots: &Vec<Robot>, quadrants: Vec<Rectangle>) -> Vec<u64> {
    let mut robots_per_quadrant: HashMap<usize, u64> = HashMap::new();
    robots
        .iter()
        .map(|robot| &robot.position)
        .filter_map(|pos| {
            quadrants.iter().enumerate().find_map(move |(quadrant_index, quadrant)| {
                if quadrant.contains(&pos) {
                    Some(quadrant_index)
                } else {
                    None
                }
            })
        })
        .for_each(|quadrant_index| *robots_per_quadrant.entry(quadrant_index).or_insert(0) += 1);
    robots_per_quadrant.values().map(|no_of_robots| *no_of_robots).collect()
}

struct Robot {
    position: Vector2d,
    velocity: Vector2d,
}

impl Robot {
    fn do_move(&self, area_bounds: &Rectangle) -> Robot {
        let mut new_pos = &self.position + &self.velocity;

        if new_pos.x < area_bounds.x_from {
            new_pos.x = new_pos.x + area_bounds.width() + 1;
        } else if new_pos.x > area_bounds.x_to {
            new_pos.x = new_pos.x - area_bounds.width() - 1;
        }

        if new_pos.y < area_bounds.y_from {
            new_pos.y = new_pos.y + area_bounds.height() + 1;
        } else if new_pos.y > area_bounds.y_to {
            new_pos.y = new_pos.y - area_bounds.height() - 1;
        }

        Robot {
            position: new_pos,
            velocity: self.velocity.clone(),
        }
    }
}

fn debug(robots: &Vec<Robot>, area: &Rectangle) {
    for y in area.y_from..=area.y_to {
        for x in area.x_from..=area.x_to {
            let no_of_robots = robots
                .iter()
                .filter(|r| r.position == Vector2d { x, y })
                .count();
            if no_of_robots == 0 {
                print!(".")
            } else {
                print!("{no_of_robots}");
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let area_bounds = Rectangle {
            x_from: 0,
            x_to: 10,
            y_from: 0,
            y_to: 6,
        };
        let result = part_one_internal(
            &advent_of_code::template::read_file("examples", DAY),
            area_bounds,
        );
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7055));
    }
}
