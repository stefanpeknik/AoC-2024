use std::fmt;
use std::iter::repeat;

advent_of_code::solution!(9);

#[derive(Clone, Debug)]
struct Block {
    file: Option<usize>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.file {
            Some(file) => write!(f, "{}", file),
            None => write!(f, "."),
        }
    }
}

#[derive(Clone, Debug)]
struct DiskMap {
    blocks: Vec<Block>,
}

impl fmt::Display for DiskMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in &self.blocks {
            write!(f, "{}", block)?;
        }
        Ok(())
    }
}

impl DiskMap {
    fn get_blocks_of_file(&self, file: usize) -> Vec<(usize, &Block)> {
        self.blocks
            .iter()
            .enumerate()
            .filter(|(_, block)| block.file == Some(file))
            .collect()
    }
}

impl DiskMap {
    fn compress_per_block(&self) -> DiskMap {
        let mut compressed = self.clone();
        // while there is any block that has file set to None between first and last file block
        loop {
            if let (Some((last_file_index, _)), Some((first_file_index, _))) = (
                compressed
                    .blocks
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, block)| block.file.is_some()),
                compressed
                    .blocks
                    .iter()
                    .enumerate()
                    .find(|(_, block)| block.file.is_some()),
            ) {
                if last_file_index > first_file_index {
                    if let Some(index_where_to_move) = compressed
                        .blocks
                        .iter()
                        .position(|block| block.file.is_none())
                    {
                        if index_where_to_move < last_file_index {
                            compressed.blocks.swap(index_where_to_move, last_file_index);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        compressed
    }

    fn compress_per_file(&self) -> DiskMap {
        let mut compressed = self.clone();

        for file_id in (0..=self
            .blocks
            .iter()
            .filter_map(|block| block.file)
            .max()
            .unwrap())
            .rev()
        {
            let file_blocks = self.get_blocks_of_file(file_id);

            if let Some(start_empty_index) = compressed.blocks
                [0..file_blocks.first().expect("File with no blocks").0]
                .windows(file_blocks.len())
                .position(|window| window.iter().all(|block| block.file.is_none()))
            {
                for (file_block_index, empty_block_index) in
                    (start_empty_index..start_empty_index + file_blocks.len()).enumerate()
                {
                    compressed
                        .blocks
                        .swap(empty_block_index, file_blocks[file_block_index].0);
                }
            }
        }

        compressed
    }
}

impl From<Vec<usize>> for DiskMap {
    fn from(input: Vec<usize>) -> Self {
        let mut blocks = Vec::new();
        let mut file_id = 0;
        let mut alternating = (0..).map(|n| n % 2 == 0);
        for size in input.iter() {
            match alternating.next() {
                Some(true) => {
                    blocks.extend(
                        repeat(Block {
                            file: Some(file_id),
                        })
                        .take(*size),
                    );
                    file_id += 1;
                }
                Some(false) => {
                    blocks.extend(repeat(Block { file: None }).take(*size));
                }
                None => panic!("Alternating iterator exhausted"),
            }
        }
        Self { blocks }
    }
}

fn parse_input(input: &str) -> DiskMap {
    DiskMap::from(
        input
            .chars()
            .enumerate()
            .map(|(i, c)| {
                c.to_digit(10)
                    .expect(format!("Invalid digit: {} at {}", c, i).as_str())
                    as usize
            })
            .collect::<Vec<usize>>(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let disk_map = parse_input(input);
    let compressed = disk_map.compress_per_block();
    let result = compressed
        .blocks
        .iter()
        .enumerate()
        .fold(0, |acc, (i, block)| match block.file {
            Some(file) => acc + i * file,
            None => acc,
        });

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let disk_map = parse_input(input);
    let compressed = disk_map.compress_per_file();
    let result = compressed
        .blocks
        .iter()
        .enumerate()
        .fold(0, |acc, (i, block)| match block.file {
            Some(file) => acc + i * file,
            None => acc,
        });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        println!("{:?}", part_one("123456789012"));
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
