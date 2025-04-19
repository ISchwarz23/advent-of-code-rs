use advent_of_code::vector::Vector2d;
use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(13);

const PATTERN_BUTTON: &str = r"Button .: X\+(\d+), Y\+(\d+)";
static REGEX_BUTTON: Lazy<Regex> = Lazy::new(|| Regex::new(PATTERN_BUTTON).unwrap());

const PATTERN_TARGET: &str = r"Prize: X=(\d+), Y=(\d+)";
static REGEX_TARGET: Lazy<Regex> = Lazy::new(|| Regex::new(PATTERN_TARGET).unwrap());

struct Game {
    button_a_movement: Vector2d,
    button_b_movement: Vector2d,
    price_location: Vector2d,
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = parse_input(input)
        .iter()
        .filter_map(|game| solve(game))
        .sum::<u64>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = parse_input(input)
        .into_iter()
        .map(|game| Game {
            button_a_movement: game.button_a_movement,
            button_b_movement: game.button_b_movement,
            price_location: Vector2d {
                x: game.price_location.x + 10_000_000_000_000,
                y: game.price_location.y + 10_000_000_000_000,
            },
        })
        .filter_map(|game| solve(&game))
        .sum::<u64>();
    Some(result)
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(4)
        .map(|game_lines| parse_game(game_lines))
        .collect()
}

fn parse_game(lines: &[&str]) -> Game {
    Game {
        button_a_movement: parse_button(lines[0]),
        button_b_movement: parse_button(lines[1]),
        price_location: parse_target(lines[2]),
    }
}

fn parse_button(input: &str) -> Vector2d {
    let captures = REGEX_BUTTON.captures(input).unwrap();
    Vector2d {
        x: captures[1].parse().unwrap(),
        y: captures[2].parse().unwrap(),
    }
}

fn parse_target(input: &str) -> Vector2d {
    let captures = REGEX_TARGET.captures(input).unwrap();
    Vector2d {
        x: captures[1].parse().unwrap(),
        y: captures[2].parse().unwrap(),
    }
}

fn solve(game: &Game) -> Option<u64> {
    // solve linear system

    // buttonA.movement.x * A + buttonB.movement.x * B = priceLocation.x
    let a1 = game.button_a_movement.x;
    let b1 = game.button_b_movement.x;
    let c1 = game.price_location.x;
    // buttonA.movement.y * A + buttonB.movement.y * B = priceLocation.y
    let a2 = game.button_a_movement.y;
    let b2 = game.button_b_movement.y;
    let c2 = game.price_location.y;

    // Calculate the determinant of the coefficient matrix
    let determinant = a1 * b2 - a2 * b1;

    // If determinant is zero, the system has no unique solution (either infinite or none)
    if determinant.abs() == 0 {
        return None;
    }

    // Using Cramer's Rule to solve for x and y
    let a = (c1 * b2 - c2 * b1) / determinant;
    let b = (a1 * c2 - a2 * c1) / determinant;

    // double check as integer division is done before
    if &game.button_a_movement * a + &game.button_b_movement * b == game.price_location {
        Some((a * 3 + b) as u64)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
