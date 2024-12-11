use itertools::{Combinations, Itertools};
use std::{
    any::Any,
    collections::HashSet,
    mem::discriminant,
    ops::{Add, Sub},
};

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Antenna {
    location: Point,
    frequency: char,
}

impl Antenna {
    fn new(row: isize, col: isize, frequency: char) -> Self {
        Self {
            location: Point(row, col),
            frequency,
        }
    }

    fn antinode_locations(&self, other: &Self) -> HashSet<Point> {
        if self.frequency != other.frequency {
            return HashSet::new();
        }

        let diff = self.location - other.location;

        HashSet::from([self.location + diff, other.location - diff])
    }
}

#[derive(Debug, PartialEq)]
struct CityField {
    antennas: HashSet<Antenna>,
    dimensions: (isize, isize),
}

impl CityField {
    fn all_antinode_locations(&self) -> HashSet<Point> {
        self.antennas
            .iter()
            .combinations(2)
            .map(|c| c[0].antinode_locations(c[1]))
            .flatten()
            .filter(|&p| p.0 >= 0 && p.0 < self.dimensions.0 && p.1 >= 0 && p.1 < self.dimensions.1)
            .collect()
    }
}

fn parse_input(input: &str) -> CityField {
    let dimensions = (
        input.lines().count() as isize,
        input.lines().next().unwrap().len() as isize,
    );
    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|&(_, val)| val != '.')
                .map(move |(col, frequency)| Antenna::new(row as isize, col as isize, frequency))
        })
        .collect();

    CityField {
        antennas,
        dimensions,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(parse_input(input).all_antinode_locations().len())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let city = parse_input(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(
            city,
            CityField {
                antennas: HashSet::from([
                    Antenna::new(1, 8, '0'),
                    Antenna::new(2, 5, '0'),
                    Antenna::new(3, 7, '0'),
                    Antenna::new(4, 4, '0'),
                    Antenna::new(5, 6, 'A'),
                    Antenna::new(8, 8, 'A'),
                    Antenna::new(9, 9, 'A'),
                ]),
                dimensions: (12, 12)
            }
        )
    }

    #[test]
    fn test_day_8_part_one_from_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_day_8_part_two_from_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
