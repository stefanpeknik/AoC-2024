use std::collections::HashMap;

advent_of_code::solution!(11);

const RULES: &[fn(usize) -> Option<Vec<usize>>] = &[
    // the stone is engraved with the number 0
    |stone: usize| -> Option<Vec<usize>> {
        if stone == 0 {
            Some(vec![1])
        } else {
            None
        }
    },
    // the stone is engraved with a number that has an even number of digits
    |stone: usize| -> Option<Vec<usize>> {
        let digits = stone.to_string();
        if digits.len() % 2 == 0 {
            let mid = digits.len() / 2;
            let left = digits[..mid].parse().expect("Failed to parse left half");
            let right = digits[mid..].parse().expect("Failed to parse right half");
            let result = vec![left, right];
            Some(result)
        } else {
            None
        }
    },
    // none of the other rules apply
    |stone: usize| -> Option<Vec<usize>> {
        let result = vec![stone * 2024];
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    },
];

fn parse_input(input: &str) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for stone in input.split(' ') {
        let stone = stone
            .parse()
            .expect(format!("Failed to parse stone: {}", stone).as_str());
        *map.entry(stone).or_insert(0) += 1;
    }
    map
}

pub fn part_one(input: &str) -> Option<usize> {
    const ITERS: usize = 25;

    let mut stones = parse_input(input);

    for _ in 0..ITERS {
        let mut new_stones = HashMap::new();
        for (stone, count) in stones {
            for rule in RULES {
                if let Some(result) = rule(stone) {
                    for new_stone in result {
                        *new_stones.entry(new_stone).or_insert(0) += count;
                    }
                    break;
                }
            }
        }
        stones = new_stones;
    }

    Some(stones.values().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    const ITERS: usize = 75;

    let mut stones = parse_input(input);

    for _ in 0..ITERS {
        let mut new_stones = HashMap::new();
        for (stone, count) in stones {
            for rule in RULES {
                if let Some(result) = rule(stone) {
                    for new_stone in result {
                        *new_stones.entry(new_stone).or_insert(0) += count;
                    }
                    break;
                }
            }
        }
        stones = new_stones;
    }

    Some(stones.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
