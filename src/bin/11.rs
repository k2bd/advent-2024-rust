use std::collections::HashMap;

advent_of_code::solution!(11);

fn blink(stone: usize) -> Vec<usize> {
    let value = stone.to_string();

    if value == "0" {
        vec![1]
    } else if value.len() % 2 == 0 {
        return vec![
            value[..value.len() / 2].parse().unwrap(),
            value[value.len() / 2..].parse().unwrap(),
        ];
    } else {
        return vec![stone * 2024];
    }
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(" ")
        .map(|v| v.parse().unwrap())
        .collect()
}

fn get_length_at_depth(
    stone: usize,
    depth: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if depth == 0 {
        return 1;
    }
    match cache.get(&(stone, depth)) {
        Some(&val) => val,
        None => {
            let stones = blink(stone);
            let result = stones
                .into_iter()
                .map(|s| get_length_at_depth(s, depth - 1, cache))
                .sum();
            cache.insert((stone, depth), result);
            result
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();

    Some(
        parse_input(input)
            .into_iter()
            .map(|stone| get_length_at_depth(stone, 25, &mut cache))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();

    Some(
        parse_input(input)
            .into_iter()
            .map(|stone| get_length_at_depth(stone, 75, &mut cache))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0, vec![1])]
    #[case(1, vec![2024])]
    #[case(10, vec![1, 0])]
    #[case(99, vec![9, 9])]
    #[case(999, vec![2021976])]
    fn test_blink(#[case] stone: usize, #[case] expected: Vec<usize>) {
        assert_eq!(blink(stone), expected)
    }

    #[test]
    fn test_day_11_part_one_from_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_day_11_part_two_from_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65_601_038_650_482));
    }
}
