advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    'outer: for line in input.lines() {
        let mut line_items = Vec::new();
        for item in line.split_whitespace() {
            line_items.push(item.parse::<i32>().unwrap());
        }
        let increasing = line_items[1] > line_items[0];
        for i in 0..line_items.len() - 1 {
            let mut diff = line_items[i + 1] - line_items[i];
            if !increasing {
                diff *= -1;
            }
            if diff < 1 || diff > 3 {
                continue 'outer;
            }
        }
        safe_count += 1;
    }
    Some(safe_count)
}

pub fn level_safe(items: Vec<i32>) -> bool{
    let increasing = items[1] > items[0];
    for i in 0..items.len() - 1 {
        let mut diff = items[i + 1] - items[i];
        if !increasing {
            diff *= -1;
        }
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    for line in input.lines() {
        let mut line_items = Vec::new();
        for item in line.split_whitespace() {
            line_items.push(item.parse::<i32>().unwrap());
        }
        for i in 0..line_items.len() {
            let mut new_items = line_items.clone();
            new_items.remove(i);
            if level_safe(new_items) {
                safe_count += 1;
                break;
            }
        }
    }
    Some(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
