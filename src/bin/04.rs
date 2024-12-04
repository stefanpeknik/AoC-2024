advent_of_code::solution!(4);

fn load_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.to_string().chars().collect())
        .collect()
}

fn occurs_in_dir(
    origin_x: isize,
    origin_y: isize,
    input: &Vec<Vec<char>>,
    word: &str,
    step_x: isize,
    step_y: isize,
) -> bool {
    let mut x = origin_x;
    let mut y = origin_y;

    for c in word.chars() {
        if let (Ok(x_usize), Ok(y_usize)) = (usize::try_from(x), usize::try_from(y)) {
            if input.get(y_usize).and_then(|row| row.get(x_usize)) != Some(&c) {
                return false;
            }
        } else {
            return false;
        }
        x += step_x;
        y += step_y;
    }

    true
}

fn occurences(input: &Vec<Vec<char>>, word: &str) -> u32 {
    let mut count = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            // Check all directions
            for step_y in -1..=1 {
                for step_x in -1..=1 {
                    if occurs_in_dir(x as isize, y as isize, input, word, step_x, step_y) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn count_x(input: &Vec<Vec<char>>, word: &str) -> u32 {
    let mut count = 0;

    for (y, line) in input.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let x = x as isize;
            let y = y as isize;
            let shift = word.len() as isize - 1;
            // Check all directions
            // right down
            if occurs_in_dir(x, y, input, word, 1, 1) {
                // shifted right, left down
                if occurs_in_dir(x + shift, y, input, word, -1, 1) {
                    count += 1;
                }
                // shifted down, right up
                if occurs_in_dir(x, y + shift, input, word, 1, -1) {
                    count += 1;
                }
            }
            // left up
            if occurs_in_dir(x, y, input, word, -1, -1) {
                // shifted left, right up
                if occurs_in_dir(x - shift, y, input, word, 1, -1) {
                    count += 1;
                }
                // shifted up, left down
                if occurs_in_dir(x, y - shift, input, word, -1, 1) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = load_input(input);
    Some(occurences(&input, "XMAS"))
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = load_input(input);
    Some(count_x(&input, "MAS"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
