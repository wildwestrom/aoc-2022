use cgmath::Point2;
use itertools::Itertools;

#[derive(Debug)]
struct TreeGrid {
    trees: Vec<Tree>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy)]
struct Tree {
    position: Point2<usize>,
    height: u32,
}

fn parse_input(input: &str) -> TreeGrid {
    let mut trees = vec![];

    let height = input.lines().collect_vec().len();
    let width = input.lines().take(1).exactly_one().unwrap().len();

    input.lines().enumerate().for_each(|(rnum, line)| {
        line.chars().enumerate().for_each(|(cnum, height)| {
            let height = height.to_digit(10).unwrap();
            let position = Point2::new(cnum + 1, rnum + 1);

            trees.push(Tree { position, height })
        })
    });
    TreeGrid {
        trees,
        width,
        height,
    }
}

fn tree_is_on_edge(pos: &Point2<usize>, max_x: usize, max_y: usize) -> bool {
    pos.x == 1 || pos.y == 1 || pos.x == max_x || pos.y == max_y
}

fn find_visible_trees(grid: &TreeGrid) -> Vec<&Tree> {
    grid.trees
        .iter()
        .filter(|tree| {
            let all_other_trees = grid.trees.iter().filter(|other_tree| {
                let is_same_tree = other_tree.position == tree.position;
                !is_same_tree || !tree_is_on_edge(&other_tree.position, grid.width, grid.height)
            });

            let trees_in_same_row = all_other_trees.clone().filter(|other_tree| {
                let is_same_row = other_tree.position.y == tree.position.y;
                is_same_row
            });

            let trees_in_same_col = all_other_trees.filter(|other_tree| {
                let is_same_col = other_tree.position.x == tree.position.x;
                is_same_col
            });

            let taller_than_left = || -> bool {
                trees_in_same_row
                    .clone()
                    .filter(|t| t.position.x < tree.position.x)
                    .all(|t| tree.height > t.height)
            };
            let taller_than_right = || -> bool {
                trees_in_same_row
                    .clone()
                    .filter(|t| t.position.x > tree.position.x)
                    .all(|t| tree.height > t.height)
            };
            let taller_than_above = || -> bool {
                trees_in_same_col
                    .clone()
                    .filter(|t| t.position.y > tree.position.y)
                    .all(|t| tree.height > t.height)
            };
            let taller_than_below = || -> bool {
                trees_in_same_col
                    .clone()
                    .filter(|t| t.position.y < tree.position.y)
                    .all(|t| tree.height > t.height)
            };

            tree_is_on_edge(&tree.position, grid.width, grid.height)
                || taller_than_left()
                || taller_than_right()
                || taller_than_above()
                || taller_than_below()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let num_of_visible_trees = find_visible_trees(&grid).len();
    Some(num_of_visible_trees)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);

    let scenic_score = grid
        .trees
        .iter()
        .map(|tree| {
            let all_other_trees = grid.trees.iter().filter(|other_tree| {
                let is_same_tree = other_tree.position == tree.position;
                let is_same_col = other_tree.position.x == tree.position.x;
                let is_same_row = other_tree.position.y == tree.position.y;

                !is_same_tree && (is_same_col || is_same_row)
            });

            let visible_trees = |trees: Vec<&Tree>| {
                let mut visible_trees = 0;
                for t in trees {
                    if t.height < tree.height {
                        visible_trees += 1;
                    } else if t.height >= tree.height {
                        visible_trees += 1;
                        break;
                    }
                }
                visible_trees
            };

            let trees_above = all_other_trees
                .clone()
                .filter(|other_tree| {
                    other_tree.position.x == tree.position.x
                        && other_tree.position.y < tree.position.y
                })
                .sorted_by(|a, b| b.position.y.cmp(&a.position.y))
                .collect_vec();
            let visible_trees_above = visible_trees(trees_above);

            let trees_left = all_other_trees
                .clone()
                .filter(|other_tree| {
                    other_tree.position.y == tree.position.y
                        && other_tree.position.x < tree.position.x
                })
                .sorted_by(|a, b| b.position.x.cmp(&a.position.x))
                .collect_vec();
            let visible_trees_left = visible_trees(trees_left);

            let trees_below = all_other_trees
                .clone()
                .filter(|other_tree| {
                    other_tree.position.x == tree.position.x
                        && other_tree.position.y > tree.position.y
                })
                .sorted_by(|a, b| a.position.y.cmp(&b.position.y))
                .collect_vec();
            let visible_trees_below = visible_trees(trees_below);

            let trees_right = all_other_trees
                .filter(|other_tree| {
                    other_tree.position.y == tree.position.y
                        && other_tree.position.x > tree.position.x
                })
                .sorted_by(|a, b| a.position.x.cmp(&b.position.x))
                .collect_vec();
            let visible_trees_right = visible_trees(trees_right);

            visible_trees_above * visible_trees_below * visible_trees_left * visible_trees_right
        })
        .max()
        .unwrap();

    Some(scenic_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
