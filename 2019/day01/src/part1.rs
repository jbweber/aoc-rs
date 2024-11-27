pub fn process(input: &str) -> anyhow::Result<String> {
    let result = input
        .lines()
        .map(|line| {
            let parsed_line = line.parse::<u32>().expect("not a number?");
            ((parsed_line as f32 / 3.0).floor() - 2.0) as u32
        })
        .sum::<u32>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "12
14
1969
100756";
        assert_eq!("34241", process(input)?);
        Ok(())
    }
}
