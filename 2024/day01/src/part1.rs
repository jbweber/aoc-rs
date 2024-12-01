pub fn process(input: &str) -> anyhow::Result<String> {
    let mut col1 = vec![];
    let mut col2 = vec![];

    input.lines().map(|l| l.split_ascii_whitespace().filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<_>>()).for_each(|v| {
        col1.push(v[0]);
        col2.push(v[1]);
    });

    col1.sort_unstable();
    col2.sort_unstable();

    let result = col1.iter().zip(col2.iter()).map(|(a, b)| a.abs_diff(*b)).sum::<u32>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
