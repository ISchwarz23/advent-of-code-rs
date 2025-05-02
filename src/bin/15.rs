use advent_of_code::rect::Rectangle;
use advent_of_code::vector::{Vector2d, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP};

advent_of_code::solution!(15);

#[derive(PartialEq, Clone)]
struct Object {
    bounds: Rectangle,
    movable: bool,
}

struct Warehouse {
    bounds: Rectangle,
    objects: Vec<Object>,
    robot_pos: Vector2d,
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_internal(input, false)
}

pub fn part_one_internal(input: &str, should_print_warehouse: bool) -> Option<u64> {
    Some(solve(input, should_print_warehouse, 1, 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    part_two_internal(input, false)
}

pub fn part_two_internal(input: &str, should_print_warehouse: bool) -> Option<u64> {
    Some(solve(input, should_print_warehouse, 2, 1))
}

fn solve(input: &str, should_print_warehouse: bool, x_factor: u8, y_factor: u8) -> u64 {
    let mut warehouse = parse_warehouse(input, x_factor, y_factor);
    if should_print_warehouse {
        print_warehouse(&warehouse);
    }

    let movements = parse_instructions(input);
    for movement in movements {
        warehouse = move_robot_if_possible(warehouse, &movement);
    }

    if should_print_warehouse {
        print_warehouse(&warehouse);
    }
    calc_box_coords_sum(&warehouse)
}

fn calc_box_coords_sum(warehouse: &Warehouse) -> u64 {
    warehouse.objects
        .iter()
        .filter(|obj| obj.movable) // only boxes
        .map(|obj| (obj.bounds.y_from * 100 + obj.bounds.x_from) as u64)
        .sum()
}

fn move_robot_if_possible(warehouse: Warehouse, direction_to_move: &Vector2d) -> Warehouse {
    let robot_pos = &warehouse.robot_pos + direction_to_move;
    let mut bounds_to_check = vec![Rectangle {
        x_from: robot_pos.x,
        x_to: robot_pos.x,
        y_from: robot_pos.y,
        y_to: robot_pos.y,
    }];

    let mut objects_to_push: Vec<&Object> = vec![];
    loop {
        // find objects that are pushed by robot
        let new_objects_to_push: Vec<&Object> = warehouse
            .objects
            .iter()
            .filter(|obj| {
                bounds_to_check
                    .iter()
                    .any(|bounds| bounds.overlaps(&obj.bounds))
            })
            .filter(|obj| !objects_to_push.contains(obj))
            .collect();

        // abort if pushing is not possible
        if new_objects_to_push.iter().any(|obj| !obj.movable) {
            return warehouse;
        }

        // finish if nothing more to be pushed
        if new_objects_to_push.is_empty() {
            break;
        }

        // update bounds to check with bounds of objects at new position
        bounds_to_check = new_objects_to_push
            .iter()
            .map(|obj| obj.bounds.move_dir(direction_to_move))
            .collect();

        // add new object to push to overall objects to push
        new_objects_to_push
            .iter()
            .for_each(|obj| objects_to_push.push(obj));
    }

    let mut objects = warehouse.objects.clone();

    // remove "objects to push" from objects
    objects.retain(|obj| !objects_to_push.contains(&obj));

    // add objects at new position to objects
    objects_to_push
        .iter()
        .map(|obj| Object {
            bounds: obj.bounds.move_dir(direction_to_move),
            movable: obj.movable,
        })
        .for_each(|obj| objects.push(obj));

    Warehouse {
        robot_pos,
        objects,
        bounds: warehouse.bounds,
    }
}

fn print_warehouse(warehouse: &Warehouse) {
    for y in warehouse.bounds.y_range() {
        for x in warehouse.bounds.x_range() {
            let pos = Vector2d { x, y };

            if warehouse.robot_pos == pos {
                print!("@");
            } else if let Some(object) = warehouse.objects.iter().find(|obj| obj.bounds.contains(&pos)) {
                match object.movable {
                    true => print!("O"),
                    false => print!("#"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn parse_instructions(input: &str) -> Vec<Vector2d> {
    input
        .lines()
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.chars())
        .filter_map(|arrow| arrow_to_direction(&arrow))
        .collect::<Vec<_>>()
}

fn parse_warehouse(input: &str, x_factor: u8, y_factor: u8) -> Warehouse {
    Warehouse {
        bounds: calc_warehouse_bounds(input, x_factor, y_factor),
        objects: parse_objects(input, x_factor, y_factor),
        robot_pos: find_robot_pos(input, x_factor, y_factor),
    }
}

fn calc_warehouse_bounds(input: &str, x_factor: u8, y_factor: u8) -> Rectangle {
    let warehouse_input = input
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    Rectangle {
        x_from: 0,
        x_to: ((warehouse_input[0].len() - 1) * x_factor as usize) as i64,
        y_from: 0,
        y_to: ((warehouse_input.len() - 1) * y_factor as usize) as i64,
    }
}

fn parse_objects(input: &str, x_factor: u8, y_factor: u8) -> Vec<Object> {
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                let x = (x * x_factor as usize) as i64;
                let y = (y * y_factor as usize) as i64;
                let bounds = Rectangle {
                    x_from: x,
                    x_to: x + (x_factor - 1) as i64,
                    y_from: y,
                    y_to: y + (y_factor - 1) as i64,
                };
                match c {
                    '#' => Some(Object {
                        bounds,
                        movable: false,
                    }),
                    'O' => Some(Object {
                        bounds,
                        movable: true,
                    }),
                    _ => None,
                }
            })
        })
        .collect::<Vec<_>>()
}

fn find_robot_pos(input: &str, x_factor: u8, y_factor: u8) -> Vector2d {
    input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars().enumerate().find_map(move |(x, c)| match c {
                '@' => Some(Vector2d {
                    x: (x * x_factor as usize) as i64,
                    y: (y * y_factor as usize) as i64,
                }),
                _ => None,
            })
        })
        .unwrap()
}

fn arrow_to_direction(arrow: &char) -> Option<Vector2d> {
    match arrow {
        '>' => Some(DIR_RIGHT),
        '<' => Some(DIR_LEFT),
        'v' => Some(DIR_DOWN),
        '^' => Some(DIR_UP),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_internal(&advent_of_code::template::read_file("examples", DAY), true);
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_internal(&advent_of_code::template::read_file("examples", DAY), true);
        assert_eq!(result, Some(9021));
    }
}
