advent_of_code::solution!(7);

use itertools::Itertools;

const OPERATORS: [&str; 3] = ["+", "*", "||"];

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(
            |line| match line.split(": ").collect::<Vec<&str>>().as_slice() {
                [result, operands] => {
                    let operands = operands
                        .split(" ")
                        .map(|operand| operand.parse().expect("Could not parse operand"))
                        .collect::<Vec<usize>>();
                    (result.parse().expect("Could not parse result"), operands)
                }
                _ => panic!("Could not parse line"),
            },
        )
        .collect()
}

fn generate_combinations(chars: &[&str], length: usize) -> Vec<Vec<String>> {
    // Create an iterator that repeats the slice `chars` `length` times
    std::iter::repeat(chars)
        // Take only `length` number of repeated slices
        .take(length)
        // Generate the Cartesian product of the repeated slices
        .multi_cartesian_product()
        // Map each combination to a vector of strings
        .map(|x| x.iter().map(|&&s| s.to_string()).collect())
        // Collect all combinations into a vector
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse_input(input);
    let mut res = 0;
    for (result, operands) in input {
        let mut can_be_solved = false;
        for combination in generate_combinations(&OPERATORS, operands.len() - 1) {
            let mut comb_result = *operands.first().expect("Could not get first operand");
            for (operator, operand) in combination.iter().zip(operands.iter().skip(1)) {
                match operator.as_str() {
                    "+" => comb_result += operand,
                    "*" => comb_result *= operand,
                    _ => (), // ignore other operators
                }
            }
            if comb_result == result {
                can_be_solved = true;
                break;
            }
        }
        if can_be_solved {
            res += result;
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse_input(input);
    let mut res = 0;
    for (result, operands) in input {
        let mut can_be_solved = false;
        for combination in generate_combinations(&OPERATORS, operands.len() - 1) {
            let mut comb_result = *operands.first().expect("Could not get first operand");
            for (operator, operand) in combination.iter().zip(operands.iter().skip(1)) {
                match operator.as_str() {
                    "+" => comb_result += operand,
                    "*" => comb_result *= operand,
                    "||" => {
                        comb_result = format!("{}{}", comb_result, operand)
                            .parse()
                            .expect("Could not parse expression")
                    }
                    _ => (), // ignore other operators
                }
            }
            if comb_result == result {
                can_be_solved = true;
                break;
            }
            if comb_result == result {
                can_be_solved = true;
                break;
            }
        }
        if can_be_solved {
            res += result;
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_generate_combinations() {
        let result = generate_combinations(&OPERATORS, 2);
        assert_eq!(
            result,
            vec![
                vec!["+".to_string(), "+".to_string()],
                vec!["+".to_string(), "*".to_string()],
                vec!["+".to_string(), "||".to_string()],
                vec!["*".to_string(), "+".to_string()],
                vec!["*".to_string(), "*".to_string()],
                vec!["*".to_string(), "||".to_string()],
                vec!["||".to_string(), "+".to_string()],
                vec!["||".to_string(), "*".to_string()],
                vec!["||".to_string(), "||".to_string()]
            ]
        );

        let result = generate_combinations(&OPERATORS, 3);
        assert_eq!(result.len(), 27);
    }
}
