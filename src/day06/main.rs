use std::fs;
use std::collections::HashMap;
use reduce::Reduce;


struct Declaration {
    yes: Vec<u8>
}


impl Declaration {
    fn create(input: &str) -> Declaration {
        Declaration{ yes: input.bytes().collect() }
    }
}


struct DeclarationGroup {
    declarations: Vec<Declaration>,
    yes_answers: HashMap<u8, usize>,
}

impl DeclarationGroup {

    fn create(input: &str) -> DeclarationGroup {
        let mut group = DeclarationGroup {
            declarations: input
                .split_whitespace()
                .map( | input| Declaration::create(input) )
                .collect(),
            yes_answers: HashMap::new(),
        };

        for declaration in &group.declarations {
            for char in &declaration.yes {
                let count = group.yes_answers.entry(*char).or_insert(0);
                *count += 1;
            }
        }

        group
    }

    fn yes_count(&self) -> usize {
        self.yes_answers.len()
    }

    fn yes_count_for_all_declarations(&self) -> usize {
        self.yes_answers
            .iter()
            .map(|(_char, count)| {
                if *count == self.declarations.len() {
                    1
                } else {
                    0
                }
            })
            .reduce(|lhs, rhs| lhs + rhs)
            .unwrap()
    }
}


fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Unable to read input.txt");

    let declarations: Vec<DeclarationGroup> = contents
        .split("\n\n")
        .map(|element|DeclarationGroup::create(element))
        .collect();

    let yes_count = declarations
        .iter()
        .map(|group| group.yes_count())
        .reduce(|lhs, rhs| lhs + rhs)
        .unwrap();

    println!("{}", yes_count);

    let yes_count_for_all_declarations = declarations
        .iter()
        .map(|group| group.yes_count_for_all_declarations())
        .reduce(|lhs, rhs| lhs + rhs)
        .unwrap();

    println!("{}", yes_count_for_all_declarations);
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_total_yes_count_per_group() {
        let declarations: Vec<DeclarationGroup> = INPUT
            .split("\n\n")
            .map(|element|DeclarationGroup::create(element))
            .collect();

        let yes_count = declarations
            .iter()
            .map(|group| group.yes_count())
            .reduce(|lhs, rhs| lhs + rhs)
            .unwrap();

        assert_eq!(yes_count, 11);
    }

    #[test]
    fn test_total_yes_count_per_group_per_question() {
        let declarations: Vec<DeclarationGroup> = INPUT
            .split("\n\n")
            .map(|element|DeclarationGroup::create(element))
            .collect();

        let yes_count = declarations
            .iter()
            .map(|group| group.yes_count_for_all_declarations())
            .reduce(|lhs, rhs| lhs + rhs)
            .unwrap();

        assert_eq!(yes_count, 6);
    }
}