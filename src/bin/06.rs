use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

advent_of_code::solution!(6);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn point(&self) -> Point {
        match self {
            Direction::NORTH => Point(-1, 0),
            Direction::EAST => Point(0, 1),
            Direction::SOUTH => Point(1, 0),
            Direction::WEST => Point(0, -1),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.point()
    }
}

struct Guard {
    position: Point,
    facing: Direction,
}

impl Guard {
    fn forward(&self) -> Point {
        self.position + self.facing
    }

    fn turn_right(&mut self) {
        self.facing = self.facing.turn_right();
    }

    fn step(&mut self) {
        self.position = self.forward();
    }
}

struct Field {
    guard: Guard,
    obstacles: HashSet<Point>,
    size: Point,
}

impl Field {
    fn step(&mut self) {
        if self.obstacles.contains(&self.guard.forward()) {
            self.guard.turn_right();
            return self.step();
        }
        self.guard.step();
    }

    fn all_steps(&mut self) -> HashSet<Point> {
        let mut visited = HashSet::new();

        while self.guard.position.0 >= 0
            && self.guard.position.0 < self.size.0
            && self.guard.position.1 >= 0
            && self.guard.position.1 < self.size.1
        {
            visited.insert(self.guard.position);
            self.step();
        }

        visited
    }
}

fn parse_input(input: &str) -> Field {
    let obstacles = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.char_indices()
                .filter(|&(_, val)| val == '#')
                .map(move |(col, val)| Point(row as isize, col as isize))
        })
        .flatten()
        .collect::<HashSet<_>>();

    let size = Point(
        input.lines().count() as isize,
        input.lines().next().unwrap().len() as isize,
    );

    let guard = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.char_indices()
                .filter(|&(_, val)| ['v', '^', '<', '>'].contains(&val))
                .map(move |(col, val)| Guard {
                    position: Point(row as isize, col as isize),
                    facing: match val {
                        '^' => Direction::NORTH,
                        '>' => Direction::EAST,
                        'v' => Direction::SOUTH,
                        '<' => Direction::WEST,
                        _ => {
                            panic!("Lazy exit for val")
                        }
                    },
                })
        })
        .flatten()
        .next()
        .unwrap();

    Field {
        obstacles,
        guard,
        size,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(parse_input(input).all_steps().len())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_6_part_one_from_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_day_6_part_two_from_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
