use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::IResult;
use std::collections::HashSet;

pub mod part1;
pub mod part2;

pub const CENTRAL_PORT: Point = Point { x: 0, y: 0 };

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn manhattan(&self, destination: &Point) -> i32 {
        (self.x - destination.x).abs() + (self.y - destination.y).abs()
    }

    fn move_direction(&self, direction: &Direction, distance: u32) -> Point {
        match direction {
            Direction::Up => Point::new(self.x, self.y + distance as i32),
            Direction::Down => Point::new(self.x, self.y - distance as i32),
            Direction::Left => Point::new(self.x - distance as i32, self.y),
            Direction::Right => Point::new(self.x + distance as i32, self.y),
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn intersection(&self, other: &Line) -> Option<Point> {
        let denominator = (self.start.x - self.end.x) * (other.start.y - other.end.y)
            - (self.start.y - self.end.y) * (other.start.x - other.end.x);

        if denominator.abs() == 0 {
            return None;
        }

        let t_numerator = (self.start.x - other.start.x) * (other.start.y - other.end.y)
            - (self.start.y - self.end.y) * (other.start.x - other.end.x);
        let t = t_numerator / denominator;

        let intersection = Point {
            x: self.start.x + t * (self.end.x - self.start.x),
            y: self.start.y + t * (self.end.y - self.start.y),
        };

        Some(intersection)
    }
}

fn line_intersection(
    p1: &Point,
    p2: &Point, // Line 1: p1 -> p2
    q1: &Point,
    q2: &Point,           // Line 2: q1 -> q2
    segment_check: bool, // If true, check for line segment intersection
) -> Option<Point> {
    // Calculate the components of the line equations
    let a1 = p2.y - p1.y;
    let b1 = p1.x - p2.x;
    let c1 = a1 * p1.x + b1 * p1.y;

    let a2 = q2.y - q1.y;
    let b2 = q1.x - q2.x;
    let c2 = a2 * q1.x + b2 * q1.y;

    // Calculate the determinant
    let determinant = a1 * b2 - a2 * b1;

    // If determinant is zero, the lines are parallel
    if (determinant.abs() as f64) < f64::EPSILON {
        return None;
    }

    // Compute the intersection point
    let x = (b2 * c1 - b1 * c2) / determinant;
    let y = (a1 * c2 - a2 * c1) / determinant;
    let intersection = Point { x, y };

    // If segment_check is true, ensure the intersection lies within both segments
    if segment_check {
        if !point_on_segment(&intersection, p1, p2) || !point_on_segment(&intersection, q1, q2) {
            return None;
        }
    }

    Some(intersection)
}

fn point_on_segment(point: &Point, seg_start: &Point, seg_end: &Point) -> bool {
    let min_x = seg_start.x.min(seg_end.x);
    let max_x = seg_start.x.max(seg_end.x);
    let min_y = seg_start.y.min(seg_end.y);
    let max_y = seg_start.y.max(seg_end.y);

    point.x >= min_x && point.x <= max_x && point.y >= min_y && point.y <= max_y
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn apply_directions2(directions: &Vec<(Direction, u32)>) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();

    let start = Point::new(0, 0);
    let end = start.move_direction(&directions[0].0, directions[0].1);

    lines.push(Line { start, end });
    for direction in directions.iter().skip(1) {
        let start = &lines.last().expect("Should have at least one line").end;
        let end = start.move_direction(&direction.0, direction.1);
        lines.push(Line {
            start: start.clone(),
            end,
        })
    }

    lines
}

fn apply_directions(directions: &Vec<(Direction, u32)>) -> anyhow::Result<HashSet<Point>> {
    let mut result = HashSet::new();
    let start = Point { x: 0, y: 0 };
    let mut last = start.clone();
    for direction in directions {
        let next = last.move_direction(&direction.0, direction.1);
        result.insert(last);
        last = next;
    }

    result.remove(&start);

    dbg!(&result);
    Ok(result)
}

fn parse_direction(input: &str) -> IResult<&str, (Direction, u32)> {
    let (input, (letter, digit)) = pair(alpha1, digit1)(input)?;
    let direction = match letter {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("unknown direction {}", letter),
    };

    let distance = digit.parse::<u32>().expect("invalid u32");

    Ok((input, (direction, distance)))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<(Direction, u32)>> {
    let (input, directions) = separated_list1(tag(","), parse_direction)(input)?;
    Ok((input, directions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let line1 = Line {
            start: Point { x: 3, y: 5 },
            end: Point { x: 3, y: 2 },
        };
        let line2 = Line {
            start: Point { x: 2, y: 3 },
            end: Point { x: 6, y: 3 },
        };

        dbg!(line1.intersection(&line2).unwrap());
    }
}
