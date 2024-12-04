
advent_of_code::solution!(4);


pub fn search_xmas(field: Vec<Vec<char>>) -> i32 {
    // Horizontal search
    let forward = ['X', 'M', 'A', 'S'];

    let mut matches = 0;
    let mut possible_dirs = [true, true, true, true, true, true, true, true];
    for x in 0..field.len() {
        for y in 0..field[0].len() {
            if field[x][y] != forward[0] {
                continue;
            }
            possible_dirs = [true, true, true, true, true, true, true, true];
            for k in 1..4 {
                if x < k {
                    possible_dirs[0] = false;
                    possible_dirs[7] = false;
                    possible_dirs[6] = false;
                }
                if x + k >= field.len() {
                    possible_dirs[2] = false;
                    possible_dirs[3] = false;
                    possible_dirs[4] = false;
                }
                if y < k {
                    possible_dirs[0] = false;
                    possible_dirs[1] = false;
                    possible_dirs[2] = false;
                }
                if y + k >= field[0].len() {
                    possible_dirs[4] = false;
                    possible_dirs[5] = false;
                    possible_dirs[6] = false;
                }

                // Check all 8 directions
                if !possible_dirs[0] || field[x - k][y - k] != forward[k] {
                    possible_dirs[0] = false;
                }
                if !possible_dirs[1] || field[x][y - k] != forward[k] {
                    possible_dirs[1] = false;
                }
                if !possible_dirs[2] || field[x + k][y - k] != forward[k] {
                    possible_dirs[2] = false;
                }
                if !possible_dirs[3] || field[x + k][y] != forward[k] {
                    possible_dirs[3] = false;
                }
                if !possible_dirs[4] || field[x + k][y + k] != forward[k] {
                    possible_dirs[4] = false;
                }
                if !possible_dirs[5] || field[x][y + k] != forward[k] {
                    possible_dirs[5] = false;
                }
                if !possible_dirs[6] || field[x - k][y + k] != forward[k] {
                    possible_dirs[6] = false;
                }
                if !possible_dirs[7] || field[x - k][y] != forward[k] {
                    possible_dirs[7] = false;
                }
            }
            for found in possible_dirs {
                if found {
                    matches += 1;
                }
            }
        }
    }
    // Vertical search

    matches
}

pub fn search_xmas2(field: Vec<Vec<char>>) -> i32 {
    // Horizontal search

    let mut matches = 0;
    for x in 1..field.len() - 1 {
        for y in 1..field[0].len() - 1 {
            if field[x][y] != 'A' {
                continue;
            }
            if !(field[x - 1][y - 1] == 'M' && field[x + 1][y + 1] == 'S') &&
                !(field[x - 1][y - 1] == 'S' && field[x + 1][y + 1] == 'M') {
                continue;
            }
            if !(field[x + 1][y - 1] == 'M' && field[x - 1][y + 1] == 'S') &&
                !(field[x + 1][y - 1] == 'S' && field[x - 1][y + 1] == 'M') {
                continue;
            }
            matches += 1;
        }
    }
    matches
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cols = 0;
    let mut rows = 0;
    // field should be a 2 dimensional array of chars
    for line in input.lines() {
        cols = line.len();
        rows += 1;
    }

    let mut field = vec![vec!['X'; cols]; rows];
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            field[i][j] = char;
        }
    }

    Some(search_xmas(field) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cols = 0;
    let mut rows = 0;
    // field should be a 2 dimensional array of chars
    for line in input.lines() {
        cols = line.len();
        rows += 1;
    }

    let mut field = vec![vec!['X'; cols]; rows];
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            field[i][j] = char;
        }
    }

    Some(search_xmas2(field) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
