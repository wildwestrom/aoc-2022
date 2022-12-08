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

    input
        .lines()
        .map(|l| {
            if l.starts_with('$') {
                let command_contents = l.chars().skip(2).collect::<String>();
                if command_contents.starts_with("cd") {
                    let destination = command_contents.chars().skip(3).collect::<String>();
                    if destination == ".." {
                        pwd.pop();
                    } else {
                        pwd.push(&destination);
                    }
                } else if command_contents.starts_with("ls") {
                    return;
                } else {
                    panic!("unexpected input")
                }
            } else {
                if l.starts_with('d') {
                    let _dirname = l.chars().skip(4).collect::<String>();
                } else {
                    let filename_chars = l.chars();
                    let filename = filename_chars
                        .clone()
                        .skip_while(|c| c.is_numeric())
                        .skip(1)
                        .collect::<String>();
                    let filesize = filename_chars
                        .take_while(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<usize>()
                        .expect("oh no, parsing string into usize failed");
                    let mut newfilename = pwd.clone();
                    newfilename.push(&filename);
                    files.insert(newfilename, filesize);
                }
            }
        })
        .collect_vec();
    files
}

fn files_to_dirs_with_sizes(files: Files) -> Files {
    let mut dirs: Files = HashMap::new();

    files.iter().for_each(|(path, size)| {
        // For each file, take their parents
        path.ancestors().for_each(|a| {
            // Then find each directory that matches that parent path
            // and add the file size of every file to the directory size
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
    let files = input_to_files(input);
    let dirs = files_to_dirs_with_sizes(files);

    let under_100k = dirs
        .iter()
        .map(|(_, s)| *s)
        .filter(|s| *s <= 100000)
        .collect_vec();

    Some(under_100k.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let total_space = 70000000;
    let unused_space_target = 30000000;
    let used_space_target = total_space - unused_space_target;

    let files = input_to_files(input);
    let dir_hashmap = files_to_dirs_with_sizes(files);
    let dirs = dir_hashmap.values();

    let total_disk_usage = dirs.clone().max().unwrap();

    let file_we_want = dirs
        .filter(|d| {
            total_disk_usage - *d <= used_space_target
        })
        .min()
        .unwrap();
    Some(*file_we_want)
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
