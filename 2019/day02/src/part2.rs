use aoc::intcode::execute_program;

pub fn process(input: &str) -> anyhow::Result<String> {
    let program = input
        .lines()
        .flat_map(|line| line.split(","))
        .map(|item| {
            item.parse::<usize>()
                .expect(&format!("not a number {:?}", item))
        })
        .collect::<Vec<_>>();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut check = program.clone();
            check[1] = noun;
            check[2] = verb;

            let result = execute_program(check)?;
            if result[0] == 19690720 {
                println!("{} {}", noun.to_string(), verb.to_string());
                let rt = 100 * noun + verb;
                return Ok(rt.to_string());
            }
        }
    }

    Err(anyhow::anyhow!("no solution found"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process() -> anyhow::Result<()> {
        // let input = "";
        // assert_eq!("", process(input)?);
        Ok(())
    }
}
