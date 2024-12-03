advent_of_code::solution!(3);

use regex::Regex;
use std::sync::LazyLock;

static INSTRUCTION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"(?P<instruction>(?P<oper>{cmds})\((?P<ops>\d{{1,3}}(,\d{{1,3}})*)?\))",
        cmds = ["mul", "do", "don't"].join("|")
    ))
    .unwrap()
});

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = 0;
    for cap in INSTRUCTION_RE.captures_iter(input) {
        match Some(cap.name("oper").unwrap().as_str()) {
            Some("mul") => {
                let ops = cap
                    .name("ops")
                    .unwrap()
                    .as_str()
                    .split(",")
                    .collect::<Vec<_>>();
                if ops.len() == 2 {
                    res += ops
                        .iter()
                        .map(|x| x.parse::<u32>().unwrap())
                        .product::<u32>()
                }
            }
            _ => (),
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = 0;
    let mut enabled = true;
    for cap in INSTRUCTION_RE.captures_iter(input) {
        match Some(cap.name("oper").unwrap().as_str()) {
            Some("mul") if enabled => {
                let ops = cap
                    .name("ops")
                    .unwrap()
                    .as_str()
                    .split(",")
                    .collect::<Vec<_>>();
                if ops.len() == 2 {
                    res += ops
                        .iter()
                        .map(|x| x.parse::<u32>().unwrap())
                        .product::<u32>()
                }
            }
            Some("do") => {
                let ops = match cap.name("ops") {
                    Some(ops) => ops.as_str().split(",").collect::<Vec<_>>(),
                    None => vec![],
                };
                if ops.len() == 0 {
                    enabled = true;
                }
            }
            Some("don't") => {
                let ops = match cap.name("ops") {
                    Some(ops) => ops.as_str().split(",").collect::<Vec<_>>(),
                    None => vec![],
                };
                if ops.len() == 0 {
                    enabled = false;
                }
            }

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
