use advent_of_code::rect::Rectangle;
use advent_of_code::vector::Vector2d;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let antennas = parse_input(input);
    let area = get_dimensions(input);

    let antinode_locations = antennas
        .values()
        .flat_map(|antenna_positions| calculate_antinodes(antenna_positions))
        .filter(|position| area.contains(&position))
        .collect::<HashSet<Vector2d>>();

    Some(antinode_locations.len() as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> HashMap<char, Vec<Vector2d>> {
    let mut antennas: HashMap<char, Vec<Vector2d>> = HashMap::new();
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '.' {
                    None
                } else {
                    Some((
                        ch,
                        Vector2d {
                            x: x as i32,
                            y: y as i32,
                        },
                    ))
                }
            })
        })
        .for_each(|(antenna, pos)| {
            antennas.entry(antenna).or_insert(Vec::new()).push(pos);
        });
    antennas
}

fn get_dimensions(input: &str) -> Rectangle {
    let y = input.lines().count() - 1;
    let x = input.lines().next().unwrap().chars().count() - 1;
    Rectangle {
        x_from: 0,
        x_to: x as i32,
        y_from: 0,
        y_to: y as i32,
    }
}

fn calculate_antinodes(antenna_locations: &Vec<Vector2d>) -> Vec<Vector2d> {
    let mut antinodes: Vec<Vector2d> = Vec::new();
    for i in 0..(antenna_locations.len() - 1) {
        for j in (i + 1)..antenna_locations.len() {
            let first_antenna = antenna_locations.get(i).unwrap();
            let second_antenna = antenna_locations.get(j).unwrap();
            let delta = second_antenna - first_antenna;
            antinodes.push(first_antenna - &delta);
            antinodes.push(second_antenna + &delta);
        }
    }
    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
