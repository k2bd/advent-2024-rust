use std::ops::{Add, Mul, Sub};

advent_of_code::solution!(13);

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

#[derive(Debug, PartialEq)]
struct ClawMachine {
    button_a: Point,
    button_b: Point,

    prize: Point,
}

impl ClawMachine {
    // fn minimum_tokens_brute(&self) -> Option<usize> {
    //     (0..)
    //         .take_while(|&a_presses| self.button_a * a_presses <= self.prize)
    //         .flat_map(|a_presses| {
    //             (0..)
    //                 .take_while(move |&b_presses| {
    //                     self.button_a * a_presses + self.button_b * b_presses <= self.prize
    //                 })
    //                 .filter(move |&b_presses| {
    //                     self.button_a * a_presses + self.button_b * b_presses == self.prize
    //                 })
    //                 .map(move |b_presses| (a_presses, b_presses))
    //         })
    //         .map(|(a_presses, b_presses)| (3 * a_presses + b_presses) as usize)
    //         .min()
    // }

    fn minimum_tokens_solve(&self) -> Option<usize> {
        let determinant = self.button_a.0 * self.button_b.1 - self.button_b.0 * self.button_a.1;
        let a_presses_numer = self.prize.0 * self.button_b.1 - self.prize.1 * self.button_b.0;
        let b_presses_numer = self.prize.1 * self.button_a.0 - self.prize.0 * self.button_a.1;

        if a_presses_numer % determinant != 0 || b_presses_numer % determinant != 0 {
            return None;
        }

        let a_presses = a_presses_numer / determinant;
        let b_presses = b_presses_numer / determinant;

        Some((3 * a_presses + b_presses) as usize)
    }
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut result = vec![];

    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        let a_part = lines.next().unwrap().strip_prefix("Button A: ").unwrap();
        let b_part = lines.next().unwrap().strip_prefix("Button B: ").unwrap();
        let goal_part = lines.next().unwrap().strip_prefix("Prize: ").unwrap();

        let (a_x_part, a_y_part) = a_part.split_once(", ").unwrap();
        let (b_x_part, b_y_part) = b_part.split_once(", ").unwrap();
        let (goal_x_part, goal_y_part) = goal_part.split_once(", ").unwrap();

        result.push(ClawMachine {
            button_a: Point(
                a_x_part.trim().strip_prefix("X").unwrap().parse().unwrap(),
                a_y_part.trim().strip_prefix("Y").unwrap().parse().unwrap(),
            ),
            button_b: Point(
                b_x_part.trim().strip_prefix("X").unwrap().parse().unwrap(),
                b_y_part.trim().strip_prefix("Y").unwrap().parse().unwrap(),
            ),
            prize: Point(
                goal_x_part
                    .trim()
                    .strip_prefix("X=")
                    .unwrap()
                    .parse()
                    .unwrap(),
                goal_y_part
                    .trim()
                    .strip_prefix("Y=")
                    .unwrap()
                    .parse()
                    .unwrap(),
            ),
        });

        if lines.peek().is_some() {
            lines.next();
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .into_iter()
            .filter_map(|machine| machine.minimum_tokens_solve())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .into_iter()
            .map(|machine| ClawMachine {
                prize: Point(10000000000000, 10000000000000) + machine.prize,
                ..machine
            })
            .filter_map(|machine| machine.minimum_tokens_solve())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            vec![
                ClawMachine {
                    button_a: Point(94, 34),
                    button_b: Point(22, 67),
                    prize: Point(8400, 5400)
                },
                ClawMachine {
                    button_a: Point(26, 66),
                    button_b: Point(67, 21),
                    prize: Point(12748, 12176)
                },
                ClawMachine {
                    button_a: Point(17, 86),
                    button_b: Point(84, 37),
                    prize: Point(7870, 6450)
                },
                ClawMachine {
                    button_a: Point(69, 23),
                    button_b: Point(27, 71),
                    prize: Point(18641, 10279)
                },
            ]
        );
    }

    #[test]
    fn test_day_13_part_one_from_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_day_13_part_two_from_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_some());
    }
}
