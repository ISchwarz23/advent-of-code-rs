use advent_of_code::vector::{Vector2d, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let all_plots = parse(input);
    let mut remaining_plots = all_plots.clone();

    let mut result = 0;
    while !remaining_plots.is_empty() {
        let plot = remaining_plots.first().unwrap();
        let region = find_region_containing(plot, &all_plots);
        remaining_plots.retain(|plot| !region.plot_locations.contains(&plot.location));
        // println!("region {:?} has area {:?} and perimeter {:?}", region.letter, region.area, region.perimeter);
        result += region.area * region.perimeter;
    }
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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

fn get_neighbour_locations(plot: &Plot) -> Vec<Vector2d> {
    let plot_loc = &plot.location;
    vec![
        plot_loc + &DIR_UP,
        plot_loc + &DIR_RIGHT,
        plot_loc + &DIR_DOWN,
        plot_loc + &DIR_LEFT,
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
    area: u64,
    perimeter: u64,
}

impl Region {
    fn new(plots: Vec<&Plot>) -> Region {
        let perimeter = plots
            .iter()
            .flat_map(|plot| get_neighbour_locations(plot))
            .filter(|location| !plots.iter().any(|plot| plot.location == *location))
            .count() as u64;

        Region {
            letter: plots.first().unwrap().letter,
            plot_locations: plots.iter().map(|plot| (*plot).location.clone()).collect(),
            area: plots.len() as u64,
            perimeter,
        }
    }
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
        assert_eq!(result, None);
    }
}
