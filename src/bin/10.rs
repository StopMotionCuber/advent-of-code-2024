use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);



pub fn dfs_results(map: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    positions.insert((x, y));

    for i in 1..10 {
        let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
        for position in &positions {
            // Check all 4 directions
            if position.0 > 0 && map[position.0 - 1][position.1] == i {
                new_positions.insert((position.0 - 1, position.1));
            }
            if position.0 < map.len() - 1 && map[position.0 + 1][position.1] == i {
                new_positions.insert((position.0 + 1, position.1));
            }
            if position.1 > 0 && map[position.0][position.1 - 1] == i {
                new_positions.insert((position.0, position.1 - 1));
            }
            if position.1 < map[0].len() - 1 && map[position.0][position.1 + 1] == i {
                new_positions.insert((position.0, position.1 + 1));
            }
        }
        positions = new_positions;
    }

    positions.len() as u32
}


pub fn dfs_results_extended(map: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut positions: HashMap<(usize, usize), usize> = HashMap::new();
    positions.insert((x, y), 1);

    for i in 1..10 {
        let mut new_positions: HashMap<(usize, usize), usize> = HashMap::new();
        for position in positions.keys() {
            // Check all 4 directions
            if position.0 > 0 && map[position.0 - 1][position.1] == i {

                *new_positions.entry((position.0 - 1, position.1)).or_insert(0) += positions[position];
            }
            if position.0 < map.len() - 1 && map[position.0 + 1][position.1] == i {
                *new_positions.entry((position.0 + 1, position.1)).or_insert(0) += positions[position];
            }
            if position.1 > 0 && map[position.0][position.1 - 1] == i {
                *new_positions.entry((position.0, position.1 - 1)).or_insert(0) += positions[position];
            }
            if position.1 < map[0].len() - 1 && map[position.0][position.1 + 1] == i {
                *new_positions.entry((position.0, position.1 + 1)).or_insert(0) += positions[position];
            }
        }
        positions = new_positions;
    }
    positions.values().sum::<usize>() as u32
}


pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<u32>> = input.lines()
        .map(|line| line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect())
        .collect();

    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                let current_result = dfs_results(&map, i, j);
                result += current_result;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<u32>> = input.lines()
        .map(|line| line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect())
        .collect();

    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                let current_result = dfs_results_extended(&map, i, j);
                result += current_result;
            }
        }
    }

    Some(result)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
