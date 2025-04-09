use advent_of_code::vector::{Vector2d, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP};
use std::collections::HashMap;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, |region| region.area() * region.perimeter()))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, |region| region.area() * region.sides()))
}

fn solve(input: &str, calc_price: fn(region: &Region) -> u64) -> u64 {
    let all_plots = parse(input);
    let mut remaining_plots = all_plots.clone();

    let mut result = 0;
    while !remaining_plots.is_empty() {
        let plot = remaining_plots.first().unwrap();
        let region = find_region_containing(plot, &all_plots);
        remaining_plots.retain(|plot| !region.plot_locations.contains(&plot.location));
        result += calc_price(&region);
    }
    result
}

fn find_region_containing(plot: &Plot, plots: &Vec<Plot>) -> Region {
    let mut plots_with_same_letter: Vec<&Plot> =
        plots.iter().filter(|p| p.letter == plot.letter).collect();
    plots_with_same_letter.retain(|p| p.location != plot.location);

    let mut plots_in_region = vec![plot];
    loop {
        let neighbouring_plots = plots_with_same_letter
            .iter()
            .filter(|potential_neighbour| {
                plots_in_region
                    .iter()
                    .any(|plot_in_region| check_are_neighbours(plot_in_region, potential_neighbour))
            })
            .map(|plot| *plot)
            .collect::<Vec<&Plot>>();

        if neighbouring_plots.is_empty() {
            break;
        };

        plots_with_same_letter.retain(|plot| !neighbouring_plots.contains(&plot));
        neighbouring_plots
            .iter()
            .for_each(|plot| plots_in_region.push(plot));
    }

    Region::new(plots_in_region)
}

fn check_are_neighbours(plot1: &Plot, plot2: &Plot) -> bool {
    let delta = &plot1.location - &plot2.location;
    delta.x.abs() + delta.y.abs() == 1
}

fn get_neighbour_locations(plot_loc: &Vector2d) -> Vec<Vector2d> {
    vec![
        plot_loc + &DIR_UP,
        plot_loc + &DIR_RIGHT,
        plot_loc + &DIR_DOWN,
        plot_loc + &DIR_LEFT,
    ]
}

enum Side {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn get_sides(plot_loc: &Vector2d) -> Vec<(Vector2d, Vector2d)> {
    vec![
        (plot_loc.clone(), DIR_UP),
        (plot_loc.clone(), DIR_RIGHT),
        (plot_loc.clone(), DIR_DOWN),
        (plot_loc.clone(), DIR_LEFT),
    ]
}

fn parse(input: &str) -> Vec<Plot> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, letter)| Plot {
                letter,
                location: Vector2d {
                    x: x as i32,
                    y: y as i32,
                },
            })
        })
        .collect()
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Plot {
    letter: char,
    location: Vector2d,
}

#[derive(Debug)]
struct Region {
    letter: char,
    plot_locations: Vec<Vector2d>,
}

impl Region {
    fn new(plots: Vec<&Plot>) -> Region {
        Region {
            letter: plots.first().unwrap().letter,
            plot_locations: plots.iter().map(|plot| (*plot).location.clone()).collect(),
        }
    }

    fn area(&self) -> u64 {
        self.plot_locations.len() as u64
    }

    fn perimeter(&self) -> u64 {
        self.plot_locations
            .iter()
            .flat_map(|plot| get_neighbour_locations(plot))
            .filter(|location| !self.plot_locations.iter().any(|l| l == location))
            .count() as u64
    }

    fn sides(&self) -> u64 {
        let mut sides: HashMap<Vector2d, Vec<Vector2d>> = HashMap::new();
        self.plot_locations
            .iter()
            .flat_map(|plot| get_sides(plot))
            .filter(|(plot, side)| !self.plot_locations.iter().any(|l| l == &(plot + side)))
            .for_each(|(plot, side)| sides.entry(side).or_insert(vec![]).insert(0, plot));

        get_number_horizontal_of_sides(sides.get(&DIR_UP).unwrap_or(&vec![]))
            + get_number_horizontal_of_sides(sides.get(&DIR_DOWN).unwrap_or(&vec![]))
            + get_number_vertical_of_sides(sides.get(&DIR_LEFT).unwrap_or(&vec![]))
            + get_number_vertical_of_sides(sides.get(&DIR_RIGHT).unwrap_or(&vec![]))
    }
}


fn get_number_horizontal_of_sides(plots: &Vec<Vector2d>) -> u64 {
    let mut rows: HashMap<i32, Vec<i32>> = HashMap::new();
    plots
        .iter()
        .for_each(|plot| rows.entry(plot.y).or_insert(vec![]).insert(0, plot.x));

    let mut sides = 0;
    for row in rows.values_mut() {
        row.sort();
        sides += row.windows(2)
            .filter(|&sides| sides[1] - sides[0] > 1)
            .count() as u64
            + 1
    }
    sides
}

fn get_number_vertical_of_sides(plots: &Vec<Vector2d>) -> u64 {
    let mut columns: HashMap<i32, Vec<i32>> = HashMap::new();
    plots
        .iter()
        .for_each(|plot| columns.entry(plot.x).or_insert(vec![]).insert(0, plot.y));

    let mut sides = 0;
    for column in columns.values_mut() {
        column.sort();
        sides += column.windows(2)
            .filter(|&sides| sides[1] - sides[0] > 1)
            .count() as u64
            + 1
    }
    sides
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
