use std::cmp::PartialEq;

advent_of_code::solution!(6);

#[derive(PartialEq, Eq)]
enum Elem {
    UNVISITED,
    VISITED,
    OBSTACLE,
    CHARACTER,
    UNKNOWN,
}

enum VisitedDir {
    NONE = 0,
    UP = 1,
    DOWN = 2,
    RIGHT = 4,
    LEFT = 8,
}

pub fn getDirFromPositions(up_dir: i32, right_dir: i32) -> VisitedDir{
    if up_dir == -1 {
        return VisitedDir::UP
    }
    else if up_dir == 1 {
        return VisitedDir::DOWN
    }
    else if right_dir == 1 {
        return VisitedDir::RIGHT
    }
    else if right_dir == -1 {
        return VisitedDir::LEFT
    }
    return VisitedDir::NONE

}

pub fn map_to_elem(input: u8) -> Elem {
    if input == '.' as u8 {
        Elem::UNVISITED
    }
    else if input == '#' as u8 {
        Elem::OBSTACLE
    }
    else if input == '^' as u8 {
        Elem::CHARACTER
    } else {
        Elem::UNKNOWN
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut items = input.lines()
        .map(|l| l.as_bytes().iter().map(|x|map_to_elem(*x)).collect::<Vec<Elem>>()).collect::<Vec<Vec<Elem>>>();
    let mut character_pos = (0i32, 0i32);
    let mut up_move = -1;
    let mut right_move = 0;
    for x in 0..items.len() as i32 {
        for y in 0..items[0].len() as i32 {
            if items[x as usize][y as usize] == Elem::CHARACTER {
                character_pos = (x, y);
            }
        }
    }

    // Do movement
    loop {
        let next_pos = (character_pos.0 + up_move, character_pos.1 + right_move);

        if next_pos.0 >= items.len() as i32 || next_pos.0 < 0 || next_pos.1 >= items[0].len() as i32 || next_pos.1 < 0 {
            items[character_pos.0 as usize][character_pos.1 as usize] = Elem::VISITED;
            break;
        }
        // Check next position
        match items[next_pos.0 as usize][next_pos.1 as usize] {
            Elem::UNVISITED | Elem::VISITED => {
                items[character_pos.0 as usize][character_pos.1 as usize] = Elem::VISITED;
                character_pos = next_pos;
            }
            Elem::OBSTACLE => {
                if up_move == -1 {
                    up_move = 0;
                    right_move = 1;
                }
                else if up_move == 1 {
                    up_move = 0;
                    right_move = -1;
                }
                else if right_move == -1 {
                    right_move = 0;
                    up_move = -1;
                }
                else {
                    right_move = 0;
                    up_move = 1;
                }
            }
            Elem::CHARACTER => {}
            _ => {}
        }
    }

    let mut result = 0;
    for i in 0..items.len() as i32 {
        for j in 0..items[0].len() as i32 {
            if items[i as usize][j as usize] == Elem::CHARACTER || items[i as usize][j as usize] == Elem::VISITED {
                result += 1
            }
        }
    }

    Some(result)
}

pub fn creates_loop(maze: &Vec<Vec<Elem>>, starting_pos: (i32, i32)) -> bool {
    let mut character_pos = starting_pos;
    let mut up_move = -1;
    let mut right_move = 0;
    let mut maze_directions = Vec::new();
    for i in 0..maze.len() {
        maze_directions.push(Vec::new());
        for j in 0..maze[0].len() {
            maze_directions[i].push(VisitedDir::NONE as i32);
        }
    }
    loop {
        let next_pos = (character_pos.0 + up_move, character_pos.1 + right_move);

        if next_pos.0 >= maze.len() as i32 || next_pos.0 < 0 || next_pos.1 >= maze[0].len() as i32 || next_pos.1 < 0 {
            return false
        }
        // Check next position
        match maze[next_pos.0 as usize][next_pos.1 as usize] {
            Elem::UNVISITED | Elem::VISITED | Elem::CHARACTER => {
                character_pos = next_pos;
                let dir = getDirFromPositions(up_move, right_move) as i32;
                if (dir & maze_directions[next_pos.0 as usize][next_pos.1 as usize]) != 0 {
                    return true
                }
                maze_directions[next_pos.0 as usize][next_pos.1 as usize] |= dir;
            }
            Elem::OBSTACLE => {
                if up_move == -1 {
                    up_move = 0;
                    right_move = 1;
                }
                else if up_move == 1 {
                    up_move = 0;
                    right_move = -1;
                }
                else if right_move == -1 {
                    right_move = 0;
                    up_move = -1;
                }
                else {
                    right_move = 0;
                    up_move = 1;
                }
            }
            _ => {}
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut items = input.lines()
        .map(|l| l.as_bytes().iter().map(|x|map_to_elem(*x)).collect::<Vec<Elem>>()).collect::<Vec<Vec<Elem>>>();
    let mut character_pos = (0i32, 0i32);
    let mut up_move = -1;
    let mut right_move = 0;
    for x in 0..items.len() as i32 {
        for y in 0..items[0].len() as i32 {
            if items[x as usize][y as usize] == Elem::CHARACTER {
                character_pos = (x, y);
            }
        }
    }
    let starting_pos = character_pos;

    // Iterate once to get all visitable
    loop {
        let next_pos = (character_pos.0 + up_move, character_pos.1 + right_move);

        if next_pos.0 >= items.len() as i32 || next_pos.0 < 0 || next_pos.1 >= items[0].len() as i32 || next_pos.1 < 0 {
            items[character_pos.0 as usize][character_pos.1 as usize] = Elem::VISITED;
            break;
        }
        // Check next position
        match items[next_pos.0 as usize][next_pos.1 as usize] {
            Elem::UNVISITED | Elem::VISITED => {
                items[character_pos.0 as usize][character_pos.1 as usize] = Elem::VISITED;
                character_pos = next_pos;
            }
            Elem::OBSTACLE => {
                if up_move == -1 {
                    up_move = 0;
                    right_move = 1;
                }
                else if up_move == 1 {
                    up_move = 0;
                    right_move = -1;
                }
                else if right_move == -1 {
                    right_move = 0;
                    up_move = -1;
                }
                else {
                    right_move = 0;
                    up_move = 1;
                }
            }
            Elem::CHARACTER => {}
            _ => {}
        }
    }
    items[starting_pos.0 as usize][starting_pos.1 as usize] = Elem::CHARACTER;

    let mut result = 0;
    for i in 0..items.len() as i32 {
        for j in 0..items[0].len() as i32 {
            if items[i as usize][j as usize] == Elem::VISITED {
                items[i as usize][j as usize] = Elem::OBSTACLE;
                if creates_loop(&items, starting_pos) {
                    println!("{i},{j}");
                    result += 1;
                }
                items[i as usize][j as usize] = Elem::VISITED;
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
