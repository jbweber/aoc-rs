use std::fmt::Formatter;

enum OpCode {
    Add,
    Multiply,
    Halt,
}

impl OpCode {
    fn from_usize(value: usize) -> Option<OpCode> {
        match value {
            1 => Some(OpCode::Add),
            2 => Some(OpCode::Multiply),
            99 => Some(OpCode::Halt),
            _ => None,
        }
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Add => write!(f, "ADD"),
            OpCode::Multiply => write!(f, "MULTIPLY"),
            OpCode::Halt => write!(f, "HALT"),
        }
    }
}

pub fn execute_program(program: Vec<usize>) -> anyhow::Result<Vec<usize>> {
    let mut program = program;
    let mut idx = 0;
    loop {
        // every 4 items

        let opcode =
            OpCode::from_usize(program[idx]).expect(&format!("not a opcode {}", program[idx]));
        if let OpCode::Halt = opcode {
            break;
        }

        let input_1 = program[idx + 1];
        let input_2 = program[idx + 2];
        let output = program[idx + 3];

        let value_1 = program[input_1];
        let value_2 = program[input_2];

        match opcode {
            OpCode::Add => {
                program[output] = value_1 + value_2;
            }
            OpCode::Multiply => {
                program[output] = value_1 * value_2;
            }
            _ => {
                Err(anyhow::anyhow!("invalid opcode {} at this point", opcode))?;
            }
        }

        idx += 4;
    }

    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,0,0,0,99], vec![2,0,0,0,99])]
    #[case(vec![2,3,0,3,99], vec![2,3,0,6,99])]
    #[case(vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801])]
    #[case(vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99])]
    fn test_execute_program(
        #[case] program: Vec<usize>,
        #[case] executed_program: Vec<usize>,
    ) -> anyhow::Result<()> {
        assert_eq!(execute_program(program)?, executed_program);
        Ok(())
    }
}
