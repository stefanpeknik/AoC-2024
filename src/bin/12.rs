use std::usize;

advent_of_code::solution!(12);

const UP: (i8, i8) = (0, -1);
const DOWN: (i8, i8) = (0, 1);
const LEFT: (i8, i8) = (-1, 0);
const RIGHT: (i8, i8) = (1, 0);
const DIRECTIONS: [(i8, i8); 4] = [UP, DOWN, LEFT, RIGHT];

fn checked_move(
    pos: (usize, usize),
    dir: (i8, i8),
    boundary: ((usize, usize), (usize, usize)),
) -> Option<(usize, usize)> {
    fn apply_offset(coord: usize, offset: i8) -> Option<usize> {
        if offset < 0 {
            coord.checked_sub(offset.unsigned_abs() as usize)
        } else {
            coord.checked_add(offset as usize)
        }
    }

    let new_x = apply_offset(pos.0, dir.0)?;
    let new_y = apply_offset(pos.1, dir.1)?;

    let ((min_x, max_x), (min_y, max_y)) = boundary;
    if new_x < min_x || new_x >= max_x || new_y < min_y || new_y >= max_y {
        return None;
    }

    Some((new_x, new_y))
}

#[derive(Debug, Clone, Copy)]
struct Plant {
    x: usize,
    y: usize,
    plant: char,
}

impl PartialEq for Plant {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.plant == other.plant
    }
}

struct GardenPlot {
    plant: char,
    plants: Vec<Plant>,
}

impl PartialEq for GardenPlot {
    fn eq(&self, other: &Self) -> bool {
        self.plant == other.plant && self.plants.iter().all(|plant| other.plants.contains(plant))
    }
}

impl GardenPlot {
    fn new(plant: char, plants: Vec<Plant>) -> Self {
        GardenPlot { plant, plants }
    }

    fn calc_price(&self) -> usize {
        self.calc_area() * self.calc_perimeter()
    }

    fn calc_price_discounted(&self) -> usize {
        self.calc_area() * self.calc_sides()
    }

    fn calc_area(&self) -> usize {
        self.plants.len()
    }

    fn locate_plant_neighbour_in_that_dir(&self, plant: &Plant, dir: (i8, i8)) -> Option<&Plant> {
        checked_move((plant.x, plant.y), dir, ((0, usize::MAX), (0, usize::MAX))).and_then(
            |(neighbour_x, neighbour_y)| {
                self.plants
                    .iter()
                    .find(|p| p.x == neighbour_x && p.y == neighbour_y)
            },
        )
    }

    fn calc_perimeter(&self) -> usize {
        self.plants.iter().fold(0, |perimeter, plant| {
            perimeter
                + DIRECTIONS
                    .iter()
                    .filter(|&&dir| {
                        self.locate_plant_neighbour_in_that_dir(plant, dir)
                            .is_none()
                    })
                    .count()
        })
    }

