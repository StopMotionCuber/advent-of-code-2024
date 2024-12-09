advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let map_len: u32 = input.chars().filter_map(|c| c.to_digit(10)).sum();
    let mut array: Vec<i64> = vec![0; map_len as usize];
    let mut idx = 0;
    for (i, c) in input.chars().enumerate() {
        if c.is_digit(10) {
            let value: i32;
            if i % 2 == 0 {
                value = i as i32/2;
            }
            else {
                value = -1;
            }
            for _ in 0..c.to_digit(10).unwrap() {
                array[idx] = value as i64;
                idx += 1;
            }
        }
    }
    let mut left_idx = 0;
    let mut right_idx = array.len() - 1;
    loop {
        while array[left_idx] != -1 {
            left_idx += 1;
        }
        while array[right_idx] == -1 {
            right_idx -= 1;
        }
        if left_idx >= right_idx {
            break;
        }
        array.swap(left_idx, right_idx);
    }

    let mut result = 0;
    // Get sum
    for i in 0..array.len() {
        if array[i] == -1 {
            continue;
        }
        result += array[i] * i as i64;
    }


    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map_len: u32 = input.chars().filter_map(|c| c.to_digit(10)).sum();
    let mut array: Vec<i64> = vec![0; map_len as usize];
    let mut idx = 0;
    for (i, c) in input.chars().enumerate() {
        if c.is_digit(10) {
            let value: i32;
            if i % 2 == 0 {
                value = i as i32/2;
            }
            else {
                value = -1;
            }
            for _ in 0..c.to_digit(10).unwrap() {
                array[idx] = value as i64;
                idx += 1;
            }
        }
    }
    let mut left_idx = 0;
    let mut right_idx = array.len() - 1;
    loop {
        while array[left_idx] != -1 {
            left_idx += 1;
        }
        while array[right_idx] == -1 {
            right_idx -= 1;
        }
        // How many do we need to swap?
        let mut right_idx_start = right_idx;
        while array[right_idx_start] == array[right_idx] {
            right_idx_start -= 1;
        }
        // We need to find a block that fits right_idx_start + 1 until right_idx
        let mut left_idx_end = left_idx;
        let mut remaining = right_idx - right_idx_start - 1;
        println!("{}", remaining);
        while remaining > 0 {
            left_idx_end += 1;
            if array[left_idx_end] == -1 {
                remaining -= 1;
                continue;
            }
            else {
                remaining = right_idx - right_idx_start;
            }
            if left_idx_end >= right_idx {
                break;
            }
        }

        if left_idx >= right_idx {
            break;
        }
        if left_idx_end >= right_idx {
            let val = array[right_idx];
            while array[right_idx] == val {
                right_idx -= 1;
            }
            continue;
        }

        for i in 0..(right_idx - right_idx_start) {
            array.swap(left_idx_end - i, right_idx_start + 1 + i);
        }
    }

    let mut result = 0;
    // Get sum
    for i in 0..array.len() {
        if array[i] == -1 {
            continue;
        }
        result += array[i] * i as i64;
    }


    Some(result as u64)
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
