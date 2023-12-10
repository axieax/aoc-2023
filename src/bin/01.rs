use lazy_static::lazy_static;
use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let first = line
                    .chars()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                let last = line
                    .chars()
                    .rev()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                first * 10 + last
            })
            .sum(),
    )
}

lazy_static! {
    static ref NUM_VALUE_MAP: HashMap<&'static str, u32> = {
        HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ])
    };
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut index_num: HashMap<usize, &str> = HashMap::new();
                for index in 0..line.len() {
                    for &num in NUM_VALUE_MAP.keys() {
                        if line[index..].starts_with(num) {
                            index_num.insert(index, num);
                        }
                    }
                }

                let mut pairs: Vec<_> = index_num.iter().collect();
                // * dereferences the index value, closure takes ownership of the index by moving it out of the tuple
                // pairs.sort_by_key(|(_, index)| *index);
                // & borrows the index value, closure takes a reference to the index
                pairs.sort_by_key(|&(index, _)| index);

                let first = *NUM_VALUE_MAP.get(pairs.first().unwrap().1).unwrap();
                let last = *NUM_VALUE_MAP.get(pairs.last().unwrap().1).unwrap();
                first * 10 + last
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
