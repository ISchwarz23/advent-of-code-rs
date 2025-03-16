use advent_of_code::vector::{Vector2d, DIRS_ALL, DIRS_DIAGONALS};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = to_grid(input);

    let result = find_locations_of_char(&grid, &'X')
        .into_iter()
        .flat_map(|x_location| read_words_from_grid(&grid, &x_location, 4, &DIRS_ALL))
        .filter(|word| word == "XMAS")
        .count();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = to_grid(input);

    let result = find_locations_of_char(&grid, &'A')
        .into_iter()
        .map(|x_location| read_diagonal_words_from_grid(&grid, &x_location, 3))
        .filter(|diagonal_words| diagonal_words.iter().filter(|word| *word == "MAS").count() == 2)
        .count();
    Some(result as u64)
}

fn to_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_locations_of_char(grid: &Vec<Vec<char>>, char_to_find: &char) -> Vec<Vector2d> {
    // TODO: check what the "move" keyword is doing?
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, c)| {
                if c == char_to_find {
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

fn read_words_from_grid(
    grid: &Vec<Vec<char>>,
    start_location: &Vector2d,
    word_length: u8,
    directions: &[Vector2d],
) -> Vec<String> {
    directions
        .iter()
        .filter_map(|direction| read_word_from_grid(grid, start_location, word_length, direction))
        .collect()
}

fn read_diagonal_words_from_grid(
    grid: &Vec<Vec<char>>,
    start_location: &Vector2d,
    word_length: u8,
) -> Vec<String> {
    DIRS_DIAGONALS
        .iter()
        .filter_map(|direction| {
            read_word_from_grid(grid, &(start_location - &direction), word_length, direction)
        })
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
        assert_eq!(result, Some(9));
    }
}
