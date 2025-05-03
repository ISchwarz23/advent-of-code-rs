use advent_of_code::rect::Rectangle;
use advent_of_code::vector::{Vector2d, DIRS_MAIN, DIR_RIGHT};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u64> {
    find_cheapest_path(&parse_maze(input))
        .first()
        .map(|path| path.score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = find_cheapest_path(&parse_maze(input))
        .into_iter()
        .flat_map(|path| path.visited_positions)
        .collect::<HashSet<Vector2d>>()
        .len();
    Some(result as u64)
}

fn find_cheapest_path(maze: &Maze) -> Vec<Path> {
    #[derive(Eq, PartialEq, Debug)]
    struct Move {
        position: Vector2d,
        orientation: Vector2d,
        cost: u64,
    }
    #[derive(Eq, PartialEq, Hash, Debug)]
    struct Kinematics {
        position: Vector2d,
        orientation: Vector2d,
    }

    fn turn_left(current_orient: &Vector2d) -> Vector2d {
        turn(current_orient, 1)
    }
    fn turn_right(current_orient: &Vector2d) -> Vector2d {
        turn(current_orient, -1)
    }
    fn turn(current_orient: &Vector2d, turn_direction: i8) -> Vector2d {
        let direction_index = DIRS_MAIN
            .iter()
            .position(|dir| dir == current_orient)
            .unwrap();
        let mut direction_index_new = (direction_index as isize) + (turn_direction as isize);
        if direction_index_new < 0 {
            direction_index_new = DIRS_MAIN.len() as isize - 1;
        } else if direction_index_new >= DIRS_MAIN.len() as isize {
            direction_index_new = 0;
        }
        DIRS_MAIN[direction_index_new as usize].clone()
    }

    fn get_possible_moves(
        maze: &Maze,
        current_pos: &Vector2d,
        current_orient: &Vector2d,
    ) -> Vec<Move> {
        let left_orient = turn_left(current_orient);
        let right_orient = turn_right(current_orient);

        vec![
            Move {
                position: current_pos + current_orient,
                orientation: current_orient.clone(),
                cost: 1,
            },
            Move {
                position: current_pos + &left_orient,
                orientation: left_orient,
                cost: 1000 + 1,
            },
            Move {
                position: current_pos + &right_orient,
                orientation: right_orient,
                cost: 1000 + 1,
            },
        ]
        .into_iter()
        .filter(|a_move| maze.can_go_to(&a_move.position))
        .collect()
    }

    // create heap that pops the state with the smallest score
    let mut paths: BinaryHeap<Reverse<Path>> = BinaryHeap::new();
    paths.push(Reverse(Path {
        last_orientation: DIR_RIGHT,
        visited_positions: vec![maze.start_pos.clone()],
        score: 0,
    }));

    // to A*
    let mut cheapest_cost_by_location: HashMap<Kinematics, u64> = HashMap::new();
    let mut cheapest_paths: Vec<Path> = Vec::new();
    while let Some(Reverse(current_path)) = paths.pop() {
        // abort, if no more path as cheap as cheapest paths so far
        if !cheapest_paths.is_empty() && current_path.score > cheapest_paths[0].score {
            break;
        }

        // add to the cheapest paths if finished (cheapness checked before)
        if current_path.visited_positions.last().unwrap() == &maze.end_pos {
            cheapest_paths.push(current_path);
            continue;
        }

        // add to cache
        cheapest_cost_by_location.insert(
            Kinematics {
                position: current_path.visited_positions.last().unwrap().clone(),
                orientation: current_path.last_orientation.clone(),
            },
            current_path.score,
        );

        // continue moving on path
        get_possible_moves(
            maze,
            current_path.visited_positions.last().unwrap(),
            &current_path.last_orientation,
        )
        .into_iter()
        .map(|possible_move| Path {
            last_orientation: possible_move.orientation,
            visited_positions: [
                current_path.visited_positions.clone(),
                vec![possible_move.position],
            ]
            .concat(),
            score: current_path.score + possible_move.cost,
        })
        .filter(|path| {
            let current_kinematics = Kinematics {
                position: path.visited_positions.last().unwrap().clone(),
                orientation: path.last_orientation.clone(),
            };
            if let Some(cheapest_cost) =
                cheapest_cost_by_location.get(&current_kinematics)
            {
                &path.score <= cheapest_cost
            } else {
                true
            }
        })
        .for_each(|path| paths.push(Reverse(path)));
    }

    cheapest_paths
}

fn parse_maze(input: &str) -> Maze {
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let map = lines
        .iter()
        .map(|line| line.chars().map(|c| c != '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let bounds = Rectangle {
        x_from: 0,
        x_to: (lines.first().unwrap().len() - 1) as i64,
        y_from: 0,
        y_to: (lines.len() - 1) as i64,
    };

    Maze {
        start_pos: find_pos(input, 'S').unwrap(),
        end_pos: find_pos(input, 'E').unwrap(),
        map,
        bounds,
    }
}

fn find_pos(input: &str, char: char) -> Option<Vector2d> {
    input.lines().enumerate().find_map(|(y, line)| {
        line.chars().enumerate().find_map(|(x, c)| {
            if c == char {
                Some(Vector2d {
                    x: x as i64,
                    y: y as i64,
                })
            } else {
                None
            }
        })
    })
}

#[allow(dead_code)]
fn print_maze(maze: &Maze, path: &Path) {
    for y in maze.bounds.y_range() {
        for x in maze.bounds.x_range() {
            let pos = Vector2d { x, y };
            if maze.start_pos == pos {
                print!("S");
            } else if maze.end_pos == pos {
                print!("E");
            } else if path.visited_positions.contains(&pos) {
                print!("O");
            } else if maze.can_go_to(&pos) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!();
}

struct Maze {
    start_pos: Vector2d,
    end_pos: Vector2d,
    map: Vec<Vec<bool>>,
    bounds: Rectangle,
}

#[derive(Eq, PartialEq, Debug)]
struct Path {
    last_orientation: Vector2d,
    visited_positions: Vec<Vector2d>,
    score: u64,
}
impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Maze {
    fn can_go_to(&self, pos: &Vector2d) -> bool {
        if self.bounds.contains(pos) {
            self.map[pos.y as usize][pos.x as usize]
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
