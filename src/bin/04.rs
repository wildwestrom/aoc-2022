use itertools::Itertools;

fn one_range_partially_contains_another(ranges: (Vec<u32>, Vec<u32>)) -> bool {
    let (range_1, range_2) = ranges;
    range_2
        .iter()
        .map(|i| range_1.contains(i))
        .reduce(|left, right| left || right)
        .unwrap()
        || range_1
            .iter()
            .map(|i| range_2.contains(i))
            .reduce(|left, right| left || right)
            .unwrap()
}

fn one_range_completely_contains_another(ranges: (Vec<u32>, Vec<u32>)) -> bool {
    let (range_1, range_2) = ranges;
    range_2
        .iter()
        .map(|i| range_1.contains(i))
        .reduce(|left, right| left && right)
        .unwrap()
        || range_1
            .iter()
            .map(|i| range_2.contains(i))
            .reduce(|left, right| left && right)
            .unwrap()
}

type RangePair = (Vec<u32>, Vec<u32>);

fn pairs_of_ranges(input: &str) -> Vec<RangePair> {
    let pairs = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|k| {
            k.split(',')
                .map(|s| {
                    s.split('-')
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    pairs
        .into_iter()
        .map(|splits| {
            let ranges = splits
                .into_iter()
                .map(|r| (r[0]..=r[1]).collect_vec())
                .collect_tuple()
                .unwrap();
            ranges
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let contained_items = pairs_of_ranges(input)
        .into_iter()
        .map(|ranges| one_range_completely_contains_another(ranges))
        .collect_vec();

    // Count all values that are true
    let n = contained_items
        .into_iter()
        .filter(|s| *s)
        .collect_vec()
        .len();
    Some(n.try_into().unwrap())
}
pub fn part_two(input: &str) -> Option<u32> {
    let contained_items = pairs_of_ranges(input)
        .into_iter()
        .map(|ranges| one_range_partially_contains_another(ranges))
        .collect_vec();

    // Count all values that are true
    let n = contained_items
        .into_iter()
        .filter(|s| *s)
        .collect_vec()
        .len();
    Some(n.try_into().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
