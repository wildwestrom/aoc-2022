use std::{cell::RefCell, collections::HashMap};

use itertools::Itertools;

type Stack = RefCell<Vec<char>>;
type Stacks = HashMap<usize, Stack>;

fn parse_initial_stacks(input: &Vec<&str>) -> Stacks {
    let stack_data_str = input.split_last().unwrap().1;

    let mut columns: Stacks = HashMap::new();

    let mut row = 0;
    stack_data_str.iter().rev().for_each(|l| {
        row += 1;
        l.chars()
            .chunks(4)
            .into_iter()
            .enumerate()
            .for_each(|(column_index, chunk)| {
                let column = column_index + 1;
                let probably_one_char = chunk
                    .into_iter()
                    .filter(|c| c.is_alphabetic())
                    .collect_vec();
                if let Some(crate_letter) = probably_one_char.into_iter().exactly_one().ok() {
                    match columns.get_mut(&column) {
                        None => {
                            let _ = columns.insert(column, RefCell::new(vec![crate_letter]));
                        }
                        Some(col) => (*col).borrow_mut().push(crate_letter),
                    };
                };
            });
    });
    columns
}

#[derive(Debug)]
struct ProcedureStep {
    crates_to_move: usize,
    from_stack: usize,
    to_stack: usize,
}

type MoveProcedure = Vec<ProcedureStep>;

fn parse_move_procedure(input: &Vec<&str>) -> MoveProcedure {
    let proc = input
        .iter()
        .map(|i| {
            let step = i
                .split_whitespace()
                .filter(|string| {
                    !(string.contains("move") || string.contains("from") || string.contains("to"))
                })
                .map(|string| string.parse::<usize>().unwrap())
                .collect_vec();
            ProcedureStep {
                crates_to_move: step[0],
                from_stack: step[1],
                to_stack: step[2],
            }
        })
        .collect_vec();
    proc
}

fn parse_input(input: &str) -> (Stacks, MoveProcedure) {
    let lines = input.lines();
    let initial_stacks_str_repr = lines.clone().take_while(|line| *line != "").collect_vec();
    let move_proc_str_repr = lines.skip_while(|line| *line != "").skip(1).collect_vec();
    (
        parse_initial_stacks(&initial_stacks_str_repr),
        parse_move_procedure(&move_proc_str_repr),
    )
}

pub fn part_one(input: &str) -> Option<String> {
    let (stacks, move_procedure) = parse_input(input);
    let procs = move_procedure.iter().enumerate();
    for (_proc_num, proc) in procs {
        for _crate_num in 1..=proc.crates_to_move {
            let stack_to_move_from = stacks
                .get(&proc.from_stack)
                .expect("Did not find a stack to move from");
            let stack_to_move_to = stacks
                .get(&proc.to_stack)
                .expect("Did not find a stack to move to");
            match stack_to_move_from.borrow_mut().pop() {
                Some(crate_to_move) => {
                    stack_to_move_to.borrow_mut().push(crate_to_move);
                }
                None => {}
            }
        }
    }
    let mut answer = String::new();
    let sorted_stacks = stacks.into_iter().sorted_by(|a, b| a.0.cmp(&b.0));
    for (_, stack) in sorted_stacks {
        let val = stack.borrow_mut().pop().unwrap();
        answer.push(val);
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks, move_procedure) = parse_input(input);
    let procs = move_procedure.iter().enumerate();
    for (_proc_num, proc) in procs {
        let mut temp_vec = Vec::new();
        let stack_to_move_from = stacks
            .get(&proc.from_stack)
            .expect("Did not find a stack to move from");
        let stack_to_move_to = stacks
            .get(&proc.to_stack)
            .expect("Did not find a stack to move to");
        for _crate_num in 1..=proc.crates_to_move {
            match stack_to_move_from.borrow_mut().pop() {
                Some(crate_to_move) => {
                    temp_vec.push(crate_to_move);
                }
                None => {}
            }
        }
        for _ in 1..=temp_vec.len() {
            let crate_to_move = temp_vec.pop().unwrap();
            stack_to_move_to.borrow_mut().push(crate_to_move);
        }
    }
    let mut answer = String::new();
    let sorted_stacks = stacks.into_iter().sorted_by(|a, b| a.0.cmp(&b.0));
    for (_, stack) in sorted_stacks {
        let val = stack.borrow_mut().pop().unwrap();
        answer.push(val);
    }
    Some(answer)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
