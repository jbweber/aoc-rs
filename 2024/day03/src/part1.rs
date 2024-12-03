use regex::Regex;

pub fn process(input: &str) -> anyhow::Result<String> {
    let result: u32 = process_muls(input).iter().map(|mul| mul.0 * mul.1).sum();

    Ok(result.to_string())
}

fn process_muls(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut results = Vec::new();
    for captures in re.captures_iter(input) {
        let num1: u32 = captures[1].parse().unwrap();
        let num2: u32 = captures[2].parse().unwrap();

        results.push((num1, num2));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mul(1,3)", vec![(1,3)])]
    #[case("mul(1,3)mul(1,3)", vec![(1,3),(1,3)])]
    #[case("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", vec![(2,4),(5,5),(11,8),(8,5)])]
    fn test_process_muls(#[case] line: &str, #[case] expected: Vec<(u32, u32)>) {
        let result = process_muls(line);
        assert_eq!(result, expected)
    }

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
