use std::{collections::HashSet, str::Chars};

static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
struct Rucksack<'a>(&'a str, &'a str);

impl<'a> From<&'a str> for Rucksack<'a> {
    fn from(line: &'a str) -> Self {
        let len = line.len();
        Rucksack(&line[0..len / 2], &line[len / 2..])
    }
}

impl<'a> Rucksack<'a> {
    fn find_duplicate_item(&self) -> char {
        let Rucksack(left, right) = *self;

        left.chars().find(|&l| right.contains(l)).unwrap()
    }
}

fn main() {
    let rucksacks = parse_rucksacks(INPUT);

    println!("Part 1: {}", sum_of_priorities(&rucksacks));
    println!("Part 1: {}", sum_of_priorities_for_badges(&rucksacks));
}

fn parse_rucksacks(input: &str) -> Vec<Rucksack<'_>> {
    input.lines().map(Rucksack::from).collect()
}

fn priority_for_item_type(item_type: char) -> usize {
    let diff = if item_type.is_ascii_lowercase() {
        96
    } else {
        38
    };

    (item_type as usize) - diff
}

fn sum_of_priorities(rucksacks: &[Rucksack]) -> usize {
    rucksacks
        .iter()
        .map(Rucksack::find_duplicate_item)
        .map(priority_for_item_type)
        .sum()
}

fn sum_of_priorities_for_badges(rucksacks: &[Rucksack]) -> usize {
    rucksacks
        .iter()
        .map(|&Rucksack(l, r)| l.to_owned() + r)
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| chunk.iter().map(|s| s.chars().collect::<HashSet<_>>()))
        .map(|mut chunk| {
            chunk
                .next()
                .unwrap()
                .intersection(&chunk.next().unwrap())
                .map(|c| *c)
                .collect::<HashSet<_>>()
                .intersection(&chunk.next().unwrap())
                .nth(0)
                .unwrap()
                .clone()
        })
        .map(priority_for_item_type)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{parse_rucksacks, sum_of_priorities, sum_of_priorities_for_badges, Rucksack};

    static SAMPLE_INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_parse_rucksacks() {
        assert_eq!(
            parse_rucksacks(SAMPLE_INPUT),
            vec![
                Rucksack("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
                Rucksack("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
                Rucksack("PmmdzqPrV", "vPwwTWBwg"),
                Rucksack("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
                Rucksack("ttgJtRGJ", "QctTZtZT"),
                Rucksack("CrZsJsPPZsGz", "wwsLwLmpwMDw"),
            ]
        );
    }

    #[test]
    fn test_find_duplicate_items() {
        let rucksacks = parse_rucksacks(SAMPLE_INPUT);

        assert_eq!(
            rucksacks
                .iter()
                .map(Rucksack::find_duplicate_item)
                .collect::<Vec<char>>(),
            vec!['p', 'L', 'P', 'v', 't', 's']
        );
    }

    #[test]
    fn test_sum_of_priorities() {
        let rucksacks = parse_rucksacks(SAMPLE_INPUT);

        assert_eq!(sum_of_priorities(&rucksacks), 157);
    }

    #[test]
    fn test_sum_of_priorities_for_badges() {
        let rucksacks = parse_rucksacks(SAMPLE_INPUT);

        assert_eq!(sum_of_priorities_for_badges(&rucksacks), 70);
    }
}
