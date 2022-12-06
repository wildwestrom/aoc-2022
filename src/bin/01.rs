type CalorieList = Vec<u32>;

fn find_total_calories_per_elf(input: &str) -> Vec<u32> {
    let mut elves_calories: Vec<CalorieList> = vec![];
    let split_strs = input.lines().map(|l| {
        if l == "" {
            None
        } else {
            Some(l.parse::<u32>().unwrap())
        }
    });

    let mut current_elf_calories: CalorieList = vec![];
    split_strs.for_each(|f| match f {
        Some(f) => {
            let _ = &current_elf_calories.push(f);
        }
        None => {
            elves_calories.push(current_elf_calories.clone());
            current_elf_calories = Vec::new();
        }
    });

    elves_calories
        .into_iter()
        .map(|g| {
            g.into_iter()
                .reduce(|acc, x| acc + x)
                .expect("If there is no number here, something's gone wrong with initial parsing")
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let calorie_sums: Vec<u32> = find_total_calories_per_elf(input);

    let total_calories: u32 = *calorie_sums
        .iter()
        .max()
        .expect("If there's no max value, some thing went wrong");

    Some(total_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calorie_sums: Vec<u32> = find_total_calories_per_elf(input);
    calorie_sums.sort();
    calorie_sums.reverse();

    let top_3: Vec<u32> = calorie_sums.into_iter().take(3).collect();

    let top_3_sum = top_3.iter().sum::<u32>();

    Some(top_3_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(41000));
    }
}
