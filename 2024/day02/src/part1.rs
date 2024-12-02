use std::cmp::PartialEq;

#[derive(Debug)]
struct Report {
    levels: Vec<u32>,
    direction: Direction,
    safe: bool,
}

impl Report {
    fn new(levels: Vec<u32>) -> Report {
        let first = levels[0];
        let second = levels[1];

        let mut direction = Direction::Increasing;
        if first > second {
            direction = Direction::Decreasing;
        }

        let mut report = Report {
            levels,
            direction,
            safe: false,
        };

        report.safety_check();

        report
    }

    fn safety_check(&mut self) {
        self.safe = false;
        for (idx, _) in self.levels.iter().enumerate() {
            if idx == self.levels.len() - 1 {
                break;
            }
            let first = self.levels[idx];
            let second = self.levels[idx + 1];
            if first < second && self.direction == Direction::Increasing {
                let diff = first.abs_diff(second);
                if diff >= 1 && diff <= 3 {
                    self.safe = true;
                    continue;
                }
            }

            if second < first && self.direction == Direction::Decreasing {
                let diff = first.abs_diff(second);
                if diff >= 1 && diff <= 3 {
                    self.safe = true;
                    continue;
                }
            }

            self.safe = false;
            break;
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

pub fn process(input: &str) -> anyhow::Result<String> {
    let reports = input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|i| i.parse::<u32>().expect("parse"))
                .collect::<Vec<_>>();
            Report::new(levels)
        })
        .collect::<Vec<_>>();

    let result = reports.iter().filter(|r| r.safe).count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
