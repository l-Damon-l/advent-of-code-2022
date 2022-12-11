mod part1;
mod part2;

use part1::get_num_visible_trees;
use part2::get_highest_scenic_score;
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
