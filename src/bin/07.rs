#![allow(unused_imports, unused_imports)]

use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use itertools::Itertools;

type Files = HashMap<PathBuf, usize>;

fn input_to_files(input: &str) -> Files {
    let mut pwd = PathBuf::new();
    let mut files = HashMap::new();

    input.lines().for_each(|l| {
        if l.starts_with('$') {
            let command_contents = l.chars().skip(2).collect::<String>();
            if command_contents.starts_with("cd") {
                let destination = command_contents.chars().skip(3).collect::<String>();
                if destination == ".." {
                    pwd.pop();
                } else {
                    pwd.push(&destination);
                }
            }
        } else {
            if !l.starts_with('d') {
                let (size_str, filename_str) = l
                    .split_whitespace()
                    .collect_tuple::<(&str, &str)>()
                    .unwrap();

                let filename = PathBuf::from(filename_str);
                let size = size_str.parse().unwrap();

                let mut full_path = pwd.clone();
                full_path.push(&filename);

                files.insert(full_path, size);
            }
        }
    });
    files
}

fn input_to_dirs(input: &str) -> Files {
    let files = input_to_files(input);
    let mut dirs: Files = HashMap::new();

    files.iter().for_each(|(path, size)| {
        path.ancestors().for_each(|a| {
            if let Some(parent) = a.parent() {
                dirs.entry(parent.to_path_buf())
                    .and_modify(|esize| *esize += size)
                    .or_insert(*size);
            }
        });
    });

    dirs
}

pub fn part_one(input: &str) -> Option<usize> {
    let dirs = input_to_dirs(input);

    let under_100k = dirs
        .iter()
        .map(|(_, s)| *s)
        .filter(|s| *s <= 100000)
        .collect_vec();

    Some(under_100k.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let dirs_hashmap = input_to_dirs(input);
    let dirs = dirs_hashmap.values();

    let total_disk_usage = dirs.clone().max().unwrap();

    let smallest_that_frees_enough_space = dirs
        .filter(|d| total_disk_usage - *d <= 40000000)
        .min()
        .unwrap();
    Some(*smallest_that_frees_enough_space)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
