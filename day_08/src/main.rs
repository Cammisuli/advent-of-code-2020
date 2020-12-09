use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = s.split(' ');
        let instruction = value.next();
        let operation = value.next().unwrap().parse::<isize>();
        match (instruction, operation) {
            (Some("acc"), Ok(operation)) => Ok(self::Instruction::Acc(operation)),
            (Some("jmp"), Ok(operation)) => Ok(self::Instruction::Jmp(operation)),
            (Some("nop"), _) => Ok(self::Instruction::Nop),
            _ => Err("Instruction not found".to_string()),
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", inputs());
    let part_one = part_one(&inputs());
    println!("{:?}", part_one);

    // 2 - 2001
}

fn inputs() -> Vec<Instruction> {
    include_str!("input.txt")
        .lines()
        .map(|s| Instruction::from_str(s).unwrap())
        .collect()
}

fn part_two(instructions: &Vec<Instruction>) -> isize {
    // for instruction in instructions {}
}

fn part_one(instructions: &Vec<Instruction>) -> (isize, bool) {
    let total = instructions.len();
    let mut index = 0;
    let mut position = 0;
    let mut acc = 0;
    let mut terminated = false;
    let mut seen = vec![false; total];
    while let Some(instruction) = instructions.get(position) {
        index += 1;
        match instruction {
            Instruction::Acc(op) => {
                position += 1;
                acc += *op;
            }
            Instruction::Jmp(op) => {
                position = if *op < 0 {
                    position - op.abs() as usize
                } else {
                    position + *op as usize
                };
            }
            Instruction::Nop => {
                position += 1;
            }
        }

        if seen[position] {
            break;
        }

        if (position >= total) {
            terminated = true;
            break;
        }

        seen[position] = true;
    }
    (acc, terminated)
}
