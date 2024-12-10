use std::collections::HashMap;
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut items = input.lines()
        .map(|l| l.as_bytes().iter().map(|x| *x).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();
    let mut map = HashMap::new();
    for i in 0..items.len() {
        for j in 0..items[i].len() {
            if items[i][j] != '.' as u8 {
                map.entry(items[i][j]).or_insert_with(Vec::new).push((i as i32, j as i32));
            }
        }
    }

    for frequencies in map.values() {
        for freq_comb in frequencies.iter().combinations(2) {
            let item1 = **freq_comb.get(0).unwrap();
            let item2 = **freq_comb.get(1).unwrap();
            let coords_diff = (item2.0 - item1.0, item2.1 - item1.1);
            let new_coords1 = (item2.0 + coords_diff.0, item2.1 + coords_diff.1);
            let new_coords2 = (item1.0 - coords_diff.0, item1.1 - coords_diff.1);
            if new_coords1.0 >= 0 && new_coords1.1 >= 0 && new_coords1.0 < items.len() as i32 && new_coords1.1 < items[0].len() as i32 {
                items[new_coords1.0 as usize][new_coords1.1 as usize] = '#' as u8;
            }
            if new_coords2.0 >= 0 && new_coords2.1 >= 0 && new_coords2.0 < items.len() as i32 && new_coords2.1 < items[0].len() as i32 {
                items[new_coords2.0 as usize][new_coords2.1 as usize] = '#' as u8;
            }
        }
    }

    let mut result = 0;

    for i in 0..items.len() {
        for j in 0..items[i].len() {
            if items[i][j] == '#' as u8 {
                result += 1;
            }
        }
    }


    Some(result)

}

pub fn part_two(input: &str) -> Option<u32> {
    let mut items = input.lines()
        .map(|l| l.as_bytes().iter().map(|x| *x).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();
    let mut map = HashMap::new();
    for i in 0..items.len() {
        for j in 0..items[i].len() {
            if items[i][j] != '.' as u8 {
                map.entry(items[i][j]).or_insert_with(Vec::new).push((i as i32, j as i32));
            }
        }
    }

    for frequencies in map.values() {
        for freq_comb in frequencies.iter().combinations(2) {
            let item1 = **freq_comb.get(0).unwrap();
            let item2 = **freq_comb.get(1).unwrap();
            let coords_diff = (item2.0 - item1.0, item2.1 - item1.1);

            let mut next_coords = item1;
            loop {
                // Go forward
                next_coords = (next_coords.0 + coords_diff.0, next_coords.1 + coords_diff.1);
                if next_coords.0 >= 0 && next_coords.1 >= 0 && next_coords.0 < items.len() as i32 && next_coords.1 < items[0].len() as i32 {
                    items[next_coords.0 as usize][next_coords.1 as usize] = '#' as u8;
                }
                else {
                    break;
                }
            }
            let mut next_coords = item2;
            loop {
                // Go backward
                next_coords = (next_coords.0 - coords_diff.0, next_coords.1 - coords_diff.1);
                if next_coords.0 >= 0 && next_coords.1 >= 0 && next_coords.0 < items.len() as i32 && next_coords.1 < items[0].len() as i32 {
                    items[next_coords.0 as usize][next_coords.1 as usize] = '#' as u8;
                }
                else {
                    break;
                }
            }
        }
    }

    let mut result = 0;

    for i in 0..items.len() {
        for j in 0..items[i].len() {
            if items[i][j] == '#' as u8 {
                result += 1;
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
