use crate::{
    apply_directions, apply_directions2, line_intersection, parse_directions, Point, CENTRAL_PORT,
};
use anyhow::bail;
use std::collections::{HashMap, HashSet};

pub fn process(input: &str) -> anyhow::Result<String> {
    let directions = input
        .lines()
        .map(|l| parse_directions(l).expect("valid line"))
        .collect::<Vec<_>>();

    if directions.len() != 2 {
        bail!("expected two directions");
    }

    let lines1 = apply_directions2(&directions[0].1);
    let lines2 = apply_directions2(&directions[1].1);

    let mut intersections = HashSet::new();

    for line1 in lines1.iter() {
        for line2 in lines2.iter() {
            if let Some(v) =
                line_intersection(&line1.start, &line1.end, &line2.start, &line2.end, true)
            {
                if v != CENTRAL_PORT {
                    dbg!("lines {} {}", &line1, &line2);
                    dbg!("found intersection {}", &v);
                    intersections.insert(v);
                }
            }
        }
    }

    let result = intersections
        .iter()
        .map(|i| i.manhattan(&CENTRAL_PORT))
        .collect::<Vec<_>>();

    let result = result.iter().min().unwrap();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "R8,U5,L5,D3
U7,R6,D4,L4";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
