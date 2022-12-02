static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum RoundOutcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl RoundOutcome {
    fn score(&self) -> usize {
        *self as usize
    }
}

impl From<char> for RoundOutcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("unexpected round outcome input {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn score(&self) -> usize {
        *self as usize
    }

    fn play_for_desired_outcome(&self, desired_outcome: RoundOutcome) -> Self {
        use RoundOutcome::{Draw, Loss, Win};
        use Shape::{Paper, Rock, Scissors};

        match (self, desired_outcome) {
            (Rock, Loss) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Win) => Paper,
            (Paper, Loss) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Loss) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Win) => Rock,
        }
    }
}

impl From<char> for Shape {
    fn from(x: char) -> Self {
        match x {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("unexpected shape input: {}", x),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Round(Shape, RoundOutcome);

impl From<&str> for Round {
    fn from(line: &str) -> Self {
        let mut chars = line.chars();

        Round(chars.nth(0).unwrap().into(), chars.nth(1).unwrap().into())
    }
}

impl Round {
    fn score(&self) -> usize {
        let my_shape = self.0.play_for_desired_outcome(self.1);
        my_shape.score() + self.1.score()
    }
}

fn main() {
    let rounds = parse_input(INPUT);

    println!("Total score: {}", total_score(&rounds));
}

fn parse_input(input: &str) -> Vec<Round> {
    input.lines().map(Round::from).collect()
}

fn total_score(rounds: &[Round]) -> usize {
    rounds.iter().map(Round::score).sum()
}

#[cfg(test)]
mod test {
    use crate::RoundOutcome::{Draw, Loss, Win};
    use crate::Shape::{Paper, Rock, Scissors};
    use crate::{parse_input, total_score, Round};

    static SAMPLE_INPUT: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn test_input_parsing() {
        let rounds: Vec<Round> = parse_input(SAMPLE_INPUT);
        assert_eq!(
            rounds,
            vec![Round(Rock, Draw), Round(Paper, Loss), Round(Scissors, Win)]
        );
    }

    #[test]
    fn test_total_score() {
        assert_eq!(total_score(&parse_input(SAMPLE_INPUT)), 12);
    }
}
