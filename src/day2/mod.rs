#[derive(Copy, Clone, PartialEq, Debug)]
enum Outcome {
    Loose,
    Draw,
    Win,
}

impl From<&str> for Outcome {
    fn from(string: &str) -> Self {
        match string {
            "X" => Outcome::Loose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unreachable!(),
        }
    }
}

impl From<Outcome> for u8 {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Loose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Shape {
    fn from(string: &str) -> Self {
        match string {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Shape {
    fn get_score(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    /// Simulates a "round" of rock, paper, scissors and returns the correct amount of points as
    /// defined in the problem.
    fn play_against(&self, opponent_shape: Self) -> u8 {
        let round_outcome = match self {
            Self::Rock => match opponent_shape {
                Self::Rock => Outcome::Draw,
                Self::Paper => Outcome::Loose,
                Self::Scissors => Outcome::Win,
            },
            Self::Paper => match opponent_shape {
                Self::Rock => Outcome::Win,
                Self::Paper => Outcome::Draw,
                Self::Scissors => Outcome::Loose,
            },
            Self::Scissors => match opponent_shape {
                Self::Rock => Outcome::Loose,
                Self::Paper => Outcome::Win,
                Self::Scissors => Outcome::Draw,
            },
        };

        self.get_score() + u8::from(round_outcome)
    }

    /// Using the opponents shape, determines what shape we have to play to trigger the desired
    /// outcome.
    fn get_shape_for_desired_outcome(&self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            Outcome::Draw => match self {
                Self::Rock => Self::Rock,
                Self::Paper => Self::Paper,
                Self::Scissors => Self::Scissors,
            },
            Outcome::Loose => match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
        }
    }
}

pub fn part_1(input: &str) -> i32 {
    parse_input::<Shape, Shape>(input)
        .into_iter()
        .fold(0, |acc, (opponent_shape, our_shape)| {
            acc + our_shape.play_against(opponent_shape) as i32
        })
}

pub fn part_2(input: &str) -> i32 {
    parse_input::<Shape, Outcome>(input).into_iter().fold(
        0,
        |acc, (opponent_shape, desired_outcome)| {
            acc + opponent_shape
                .get_shape_for_desired_outcome(desired_outcome)
                .play_against(opponent_shape) as i32
        },
    )
}

fn parse_input<O, U>(input: &str) -> Vec<(O, U)>
where
    O: for<'a> From<&'a str>,
    U: for<'a> From<&'a str>,
{
    input
        .trim()
        .split('\n')
        .map(|round| {
            let mut split = round.split_whitespace();
            let mut next = || split.next().unwrap();

            (next().into(), next().into())
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day2::{parse_input, part_1, part_2, Outcome, Shape};

    const EXAMPLE_INPUT: &str = "A Y\nB X\nC Z\n";
    const FILE_INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT), 15);
        assert_eq!(part_1(FILE_INPUT), 14297);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT), 12);
        assert_eq!(part_2(FILE_INPUT), 10498);
    }

    #[test]
    fn test_prepare_input_for_part_1() {
        let result = parse_input(EXAMPLE_INPUT);
        let expected = vec![
            (Shape::Rock, Shape::Paper),
            (Shape::Paper, Shape::Rock),
            (Shape::Scissors, Shape::Scissors),
        ];

        assert_eq!(result.len(), 3);
        for (result, expected) in result.iter().zip(expected.iter()) {
            assert_eq!(result, expected)
        }
    }

    #[test]
    fn test_prepare_input_for_part_2() {
        let result = parse_input(EXAMPLE_INPUT);
        let expected = vec![
            (Shape::Rock, Outcome::Draw),
            (Shape::Paper, Outcome::Loose),
            (Shape::Scissors, Outcome::Win),
        ];

        assert_eq!(result.len(), 3);
        for (result, expected) in result.iter().zip(expected.iter()) {
            assert_eq!(result, expected)
        }
    }
}
