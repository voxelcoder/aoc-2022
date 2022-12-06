struct Stack {
    pub id: u8,
    pub crates: Vec<char>,
}

impl Stack {
    fn push(&mut self, crate_: char) {
        let mut crates = [crate_].to_vec();
        crates.append(&mut self.crates);

        self.crates = crates;
    }

    fn pop(&mut self) -> Option<char> {
        if let Some(pos) = self
            .crates
            .iter()
            .position(|x| x == self.crates.first().unwrap())
        {
            let first = self.crates[0];
            self.crates.remove(0);
            return Some(first);
        }

        None
    }
}

struct Instruction {
    pub count: u8,
    pub from: u8,
    pub to: u8,
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let split = string.split("from").map(str::trim).collect::<Vec<_>>();
        let from_to = split[1].split("to").map(str::trim).collect::<Vec<_>>();

        Self {
            count: split[0].replace("move ", "").parse::<u8>().unwrap(),
            from: from_to[0].parse::<u8>().unwrap(),
            to: from_to[1].replace("to ", "").trim().parse::<u8>().unwrap(),
        }
    }
}

fn construct_stacks(input: Vec<Vec<String>>) -> Vec<Stack> {
    (0..input[0].len())
        .map(|x| Stack {
            id: x as u8,
            crates: (0..input.len())
                .map(|y| input[y][x].chars().into_iter().collect::<Vec<_>>()[1])
                .filter(|crate_| !crate_.is_whitespace())
                .collect(),
        })
        .collect()
}

fn parse_crates(input: &str) -> Vec<Vec<String>> {
    input
        .split('\n')
        .take(input.split('\n').into_iter().count() - 1)
        .map(|row| {
            row.chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|c| String::from_iter(c.iter()))
                .map(|cell| {
                    if cell.trim().is_empty() {
                        "[ ]".to_string()
                    } else {
                        cell.trim().to_string()
                    }
                })
                .collect()
        })
        .collect()
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input.trim().split('\n').map(Instruction::from).collect()
}

fn prepare_input(input: &str) -> (Vec<Stack>, Vec<Instruction>) {
    let parts = input
        .split("\n\n")
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();

    (
        construct_stacks(parse_crates(parts[0])),
        parse_instructions(parts[1]),
    )
}

fn execute(input: &str, part: u8) -> String {
    let (mut stacks, instructions) = prepare_input(input);

    for instruction in instructions {
        let mut popped_chars: Vec<char> = (0..instruction.count)
            .filter_map(|_| stacks[instruction.from as usize - 1].pop())
            .collect();

        if part == 1 {
            popped_chars.reverse();
        }

        let to_stack = &mut stacks[instruction.to as usize - 1];
        popped_chars.append(&mut to_stack.crates);
        to_stack.crates = popped_chars;
    }

    String::from_iter(stacks.iter().map(|stack| stack.crates.first().unwrap()))
}

pub fn part_1(input: &str) -> String {
    execute(input, 1)
}

pub fn part_2(input: &str) -> String {
    execute(input, 2)
}

#[cfg(test)]
mod test {
    use crate::day5::{part_1, part_2};

    const FILE_INPUT: &str = include_str!("input.txt");
    const EXAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT), "CMZ");
        assert_eq!(part_1(FILE_INPUT), "MQTPGLLDN");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT), "MCD");
        assert_eq!(part_2(FILE_INPUT), "LVZPSTTCZ");
    }
}
