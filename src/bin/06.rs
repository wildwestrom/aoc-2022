use std::collections::VecDeque;

use itertools::Itertools;

fn solve_for_packet_length(input: &str, length: usize) -> Option<u32> {
    let mut chars_in: VecDeque<char> = input.chars().collect();
    let mut packet = VecDeque::new();

    fn duplicates_present(p: VecDeque<char>) -> bool {
        !p.iter().duplicates().collect_vec().is_empty()
    }

    let mut num_of_chars = 0;
    while duplicates_present(packet.clone()) || packet.len() < length {
        if packet.len() < length {
            packet.push_back(chars_in.pop_front().unwrap());
        } else {
            packet.pop_front();
            packet.push_back(chars_in.pop_front().unwrap());
        }
        num_of_chars += 1;
    }

    Some(num_of_chars as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve_for_packet_length(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve_for_packet_length(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
