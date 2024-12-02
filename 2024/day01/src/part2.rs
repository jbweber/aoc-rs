pub fn process(input: &str) -> anyhow::Result<String> {
    let mut col1 = vec![];
    let mut col2 = vec![];

    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .for_each(|v| {
            col1.push(v[0]);
            col2.push(v[1]);
        });

    let result = col1
        .iter()
        .map(|v| col2.iter().filter(|&s| v == s).count() as u32 * v)
        .sum::<u32>();

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
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
