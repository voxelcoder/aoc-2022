pub fn part_1(input: &str) -> u32 {
    let matrix = parse_input(input);

    matrix
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().fold(0, |acc, (x, tree)| {
                acc + u32::from(
                    is_on_forest_edge(&matrix, row, x, y)
                        || get_all_trees_around(&matrix, row, x, y)
                            .iter()
                            .any(|row| row.iter().all(|t| t < tree)),
                )
            })
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    let forest = parse_input(input);
    let mut highest_score = 0;

    for (y, row) in forest.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let [below, above, left, right] = &get_all_trees_around(&forest, row, x, y);

            let score = get_max_view(left, *tree, true)
                * get_max_view(right, *tree, false)
                * get_max_view(above, *tree, true)
                * get_max_view(below, *tree, false);

            if score > highest_score {
                highest_score = score
            }
        }
    }

    highest_score
}

fn get_max_view(row: &[Tree], current_tree: Tree, rev: bool) -> u32 {
    let mut count = 0;

    let mut inner = |other_tree: u32| {
        count += 1;
        other_tree < current_tree
    };

    if rev {
        for tree in row.iter().rev() {
            if !inner(*tree) {
                break;
            }
        }
    } else {
        for tree in row.iter() {
            if !inner(*tree) {
                break;
            }
        }
    }

    count
}

fn get_all_trees_around(
    matrix: &[Vec<Tree>],
    current_row: &[Tree],
    x: usize,
    y: usize,
) -> [Vec<Tree>; 4] {
    [
        get_all_trees_below(matrix, x, y),
        get_all_trees_above(matrix, x, y),
        get_all_trees_to_the_left(current_row, x).to_vec(),
        get_all_trees_to_the_right(current_row, x).to_vec(),
    ]
}

#[inline]
fn get_all_trees_above(matrix: &[Vec<Tree>], x: usize, y: usize) -> Vec<Tree> {
    matrix.iter().map(|row| row[x]).take(y).collect()
}

#[inline]
fn get_all_trees_below(matrix: &[Vec<Tree>], x: usize, y: usize) -> Vec<Tree> {
    matrix.iter().map(|row| row[x]).skip(y + 1).collect()
}

#[inline]
fn get_all_trees_to_the_left(current_row: &[Tree], x: usize) -> Vec<Tree> {
    current_row[..x].to_vec()
}

#[inline]
fn get_all_trees_to_the_right(current_row: &[Tree], x: usize) -> Vec<Tree> {
    current_row[x + 1..].to_vec()
}

#[inline]
fn is_on_forest_edge(matrix: &[Vec<Tree>], current_row: &[Tree], x: usize, y: usize) -> bool {
    x == 0 || x == current_row.len() - 1 || y == 0 || y == matrix.len() - 1
}

fn parse_input(input: &str) -> Vec<Vec<Tree>> {
    input
        .trim()
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|number| number.to_string().parse::<Tree>().unwrap())
                .collect()
        })
        .collect()
}

type Tree = u32;

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
