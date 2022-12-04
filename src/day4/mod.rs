enum Intersection {
    Overlap,
    Includes,
}

type Assignment = (i32, i32);
type AssignmentGroup = Vec<Assignment>;

fn prepare_input(input: &str) -> Vec<AssignmentGroup> {
    input
        .trim()
        .split('\n')
        .map(|assignment_pair| {
            assignment_pair
                .split(',')
                .map(|assignment| {
                    let parts = assignment
                        .split('-')
                        .map(|id| id.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();

                    (parts[0], parts[1])
                })
                .collect()
        })
        .collect()
}

fn sort_assignment_group(group: &AssignmentGroup) -> AssignmentGroup {
    let mut sorted = group.clone();
    sorted.sort_by(|(id_1_1, id_1_2), (id_2_1, id_2_2)| {
        (id_2_1 - id_2_2).abs().cmp(&(id_1_1 - id_1_2).abs())
    });

    sorted
}

fn check_for_intersection(
    assignments: Vec<AssignmentGroup>,
    intersection_type: Intersection,
) -> usize {
    assignments
        .into_iter()
        .filter(|assignment_group| {
            let assignment_group = sort_assignment_group(assignment_group);
            let [(first_start, first_end), (second_start, second_end)]: &[_; 2] =
                assignment_group.as_slice().try_into().unwrap();

            let first_set = *first_start..=*first_end;
            let mut second_set = *second_start..=*second_end;

            let contains_id = |id: i32| first_set.contains(&id);
            match intersection_type {
                Intersection::Overlap => second_set.any(contains_id),
                Intersection::Includes => second_set.all(contains_id),
            }
        })
        .count()
}

pub fn part_1(input: &str) -> usize {
    check_for_intersection(prepare_input(input), Intersection::Includes)
}

pub fn part_2(input: &str) -> usize {
    check_for_intersection(prepare_input(input), Intersection::Overlap)
}

#[cfg(test)]
mod test {
    use crate::day4::{part_1, part_2, prepare_input};

    const FILE_INPUT: &str = include_str!("input.txt");
    const EXAMPLE_INPUT_PART_1: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
    const EXAMPLE_INPUT_PART_2: &str = "5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT_PART_1), 2);
        assert_eq!(part_1(FILE_INPUT), 595);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT_PART_2), 4);
        assert_eq!(part_2(FILE_INPUT), 952);
    }

    #[test]
    fn test_prepare_input() {
        let result = prepare_input(EXAMPLE_INPUT_PART_1);
        let expected = vec![
            vec![(2, 4), (6, 8)],
            vec![(2, 3), (4, 5)],
            vec![(5, 7), (7, 9)],
            vec![(2, 8), (3, 7)],
            vec![(6, 6), (4, 6)],
            vec![(2, 6), (4, 8)],
        ];

        assert_eq!(expected.len(), 6);
        for (result, expected) in result.iter().zip(expected.iter()) {
            assert_eq!(result, expected)
        }
    }
}
