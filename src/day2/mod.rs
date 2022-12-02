#[derive(Copy, Clone, PartialEq, Debug)]
enum Outcome {
    Loose,
    Draw,
    Win,
}

impl From<&str> for Outcome {
    /// "Anyway, the second column says how the round needs to end:
    /// - X means you need to lose,
    /// - Y means you need to end the round in a draw, and
    /// - Z means you need to win.
    /// Good luck!"
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

impl Shape {
    fn get_score(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

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

    fn get_desired_outcome_against(&self, opponent_hand: Shape, desired_outcome: Outcome) -> u8 {
        let our_hand = self.get_shape_for_desired_outcome(desired_outcome);
        our_hand.play_against(opponent_hand)
    }

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

impl From<&str> for Shape {
    /// "The first column is what your opponent is going to play:
    /// - A for Rock,
    /// - B for Paper, and
    /// - C for Scissors.
    /// The second column--"...
    /// The second column, you reason, must be what you should play in response:
    /// - X for Rock,
    /// - Y for Paper, and
    /// - Z for Scissors.
    fn from(string: &str) -> Self {
        match string {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

pub fn part_1(input: &str) -> i32 {
    prepare_input_for_part_1(input)
        .into_iter()
        .fold(0, |acc, curr| acc + curr.1.play_against(curr.0) as i32)
}

pub fn part_2(input: &str) -> i32 {
    prepare_input_for_part_2(input)
        .into_iter()
        .fold(0, |acc, curr| {
            acc + {
                let opponent_shape = curr.0;
                let our_hand = curr.0.get_shape_for_desired_outcome(curr.2);

                our_hand.play_against(opponent_shape)
            } as i32
        })
}

fn prepare_input_for_part_1(input: &str) -> Vec<(Shape, Shape)> {
    parse_rounds(input, |round| {
        let mut split = round.split_whitespace();
        (split.next().unwrap().into(), split.next().unwrap().into())
    })
}

fn prepare_input_for_part_2(input: &str) -> Vec<(Shape, Shape, Outcome)> {
    parse_rounds(input, |round| {
        let mut split = round.split_whitespace();
        let (opponent_shape, our_shape_str): (Shape, &str) =
            (split.next().unwrap().into(), split.next().unwrap());

        (opponent_shape, our_shape_str.into(), our_shape_str.into())
    })
}

fn parse_rounds<R, F>(input: &str, predicate: F) -> Vec<R>
where
    F: FnMut(&str) -> R,
{
    input.trim().split('\n').map(predicate).collect()
}

#[cfg(test)]
mod test {
    use crate::day2::{
        part_1, part_2, prepare_input_for_part_1, prepare_input_for_part_2, Outcome, Shape,
    };

    const EXAMPLE_INPUT: &str = "A Y\nB X\nC Z\n";
    const FILE_INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE_INPUT), 15);
        assert_eq!(part_1(FILE_INPUT), 14297);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE_INPUT), 12)
    }

    #[test]
    fn test_prepare_input_for_part_1() {
        let result = prepare_input_for_part_1(EXAMPLE_INPUT);
        let expected = vec![
            (Shape::Rock, Shape::Paper),
            (Shape::Paper, Shape::Rock),
            (Shape::Scissors, Shape::Scissors),
        ];

        assert_eq!(result.len(), 3);
        for (index, (result, expected)) in result.iter().zip(expected.iter()).enumerate() {
            assert_eq!(result, expected)
        }
    }

    #[test]
    fn test_prepare_input_for_part_2() {
        let result = prepare_input_for_part_2(EXAMPLE_INPUT);
        let expected = vec![
            (Shape::Rock, Shape::Paper, Outcome::Draw),
            (Shape::Paper, Shape::Rock, Outcome::Loose),
            (Shape::Scissors, Shape::Scissors, Outcome::Win),
        ];

        assert_eq!(result.len(), 3);
        for (index, (result, expected)) in result.iter().zip(expected.iter()).enumerate() {
            assert_eq!(result, expected)
        }
    }
}
