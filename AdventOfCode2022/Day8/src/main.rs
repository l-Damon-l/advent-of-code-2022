use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_lines = collect_file("Day8Puzzle.txt");
    let lines_as_ints = convert_chars_to_ints(file_lines);
    let part1_result = get_num_visible_trees(&lines_as_ints);
    let part2_result = get_highest_scenic_score(&lines_as_ints);
    println!("Solution to part 1: {}", part1_result);
    println!("Solution to part 2: {}", part2_result);
}

// Read the file into a vector of strings
fn collect_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("file not found");
    // Get lines as string array
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .collect()
}

// Convert the strings into a multidimensional vector of ints
fn convert_chars_to_ints(file_lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut final_vec = Vec::new();
    for line in file_lines {
        let line_to_ints = line
            .chars()
            .map(|c| c.to_digit(10).expect("Couldn't convert char to int") as i32)
            .collect::<Vec<i32>>();
        final_vec.push(line_to_ints);
    }
    final_vec
}

// Part 1 functions

fn tree_is_on_outside(vec: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    if row == 0 || row == vec.len() - 1 || col == 0 || col == vec[0].len() - 1 {
        return true;
    }
    false
}

fn is_tree_visible_by_row(vec: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let cur_int = vec[row][col];
    // Assume it is visible unless we find a number that is greater than it on that side
    let mut visible_from_left = true;
    let mut visible_from_right = true;
    for i in 0..vec[row].len() {
        if i == col {
            continue;
        }
        // Check to the left of current int
        if i < col {
            if vec[row][i] >= cur_int {
                visible_from_left = false;
            }
        }
        // Check to the right of current int
        if i > col {
            if vec[row][i] >= cur_int {
                visible_from_right = false;
            }
        }
    }
    // true if visible from left or right (or both)
    visible_from_left || visible_from_right
}

fn is_tree_visible_by_column(vec: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let cur_int = vec[row][col];
    // Assume it is visible unless we find a number that is greater than it on that side
    let mut visible_from_top = true;
    let mut visible_from_bottom = true;
    for i in 0..vec.len() {
        if i == row {
            continue;
        }
        // Check above current int
        if i < row {
            if vec[i][col] >= cur_int {
                visible_from_top = false;
            }
        }
        // Check below current int
        if i > row {
            if vec[i][col] >= cur_int {
                visible_from_bottom = false;
            }
        }
    }
    // true if visible from top or bottom (or both)
    visible_from_top || visible_from_bottom
}

fn get_num_visible_trees(vec: &Vec<Vec<i32>>) -> i32 {
    let mut num_visible_trees = 0;
    for row in 0..vec.len() {
        for col in 0..vec[row].len() {
            if tree_is_on_outside(vec, row, col) {
                num_visible_trees += 1;
                continue;
            }
            if is_tree_visible_by_row(vec, row, col) || is_tree_visible_by_column(vec, row, col) {
                num_visible_trees += 1;
            }
        }
    }
    num_visible_trees
}

// Part 2 functions

fn num_trees_visible_left_and_right(vec: &Vec<Vec<i32>>, row: usize, col: usize) -> (i32, i32) {
    let cur_int = vec[row][col];
    let mut left_trees = 0;
    let mut right_trees = 0;
    // Check all numbers to the left of current int
    for i in (0..col).rev() {
        // Can't start from col - 1, because usize will underflow
        // ... there is probably a better way though
        if i == col {
            continue;
        }
        if vec[row][i] < cur_int {
            left_trees += 1;
        } else {
            left_trees += 1;
            break;
        }
    }
    // Check all numbers to the right of current int
    for i in col..vec[row].len() {
        if i == col {
            continue;
        }
        if vec[row][i] < cur_int {
            right_trees += 1;
        } else {
            right_trees += 1;
            break;
        }
    }
    (left_trees, right_trees)
}

fn num_trees_visible_top_and_bottom(vec: &Vec<Vec<i32>>, row: usize, col: usize) -> (i32, i32) {
    let cur_int = vec[row][col];
    let mut top_trees = 0;
    let mut bottom_trees = 0;
    // Check all numbers above current int
    for i in (0..row).rev() {
        if i == row {
            continue;
        }
        if vec[i][col] < cur_int {
            top_trees += 1;
        } else {
            top_trees += 1;
            break;
        }
    }
    // Check all numbers below current int
    for i in row..vec.len() {
        if i == row {
            continue;
        }
        if vec[i][col] < cur_int {
            bottom_trees += 1;
        } else {
            bottom_trees += 1;
            break;
        }
    }
    (top_trees, bottom_trees)
}

fn get_highest_scenic_score(lines_as_ints: &Vec<Vec<i32>>) -> i32 {
    let mut cur_high_score = 0;
    for row in 0..lines_as_ints.len() {
        for col in 0..lines_as_ints[row].len() {
            let (left_trees, right_trees) =
                num_trees_visible_left_and_right(&lines_as_ints, row, col);
            let (top_trees, bottom_trees) =
                num_trees_visible_top_and_bottom(&lines_as_ints, row, col);
            let total_score = left_trees * right_trees * top_trees * bottom_trees;
            if total_score > cur_high_score {
                cur_high_score = total_score;
            }
        }
    }
    cur_high_score
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_part1_example() {
        let lines = collect_file("Day8TestPuzzle.txt");
        let lines_as_ints = convert_chars_to_ints(lines);
        let result = get_num_visible_trees(&lines_as_ints);
        assert_eq!(result, 21);
    }

    #[test]
    fn aoc_part2_example() {
        let lines = collect_file("Day8TestPuzzle.txt");
        let lines_as_ints = convert_chars_to_ints(lines);
        let result = get_highest_scenic_score(&lines_as_ints);
        assert_eq!(result, 8);
    }
}
