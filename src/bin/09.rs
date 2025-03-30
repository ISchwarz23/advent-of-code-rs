advent_of_code::solution!(9);

const EMPTY_BLOCK_ID: i32 = -1;

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk_map = parse_disk_to_blocks(input);
    compact_drive_by_blocks_sfct(&mut disk_map);
    Some(calculate_checksum(&disk_map))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk_map = parse_disk_to_files(input);
    compact_drive_by_files_sfct(&mut disk_map);
    let disk_block_map = disk_map
        .iter()
        .flat_map(|(block_id, amount)| std::iter::repeat(*block_id).take(*amount))
        .collect::<Vec<i32>>();
    Some(calculate_checksum(&disk_block_map))
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

fn parse_disk_to_files(input: &str) -> Vec<(i32, usize)> {
    input
        .chars()
        .enumerate()
        .map(|(index, amount_char)| (index, amount_char.to_digit(10).unwrap() as usize))
        .map(|(index, amount)| {
            let is_empty_space = index % 2 == 1;
            let block_id = if is_empty_space {
                EMPTY_BLOCK_ID
            } else {
                index as i32 / 2
            };
            (block_id, amount)
        })
        .collect::<Vec<(i32, usize)>>()
}

fn compact_drive_by_blocks_sfct(disk_map: &mut Vec<i32>) {
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

fn compact_drive_by_files_sfct(disk_map: &mut Vec<(i32, usize)>) {
    let mut right_index: usize = disk_map.len() - 1;
    while right_index > 0 {
        // find file from right
        while disk_map[right_index].0 == EMPTY_BLOCK_ID {
            right_index -= 1;
        }

        // find matching empty space from left
        let mut left_index: usize = 0;
        while left_index < right_index
            && !(disk_map[left_index].0 == EMPTY_BLOCK_ID
            && disk_map[left_index].1 >= disk_map[right_index].1)
        {
            left_index += 1;
        }

        // move file, if matching empty space has been found
        if disk_map[left_index].0 == EMPTY_BLOCK_ID && disk_map[left_index].1 >= disk_map[right_index].1 {
            let (file_block_id, amount_file_blocks) = disk_map.remove(right_index);
            let (_, amount_empty_blocks) = disk_map.remove(left_index);

            // add remaining free space
            let remaining_free_space = amount_empty_blocks - amount_file_blocks;
            disk_map.insert(left_index, (EMPTY_BLOCK_ID, remaining_free_space));
            // move file
            disk_map.insert(left_index, (file_block_id, amount_file_blocks));
            // replace previous file with empty space (could be skipped)
            disk_map.insert(right_index+1, (EMPTY_BLOCK_ID, amount_file_blocks));
        } else {
            right_index -= 1;
        }
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
        assert_eq!(result, Some(2858));
    }
}
