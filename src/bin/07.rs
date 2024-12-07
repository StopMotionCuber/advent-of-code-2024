advent_of_code::solution!(7);

pub fn solvable01(current: i64, target: i64, numbers: &Vec<i64>, idx: usize) -> bool {
    if idx == numbers.len() {
        return current == target;
    }
    if current > target {
        return false;
    }

    solvable01(current * numbers[idx], target, numbers, idx + 1) ||
        solvable01(current + numbers[idx], target, numbers, idx + 1)
}


pub fn solvable02(current: i64, target: i64, numbers: &Vec<i64>, idx: usize) -> bool {
    if idx == numbers.len() {
        return current == target;
    }
    if current > target {
        return false;
    }

    solvable02(current * numbers[idx], target, numbers, idx + 1) ||
        solvable02(current + numbers[idx], target, numbers, idx + 1) ||
        solvable02(current * 10i64.pow((numbers[idx] as f64).log10().floor() as u32 + 1)  + numbers[idx], target, numbers, idx + 1)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        let splitted = line.split(":").collect::<Vec<&str>>();
        let target = splitted[0].parse::<i64>().unwrap();
        let numbers = splitted[1].strip_prefix(" ").unwrap().split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        if solvable01(numbers[0], target, &numbers, 1) {
            result += target as u64;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    for line in input.lines() {
        let splitted = line.split(":").collect::<Vec<&str>>();
        let target = splitted[0].parse::<i64>().unwrap();
        let numbers = splitted[1].strip_prefix(" ").unwrap().split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        if solvable02(numbers[0], target, &numbers, 1) {
            result += target as u64;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
