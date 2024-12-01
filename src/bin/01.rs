advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a: usize = parts.next().unwrap().parse().expect("Invalid input");
            let b: usize = parts.next().unwrap().parse().expect("Invalid input");
            (a, b)
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lists = parse_lists(input);
    lists.0.sort_unstable();
    lists.1.sort_unstable();
    let total_distance = lists
        .0
        .into_iter()
        .zip(lists.1.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum();
    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lists = parse_lists(input);

    let mut similarity_score = 0;
    for a in lists.0 {
        similarity_score += a * lists.1.iter().filter(|&b| a == *b).count();
    }

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result: Option<usize> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
