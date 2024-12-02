advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn report_safe(report: Vec<isize>, tolerance: bool) -> bool {
    report
        .windows(2)
        .try_fold(None, |acc: Option<isize>, pair: &[isize]| {
            let diff = pair[1] - pair[0];

            if diff.abs() < 1 || diff.abs() > 3 {
                return None;
            }

            match acc {
                None => Some(Some(diff.signum())),
                Some(v) => {
                    if v.signum() == diff.signum() {
                        Some(acc)
                    } else {
                        None
                    }
                }
            }
        })
        .is_some()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .into_iter()
            .map(|report| report_safe(report, false) as usize)
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

    #[test]
    fn test_parse_input() {
        let result = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]
        );
    }

    #[rstest]
    #[case(vec![7, 6, 4, 2, 1], false, true)]
    #[case(vec![1, 2, 7, 8, 9], false, false)]
    #[case(vec![9, 7, 6, 2, 1], false, false)]
    #[case(vec![1, 3, 2, 4, 5], false, false)]
    #[case(vec![8, 6, 4, 4, 1], false, false)]
    #[case(vec![1, 3, 6, 7, 9], false, true)]
    fn test_report_safe(
        #[case] line: Vec<isize>,
        #[case] tolerance: bool,
        #[case] expected_safe: bool,
    ) {
        assert_eq!(report_safe(line, tolerance), expected_safe);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
