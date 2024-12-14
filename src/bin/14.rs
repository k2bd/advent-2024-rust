use std::ops::{Add, Mul, Rem, Sub};

use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Rem<Point> for Point {
    type Output = Self;

    fn rem(self, rhs: Point) -> Self::Output {
        Self(self.0.rem_euclid(rhs.0), self.1.rem_euclid(rhs.1))
    }
}

#[derive(Debug, PartialEq)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn from_line(line: &str) -> Self {
        let (pos_part, vel_part) = line.split_once(" ").unwrap();
        let (pos_x_part, pos_y_part) = pos_part
            .strip_prefix("p=")
            .unwrap()
            .split_once(",")
            .unwrap();
        let (vel_x_part, vel_y_part) = vel_part
            .strip_prefix("v=")
            .unwrap()
            .split_once(",")
            .unwrap();
        Self {
            position: Point(pos_x_part.parse().unwrap(), pos_y_part.parse().unwrap()),
            velocity: Point(vel_x_part.parse().unwrap(), vel_y_part.parse().unwrap()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct RoboRoom {
    robots: Vec<Robot>,
    size: Point,
}

impl RoboRoom {
    fn from_input(input: &str, width: usize, height: usize) -> Self {
        Self {
            robots: input.lines().map(Robot::from_line).collect(),
            size: Point(width as isize, height as isize),
        }
    }

    fn positions_at_timepoint(&self, time: isize) -> Vec<Point> {
        self.robots
            .iter()
            .map(|robot| (robot.position + robot.velocity * time) % self.size)
            .collect()
    }

    fn safety_factor_at_timepoint(&self, time: isize) -> usize {
        let positions = self.positions_at_timepoint(time);

        [
            ((0, self.size.0 / 2), (0, self.size.1 / 2)),
            ((0, self.size.0 / 2), ((self.size.1 / 2) + 1, self.size.1)),
            (((self.size.0 / 2) + 1, self.size.0), (0, self.size.1 / 2)),
            (
                ((self.size.0 / 2) + 1, self.size.0),
                ((self.size.1 / 2) + 1, self.size.1),
            ),
        ]
        .into_iter()
        .map(|((x_min, x_max), (y_min, y_max))| {
            positions
                .iter()
                .filter(|&&Point(x, y)| x >= x_min && x < x_max && y >= y_min && y < y_max)
                .count()
        })
        .product::<usize>()
    }

    /// I hate this question. I looked up what the tree looks like and then
    /// wrote some garbage that would probably match on it
    fn time_to_christmas(&self) -> usize {
        let answer = (0usize..)
            .take_while_inclusive(|&time| {
                let positions = self.positions_at_timepoint(time as isize);

                let squares = positions
                    .iter()
                    .filter(|&&p| {
                        [
                            Point(-1, -1),
                            Point(-1, 0),
                            Point(-1, 1),
                            Point(0, -1),
                            Point(0, 1),
                            Point(1, -1),
                            Point(1, 0),
                            Point(1, 1),
                        ]
                        .into_iter()
                        .all(|d| positions.contains(&(p + d)))
                    })
                    .count();

                squares < 5
            })
            .last()
            .unwrap();

        let positions = self.positions_at_timepoint(answer as isize);
        (0..self.size.1).for_each(|y| {
            println!(
                "|{}|",
                (0..self.size.0)
                    .map(|x| {
                        if positions.contains(&Point(x, y)) {
                            "X"
                        } else {
                            " "
                        }
                    })
                    .join("")
            )
        });

        answer
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(RoboRoom::from_input(input, 101, 103).safety_factor_at_timepoint(100))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(RoboRoom::from_input(input, 101, 103).time_to_christmas())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let robo_room =
            RoboRoom::from_input(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        assert_eq!(
            robo_room,
            RoboRoom {
                robots: vec![
                    Robot {
                        position: Point(0, 4),
                        velocity: Point(3, -3)
                    },
                    Robot {
                        position: Point(6, 3),
                        velocity: Point(-1, -3)
                    },
                    Robot {
                        position: Point(10, 3),
                        velocity: Point(-1, 2)
                    },
                    Robot {
                        position: Point(2, 0),
                        velocity: Point(2, -1)
                    },
                    Robot {
                        position: Point(0, 0),
                        velocity: Point(1, 3)
                    },
                    Robot {
                        position: Point(3, 0),
                        velocity: Point(-2, -2)
                    },
                    Robot {
                        position: Point(7, 6),
                        velocity: Point(-1, -3)
                    },
                    Robot {
                        position: Point(3, 0),
                        velocity: Point(-1, -2)
                    },
                    Robot {
                        position: Point(9, 3),
                        velocity: Point(2, 3)
                    },
                    Robot {
                        position: Point(7, 3),
                        velocity: Point(-1, 2)
                    },
                    Robot {
                        position: Point(2, 4),
                        velocity: Point(2, -3)
                    },
                    Robot {
                        position: Point(9, 5),
                        velocity: Point(-3, -3)
                    },
                ],
                size: Point(11, 7),
            }
        )
    }

    #[test]
    fn test_positions_at_timepoint() {
        let result =
            RoboRoom::from_input(&advent_of_code::template::read_file("examples", DAY), 11, 7)
                .positions_at_timepoint(100);
        assert_eq!(
            result,
            vec![
                Point(3, 5),
                Point(5, 4),
                Point(9, 0),
                Point(4, 5),
                Point(1, 6),
                Point(1, 3),
                Point(6, 0),
                Point(2, 3),
                Point(0, 2),
                Point(6, 0),
                Point(4, 5),
                Point(6, 6),
            ]
        );
    }

    #[test]
    fn test_safety_factor_at_timepoint() {
        let result =
            RoboRoom::from_input(&advent_of_code::template::read_file("examples", DAY), 11, 7)
                .safety_factor_at_timepoint(100);
        assert_eq!(result, 12);
    }
}