    fn calc_sides(&self) -> usize {
        let mut sides = 0;

        let bounding_box = (
            (
                self.plants.iter().map(|p| p.x).min().unwrap(),
                self.plants.iter().map(|p| p.x).max().unwrap(),
            ),
            (
                self.plants.iter().map(|p| p.y).min().unwrap(),
                self.plants.iter().map(|p| p.y).max().unwrap(),
            ),
        );

        // check up and down sides
        for y in bounding_box.1 .0..=bounding_box.1 .1 {
            let y_lvl_plants = self.plants.iter().filter(|p| p.y == y);
            for plant in y_lvl_plants {
                if self.locate_plant_neighbour_in_that_dir(plant, UP).is_none()
                    && (self.locate_plant_neighbour_in_that_dir(plant, LEFT).map_or(
                        true,
                        |left_neighbour| {
                            self.locate_plant_neighbour_in_that_dir(left_neighbour, UP)
                                .is_some()
                        },
                    ))
                {
                    sides += 1;
                }
                if self
                    .locate_plant_neighbour_in_that_dir(plant, DOWN)
                    .is_none()
                    && (self.locate_plant_neighbour_in_that_dir(plant, LEFT).map_or(
                        true,
                        |left_neighbour| {
                            self.locate_plant_neighbour_in_that_dir(left_neighbour, DOWN)
                                .is_some()
                        },
                    ))
                {
                    sides += 1;
                }
            }
        }

        // check left and right sides
        for x in bounding_box.0 .0..=bounding_box.0 .1 {
            let x_lvl_plants = self.plants.iter().filter(|p| p.x == x);
            for plant in x_lvl_plants {
                if self
                    .locate_plant_neighbour_in_that_dir(plant, LEFT)
                    .is_none()
                    && (self.locate_plant_neighbour_in_that_dir(plant, UP).map_or(
                        true,
                        |up_neighbour| {
                            self.locate_plant_neighbour_in_that_dir(up_neighbour, LEFT)
                                .is_some()
                        },
                    ))
                {
                    sides += 1;
                }
                if self
                    .locate_plant_neighbour_in_that_dir(plant, RIGHT)
                    .is_none()
                    && (self.locate_plant_neighbour_in_that_dir(plant, UP).map_or(
                        true,
                        |up_neighbour| {
                            self.locate_plant_neighbour_in_that_dir(up_neighbour, RIGHT)
                                .is_some()
                        },
                    ))
                {
                    sides += 1;
                }
            }
        }

        sides
    }
}

struct Farm {
    map: Vec<Vec<Plant>>,
}

impl Farm {
    fn new(map: Vec<Vec<char>>) -> Self {
        let mut farm = Farm { map: Vec::new() };
        for (y, row) in map.iter().enumerate() {
            let mut plants = Vec::new();
            for (x, plant) in row.iter().enumerate() {
                plants.push(Plant {
                    x,
                    y,
                    plant: *plant,
                });
            }
            farm.map.push(plants);
        }
        farm
    }

    fn list_garden_plots(&self) -> Vec<GardenPlot> {
        fn find_garden_plot_plant_is_in(
            map: &Vec<Vec<Plant>>,
            plant: &Plant,
        ) -> Option<GardenPlot> {
            if map.first().is_none() {
                return None;
            }
            let mut plants = Vec::new();
            let mut visited = vec![vec![false; map[0].len()]; map.len()];
            let mut stack = vec![plant];

            let boundary = ((0, map[0].len()), (0, map.len()));

            while let Some(plant) = stack.pop() {
                if visited[plant.y][plant.x] {
                    continue;
                }
                visited[plant.y][plant.x] = true;
                plants.push(*plant);

                for (dx, dy) in &DIRECTIONS {
                    if let Some((neighbour_x, neighbour_y)) =
                        checked_move((plant.x, plant.y), (*dx, *dy), boundary)
                    {
                        if map[neighbour_y][neighbour_x].plant == plant.plant {
                            stack.push(&map[neighbour_y][neighbour_x]);
                        }
                    }
                }
            }

            Some(GardenPlot::new(plant.plant, plants))
        }
        let mut garden_plots = Vec::new();
        for row in self.map.iter() {
            for plant in row.iter() {
                if let Some(garden_plot) = find_garden_plot_plant_is_in(&self.map, plant) {
                    if !garden_plots.contains(&garden_plot) {
                        garden_plots.push(garden_plot);
                    }
                }
            }
        }

        garden_plots
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let farm = Farm::new(map);
    let garden_plots = farm.list_garden_plots();

    let full_price = garden_plots.iter().map(|plot| plot.calc_price()).sum();

    Some(full_price)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let farm = Farm::new(map);
    let garden_plots = farm.list_garden_plots();

    let full_price = garden_plots
        .iter()
        .map(|plot| plot.calc_price_discounted())
        .sum();

    Some(full_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
