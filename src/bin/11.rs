use std::collections::{HashMap};

advent_of_code::solution!(11);


pub fn get_result(input: &str, iteration_count: usize) -> Option<u64> {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for entry in input.split_whitespace() {
        let stone = entry.parse::<u64>().unwrap();
        // Insert stone with value 0 if none exists
        *stones.entry(stone).or_insert(0) += 1;
    }

    for i in 0..iteration_count {
        println!("Iteration: {}", i);
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        for stone in stones.keys() {
            // Parse stone to string
            if *stone == 0 {
                *new_stones.entry(1).or_insert(0) += stones[stone];
                continue;
            }
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                // Split string in middle

                let (left, right) = stone_str.split_at(stone_str.len() / 2);
                let left_num = left.parse::<u64>().unwrap();
                let right_num = right.parse::<u64>().unwrap();
                *new_stones.entry(left_num).or_insert(0) += stones[stone];
                *new_stones.entry(right_num).or_insert(0) += stones[stone];
            }
            else {
                *new_stones.entry(stone * 2024).or_insert(0) += stones[stone];
            }
        }
        stones = new_stones;
    }
    Some(stones.values().sum())

}


pub fn part_one_optimized(input: &str) -> Option<u64> {
    get_result(input, 25)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones: Vec<u64> = Vec::new();
    for entry in input.split_whitespace() {
        let stone = entry.parse::<u64>().unwrap();
        stones.push(stone);
    }

    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for stone in stones.iter() {
            // Parse stone to string
            if *stone == 0 {
                new_stones.push(1);
                continue;
            }
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                // Split string in middle

                let (left, right) = stone_str.split_at(stone_str.len() / 2);
                new_stones.push(left.parse::<u64>().unwrap());
                new_stones.push(right.parse::<u64>().unwrap());
            }
            else {
                new_stones.push(stone * 2024)
            }
        }
        stones = new_stones;

    }
    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    get_result(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_optimized(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43805764722150));
    }
}
