use std::collections::HashSet;

pub fn process(input: &str) -> anyhow::Result<String> {
    let lab = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let start = find_start(&lab).expect("no start found");

    let mut direction = Direction::North;
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut current = start.clone();
    loop {
        if let Some(next) = find_next(&lab, &current, &direction) {
            match next.0 {
                '#' => {
                    direction = direction.turn();
                    //     dbg!("hit item turning {}", &direction);
                    continue;
                }
                _ => {
                    visited.insert(current);
                    current = next.1.clone();
                    //   dbg!("{} at {}", &direction, current);
                    continue;
                }
            }
        } else {
            // move off the lab -> end
            visited.insert(current);
            break;
        }
    }

    //    dbg!(visited);
    Ok(visited.len().to_string())
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

#[derive(Debug)]
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
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
