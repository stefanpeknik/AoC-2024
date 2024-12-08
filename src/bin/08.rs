use std::collections::HashSet;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Spot {
    x: usize,
    y: usize,
    antenna: Option<char>,
}

impl PartialEq for Spot {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.antenna == other.antenna
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    spots: Vec<Vec<Spot>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut spots = Vec::new();
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let spot = Spot {
                    x,
                    y,
                    antenna: match c {
                        '.' => None,
                        _ => Some(c),
                    },
                };
                row.push(spot);
            }
            spots.push(row);
        }

        let width = spots.first().unwrap_or(&Vec::new()).len();
        let height = spots.len();

        Self {
            width,
            height,
            spots,
        }
    }

    fn get_spots_with_antennas(&self) -> Vec<&Spot> {
        let mut antennas = Vec::new();
        for row in &self.spots {
            for spot in row {
                if spot.antenna.is_some() {
                    antennas.push(spot);
                }
            }
        }
        antennas
    }

    fn calc_antinode_location_part1(
        &self,
        antenna_a: &Spot,
        antenna_b: &Spot,
    ) -> Option<(usize, usize)> {
        let distance_x = antenna_b.x as isize - antenna_a.x as isize;
        let distance_y = antenna_b.y as isize - antenna_a.y as isize;

        let antinode_x = antenna_a.x as isize + distance_x * 2;
        let antinode_y = antenna_a.y as isize + distance_y * 2;

        if antinode_x < 0 || antinode_x >= self.width as isize {
            // Antinode is outside the map in the x direction
            return None;
        }
        if antinode_y < 0 || antinode_y >= self.height as isize {
            // Antinode is outside the map in the y direction
            return None;
        }

        Some((antinode_x as usize, antinode_y as usize))
    }

    fn locate_antinodes_part1(&self) -> Vec<(usize, usize)> {
        let mut antinodes = HashSet::<(usize, usize)>::new();
        let antennas = self.get_spots_with_antennas();

        for antenna in &antennas {
            let same_freq_antennas = antennas
                .iter()
                .filter(|a| a.antenna == antenna.antenna && a != &antenna)
                .collect::<Vec<_>>();

            for other_antenna in same_freq_antennas {
                if let Some(antinode) = self.calc_antinode_location_part1(antenna, other_antenna) {
                    antinodes.insert(antinode);
                }
            }
        }

        antinodes.into_iter().collect()
    }

    fn locate_antinodes_part2(&self) -> Vec<(usize, usize)> {
        let mut antinodes = HashSet::<(usize, usize)>::new();
        let antennas = self.get_spots_with_antennas();

        for antenna in &antennas {
            let same_freq_antennas = antennas
                .iter()
                .filter(|a| a.antenna == antenna.antenna && a != &antenna)
                .collect::<Vec<_>>();

            for other_antenna in same_freq_antennas {
                antinodes.extend(self.calc_antinode_location_part2(antenna, other_antenna));
            }
        }

        antinodes.into_iter().collect()
    }

    fn calc_antinode_location_part2(
        &self,
        antenna_a: &Spot,
        antenna_b: &Spot,
    ) -> Vec<(usize, usize)> {
        let distance_x = antenna_b.x as isize - antenna_a.x as isize;
        let distance_y = antenna_b.y as isize - antenna_a.y as isize;

        let mut antinodes = Vec::new();
        let mut antinode_x = antenna_a.x as isize + distance_x;
        let mut antinode_y = antenna_a.y as isize + distance_y;

        loop {
            if antinode_x < 0 || antinode_x >= self.width as isize {
                // Antinode is outside the map in the x direction
                break;
            }
            if antinode_y < 0 || antinode_y >= self.height as isize {
                // Antinode is outside the map in the y direction
                break;
            }
            antinodes.push((antinode_x as usize, antinode_y as usize));
            antinode_x += distance_x;
            antinode_y += distance_y;
        }
        antinodes
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::new(input);
    let antinodes = map.locate_antinodes_part1();
    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::new(input);
    let antinodes = map.locate_antinodes_part2();

    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
