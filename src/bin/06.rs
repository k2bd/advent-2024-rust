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

#[derive(PartialEq, Clone, Eq, Hash, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn point(&self) -> Point {
        match self {
            Direction::North => Point(-1, 0),
            Direction::East => Point(0, 1),
            Direction::South => Point(1, 0),
            Direction::West => Point(0, -1),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + rhs.point()
    }
}

#[derive(PartialEq, Clone, Eq, Hash)]
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

#[derive(PartialEq, Clone)]
struct Field {
    guard: Guard,
    obstacles: HashSet<Point>,
    size: Point,
}

impl Field {
    fn step(&mut self) {
        if self.obstacles.contains(&self.guard.forward()) {
            self.guard.turn_right();
        } else {
            self.guard.step();
        }
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

    fn add_obstacle(&mut self, at: Point) -> bool {
        self.obstacles.insert(at)
    }

    fn is_loop(&mut self) -> bool {
        let mut past_guards = HashSet::from([self.guard.clone()]);

        while self.guard.position.0 >= 0
            && self.guard.position.0 < self.size.0
            && self.guard.position.1 >= 0
            && self.guard.position.1 < self.size.1
        {
            self.step();

            if past_guards.contains(&self.guard) {
                return true;
            }

            past_guards.insert(self.guard.clone());
        }

        false
    }
}

fn parse_input(input: &str) -> Field {
    let obstacles = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|&(_, val)| val == '#')
                .map(move |(col, _)| Point(row as isize, col as isize))
        })
        .collect::<HashSet<_>>();

    let size = Point(
        input.lines().count() as isize,
        input.lines().next().unwrap().len() as isize,
    );

    let guard = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|&(_, val)| ['v', '^', '<', '>'].contains(&val))
                .map(move |(col, val)| Guard {
                    position: Point(row as isize, col as isize),
                    facing: match val {
                        '^' => Direction::North,
                        '>' => Direction::East,
                        'v' => Direction::South,
                        '<' => Direction::West,
                        _ => {
                            panic!("Lazy exit for val")
                        }
                    },
                })
        })
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
    let field = parse_input(input);

    Some(
        (0..field.size.0)
            .flat_map(|obs_row| (0..field.size.1).map(move |obs_col| Point(obs_row, obs_col)))
            .filter(|&p| {
                if field.obstacles.contains(&p) || field.guard.position == p {
                    false
                } else {
                    let mut new_field = field.clone();
                    new_field.add_obstacle(p);
                    new_field.is_loop()
                }
            })
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_day_6_part_one_from_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[rstest]
    #[case(Point(6, 3), true)]
    #[case(Point(5, 4), false)]
    fn test_is_loop(#[case] added_obstacle: Point, #[case] should_loop: bool) {
        let mut field = parse_input(&advent_of_code::template::read_file("examples", DAY));
        field.add_obstacle(added_obstacle);

        assert_eq!(field.is_loop(), should_loop);
    }

    #[test]
    fn test_day_6_part_two_from_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
