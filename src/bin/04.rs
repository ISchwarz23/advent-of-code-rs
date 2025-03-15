use std::ops::Add;

advent_of_code::solution!(4);

const DIR_RIGHT: Vector2d = Vector2d { x: 1, y: 0 };
const DIR_DOWN: Vector2d = Vector2d { x: 0, y: 1 };
const DIR_LEFT: Vector2d = Vector2d { x: -1, y: 0 };
const DIR_UP: Vector2d = Vector2d { x: 0, y: -1 };

const DIR_RIGHT_DOWN: Vector2d = Vector2d { x: 1, y: 1 };
const DIR_RIGHT_UP: Vector2d = Vector2d { x: 1, y: -1 };
const DIR_LEFT_DOWN: Vector2d = Vector2d { x: -1, y: 1 };
const DIR_LEFT_UP: Vector2d = Vector2d { x: -1, y: -1 };

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = to_grid(input);

    let all_directions: Vec<Vector2d> = vec![
        DIR_RIGHT,
        DIR_RIGHT_UP,
        DIR_UP,
        DIR_LEFT_UP,
        DIR_LEFT,
        DIR_LEFT_DOWN,
        DIR_DOWN,
        DIR_RIGHT_DOWN,
    ];

    // TODO: check what the "move" keyword is doing?
    let result = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, c)| {
                if c == &'X' {
                    Some(Vector2d {
                        x: x as i32,
                        y: y as i32,
                    })
                } else {
                    None
                }
            })
        })
        .flat_map(|x_location| read_words_from_grid(&grid, &x_location, 4, &all_directions))
        .filter(|word| word == "XMAS")
        .count();
    Some(result as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn to_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn read_words_from_grid(
    grid: &Vec<Vec<char>>,
    start_location: &Vector2d,
    word_length: u8,
    directions: &Vec<Vector2d>,
) -> Vec<String> {
    directions
        .iter()
        .filter_map(|direction| read_word_from_grid(grid, start_location, word_length, direction))
        .collect()
}

fn read_word_from_grid(
    grid: &Vec<Vec<char>>,
    start_location: &Vector2d,
    word_length: u8,
    direction: &Vector2d,
) -> Option<String> {
    let mut current_location = start_location.clone();
    let mut word = "".to_string();
    for _i in 0..word_length {
        let row = grid
            .get(current_location.y as usize)?
            .get(current_location.x as usize)?;
        word.push(*row);
        current_location = &current_location + direction;
    }
    Some(word)
}

// TODO: move to utils

#[derive(Clone, Debug)]
struct Vector2d {
    x: i32,
    y: i32,
}

impl Add for Vector2d {
    type Output = Vector2d;

    fn add(self, other: Self) -> Vector2d {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Vector2d {
    type Output = Vector2d;

    fn add(self, other: Self) -> Self::Output {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
