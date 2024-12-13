advent_of_code::solution!(7);

#[derive(Debug, PartialEq)]
struct Calibration {
    goal: usize,
    instructions: Vec<usize>,
}

impl Calibration {
    fn can_resolve(&self, concat_allowed: bool) -> bool {
        if self.instructions.len() == 1 {
            return self.goal == self.instructions[0];
        }
        if self.instructions[0] > self.goal {
            return false;
        }

        let mut add_instr = vec![self.instructions[0] + self.instructions[1]];
        add_instr.extend_from_slice(&self.instructions[2..]);

        let mut mul_instr = vec![self.instructions[0] * self.instructions[1]];
        mul_instr.extend_from_slice(&self.instructions[2..]);

        let mut result = Calibration {
            goal: self.goal,
            instructions: add_instr,
        }
        .can_resolve(concat_allowed)
            || Calibration {
                goal: self.goal,
                instructions: mul_instr,
            }
            .can_resolve(concat_allowed);

        if concat_allowed {
            let mut concat_str = self.instructions[0].to_string();
            concat_str.push_str(&self.instructions[1].to_string());
            let mut ca_instr = vec![concat_str.parse().unwrap()];
            ca_instr.extend_from_slice(&self.instructions[2..]);

            result |= Calibration {
                goal: self.goal,
                instructions: ca_instr,
            }
            .can_resolve(concat_allowed);
        }

        result
    }
}

fn parse_input(input: &str) -> Vec<Calibration> {
    input
        .lines()
        .map(|line| {
            let (goal_str, rest) = line.split_once(":").unwrap();

            Calibration {
                goal: goal_str.parse().unwrap(),
                instructions: rest
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .into_iter()
            .filter(|calibration| calibration.can_resolve(false))
            .map(|calibration| calibration.goal)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .into_iter()
            .filter(|calibration| calibration.can_resolve(true))
            .map(|calibration| calibration.goal)
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
                Calibration {
                    goal: 190,
                    instructions: vec![10, 19]
                },
                Calibration {
                    goal: 3267,
                    instructions: vec![81, 40, 27]
                },
                Calibration {
                    goal: 83,
                    instructions: vec![17, 5]
                },
                Calibration {
                    goal: 156,
                    instructions: vec![15, 6]
                },
                Calibration {
                    goal: 7290,
                    instructions: vec![6, 8, 6, 15]
                },
                Calibration {
                    goal: 161011,
                    instructions: vec![16, 10, 13]
                },
                Calibration {
                    goal: 192,
                    instructions: vec![17, 8, 14]
                },
                Calibration {
                    goal: 21037,
                    instructions: vec![9, 7, 18, 13]
                },
                Calibration {
                    goal: 292,
                    instructions: vec![11, 6, 16, 20]
                },
            ]
        );
    }

    #[test]
    fn test_day_7_part_one_from_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_day_7_part_two_from_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
