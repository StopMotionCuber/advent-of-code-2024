use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for capture in re.captures_iter(input) {
        result += capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Split at every do instruction
    let valid_ranges = Regex::new(r"do\(\)").unwrap();
    let until_invalid = Regex::new(r"don't\(\)").unwrap();
    let mut result = 0u32;
    for range in valid_ranges.split(input) {
        let valid = until_invalid.splitn(range, 2).next().unwrap();
        println!("{valid}");
        result += part_one(valid).unwrap();
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
