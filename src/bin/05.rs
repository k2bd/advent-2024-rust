advent_of_code::solution!(5);

type OrderingRule = (usize, usize);

fn parse_input(input: &str) -> (Vec<OrderingRule>, Vec<Vec<usize>>) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut split_lines = lines.split(|line| line.is_empty());
    let rule_lines = split_lines.next().unwrap();
    let update_lines = split_lines.next().unwrap();

    let rule_part = rule_lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split("|").map(|val| val.parse::<usize>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect::<Vec<_>>();
    let update_part = update_lines
        .iter()
        .map(|line| {
            line.split(",")
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rule_part, update_part)
}

fn is_in_order(pages: &[usize], rules: &[OrderingRule]) -> bool {
    rules.iter().all(|&(l, r)| {
        if let (Some(l_index), Some(r_index)) = (
            pages.iter().position(|&v| v == l),
            pages.iter().position(|&v| v == r),
        ) {
            l_index < r_index
        } else {
            true
        }
    })
}

fn reorder(pages: &[usize], rules: &[OrderingRule]) -> Vec<usize> {
    let mut remaining_rules = rules
        .iter()
        .filter(|&(left, right)| pages.contains(left) && pages.contains(right))
        .collect::<Vec<_>>();
    let mut ordered_pages: Vec<usize> = Vec::new();
    let mut remaining_pages = pages.to_vec();

    while let Some(new_page_index) = remaining_pages
        .iter()
        .position(|&page| remaining_rules.iter().all(|&&rule| rule.1 != page))
    {
        let page = remaining_pages.remove(new_page_index);
        ordered_pages.push(page);
        remaining_rules.retain(|rule| rule.0 != page);
    }

    ordered_pages
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, updates) = parse_input(input);

    Some(
        updates
            .into_iter()
            .filter(|pages| is_in_order(pages, &rules))
            .map(|pages| pages[pages.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rules, updates) = parse_input(input);

    Some(
        updates
            .into_iter()
            .filter(|pages| !is_in_order(pages, &rules))
            .map(|pages| reorder(&pages, &rules))
            .map(|pages| pages[pages.len() / 2])
            .sum(),
    )
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
            (
                vec![
                    (47, 53),
                    (97, 13),
                    (97, 61),
                    (97, 47),
                    (75, 29),
                    (61, 13),
                    (75, 53),
                    (29, 13),
                    (97, 29),
                    (53, 29),
                    (61, 53),
                    (97, 53),
                    (61, 29),
                    (47, 13),
                    (75, 47),
                    (97, 75),
                    (47, 61),
                    (75, 61),
                    (47, 29),
                    (75, 13),
                    (53, 13),
                ],
                vec![
                    vec![75, 47, 61, 53, 29],
                    vec![97, 61, 53, 29, 13],
                    vec![75, 29, 13],
                    vec![75, 97, 47, 61, 53],
                    vec![61, 13, 29],
                    vec![97, 13, 75, 29, 47],
                ]
            )
        )
    }

    #[test]
    fn test_day_5_part_one_from_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[rstest]
    #[case(vec![], vec![])]
    #[case(vec![75,97,47,61,53], vec![97,75,47,61,53])]
    #[case(vec![61,13,29], vec![61,29,13])]
    #[case(vec![97,13,75,29,47], vec![ 97,75,47,29,13])]
    fn test_reorder(#[case] pages: Vec<usize>, #[case] expected: Vec<usize>) {
        let rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        assert_eq!(reorder(&pages, &rules), expected);
    }

    #[test]
    fn test_day_5_part_two_from_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
