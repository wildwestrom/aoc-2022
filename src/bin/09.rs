use std::collections::HashSet;

use cgmath::Point2;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (dir_ch, step_ch) = line.split_whitespace().collect_tuple().unwrap();
            let direction = match dir_ch {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                c => panic!("Unexpected input {c}"),
            };
            let steps: u32 = step_ch.parse().unwrap();
            Instruction { direction, number_of_moves: steps }
        })
        .collect_vec()
}

struct Instruction {
    direction: Direction,
    number_of_moves: u32,
}

struct Rope {
    segments: Vec<Point>,
    unique_tail_locations: HashSet<Point>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Rope {
    fn new(n: usize) -> Self {
        Self {
            segments: vec![Point { x: 0, y: 0 }; n],
            unique_tail_locations: HashSet::from([Point { x: 0, y: 0 }]),
        }
    }

    fn move_rope(&mut self, instruction: &Instruction) {
        for _move in 0..instruction.number_of_moves {
            self.move_head(&instruction.direction);
            self.update_rest_of_segments();
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        let head = self.segments.first_mut().unwrap();
        match direction {
            Direction::Up => head.y += 1,
            Direction::Right => head.x += 1,
            Direction::Left => head.x -= 1,
            Direction::Down => head.y -= 1,
        }
    }

    fn update_rest_of_segments(&mut self) {
        let mut tail = self.segments[0];

        for segment in self.segments.iter_mut().skip(1) {
            let (dx, dy) = (tail.x - segment.x, tail.y - segment.y);

            if dx.abs() > 1 || dy.abs() > 1 {
                segment.x += dx.signum();
                segment.y += dy.signum();
            }

            tail = *segment;
        }
        self.unique_tail_locations.insert(*self.segments.last().unwrap());
    }
}

type Point = Point2<i32>;

pub fn part_one(input: &str) -> Option<usize> {
    let move_instruction = parse_input(input);

    let mut rope = Rope::new(2);
    move_instruction
        .iter()
        .for_each(|instruction| rope.move_rope(instruction));
    Some(rope.unique_tail_locations.len())
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    let move_instruction = parse_input(input);

    let mut rope = Rope::new(10);
    move_instruction
        .iter()
        .for_each(|instruction| rope.move_rope(instruction));
    Some(rope.unique_tail_locations.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
