pub fn process(_input: &str) -> anyhow::Result<String> {
    Ok(String::from("part 2"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
