pub fn process(input: &str) -> anyhow::Result<String> {
    let input = input
        .lines()
        .map(|line| {
            if let Some((a, b)) = line.split_once(":") {
                let a = a.parse::<i64>().expect("invalid number");
                let b = b
                    .trim()
                    .split_ascii_whitespace()
                    .map(|word| word.parse::<i64>().expect("invalid number"))
                    .collect::<Vec<_>>();
                (a, b)
            } else {
                unreachable!("invalid input")
            }
        })
        .collect::<Vec<_>>();

    let result = input
        .iter()
        .filter(|i| check(i.0, &i.1))
        .map(|i| i.0)
        .sum::<i64>();

    Ok(result.to_string())
}

fn check(a: i64, b: &Vec<i64>) -> bool {
    let ops = b.len() - 1;
    for bits in 0..(1 << ops) {
        let mut result = b[0];
        for i in 0..ops {
            if (bits & (1 << i)) != 0 {
                result += b[i + 1];
            } else {
                result *= b[i + 1];
            }
        }
        if result == a {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
