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
    let num_ops = b.len() - 1;
    for op in 0..(3u32.pow(num_ops as u32)) {
        let mut result = b[0];
        let mut current = op;
        for i in 0..num_ops {
            let do_op = current % 3;
            current /= 3;

            match do_op {
                0 => result += b[i + 1],
                1 => result *= b[i + 1],
                2 => result = concat(result, b[i + 1]),
                _ => unreachable!(),
            }
        }
        if result == a {
            return true;
        }
    }

    false
}

fn concat(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b)
        .parse::<i64>()
        .expect("invalid number")
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
        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
