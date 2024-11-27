pub fn process(input: &str) -> anyhow::Result<String> {
    let result = input
        .lines()
        .map(|line| {
            let parsed_line = line.parse::<u32>().expect("not a number?");
            calculate_fuel(parsed_line)
        })
        .sum::<u32>();
    Ok(result.to_string())
}

fn calculate_fuel(fuel: u32) -> u32 {
    let next = (fuel as f32 / 3.0).floor() as i32 - 2;
    if next < 0 {
        return 0;
    }

    next as u32 + calculate_fuel(next as u32)
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
        assert_eq!("51316", process(input)?);
        Ok(())
    }
}
