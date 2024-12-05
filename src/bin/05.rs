use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn check_order_correct(items: &Vec<u32>, backward_rules: &HashMap<u32, HashSet<u32>>) -> bool {
    for i in 0..items.len() {
        let ruleset = backward_rules.get(&items[i]);
        if !ruleset.is_some() {
            continue;
        }
        for j in i+1..items.len() {
            if ruleset.unwrap().contains(&items[j]) {
                return false;
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    println!("{input}");
    let sections = input.splitn(2, "\n\n").collect::<Vec<&str>>();

    let rules = sections[0];
    let recipes = sections[1];

    let mut forward_rules = HashMap::new();
    let mut backwards_rules = HashMap::new();
    for line in rules.lines() {
        let items = line.split('|').collect::<Vec<&str>>();
        let first = items[0].parse::<u32>().unwrap();
        let second = items[1].parse::<u32>().unwrap();
        if !forward_rules.contains_key(&first) {
            forward_rules.insert(first, HashSet::new());
        }
        forward_rules.get_mut(&first).unwrap().insert(second);
        if !backwards_rules.contains_key(&second) {
            backwards_rules.insert(second, HashSet::new());
        }
        backwards_rules.get_mut(&second).unwrap().insert(first);
    }

    let mut result = 0u32;

    for line in recipes.lines() {
        let items = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let mut order_correct = check_order_correct(&items, &backwards_rules);

        if order_correct {
            let cur_result = items[(items.len() - 1) / 2];
            println!("Order correct for {line}: {order_correct}, adding {cur_result}");
            result += cur_result;
        }
        if !order_correct {
            // We need some sorting now
        }
    }



    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sections = input.splitn(2, "\n\n").collect::<Vec<&str>>();

    let rules = sections[0];
    let recipes = sections[1];

    let mut forward_rules = HashMap::new();
    let mut backwards_rules = HashMap::new();
    for line in rules.lines() {
        let items = line.split('|').collect::<Vec<&str>>();
        let first = items[0].parse::<u32>().unwrap();
        let second = items[1].parse::<u32>().unwrap();
        if !forward_rules.contains_key(&first) {
            forward_rules.insert(first, HashSet::new());
        }
        forward_rules.get_mut(&first).unwrap().insert(second);
        if !backwards_rules.contains_key(&second) {
            backwards_rules.insert(second, HashSet::new());
        }
        backwards_rules.get_mut(&second).unwrap().insert(first);
    }

    let mut result = 0u32;

    for line in recipes.lines() {
        let items = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let mut order_correct = check_order_correct(&items, &backwards_rules);
        if !order_correct {
            // We need some insertion sorting now
            let mut sorted = Vec::new();
            sorted.insert(0, items[0]);
            for i in 1..items.len() {
                let mut idx = i;
                for j in 0..i {
                    let ruleset = backwards_rules.get(&items[j]);
                    if !ruleset.is_some() {
                        continue
                    }
                    if ruleset.unwrap().contains(&items[i]) {
                        idx = j;
                        break;
                    }
                }
                sorted.insert(idx, items[i]);
            }

            order_correct = check_order_correct(&sorted, &backwards_rules);
            while !order_correct {
                println!("Order correct for {line} now: {order_correct}");
                for i in 0..sorted.len() {
                    for j in i+1..sorted.len() {
                        let ruleset = backwards_rules.get(&sorted[i]);
                        if !ruleset.is_some() {
                            break
                        }
                        if ruleset.unwrap().contains(&sorted[j]) {
                            let tmp = sorted[i];
                            sorted[i] = sorted[j];
                            sorted[j] = tmp;
                        }
                    }
                }
                order_correct = check_order_correct(&sorted, &backwards_rules);
            }
            println!("Order correct for {line} now: {order_correct}");
            result += sorted[(sorted.len() - 1) / 2];
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
