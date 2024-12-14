advent_of_code::solution!(9);

#[derive(Debug, PartialEq)]
struct FileSystem {
    blocks: Vec<Option<usize>>,
}

impl FileSystem {
    fn from_dense_rep(input: &str) -> Self {
        let mut current_id = 0;
        let mut blocks: Vec<Option<usize>> = Vec::new();
        let mut pushing_file = true;
        input.trim().chars().for_each(|c| {
            let quantity = c.to_digit(10).unwrap();
            (0..quantity).for_each(|_| {
                blocks.push(if pushing_file { Some(current_id) } else { None });
            });
            if pushing_file {
                current_id += 1
            };
            pushing_file = !pushing_file;
        });

        Self { blocks }
    }

    fn fragged(&self) -> Vec<usize> {
        let mut left_index = 0;
        let mut right_index = self.blocks.len() - 1;

        let mut result = Vec::new();

        while left_index <= right_index {
            if let Some(value) = self.blocks[left_index] {
                result.push(value);
            } else {
                while self.blocks[right_index].is_none() {
                    right_index -= 1;
                }
                result.push(self.blocks[right_index].unwrap());
                right_index -= 1;
            }

            left_index += 1;
        }

        result
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        FileSystem::from_dense_rep(input)
            .fragged()
            .into_iter()
            .enumerate()
            .map(|(index, value)| index * value)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_dense_rep() {
        let file_system = FileSystem::from_dense_rep("2333133121414131402");
        assert_eq!(
            file_system,
            FileSystem {
                blocks: vec![
                    Some(0),
                    Some(0),
                    None,
                    None,
                    None,
                    Some(1),
                    Some(1),
                    Some(1),
                    None,
                    None,
                    None,
                    Some(2),
                    None,
                    None,
                    None,
                    Some(3),
                    Some(3),
                    Some(3),
                    None,
                    Some(4),
                    Some(4),
                    None,
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    None,
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    None,
                    Some(7),
                    Some(7),
                    Some(7),
                    None,
                    Some(8),
                    Some(8),
                    Some(8),
                    Some(8),
                    Some(9),
                    Some(9),
                ]
            }
        )
    }

    #[test]
    fn test_fragged() {
        let file_system = FileSystem::from_dense_rep("2333133121414131402");
        assert_eq!(
            file_system.fragged(),
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6
            ],
        )
    }

    #[test]
    fn test_day_9_part_one_from_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_day_9_part_two_from_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
