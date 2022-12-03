use itertools::Itertools;

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

fn char_to_choice(input: char) -> Choice {
    match input {
        'A' | 'X' => Choice::Rock,
        'B' | 'Y' => Choice::Paper,
        'C' | 'Z' => Choice::Scissors,
        c => panic!("Did not expect character {c}"),
    }
}

fn char_to_game_result(input: char) -> GameResult {
    match input {
        'X' => GameResult::Lose,
        'Y' => GameResult::Draw,
        'Z' => GameResult::Win,
        c => panic!("Did not expect character {c}"),
    }
}

fn lines_to_string_list(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|c| c.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn string_to_char_pair(string: &str) -> (char, char) {
    string
        .chars()
        .into_iter()
        .filter(|c| c != &' ')
        .next_tuple()
        .unwrap()
}

fn parse_into_choice_pairs(input: &str) -> Vec<(Choice, Choice)> {
    let strings = lines_to_string_list(input);
    strings
        .iter()
        .map(|s| {
            let (opponent_choice, your_choice) = string_to_char_pair(s);
            (char_to_choice(opponent_choice), char_to_choice(your_choice))
        })
        .collect()
}

fn parse_into_strategies(input: &str) -> Vec<(Choice, GameResult)> {
    let strings = lines_to_string_list(input);
    let strategy = strings
        .iter()
        .map(|s| {
            let (opponent_choice, desired_outcome) = string_to_char_pair(s);
            (
                char_to_choice(opponent_choice),
                char_to_game_result(desired_outcome),
            )
        })
        .collect();
    strategy
}

#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

fn choice_to_value(choice: &Choice) -> u32 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

fn game_result_to_value(game: &GameResult) -> u32 {
    match game {
        GameResult::Win => 6,
        GameResult::Lose => 0,
        GameResult::Draw => 3,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs: Vec<(Choice, Choice)> = parse_into_choice_pairs(input);
    let scores: Vec<u32> = pairs
        .iter()
        .map(|(opponents_choice, your_choice)| {
            let game_result = match your_choice {
                Choice::Rock => match opponents_choice {
                    Choice::Scissors => GameResult::Win,
                    Choice::Paper => GameResult::Lose,
                    Choice::Rock => GameResult::Draw,
                },
                Choice::Paper => match opponents_choice {
                    Choice::Rock => GameResult::Win,
                    Choice::Scissors => GameResult::Lose,
                    Choice::Paper => GameResult::Draw,
                },
                Choice::Scissors => match opponents_choice {
                    Choice::Paper => GameResult::Win,
                    Choice::Rock => GameResult::Lose,
                    Choice::Scissors => GameResult::Draw,
                },
            };
            let choice_score = choice_to_value(your_choice);
            let game_score = game_result_to_value(&game_result);
            choice_score + game_score
        })
        .collect();
    let total_score = scores.iter().sum();
    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs: Vec<(Choice, GameResult)> = parse_into_strategies(input);
    let scores: Vec<u32> = pairs
        .iter()
        .map(|(opponents_choice, desired_game_result)| {
            let your_choice = match desired_game_result {
                GameResult::Win => match opponents_choice {
                    Choice::Rock => Choice::Paper,
                    Choice::Paper => Choice::Scissors,
                    Choice::Scissors => Choice::Rock,
                },
                GameResult::Draw => match opponents_choice {
                    Choice::Rock => Choice::Rock,
                    Choice::Paper => Choice::Paper,
                    Choice::Scissors => Choice::Scissors,
                },
                GameResult::Lose => match opponents_choice {
                    Choice::Rock => Choice::Scissors,
                    Choice::Paper => Choice::Rock,
                    Choice::Scissors => Choice::Paper,
                },
            };
            let choice_score = choice_to_value(&your_choice);
            let game_score = game_result_to_value(desired_game_result);
            choice_score + game_score
        })
        .collect();
    let total_score = scores.iter().sum();
    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
