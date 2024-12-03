advent_of_code::solution!(3);

use regex::Regex;
use std::sync::LazyLock;

struct Instruction {
    oper: String,
    ops: Vec<u32>,
}

impl Instruction {
    fn from(caps: &regex::Captures) -> Self {
        Instruction {
            oper: caps["oper"].to_string(),
            ops: if let Some(ops) = caps.name("ops") {
                ops.as_str()
                    .split(',')
                    .map(|op| op.parse().unwrap())
                    .collect()
            } else {
                Vec::new()
            },
        }
    }
}

struct InstructionInterpreter;
enum InstructionResult {
    Mul(u32),
    Do,
    Dont,
    Err,
}

impl InstructionInterpreter {
    pub const OPERATIONS: &'static [&'static str] = &["mul", "do", "don't"];

    fn interpret(instruction: &Instruction) -> InstructionResult {
        match instruction.oper.as_str() {
            "mul" if instruction.ops.len() == 2 => {
                InstructionResult::Mul(instruction.ops.iter().product())
            }
            "do" => InstructionResult::Do,
            "don't" => InstructionResult::Dont,
            _ => InstructionResult::Err,
        }
    }
}

static INSTRUCTION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"(?P<instruction>(?P<oper>{cmds})\((?P<ops>\d{{1,3}}(,\d{{1,3}})*)?\))",
        cmds = InstructionInterpreter::OPERATIONS.join("|")
    ))
    .unwrap()
});

struct Scanner<'a> {
    gen: regex::CaptureMatches<'a, 'a>,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Scanner {
            gen: INSTRUCTION_RE.captures_iter(input),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.gen.next() {
            Some(caps) => Some(Instruction::from(&caps)),
            None => None,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = 0;
    for instruc in Scanner::new(input) {
        match InstructionInterpreter::interpret(&instruc) {
            InstructionResult::Mul(val) => res += val,
            _ => (),
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = 0;
    let mut enabled = true;
    for instruc in Scanner::new(input) {
        match InstructionInterpreter::interpret(&instruc) {
            InstructionResult::Mul(val) if enabled => res += val,
            InstructionResult::Do => enabled = true,
            InstructionResult::Dont => enabled = false,
            _ => (),
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
