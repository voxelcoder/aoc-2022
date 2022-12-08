pub fn part_1(input: &str) -> u32 {
    let matrix = parse_input(input);

    matrix
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().fold(0, |acc, (x, tree)| {
                acc + u32::from(
                    is_on_matrix_edge(&matrix, row, x, y)
                        || get_all_trees_around(&matrix, row, x, y)
                            .iter()
                            .any(|x| x.iter().all(|t| t < tree)),
                )
            })
        })
        .sum::<u32>()
}

pub fn part_2(input: &str) -> u32 {
    let matrix = parse_input(input);

    let mut scenic_score_matrix: Vec<Vec<u32>> = vec![vec![0; matrix[0].len()]; matrix.len()];
    for (y, row) in matrix.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let [below, above, left, right] = &get_all_trees_around(&matrix, row, x, y);

            scenic_score_matrix[x][y] = [
                get_max_view(left, *tree, true),
                get_max_view(right, *tree, false),
                get_max_view(above, *tree, true),
                get_max_view(below, *tree, false),
            ]
            .iter()
            .product();
        }
    }

    *scenic_score_matrix
        .iter()
        .map(|row| row.iter().max().unwrap_or(&0))
        .max()
        .unwrap_or(&0)
}

fn get_max_view(row: &[u8], current_tree: u8, rev: bool) -> u32 {
    let mut count = 0;
    let mut row = row.to_vec();

    if rev {
        row.reverse();
    }

    for other_tree in row {
        count += 1;
        if other_tree >= current_tree {
            break;
        }
    }

    count
}

fn get_all_trees_around(
    matrix: &[Vec<u8>],
    current_row: &[u8],
    x: usize,
    y: usize,
) -> [Vec<u8>; 4] {
    [
        get_all_trees_below(matrix, x, y),
        get_all_trees_above(matrix, x, y),
        get_all_trees_to_the_left(current_row, x).to_vec(),
        get_all_trees_to_the_right(current_row, x).to_vec(),
    ]
}

#[inline]
fn get_all_trees_above(matrix: &[Vec<u8>], x: usize, y: usize) -> Vec<u8> {
    matrix.iter().map(|row| row[x]).take(y).collect()
}

#[inline]
fn get_all_trees_below(matrix: &[Vec<u8>], x: usize, y: usize) -> Vec<u8> {
    matrix.iter().map(|row| row[x]).skip(y + 1).collect()
}

#[inline]
fn get_all_trees_to_the_left(current_row: &[u8], x: usize) -> Vec<u8> {
    current_row[..x].to_vec()
}

#[inline]
fn get_all_trees_to_the_right(current_row: &[u8], x: usize) -> Vec<u8> {
    current_row[x + 1..].to_vec()
}

#[inline]
fn is_on_matrix_edge(matrix: &[Vec<u8>], current_row: &[u8], x: usize, y: usize) -> bool {
    x == 0 || x == current_row.len() - 1 || y == 0 || y == matrix.len() - 1
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|number| number.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use crate::day8::{part_1, part_2};

    const FILE_INPUT: &str = include_str!("input.txt");
    const EXAMPLE_INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT), 21);
        assert_eq!(part_1(FILE_INPUT), 1851);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT), 8);
        assert_eq!(part_2(FILE_INPUT), 574080);
    }
}
