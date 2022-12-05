use krate::CrateStacks;
use procedure::Procedure;

mod krate;
mod procedure;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let [crate_stacks_and_cols, procedures]: [_; 2] =
        INPUT.split("\n\n").collect::<Vec<_>>().try_into().unwrap();

    let crate_stacks = CrateStacks::from(crate_stacks_and_cols);
    let procedures = procedures.lines().map(Procedure::from).collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        get_crate_stack_tops(&crate_stacks, &procedures)
    );
}

fn get_crate_stack_tops(crate_stacks: &CrateStacks, procedures: &[Procedure]) -> String {
    let mut crate_stacks = crate_stacks.clone();

    for procedure in procedures {
        crate_stacks.do_procedure(procedure);
    }

    crate_stacks
        .into_iter()
        .map(|stack| stack.peek_inner())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_procedures() {
        let crate_stacks = CrateStacks::from(
            r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "#,
        );
        let procedures = r#"move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
            .lines()
            .map(Procedure::from)
            .collect::<Vec<_>>();

        assert_eq!(
            get_crate_stack_tops(&crate_stacks, &procedures),
            String::from("CMZ")
        );
    }
}
