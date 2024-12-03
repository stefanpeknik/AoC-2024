advent_of_code::solution!(2);

fn load_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse().expect("Invalid input"))
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let list_of_levels = load_input(input);
    let mut res = 0;
    let range = 1..=3;
    for levels in list_of_levels {
        let expected_order = match &levels.iter().take(2).collect::<Vec<_>>().as_slice() {
            [a, b] => a.cmp(b),
            _ => unreachable!(),
        };
        let mut safe = true;
        let mut previous = levels.first().expect("Empty list");
        for curr in levels.iter().skip(1) {
            if expected_order != previous.cmp(curr) || !range.contains(&previous.abs_diff(*curr)) {
                safe = false;
                break;
            }
            previous = curr;
        }
        if safe {
            res += 1;
        }
    }

    Some(res)
}

fn determine_ordering(inp: &Vec<impl Ord>) -> core::cmp::Ordering {
    // pick 3 subject pairs, compare them and return the ordering that appears at least twice
    let mut orderings = Vec::new();
    for (i, a) in inp.iter().enumerate() {
        for b in inp.iter().skip(i + 1) {
            orderings.push(a.cmp(b));
        }
    }

    if orderings
        .iter()
        .filter(|&&x| x == core::cmp::Ordering::Less)
        .count()
        > orderings
            .iter()
            .filter(|&&x| x == core::cmp::Ordering::Greater)
            .count()
    {
        core::cmp::Ordering::Less
    } else {
        core::cmp::Ordering::Greater
    }
}

fn lvl_safety_check(ord: core::cmp::Ordering, a: &usize, b: &usize) -> bool {
    let range = 1..=3;
    ord == a.cmp(b) && range.contains(&a.abs_diff(*b))
}

pub fn part_two(input: &str) -> Option<u32> {
    let list_of_levels = load_input(input);
    let mut res = 0;
    for levels in list_of_levels {
        let order = determine_ordering(&levels);
        let mut safe = true;
        for window in levels.windows(2) {
            if !lvl_safety_check(order, &window[0], &window[1]) {
                safe = false;
                break;
            }
        }
        if safe {
            // println!("Safe by default: {:?}", levels);
            res += 1;
            continue;
        }

        let mut overall_safe = false;
        for i in 0..levels.len() {
            let mut safe = true;
            for window in levels
                .iter()
                .enumerate()
                .filter_map(|(n, el)| (i != n).then(|| el))
                .collect::<Vec<_>>()
                .windows(2)
            {
                if !lvl_safety_check(order, window[0], window[1]) {
                    safe = false;
                    break;
                }
            }
            if safe {
                overall_safe = true;
                // println!("Safe by removing el on index {}: {:?}", i, levels);
                break;
            }
        }

        if overall_safe {
            res += 1;
            continue;
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
