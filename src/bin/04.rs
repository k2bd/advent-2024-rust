advent_of_code::solution!(4);

const SEARCH_DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone, Default)]
struct WordSearcherState {
    position: (usize, usize),
    direction_index: usize,
}

struct WordSearcher {
    // Puzzle info
    puzzle: Vec<Vec<char>>,
    target_word: String,

    // Search state
    current_search: WordSearcherState,
}

impl WordSearcher {
    fn new(puzzle: Vec<Vec<char>>, target_word: &str) -> Self {
        Self {
            puzzle,
            target_word: target_word.to_owned(),
            current_search: WordSearcherState::default(),
        }
    }

    fn from_input(input: &str, target_word: &str) -> Self {
        Self::new(
            input.lines().map(|line| line.chars().collect()).collect(),
            target_word,
        )
    }

    /// Get, without mutation, what the next search state should be
    fn next_position(&self) -> Option<WordSearcherState> {
        let position = self.current_search.position;
        let direction_index = self.current_search.direction_index;

        if direction_index == (SEARCH_DIRECTIONS.len() - 1) {
            if position.0 < self.puzzle.len() && position.1 < &self.puzzle[position.0].len() - 1 {
                Some(WordSearcherState {
                    position: (position.0, position.1 + 1),
                    direction_index: 0,
                })
            } else if position.0 >= self.puzzle.len() {
                None
            } else {
                Some(WordSearcherState {
                    position: (position.0 + 1, 0),
                    direction_index: 0,
                })
            }
        } else {
            Some(WordSearcherState {
                position,
                direction_index: direction_index + 1,
            })
        }
    }

    fn match_found(&self) -> bool {
        (0..self.target_word.len()).all(|i| {
            let target_space = (
                self.current_search.position.0 as isize
                    + i as isize * SEARCH_DIRECTIONS[self.current_search.direction_index].0,
                self.current_search.position.1 as isize
                    + i as isize * SEARCH_DIRECTIONS[self.current_search.direction_index].1,
            );
            if target_space.0 >= 0
                && target_space.0 < (self.puzzle.len() as isize)
                && target_space.1 >= 0
                && target_space.1 < (self.puzzle[target_space.0 as usize].len() as isize)
            {
                self.puzzle[target_space.0 as usize][target_space.1 as usize]
                    == self.target_word.chars().nth(i).unwrap()
            } else {
                false
            }
        })
    }
}

impl Iterator for WordSearcher {
    type Item = WordSearcherState;

    /// Iterate over positions/directions of *unique matches* in the word search
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(next) = self.next_position() {
                self.current_search = next;
                if self.match_found() {
                    return Some(self.current_search.clone());
                }
            } else {
                return None;
            }
        }
    }
}

struct XMasSearcher {
    // Puzzle info
    puzzle: Vec<Vec<char>>,

    // Search state
    position: (usize, usize),
}

impl XMasSearcher {
    fn new(puzzle: Vec<Vec<char>>) -> Self {
        Self {
            puzzle,
            position: (0, 0),
        }
    }

    fn from_input(input: &str) -> Self {
        Self::new(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn next_position(&self) -> Option<(usize, usize)> {
        if self.position.0 < self.puzzle.len() - 2
            && self.position.1 < &self.puzzle[self.position.0].len() - 3
        {
            Some((self.position.0, self.position.1 + 1))
        } else if self.position.0 < self.puzzle.len() - 3 {
            Some((self.position.0 + 1, 0))
        } else {
            None
        }
    }

    fn match_found(&self) -> bool {
        let vecs = [[(0, 0), (1, 1), (2, 2)], [(2, 0), (1, 1), (0, 2)]];

        vecs.into_iter().all(|vec| {
            let word = vec
                .map(|p| self.puzzle[self.position.0 + p.0][self.position.1 + p.1].to_owned())
                .iter()
                .collect::<String>();
            word == "MAS" || word == "SAM"
        })
    }
}

impl Iterator for XMasSearcher {
    type Item = (usize, usize);

    /// Iterate over left corners of matches
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(next) = self.next_position() {
                self.position = next;
                if self.match_found() {
                    return Some(self.position);
                }
            } else {
                return None;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(WordSearcher::from_input(input, "XMAS").count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(XMasSearcher::from_input(input).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_4_part_one_from_example() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_day_4_part_two_from_example() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
