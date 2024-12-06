advent_of_code::solution!(6);

use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Orientation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Orientation::Up),
            "v" => Ok(Orientation::Down),
            "<" => Ok(Orientation::Left),
            ">" => Ok(Orientation::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    original_position: (usize, usize),
    position: (usize, usize),
    orientation: Orientation,
    exited: bool,
    visited_states: Vec<((usize, usize), Orientation)>,
    in_loop: bool,
}

impl Guard {
    fn rotate(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    ever_guarded: bool,
    occupied: bool,
}

#[derive(Debug, Clone)]
struct Lab {
    guard: Guard,
    map: Vec<Vec<Tile>>,
}

impl fmt::Display for Lab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if self.guard.position == (x, y) {
                    write!(
                        f,
                        "{}",
                        match self.guard.orientation {
                            Orientation::Up => "^",
                            Orientation::Down => "v",
                            Orientation::Left => "<",
                            Orientation::Right => ">",
                        }
                    )?;
                } else {
                    write!(f, "{}", if tile.occupied { "#" } else { "." })?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Lab {
    fn tick(&mut self) {
        let map_width = self.map[0].len() as isize;
        let map_height = self.map.len() as isize;

        let mut moved = false;
        while !self.guard.exited && !self.guard.in_loop && !moved {
            let (mut new_pos_x, mut new_pos_y) = (
                self.guard.position.0 as isize,
                self.guard.position.1 as isize,
            );
            match self.guard.orientation {
                Orientation::Up => new_pos_y -= 1,
                Orientation::Down => new_pos_y += 1,
                Orientation::Left => new_pos_x -= 1,
                Orientation::Right => new_pos_x += 1,
            }
            if new_pos_x < 0 || new_pos_x >= map_width || new_pos_y < 0 || new_pos_y >= map_height {
                self.guard.exited = true;
                moved = true;
                continue;
            }
            if let (Ok(new_pos_x), Ok(new_pos_y)) =
                (usize::try_from(new_pos_x), usize::try_from(new_pos_y))
            {
                if self.map[new_pos_y][new_pos_x].occupied {
                    self.guard.rotate();
                } else {
                    self.map[self.guard.position.1][self.guard.position.0].occupied = false;
                    self.guard.position = (new_pos_x, new_pos_y);
                    if !self.guard.in_loop {
                        self.guard.in_loop =
                            self.guard.visited_states.iter().any(|(pos, orientation)| {
                                *pos == self.guard.position
                                    && *orientation == self.guard.orientation
                            });
                        self.guard
                            .visited_states
                            .push((self.guard.position, self.guard.orientation.clone()));
                    }
                    self.map[self.guard.position.1][self.guard.position.0].ever_guarded = true;
                    moved = true;
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Lab {
    let mut guard = None;
    let mut map = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' | 'v' | '<' | '>' => {
                    guard = Some(Guard {
                        original_position: (x, y),
                        position: (x, y),
                        orientation: c.to_string().parse().unwrap(),
                        exited: false,
                        visited_states: Vec::new(),
                        in_loop: false,
                    });
                    row.push(Tile {
                        ever_guarded: true,
                        occupied: true,
                    });
                }
                '#' => row.push(Tile {
                    ever_guarded: false,
                    occupied: true,
                }),
                '.' => row.push(Tile {
                    ever_guarded: false,
                    occupied: false,
                }),
                _ => panic!("Invalid character in input"),
            }
        }
        map.push(row);
    }

    Lab {
        guard: guard.expect("No guard found in input"),
        map,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lab = parse_input(input);
    while !lab.guard.exited {
        lab.tick();
    }
    Some(
        lab.map
            .iter()
            .flatten()
            .filter(|tile| tile.ever_guarded)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lab = parse_input(input);
    let mut result = 0;
    for y in 0..lab.map.len() {
        for x in 0..lab.map[y].len() {
            if !lab.map[y][x].occupied {
                let mut new_lab = lab.clone();
                new_lab.map[y][x].occupied = true;
                while !new_lab.guard.exited && !new_lab.guard.in_loop {
                    new_lab.tick();
                }
                if new_lab.guard.in_loop {
                    result += 1;
                }
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
