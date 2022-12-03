use std::collections::HashSet;

use itertools::Itertools;

fn find_unique_char(first: &str, second: &str) -> char {
    let mut first_set = HashSet::new();
    first.chars().for_each(|c| {
        first_set.insert(c);
    });
    let mut second_set = HashSet::new();
    second.chars().for_each(|c| {
        second_set.insert(c);
    });
    let common_chars = first_set.intersection(&second_set);
    *common_chars
        .exactly_one()
        .expect("There were more or less than 1 chars")
}

fn char_to_priority(c: &char) -> u32 {
    let mut num = if c.is_alphabetic() {
        c.to_digit(36).unwrap()
    } else {
        panic!("Didn't expect {c}");
    };
    if c.is_lowercase() {
        num -= 9;
    } else if c.is_uppercase() {
        num += 17;
    } else {
        panic!("Didn't expect {c}")
    }
    num
}

pub fn part_one(input: &str) -> Option<u32> {
    let unique_chars: Vec<char> = input
        .lines()
        .map(|s: &str| -> char {
            let compartments = s.split_at(s.len() / 2);
            let (first, second) = compartments;
            find_unique_char(first, second)
        })
        .collect();
    let priorities: Vec<u32> = unique_chars.iter().map(|c| char_to_priority(c)).collect();
    Some(priorities.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec_of_lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let elf_groups = vec_of_lines
        .chunks(3)
        .map(|elves| {
            let elf_1: HashSet<char> = HashSet::from_iter(elves[0].clone());
            let elf_2: HashSet<char> = HashSet::from_iter(elves[1].clone());
            let elf_3: HashSet<char> = HashSet::from_iter(elves[2].clone());

            let int_1: HashSet<char> =
                HashSet::from_iter(elf_1.intersection(&elf_2).map(|val| *val).collect_vec());
            let int_2: HashSet<char> =
                HashSet::from_iter(elf_2.intersection(&elf_3).map(|val| *val).collect_vec());

            let elves_1_and_2 = HashSet::from_iter(int_1);
            let final_intersection = elves_1_and_2.intersection(&int_2).exactly_one().unwrap();
            char_to_priority(&final_intersection)
        })
        .collect_vec();
    Some(elf_groups.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
