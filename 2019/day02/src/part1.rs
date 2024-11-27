pub fn process(input: &str) -> anyhow::Result<String> {
    let mut program = input
        .lines()
        .flat_map(|line| line.split(","))
        .map(|item| {
            item.parse::<usize>()
                .expect(&format!("not a number {:?}", item))
        })
        .collect::<Vec<_>>();

    program[1] = 12;
    program[2] = 2;

    let executed_program = aoc::intcode::execute_program(program)?;

    Ok(executed_program[0].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "1,0,0,0,99,0,0,0,99,0,0,0,99,0,0,0";

        assert_eq!("101", process(input)?);
        Ok(())
    }
}
