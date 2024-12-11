advent_of_code::solution!(11);

#[derive(Debug, PartialEq)]
struct PlutoStone {
    value: usize,
}

impl PlutoStone {
    fn new(value: usize) -> Self {
        Self { value }
    }

    fn blink(&self) -> Vec<PlutoStone> {
        let value = self.value.to_string();

        if value == "0" {
            return vec![PlutoStone::new(1)];
        } else if value.len() % 2 == 0 {
            return vec![
                PlutoStone::new(value[..value.len() / 2].parse().unwrap()),
                PlutoStone::new(value[value.len() / 2..].parse().unwrap()),
            ];
        } else {
            return vec![PlutoStone::new(self.value * 2024)];
        }
    }
}

fn parse_input(input: &str) -> Vec<PlutoStone> {
    input
        .trim()
        .split(" ")
        .map(|v| PlutoStone::new(v.parse().unwrap()))
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        (0..25)
            .fold(parse_input(input), |acc, _| {
                acc.iter().map(|stone| stone.blink()).flatten().collect()
            })
            .len(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        (0..75)
            .fold(parse_input(input), |acc, _| {
                acc.iter().map(|stone| stone.blink()).flatten().collect()
            })
            .len(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(PlutoStone::new(0), vec![PlutoStone::new(1)])]
    #[case(PlutoStone::new(1), vec![PlutoStone::new(2024)])]
    #[case(PlutoStone::new(10), vec![PlutoStone::new(1), PlutoStone::new(0)])]
    #[case(PlutoStone::new(99), vec![PlutoStone::new(9), PlutoStone::new(9)])]
    #[case(PlutoStone::new(999), vec![PlutoStone::new(2021976)])]
    fn test_blink(#[case] stone: PlutoStone, #[case] expected: Vec<PlutoStone>) {
        assert_eq!(stone.blink(), expected)
    }

    #[test]
    fn test_day_11_part_one_from_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_day_11_part_two_from_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
