use std::ops::Index;

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    match input
        .split("\n\n")
        .map(|block| block.lines().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .as_slice()
    {
        [orderings, updates] => (
            orderings
                .iter()
                .map(
                    |line| match line.split("|").collect::<Vec<&str>>().as_slice() {
                        [page_number_x, page_number_y] => (
                            page_number_x.parse().unwrap(),
                            page_number_y.parse().unwrap(),
                        ),
                        _ => panic!("Invalid input"),
                    },
                )
                .collect(),
            updates
                .iter()
                .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
                .collect(),
        ),
        _ => panic!("Invalid input"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (orderings, updates) = parse_input(input);
    let mut res = 0;
    for update in updates {
        let mut is_correct = true;
        for (update_page_num_index, update_page_num) in update.iter().enumerate() {
            let rules = orderings
                .iter()
                .filter(|(_, page_number_y)| page_number_y == update_page_num);
            for (page_number_x, _) in rules {
                if let Some(must_precede_index) = update.iter().position(|x| x == page_number_x) {
                    if must_precede_index >= update_page_num_index {
                        is_correct = false;
                        break;
                    }
                }
            }
            if !is_correct {
                break;
            }
        }
        if is_correct {
            res += update
                .get(update.len() / 2)
                .expect("Was not able to get the middle element");
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (orderings, updates) = parse_input(input);
    let incorrect_updates = updates
        .iter()
        .filter(|update| {
            let mut is_correct = true;
            for (update_page_num_index, update_page_num) in update.iter().enumerate() {
                let rules = orderings
                    .iter()
                    .filter(|(_, page_number_y)| page_number_y == update_page_num);
                for (page_number_x, _) in rules {
                    if let Some(must_precede_index) = update.iter().position(|x| x == page_number_x)
                    {
                        if must_precede_index >= update_page_num_index {
                            is_correct = false;
                            break;
                        }
                    }
                }
                if !is_correct {
                    break;
                }
            }
            !is_correct
        })
        .collect::<Vec<&Vec<u32>>>();

    let fixed_updates = incorrect_updates
        .iter()
        .map(|update| {
            let mut fixed_update = (*update).clone();
            loop {
                let mut clean_run = true;
                for (page_num_x, page_num_y) in orderings.iter() {
                    if let Some(page_num_x_index) =
                        fixed_update.iter().position(|x| x == page_num_x)
                    {
                        if let Some(page_num_y_index) =
                            fixed_update.iter().position(|y| y == page_num_y)
                        {
                            if page_num_x_index >= page_num_y_index {
                                let removed = fixed_update.remove(page_num_x_index);
                                fixed_update.insert(0, removed);
                                clean_run = false;
                            }
                        }
                    }
                }
                if clean_run {
                    break;
                }
            }
            fixed_update
        })
        .collect::<Vec<Vec<u32>>>();

    // println!("{:?}", incorrect_updates);
    // println!("{:?}", fixed_updates);

    let mut res = 0;
    for update in fixed_updates {
        res += update
            .get(update.len() / 2)
            .expect("Was not able to get the middle element");
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
