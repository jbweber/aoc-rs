use std::collections::HashSet;

pub fn process(input: &str) -> anyhow::Result<String> {
    let lab = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let start = find_start(&lab).expect("no start found");

    let mut direction = Direction::North;

    let mut current = start.clone();
    let mut obstructions = HashSet::new();
    loop {
        if let Some(next) = find_next(&lab, &current, &direction) {
            match next.0 {
                '#' => {
                    direction = direction.turn();
                    continue;
                }
                _ => {
                    if let Some(obstruction) = find_obstruction(&lab, &current, &direction) {
                        obstructions.insert(next.1.clone());
                    }
                    current = next.1.clone();
                    continue;
                }
            }
        } else {
            // move off the lab -> end
            break;
        }
    }

    let mut count = 0;
    // check for loops
    for obstruction in &obstructions {
        if contains_loop(&lab, &start, &obstruction) {
            count += 1;
        }
        break;
    }

    Ok(count.to_string())
}

fn contains_loop(
    lab: &Vec<Vec<char>>,
    start: &(usize, usize),
    obstruction: &(usize, usize),
) -> bool {
    let mut current = start.clone();
    let mut direction = Direction::North;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    loop {
        if current == (4, 0) {
            dbg!("4,0");
        }
        if let Some(next) = find_next(&lab, &current, &direction) {
            let visited_position = current.clone();
            match next.0 {
                '#' => {
                    direction = direction.turn();
                    continue;
                }
                '.' | '^' => {
                    if &next.1 == obstruction {
                        direction = direction.turn();
                        continue;
                    }

                    if visited.contains(&visited_position) {
                        return true;
                    }

                    visited.insert(visited_position.clone());

                    current = next.1.clone();
                    continue;
                }
                _ => unreachable!(),
            }
        } else {
            // move off the lab -> end
            break;
        }
    }

    false
}

fn find_obstruction(
    lab: &Vec<Vec<char>>,
    current_position: &(usize, usize),
    current_direction: &Direction,
) -> Option<(usize, usize)> {
    let new_direction = current_direction.turn();
    let mut current = current_position.clone();

    loop {
        if let Some(next) = find_next(&lab, &current, &new_direction) {
            match next.0 {
                '#' => return Some(next.1.clone()),
                _ => {
                    current = next.1.clone();
                    continue;
                }
            }
        } else {
            // move off the lab -> end
            break;
        }
    }
    None
}

fn find_start(lab: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for y in lab.iter().enumerate() {
        for x in y.1.iter().enumerate() {
            if x.1 == &'^' {
                return Some((x.0, y.0));
            }
        }
    }
    None
}

fn find_next(
    lab: &Vec<Vec<char>>,
    current: &(usize, usize),
    direction: &Direction,
) -> Option<(char, (usize, usize))> {
    match direction {
        Direction::North => {
            if current.1 as i32 - 1 < 0 {
                return None;
            }
            Some((lab[current.1 - 1][current.0], (current.0, current.1 - 1)))
        }
        Direction::South => {
            if current.1 + 1 >= lab.len() {
                return None;
            }
            Some((lab[current.1 + 1][current.0], (current.0, current.1 + 1)))
        }
        Direction::East => {
            if current.0 + 1 >= lab[0].len() {
                return None;
            }
            Some((lab[current.1][current.0 + 1], (current.0 + 1, current.1)))
        }
        Direction::West => {
            if current.0 as i32 - 1 < 0 {
                return None;
            }
            Some((lab[current.1][current.0 - 1], (current.0 - 1, current.1)))
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl From<&char> for Direction {
    fn from(value: &char) -> Self {
        match value {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => panic!("unknown direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
