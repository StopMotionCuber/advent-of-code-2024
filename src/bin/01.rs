use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        list1.push(iter.next().unwrap().parse::<i32>().unwrap());
        list2.push(iter.next().unwrap().parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();

    let mut res = 0;
    for i in 0..list1.len() {
        res += (list1[i] - list2[i]).abs();
    }

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1 = Vec::new();
    let mut list2 = HashMap::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        list1.push(iter.next().unwrap().parse::<i32>().unwrap());
        let list2num = iter.next().unwrap().parse::<i32>().unwrap();
        if list2.contains_key(&list2num) {
            list2.insert(list2num, list2.get(&list2num).unwrap() + 1);
        }
        else {
            list2.insert(list2num, 1);
        }
    }
    let mut result = 0;
    for num in list1 {
        if list2.contains_key(&num) {
            result += list2.get(&num).unwrap() * num;
        }
    }

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
