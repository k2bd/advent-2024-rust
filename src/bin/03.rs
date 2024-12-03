use regex::Regex;

advent_of_code::solution!(3);

const VALID_EXPRESSION_REGEX: &str = r"mul\(\d{1,3},\d{1,3}\)";
const EXTENDED_EXPRESSION_REGEX: &str = r"|do\(\)|don't\(\)";

fn eval_mul_expression(expr: &str) -> u32 {
    let (left, right) = expr
        .strip_prefix("mul(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split_once(",")
        .unwrap();

    left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(VALID_EXPRESSION_REGEX).unwrap();

    Some(
        re.find_iter(input)
            .map(|c| c.as_str())
            .map(eval_mul_expression)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(&(VALID_EXPRESSION_REGEX.to_owned() + EXTENDED_EXPRESSION_REGEX)).unwrap();

    Some(
        re.find_iter(input)
            .fold((0, true), |(result, mul_enabled), expr| {
                match expr.as_str() {
                    "do()" => (result, true),
                    "don't()" => (result, false),
                    e => {
                        if mul_enabled {
                            (result + eval_mul_expression(e), mul_enabled)
                        } else {
                            (result, mul_enabled)
                        }
                    }
                }
            })
            .0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part_two(input);
        assert_eq!(result, Some(48));
    }
}
