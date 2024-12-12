use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, PartialEq)]
struct GardenPlot {
    points: HashSet<(isize, isize)>,
}

impl GardenPlot {
    fn perimeter(&self) -> usize {
        self.points
            .iter()
            .map(|&(row, col)| {
                NEIGHBORS
                    .iter()
                    .filter(|(dr, dc)| !self.points.contains(&(row + dr, col + dc)))
                    .count()
            })
            .sum()
    }

    fn area(&self) -> usize {
        self.points.len()
    }
}

fn parse_input(input: &str) -> Vec<GardenPlot> {
    let mut all_points: HashMap<(isize, isize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .map(move |(col, char)| ((row as isize, col as isize), char))
        })
        .collect();

    let mut result = Vec::<GardenPlot>::new();

    while !all_points.is_empty() {
        let key = *all_points.keys().next().unwrap();
        let plot_char = all_points.remove(&key).unwrap();

        let mut plot_points = HashSet::from([key]);

        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut to_visit: Vec<(isize, isize)> = NEIGHBORS
            .iter()
            .map(|&(dr, dc)| (key.0 + dr, key.1 + dc))
            .collect();

        while let Some(neighbor_point) = to_visit.pop() {
            visited.insert(neighbor_point);

            if let Some(neighbor_char) = all_points.get(&neighbor_point) {
                if plot_char == *neighbor_char {
                    all_points.remove(&neighbor_point);
                    plot_points.insert(neighbor_point);
                    NEIGHBORS
                        .iter()
                        .map(|&(dr, dc)| (neighbor_point.0 + dr, neighbor_point.1 + dc))
                        .for_each(|p| {
                            if !visited.contains(&p) {
                                to_visit.push(p);
                            }
                        });
                }
            }
        }

        result.push(GardenPlot {
            points: plot_points,
        });
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .into_iter()
            .map(|plot| plot.area() * plot.perimeter())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(GardenPlot{ points: HashSet::from([(3, 3)]) }, 1)]
    #[case(GardenPlot{ points: HashSet::from([(3, 3), (3, 4), (4, 3), (4, 4)]) }, 4)]
    #[case(GardenPlot{ points: HashSet::from([(0, 0), (0, 1), (0, 2)]) }, 3)]
    #[case(GardenPlot{ points: HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3)]) }, 4)]
    #[case(GardenPlot{ points: HashSet::from([(0, 0), (1, 0), (1, 1), (2, 1)]) }, 4)]
    fn test_area(#[case] plot: GardenPlot, #[case] expected_area: usize) {
        assert_eq!(plot.area(), expected_area);
    }

    #[rstest]
    #[case(GardenPlot{ points: HashSet::from([(3, 3)]) }, 4)]
    #[case(GardenPlot{ points: HashSet::from([(3, 3), (3, 4), (4, 3), (4, 4)]) }, 8)]
    #[case(GardenPlot{ points: HashSet::from([(0, 0), (0, 1), (0, 2)]) }, 8)]
    #[case(GardenPlot{ points: HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3)]) }, 10)]
    #[case(GardenPlot{ points: HashSet::from([(0, 0), (1, 0), (1, 1), (2, 1)]) }, 10)]
    fn test_perimeter(#[case] plot: GardenPlot, #[case] expected_perimeter: usize) {
        assert_eq!(plot.perimeter(), expected_perimeter);
    }

    #[test]
    fn test_day_12_part_one_from_example_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_day_12_part_one_from_example_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_day_12_part_one_from_example_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_day_12_part_two_from_example_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }

    #[test]
    fn test_day_12_part_two_from_example_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, None);
    }

    #[test]
    fn test_day_12_part_two_from_example_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, None);
    }

    #[test]
    fn test_day_12_part_two_from_example_5() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, None);
    }
}
