const LETTERS_LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const LETTERS_UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

trait ToVec<T>
where
    T: Sized,
{
    fn to_vec(self) -> Vec<T>;
}

impl<T> ToVec<T> for (T, T) {
    fn to_vec(self) -> Vec<T> {
        vec![self.0, self.1]
    }
}

impl<T> ToVec<T> for (T, (T, T)) {
    fn to_vec(self) -> Vec<T> {
        vec![self.0, self.1 .0, self.1 .1]
    }
}

fn get_priority(letter: &str) -> u8 {
    let letter = letter.chars().next().unwrap();
    let find_position = |set: [char; 26]| set.into_iter().position(|char| char == letter);

    1 + if let Some(position) = find_position(LETTERS_LOWERCASE) {
        position
    } else {
        find_position(LETTERS_UPPERCASE).unwrap() + LETTERS_LOWERCASE.len()
    } as u8
}

fn prepare_input_part_1(input: &str) -> Vec<[&str; 2]> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
            [first_compartment, second_compartment]
        })
        .collect()
}

fn prepare_input_part_2(input: &str) -> Vec<[&str; 3]> {
    input
        .trim()
        .split('\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| [group[0], group[1], group[2]])
        .collect()
}

fn compartment_has_char(compartment: &[&str], char: char) -> Option<char> {
    let contains = |str: &str| str.chars().any(|c| c == char);

    if compartment.iter().all(|comp| contains(comp)) {
        return Some(char);
    }

    None
}

fn get_common_item_part_1(compartment: [&str; 2]) -> Option<char> {
    compartment[0]
        .chars()
        .zip(compartment[1].chars())
        .find_map(|chars| {
            chars
                .to_vec()
                .into_iter()
                .find_map(|c| compartment_has_char(&compartment, c))
        })
}

fn get_common_item_part_2(group: [&str; 3]) -> Option<char> {
    group[0]
        .chars()
        .zip(group[1].chars().zip(group[2].chars()))
        .find_map(|chars| {
            chars
                .to_vec()
                .into_iter()
                .find_map(|c| compartment_has_char(&group, c))
        })
}

fn sum_priorities<T>(groups: Vec<T>, get_char_predicate: fn(T) -> Option<char>) -> u32 {
    groups.into_iter().fold(0, |acc, compartment| {
        acc + get_priority(&get_char_predicate(compartment).unwrap().to_string()) as u32
    })
}

pub fn part_1(groups: Vec<[&str; 2]>) -> u32 {
    sum_priorities(groups, get_common_item_part_1)
}

pub fn part_2(groups: Vec<[&str; 3]>) -> u32 {
    sum_priorities(groups, get_common_item_part_2)
}

#[cfg(test)]
mod test {
    use crate::day3::{
        get_common_item_part_1, part_1, part_2, prepare_input_part_1, prepare_input_part_2,
    };

    const EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    const FILE_INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(prepare_input_part_1(EXAMPLE_INPUT)), 157);
        assert_eq!(part_1(prepare_input_part_1(FILE_INPUT)), 7553);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(prepare_input_part_2(EXAMPLE_INPUT)), 70);
        assert_eq!(part_2(prepare_input_part_2(FILE_INPUT)), 2758);
    }

    #[test]
    fn test_prepare_input() {
        let result = prepare_input_part_1(EXAMPLE_INPUT);
        let expected = vec![
            ["vJrwpWtwJgWr", "hcsFMMfFFhFp"],
            ["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"],
            ["PmmdzqPrV", "vPwwTWBwg"],
            ["wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"],
            ["ttgJtRGJ", "QctTZtZT"],
            ["CrZsJsPPZsGz", "wwsLwLmpwMDw"],
        ];

        assert_eq!(result, expected);
    }
}
