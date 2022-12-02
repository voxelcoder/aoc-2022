use std::cmp::Reverse;

pub fn part_1(blocks: &[Vec<i32>]) -> (usize, i32) {
    let max_element_and_index = blocks
        .iter()
        .map(|part| part.iter().sum::<i32>())
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b));

    max_element_and_index.unwrap()
}

pub fn part_2(blocks: &[Vec<i32>]) -> i32 {
    let mut summed_blocks = blocks
        .iter()
        .map(|part| part.iter().sum())
        .collect::<Vec<i32>>();

    summed_blocks.sort_by_key(|b| Reverse(*b));
    let calories = summed_blocks[..3].iter().sum::<i32>();

    calories
}

fn split_string(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .split('\n')
                .map(|part| part.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

#[cfg(test)]
mod test {
    use crate::day1::{part_1, part_2, split_string};

    const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n\n10000";

    #[test]
    fn test_part_1() {
        assert_eq!((3, 24000i32), part_1(&split_string(INPUT)));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(45000, part_2(&split_string(INPUT)));
    }
}
