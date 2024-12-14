use std::collections::HashMap;

advent_of_code::solution!(12);

const SAMPLE2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";


const SAMPLE3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";


const SAMPLE4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

const SAMPLE5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

pub fn get_field_size_bounding(field: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> (u32, u32) {
    visited[x][y] = true;
    let mut size = 1;
    let mut bounding = 0;
    // Check all 4 directions
    if x > 0 && field[x - 1][y] == field[x][y] {
        if !visited[x - 1][y] {
            let (inner_size, inner_bounding) = get_field_size_bounding(field, visited, x - 1, y);
            size += inner_size;
            bounding += inner_bounding;
        }
    } else {
        bounding += 1;
    }
    if x < field.len() - 1  && field[x + 1][y] == field[x][y] {
        if !visited[x + 1][y] {
            let (inner_size, inner_bounding) = get_field_size_bounding(field, visited, x + 1, y);
            size += inner_size;
            bounding += inner_bounding;
        }
    } else {
        bounding += 1;
    }
    if y > 0 &&  field[x][y - 1] == field[x][y] {
        if !visited[x][y - 1] {
            let (inner_size, inner_bounding) = get_field_size_bounding(field, visited, x, y - 1);
            size += inner_size;
            bounding += inner_bounding;
        }
    } else {
        bounding += 1;
    }
    if y < field[0].len() - 1 && field[x][y + 1] == field[x][y] {
        if !visited[x][y + 1] {
            let (inner_size, inner_bounding) = get_field_size_bounding(field, visited, x, y + 1);
            size += inner_size;
            bounding += inner_bounding;
        }
    } else {
        bounding += 1;
    }
    (size, bounding)
}

pub fn get_field_size_bounding_advanced(field: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> (u32, HashMap<usize, Vec<usize>>, HashMap<usize,Vec<usize>>) {
    visited[x][y] = true;
    let mut size = 1;
    let mut x_bounding = HashMap::new();
    let mut y_bounding = HashMap::new();

    // Check all 4 directions
    if x > 0 && field[x - 1][y] == field[x][y] {
        if !visited[x - 1][y] {
            let (inner_size, inner_x_bounding, inner_y_bounding) = get_field_size_bounding_advanced(field, visited, x - 1, y);
            size += inner_size;
            for (k, v) in inner_x_bounding {
                x_bounding.entry(k).or_insert(Vec::new()).extend(v);
            }
            for (k, v) in inner_y_bounding {
                y_bounding.entry(k).or_insert(Vec::new()).extend(v);
            }
        }
    } else {
        x_bounding.entry(x).or_insert(Vec::new()).push(y);
    }
    if x < field.len() - 1  && field[x + 1][y] == field[x][y] {
        if !visited[x + 1][y] {
            let (inner_size, inner_x_bounding, inner_y_bounding) = get_field_size_bounding_advanced(field, visited, x + 1, y);
            size += inner_size;
            for (k, v) in inner_x_bounding {
                x_bounding.entry(k).or_insert(Vec::new()).extend(v);
            }
            for (k, v) in inner_y_bounding {
                y_bounding.entry(k).or_insert(Vec::new()).extend(v);
            }
        }
    } else {
        x_bounding.entry(x + 1).or_insert(Vec::new()).push(y);
    }
    if y > 0 &&  field[x][y - 1] == field[x][y] {
        if !visited[x][y - 1] {
            let (inner_size, inner_x_bounding, inner_y_bounding) = get_field_size_bounding_advanced(field, visited, x, y - 1);
            size += inner_size;
            for (k, v) in inner_x_bounding {
                x_bounding.entry(k).or_insert(Vec::new()).extend(v);
            }
            for (k, v) in inner_y_bounding {
                y_bounding.entry(k).or_insert(Vec::new()).extend(v);
            }
        }
    } else {
        y_bounding.entry(y).or_insert(Vec::new()).push(x);
    }
    if y < field[0].len() - 1 && field[x][y + 1] == field[x][y] {
        if !visited[x][y + 1] {
            let (inner_size, inner_x_bounding, inner_y_bounding) = get_field_size_bounding_advanced(field, visited, x, y + 1);
            size += inner_size;
            for (k, v) in inner_x_bounding {
                x_bounding.entry(k).or_insert(Vec::new()).extend(v);
            }
            for (k, v) in inner_y_bounding {
                y_bounding.entry(k).or_insert(Vec::new()).extend(v);
            }
        }
    } else {
        y_bounding.entry(y + 1).or_insert(Vec::new()).push(x);
    }
    (size, x_bounding, y_bounding)
}


pub fn part_one(input: &str) -> Option<u32> {
    // Read 2 dimensional array of chars
    let field = input.lines()
        .map(|line| {
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let mut visited = vec![vec![false; field[0].len()]; field.len()];
    let mut result = 0;
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if visited[i][j] {
                continue;
            }
            let (size, bounding) = get_field_size_bounding(&field, &mut visited, i, j);
            println!("Size: {}, Bounding: {} on char {}", size, bounding, field[i][j]);
            result += size * bounding;
            visited[i][j] = true;
        }
    }
    Some(result)
}


pub fn part_two(input: &str) -> Option<u32> {
    // Read 2 dimensional array of chars
    let field = input.lines()
        .map(|line| {
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let mut visited = vec![vec![false; field[0].len()]; field.len()];
    let mut result = 0;
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if visited[i][j] {
                continue;
            }
            let mut bounding = 0;
            let (size, x_bounding, y_bounding) = get_field_size_bounding_advanced(&field, &mut visited, i, j);
            // Calculate bounding
            for (x, vals) in x_bounding {
                let mut new_vals = vals.clone();
                new_vals.sort();
                let mut last = new_vals[0];
                bounding += 1;
                for val in new_vals.iter().skip(1) {
                    if *val - last > 1 || (x > 0 && x < field.len() && field[x - 1][last] != field[x - 1][*val] &&
                        (field[x- 1][last] == field[i][j] || field[x - 1][*val] == field[i][j])) {
                        bounding += 1;
                    }
                    last = *val;
                }
            }
            for (y, vals) in y_bounding {
                let mut new_vals = vals.clone();
                new_vals.sort();
                let mut last = new_vals[0];
                bounding += 1;
                for val in new_vals.iter().skip(1) {
                    if *val - last > 1 || (y > 0 && y < field[0].len() && field[last][y - 1] != field[last + 1][y - 1]) &&
                        (field[last][y - 1] == field[i][j] || field[last + 1][y-1] == field[i][j]) {
                        bounding += 1;
                    }
                    last = *val;
                }
            }

            println!("Size: {}, Bounding: {} on char {}", size, bounding, field[i][j]);
            result += size * bounding;
            visited[i][j] = true;
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
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_sample2() {
        let result = part_one(SAMPLE2);
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_sample3() {
        let result = part_one(SAMPLE3);
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_sample4() {
        let result = part_two(SAMPLE4);
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_sample5() {
        let result = part_two(SAMPLE5);
        assert_eq!(result, Some(368));
    }

    #[test]
    fn test_part_two_sample3() {
        let result = part_two(SAMPLE3);
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_full() {
        let result = part_two(FULL_INPUT);
        assert_eq!(result, Some(1206));
    }

}
