static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum RoundOutcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
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

    fn versus(&self, other: &Shape) -> RoundOutcome {
        use RoundOutcome::{Draw, Loss, Win};
        use Shape::{Paper, Rock, Scissors};

        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Loss,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Loss,
            (Scissors, Rock) => Loss,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }
}

impl From<char> for Shape {
    fn from(x: char) -> Self {
        match x {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("unexpected shape input: {}", x),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Round(Shape, Shape);

impl From<&str> for Round {
    fn from(line: &str) -> Self {
        let mut chars = line.chars();

        Round(chars.nth(0).unwrap().into(), chars.nth(1).unwrap().into())
    }
}

impl Round {
    fn score(&self) -> usize {
        self.1.score() + (self.1.versus(&self.0) as usize)
    }
}

fn main() {
    let rounds = parse_input(INPUT);

    println!("Part 1: {}", total_score(&rounds));
}

fn parse_input(input: &str) -> Vec<Round> {
    input.lines().map(Round::from).collect()
}

fn total_score(rounds: &[Round]) -> usize {
    rounds.iter().map(Round::score).sum()
}

#[cfg(test)]
mod test {
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
            vec![
                Round(Rock, Paper),
                Round(Paper, Rock),
                Round(Scissors, Scissors)
            ]
        );
    }

    #[test]
    fn test_total_score() {
        assert_eq!(total_score(&parse_input(SAMPLE_INPUT)), 15);
    }
}
