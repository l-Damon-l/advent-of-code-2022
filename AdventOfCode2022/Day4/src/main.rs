use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Part 1 Solution: {}", get_part1_solution()); // 532
    println!("Part 2 Solution: {}", get_part2_solution()); // 854
}

fn get_part1_solution() -> u32 {
    let mut num_ranges_covered = 0u32;
    let file = File::open("Day4Puzzle.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        // Unwrap should be fine here
        let line = line.unwrap();

        // For example 22-77,14-96 is split into Vec[22, 77] and Vec[14, 96]
        let mut split_line = line.split(",");
        let first = split_line.next().unwrap();
        let first_nums = first.split("-").collect::<Vec<&str>>();
        let second = split_line.next().unwrap();
        let second_nums = second.split("-").collect::<Vec<&str>>();

        // Should parse as ints without any issue
        let first_ints = first_nums
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let second_ints = second_nums
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if range_is_covered(first_ints, second_ints) {
            num_ranges_covered += 1;
        }
    }
    num_ranges_covered
}

fn range_is_covered(first_ints: Vec<i32>, second_ints: Vec<i32>) -> bool {
    if first_ints[0] >= second_ints[0] && first_ints[1] <= second_ints[1] {
        return true;
    } else if second_ints[0] >= first_ints[0] && second_ints[1] <= first_ints[1] {
        return true;
    }
    false
}

fn get_part2_solution() -> u32 {
    let mut num_overlaps = 0u32;
    let file = File::open("Day4Puzzle.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        // Unwrap should be fine here
        let line = line.unwrap();

        // For example 22-77,14-96 is split into Vec[22, 77] and Vec[14, 96]
        let mut split_line = line.split(",");
        let first = split_line.next().unwrap();
        let first_nums = first.split("-").collect::<Vec<&str>>();
        let second = split_line.next().unwrap();
        let second_nums = second.split("-").collect::<Vec<&str>>();

        // Should parse as ints without any issue
        let first_ints = first_nums
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let second_ints = second_nums
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if range_overlaps(&first_ints, &second_ints) {
            num_overlaps += 1;
        }
    }
    num_overlaps
}

fn range_overlaps(first_ints: &Vec<i32>, second_ints: &Vec<i32>) -> bool {
    // Doing it lazily with inclusive ranges
    let first_range = first_ints[0]..=first_ints[1];
    let second_range = second_ints[0]..=second_ints[1];
    if first_range.contains(&second_ints[0]) || first_range.contains(&second_ints[1]) {
        return true;
    } else if second_range.contains(&first_ints[0]) || second_range.contains(&first_ints[1]) {
        return true;
    }
    false
}
