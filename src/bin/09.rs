advent_of_code::solution!(9);

const EMPTY_BLOCK_ID: i32 = -1;

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map = parse_disk_to_blocks(input);
    compact_drive_by_blocks_SFCT(&disk_map);
    Some(calculate_checksum(&disk_map))
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn parse_disk_to_blocks(input: &str) -> Vec<i32> {
    input
        .chars()
        .enumerate()
        .map(|(index, amount_char)| (index, amount_char.to_digit(10).unwrap() as usize))
        .flat_map(|(index, amount)| {
            let is_empty_space = index % 2 == 1;
            let block_id = if is_empty_space {
                EMPTY_BLOCK_ID
            } else {
                index as i32 / 2
            };
            std::iter::repeat(block_id).take(amount)
        })
        .collect::<Vec<i32>>()
}

fn compact_drive_by_blocks_SFCT(disk_map: &Vec<i32>) {
    let mut left_index: usize = 0;
    let mut right_index: usize = disk_map.len() - 1;
    while left_index < right_index {
        while disk_map[right_index] == EMPTY_BLOCK_ID {
            right_index -= 1;
        }
        if disk_map[left_index] == EMPTY_BLOCK_ID {
            disk_map[left_index] = disk_map[right_index];
            disk_map[right_index] = EMPTY_BLOCK_ID;
        };
        left_index += 1;
    }
}

fn calculate_checksum(disk_map: &Vec<i32>) -> u64 {
    disk_map
        .iter()
        .enumerate()
        .filter_map(|(index, block_id)| {
            if block_id == &EMPTY_BLOCK_ID {
                None
            } else {
                Some((index as u64) * (*block_id as u64))
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
