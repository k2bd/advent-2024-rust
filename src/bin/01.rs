use std::iter::zip;

advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut vals = line
                .split_whitespace()
                .map(|val| val.parse::<u32>().unwrap());
            (vals.next().unwrap(), vals.next().unwrap())
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_lists(input);
    left.sort();
    right.sort();
    Some(zip(left, right).map(|(l, r)| (r).abs_diff(l)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_lists(input);

    Some(
        left.into_iter()
            .map(|l| l * (right.iter().filter(|&&r| r == l).count() as u32))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lists() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = parse_lists(input);

        assert_eq!(result, (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
