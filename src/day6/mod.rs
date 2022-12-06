use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    execute(input, 4).unwrap()
}

pub fn part_2(input: &str) -> usize {
    execute(input, 14).unwrap()
}

fn execute(input: &str, unique_count: usize) -> Option<usize> {
    let mut stream_window: Vec<char> = vec![input.chars().next().unwrap(); unique_count];

    for (i, char) in input.chars().enumerate() {
        stream_window.push(char);
        stream_window.drain(..1);

        let current_set: HashSet<char> = stream_window.iter().cloned().collect::<HashSet<char>>();
        if current_set.len() == unique_count {
            return Some(i + 1);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::day6::{part_1, part_2};

    const FILE_INPUT: &str = include_str!("input.txt");
    const EXAMPLE_INPUTS: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUTS[0]), 7);
        assert_eq!(part_1(EXAMPLE_INPUTS[1]), 5);
        assert_eq!(part_1(EXAMPLE_INPUTS[2]), 6);
        assert_eq!(part_1(EXAMPLE_INPUTS[3]), 10);
        assert_eq!(part_1(EXAMPLE_INPUTS[4]), 11);

        assert_eq!(part_1(FILE_INPUT), 1965);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUTS[0]), 19);
        assert_eq!(part_2(EXAMPLE_INPUTS[1]), 23);
        assert_eq!(part_2(EXAMPLE_INPUTS[2]), 23);
        assert_eq!(part_2(EXAMPLE_INPUTS[3]), 29);
        assert_eq!(part_2(EXAMPLE_INPUTS[4]), 26);

        assert_eq!(part_2(FILE_INPUT), 2773);
    }
}
