static INPUT: &str = include_str!("../input.txt");

fn main() {
    let calories = calories_by_elf_sorted(INPUT);

    println!("Part 1: {}", max_calories_carried(&calories));
    println!("Part 2: {}", top_three_calories_sum(&calories));
}

fn calories_by_elf_sorted(input: &str) -> Vec<usize> {
    let mut calories_by_elf: Vec<usize> = input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<usize>().unwrap()).collect())
        .map(|items: Vec<usize>| items.iter().sum())
        .collect();

    calories_by_elf.sort_by(|cal1, cal2| cal2.cmp(cal1));

    calories_by_elf
}

fn max_calories_carried(calories_by_elf: &[usize]) -> usize {
    *calories_by_elf.iter().max().unwrap()
}

fn top_three_calories_sum(calories_by_elf: &[usize]) -> usize {
    calories_by_elf.iter().take(3).sum()
}

#[cfg(test)]
mod test {
    use crate::{calories_by_elf_sorted, max_calories_carried, top_three_calories_sum};

    static SAMPLE_INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_max_calories_carried() {
        let calories = calories_by_elf_sorted(SAMPLE_INPUT);

        assert_eq!(max_calories_carried(&calories), 24000);
    }

    #[test]
    fn test_top_three_calories_sum() {
        let calories = calories_by_elf_sorted(SAMPLE_INPUT);

        assert_eq!(top_three_calories_sum(&calories), 45000);
    }
}
