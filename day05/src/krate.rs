use std::collections::VecDeque;

use crate::procedure::Procedure;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Crate(char);

impl Crate {
    fn try_from(x: &str) -> Option<Self> {
        if x.trim().is_empty() {
            return None;
        }

        Some(Crate(x.chars().nth(1).unwrap()))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CrateStack(VecDeque<Crate>);

impl CrateStack {
    fn new() -> Self {
        Self(VecDeque::new())
    }

    fn insert(&mut self, c: Crate) {
        self.0.push_back(c);
    }

    fn push(&mut self, c: Crate) {
        self.0.push_front(c);
    }

    fn pop(&mut self) -> Option<Crate> {
        self.0.pop_front()
    }

    pub fn peek_inner(&self) -> char {
        self.0.front().unwrap().0
    }
}

impl IntoIterator for CrateStack {
    type Item = Crate;

    type IntoIter = <VecDeque<Crate> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CrateStacks(Vec<CrateStack>);

impl CrateStacks {
    pub fn do_procedure(&mut self, procedure: &Procedure) {
        for _ in 0..procedure.quantity {
            let c = self.0[procedure.from_stack].pop();

            match c {
                None => continue,
                Some(c) => self.0[procedure.to_stack].push(c),
            }
        }
    }
}

impl From<&str> for CrateStacks {
    fn from(lines: &str) -> Self {
        let num_cols = (lines.lines().last().unwrap().trim().chars().last().unwrap() as usize) - 48;
        let num_lines_in_input = lines.lines().count();

        let mut stacks = CrateStacks(vec![CrateStack::new(); num_cols]);

        for line in lines.lines().take(num_lines_in_input - 1) {
            let crates = line
                .chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|chunk| Crate::try_from(&chunk.iter().collect::<String>()))
                .collect::<Vec<_>>();

            for (idx, &c) in crates.iter().enumerate() {
                match c {
                    None => continue,
                    Some(c) => stacks.0[idx].insert(c),
                }
            }
        }

        stacks
    }
}

impl IntoIterator for CrateStacks {
    type Item = CrateStack;

    type IntoIter = <Vec<CrateStack> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crate_from_str() {
        assert_eq!(Crate::try_from("[D]"), Some(Crate('D')));
        assert_eq!(Crate::try_from("[D] "), Some(Crate('D')));
        assert_eq!(Crate::try_from("    "), None);
        assert_eq!(Crate::try_from("   "), None);
    }

    #[test]
    fn test_crate_stack_insert() {
        let mut stack = CrateStack::new();
        stack.insert(Crate('A'));
        stack.insert(Crate('B'));
        stack.insert(Crate('C'));

        assert_eq!(
            stack.into_iter().collect::<Vec<_>>(),
            vec![Crate('A'), Crate('B'), Crate('C')]
        );
    }

    #[test]
    fn test_parse_stacks() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "#;

        let mut stacks = vec![CrateStack::new(); 3];

        stacks[0].insert(Crate('N'));
        stacks[0].insert(Crate('Z'));
        stacks[1].insert(Crate('D'));
        stacks[1].insert(Crate('C'));
        stacks[1].insert(Crate('M'));
        stacks[2].insert(Crate('P'));

        assert_eq!(
            CrateStacks::from(input).into_iter().collect::<Vec<_>>(),
            stacks
        );
    }

    #[test]
    fn test_do_procedure() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "#;
        let mut parsed_stacks = CrateStacks::from(input);

        let procedure = Procedure::from("move 1 from 2 to 1");
        parsed_stacks.do_procedure(&procedure);

        let mut stacks = vec![CrateStack::new(); 3];

        stacks[0].insert(Crate('D'));
        stacks[0].insert(Crate('N'));
        stacks[0].insert(Crate('Z'));
        stacks[1].insert(Crate('C'));
        stacks[1].insert(Crate('M'));
        stacks[2].insert(Crate('P'));

        assert_eq!(parsed_stacks.into_iter().collect::<Vec<_>>(), stacks);
    }
}
