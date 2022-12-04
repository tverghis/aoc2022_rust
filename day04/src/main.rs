use std::ops::RangeInclusive;

static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Clone)]
struct SectionAssignment(RangeInclusive<usize>);

impl From<&str> for SectionAssignment {
    fn from(x: &str) -> Self {
        let [start, end]: [_; 2] = x
            .split('-')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        SectionAssignment(start..=end)
    }
}

impl SectionAssignment {
    fn has_any_overlap(&self, other: &Self) -> bool {
        let mut this_range_nums = self.0.clone();
        let other_range_nums = &other.0;

        this_range_nums.any(|n| other_range_nums.contains(&n))
    }

    fn contains(&self, other: &Self) -> bool {
        let this_range = &self.0;
        let other_range = &other.0;

        this_range.start() <= other_range.start() && this_range.end() >= other_range.end()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ElfAssignmentPair(SectionAssignment, SectionAssignment);

impl From<&str> for ElfAssignmentPair {
    fn from(line: &str) -> Self {
        let [first, second]: [_; 2] = line
            .split(',')
            .map(SectionAssignment::from)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        ElfAssignmentPair(first, second)
    }
}

impl ElfAssignmentPair {
    fn is_fully_overlapping(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn has_any_overlap(&self) -> bool {
        self.0.has_any_overlap(&self.1)
    }
}

fn main() {
    let assignments = INPUT
        .lines()
        .map(ElfAssignmentPair::from)
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        count_fully_overlapping_assignments(&assignments)
    );
    println!(
        "Part 2: {}",
        count_any_overlapping_assignments(&assignments)
    );
}

fn count_fully_overlapping_assignments(assignments: &[ElfAssignmentPair]) -> usize {
    assignments
        .iter()
        .filter(|assignment| assignment.is_fully_overlapping())
        .count()
}

fn count_any_overlapping_assignments(assignments: &[ElfAssignmentPair]) -> usize {
    assignments
        .iter()
        .filter(|assignment| assignment.has_any_overlap())
        .count()
}

#[cfg(test)]
mod test {
    use crate::{
        count_any_overlapping_assignments, count_fully_overlapping_assignments, ElfAssignmentPair,
        SectionAssignment,
    };

    static SAMPLE_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_parse_single_section_assignment() {
        let assignment_str = "20-30";
        assert_eq!(
            SectionAssignment::from(assignment_str),
            SectionAssignment(20..=30)
        );
    }

    #[test]
    fn test_contains() {
        assert_eq!(
            SectionAssignment::from("2-4").contains(&SectionAssignment::from("6-8")),
            false
        );
        assert_eq!(
            SectionAssignment::from("2-8").contains(&SectionAssignment::from("3-7")),
            true
        );
        assert_eq!(
            SectionAssignment::from("6-6").contains(&SectionAssignment::from("4-6")),
            false
        );
        assert_eq!(
            SectionAssignment::from("4-6").contains(&SectionAssignment::from("6-6")),
            true
        );
        assert_eq!(
            SectionAssignment::from("3-7").contains(&SectionAssignment::from("2-8")),
            false
        );
    }

    #[test]
    fn test_parse_assignment_pair() {
        let assignment_pair_str = "20-30,40-50";
        assert_eq!(
            ElfAssignmentPair::from(assignment_pair_str),
            ElfAssignmentPair(SectionAssignment(20..=30), SectionAssignment(40..=50))
        );
    }

    #[test]
    fn test_is_overlapping_assignment() {
        assert_eq!(
            SAMPLE_INPUT
                .lines()
                .map(ElfAssignmentPair::from)
                .map(|p| p.is_fully_overlapping())
                .collect::<Vec<_>>(),
            vec![false, false, false, true, true, false]
        )
    }

    #[test]
    fn test_count_fully_overlapping_assignments() {
        let assignments = SAMPLE_INPUT
            .lines()
            .map(ElfAssignmentPair::from)
            .collect::<Vec<_>>();
        assert_eq!(count_fully_overlapping_assignments(&assignments), 2);
    }

    #[test]
    fn test_has_any_overlap() {
        assert_eq!(
            SectionAssignment::from("2-4").has_any_overlap(&SectionAssignment::from("6-8")),
            false
        );
        assert_eq!(
            SectionAssignment::from("2-8").has_any_overlap(&SectionAssignment::from("3-7")),
            true
        );
        assert_eq!(
            SectionAssignment::from("6-6").has_any_overlap(&SectionAssignment::from("4-6")),
            true
        );
        assert_eq!(
            SectionAssignment::from("4-6").has_any_overlap(&SectionAssignment::from("6-6")),
            true
        );
        assert_eq!(
            SectionAssignment::from("3-7").has_any_overlap(&SectionAssignment::from("2-8")),
            true
        );
    }

    #[test]
    fn test_assignment_pair_has_any_overlap() {
        assert_eq!(
            SAMPLE_INPUT
                .lines()
                .map(ElfAssignmentPair::from)
                .map(|p| p.has_any_overlap())
                .collect::<Vec<_>>(),
            vec![false, false, true, true, true, true]
        );
    }

    #[test]
    fn test_count_any_overlapping_assignments() {
        let assignments = SAMPLE_INPUT
            .lines()
            .map(ElfAssignmentPair::from)
            .collect::<Vec<_>>();
        assert_eq!(count_any_overlapping_assignments(&assignments), 4);
    }
}
