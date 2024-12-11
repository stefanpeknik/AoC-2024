use itertools::Itertools;

advent_of_code::solution!(10);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn find_all_hiking_tracks(
    map: &Vec<Vec<u32>>,
    trailheads: &Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    let mut tracks = Vec::new();

    let possible_directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    for (x, y) in trailheads {
        let trail = trail_step(map, (*x, *y), &possible_directions);

        for t in trail.into_iter() {
            if t.len() > 0 {
                tracks.push(t);
            }
        }
    }

    tracks
}

fn is_in_bounds(map: &Vec<Vec<u32>>, x: isize, y: isize) -> bool {
    x >= 0 && y >= 0 && x < map[0].len() as isize && y < map.len() as isize
}

fn trail_step(
    map: &Vec<Vec<u32>>,
    current_location: (usize, usize),
    possible_directions: &Vec<(isize, isize)>,
) -> Vec<Vec<(usize, usize)>> {
    let (x, y) = current_location;

    let current_value = map
        .get(y)
        .and_then(|row| row.get(x))
        .expect("current location is out of bounds");

    if *current_value == 9 {
        return vec![vec![(x, y)]];
    }

    let mut trails = Vec::new();

    for (dx, dy) in possible_directions {
        let next_x = x as isize + dx;
        let next_y = y as isize + dy;

        if !is_in_bounds(map, next_x, next_y) {
            continue;
        }

        let next_value = map
            .get(next_y as usize)
            .and_then(|row| row.get(next_x as usize))
            .expect("next location is out of bounds");

        if *next_value != current_value + 1 {
            continue;
        }

        let next_trails = trail_step(map, (next_x as usize, next_y as usize), possible_directions);

        for trail in next_trails.into_iter() {
            if trail.len() > 0 {
                let mut new_trail = vec![(x, y)];
                new_trail.extend(trail);
                trails.push(new_trail);
            }
        }
    }

    trails
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);

    let trailhead_value = 0;
    let trailheads = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &spot)| {
                if spot == trailhead_value {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<(usize, usize)>>();

    let tracks = find_all_hiking_tracks(&map, &trailheads);

    // for each trailhead, count number of unique final locations
    let result: usize = trailheads
        .iter()
        .map(|(x, y)| {
            tracks
                .iter()
                .filter(|trail| trail.first() == Some(&(*x, *y)))
                .unique_by(|trail| trail.last())
                .count()
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);

    let trailhead_value = 0;
    let trailheads = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &spot)| {
                if spot == trailhead_value {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<(usize, usize)>>();

    let tracks = find_all_hiking_tracks(&map, &trailheads);

    Some(tracks.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
