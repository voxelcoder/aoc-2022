use std::collections::HashMap;
use std::ops::Not;
use std::str::Chars;

const LETTERS_LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const LETTERS_UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn get_priority(letter: &str) -> u8 {
    let letter = letter.chars().next().unwrap();
    let find_position = |set: [char; 26]| set.into_iter().position(|char| char == letter);

    1 + if let Some(position) = find_position(LETTERS_LOWERCASE) {
        position
    } else {
        find_position(LETTERS_UPPERCASE).unwrap() + LETTERS_LOWERCASE.len()
    } as u8
}

fn prepare_input(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .split('\n')
        .map(|line| line.split_at(line.len() / 2))
        .collect()
}

pub(crate) fn prepare_input_part_2(input: &str) -> Vec<(&str, &str, &str)> {
    input
        .trim()
        .split('\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| (group[0], group[1], group[2]))
        .collect()
}

fn part_1(compartments: Vec<(&str, &str)>) -> u32 {
    compartments.into_iter().fold(0, |acc, compartment| {
        acc + get_priority(&get_common_item(compartment).to_string()) as u32
    })
}

pub(crate) fn part_2(groups: Vec<(&str, &str, &str)>) -> u32 {
    groups.into_iter().fold(0, |acc, compartment| {
        acc + get_priority(&get_common_item_part_2(compartment).to_string()) as u32
    })
}

fn get_common_item(compartment: (&str, &str)) -> char {
    let has_char = |char: char| {
        let contains = |str: &str| str.chars().any(|c| c == char);

        if contains(compartment.0) && contains(compartment.1) {
            Some(char)
        } else {
            None
        }
    };

    for (first, second) in compartment.0.chars().zip(compartment.1.chars()) {
        if let Some(char) = has_char(first).or_else(|| has_char(second)) {
            return char;
        }
    }

    unreachable!();
}

fn get_common_item_part_2(group: (&str, &str, &str)) -> char {
    let has_char = |char: char| {
        let contains = |str: &str| str.chars().any(|c| c == char);

        if contains(group.0) && contains(group.1) && contains(group.2) {
            Some(char)
        } else {
            None
        }
    };

    for (first, (second, third)) in group.0.chars().zip(group.1.chars().zip(group.2.chars())) {
        if let Some(char) = has_char(first).or_else(|| has_char(second).or_else(|| has_char(third)))
        {
            return char;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::day3::{get_common_item, part_1, part_2, prepare_input, prepare_input_part_2};

    const EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    const FILE_INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(prepare_input(EXAMPLE_INPUT)), 157);
        assert_eq!(part_1(prepare_input(FILE_INPUT)), 7553);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(prepare_input_part_2(EXAMPLE_INPUT)), 70);
        assert_eq!(part_2(prepare_input_part_2(FILE_INPUT)), 2758);
    }

    #[test]
    fn test_prepare_input() {
        let result = prepare_input(EXAMPLE_INPUT);
        let expected = vec![
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            ("PmmdzqPrV", "vPwwTWBwg"),
            ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
            ("ttgJtRGJ", "QctTZtZT"),
            ("CrZsJsPPZsGz", "wwsLwLmpwMDw"),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_common_item() {
        assert_eq!(get_common_item(("vJrwpWtwJgWr", "hcsFMMfFFhFp")), 'p');
    }
}
