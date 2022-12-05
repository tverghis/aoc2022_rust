#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Procedure {
    pub quantity: usize,
    pub from_stack: usize,
    pub to_stack: usize,
}

impl From<&str> for Procedure {
    fn from(line: &str) -> Self {
        let [_, quantity, _, from_stack, _, to_stack]: [&str; 6] = line
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            quantity: quantity.parse().unwrap(),
            from_stack: from_stack.parse::<usize>().unwrap() - 1,
            to_stack: to_stack.parse::<usize>().unwrap() - 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_procedure_from_str() {
        assert_eq!(
            Procedure::from("move 10 from 1 to 29"),
            Procedure {
                quantity: 10,
                from_stack: 0,
                to_stack: 28
            }
        );
    }
}
