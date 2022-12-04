use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let part1_solution = get_part1_solution();
    println!("Part 1: {}", part1_solution); // 8088
    let part2_solution = get_part2_solution();
    println!("Part 2: {}", part2_solution); // 2522
}

fn get_part1_solution() -> u32 {
    let mut result: u32 = 0;
    // Buffered reader to read file line by line
    let file = File::open("Day3Puzzle.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let unwrapped_line = line.unwrap(); // fine to just unwrap here
        let u8_from_line = get_u8_from_line(unwrapped_line);
        result += u8_from_line as u32;
    }
    result
}

fn get_u8_from_line(line: String) -> u8 {
    let mid = line.len() / 2;
    let (left, right) = line.split_at(mid);
    let left_set = left.chars().collect::<HashSet<_>>();
    let right_set = right.chars().collect::<HashSet<_>>();
    let mut intersection = left_set.intersection(&right_set);
    let intersection_to_char = intersection.next().unwrap();

    // a - z should end up as 1 - 26, A - Z should end up as 27 - 52
    return if intersection_to_char.is_lowercase() {
        let char_as_int = *intersection_to_char as u8 - 96;
        char_as_int
    } else {
        // It is an uppercase letter
        let char_as_int = *intersection_to_char as u8 - 38;
        char_as_int
    };
}

fn get_part2_solution() -> u32 {
    let mut result: u32 = 0;
    // Buffered reader to read file line by line
    let file = File::open("Day3Puzzle.txt").unwrap();
    let reader = BufReader::new(file);

    // check 3 lines at a time
    let mut lines: [String; 3] = Default::default();
    let mut cur_arr_pos = 0;
    for line in reader.lines() {
        let unwrapped_line = line.unwrap(); // fine to just unwrap here
        lines[cur_arr_pos] = unwrapped_line;
        if cur_arr_pos == 2 {
            // 3 lines have been entered into the array
            let u8_from_line = get_u8_from_3_lines(lines);
            result += u8_from_line as u32;
            // Reset the array and current position
            lines = Default::default();
            cur_arr_pos = 0;
            continue;
        }
        cur_arr_pos += 1;
    }
    result
}

fn get_u8_from_3_lines(lines: [String; 3]) -> u8 {
    // Turn the 3 lines into 3 sets
    let line_set = lines[0].chars().collect::<HashSet<_>>();
    let line_set2 = lines[1].chars().collect::<HashSet<_>>();
    let line_set3 = lines[2].chars().collect::<HashSet<_>>();

    // Get the intersections between line 1 and 2, then line 1 and 3
    let intersection = line_set.intersection(&line_set2);
    let intersection2 = line_set.intersection(&line_set3);

    // Get the intersection between the other two intersections
    let intersection_to_set = intersection.collect::<HashSet<_>>();
    let intersection_to_set2 = intersection2.collect::<HashSet<_>>();
    let mut final_intersection = intersection_to_set.intersection(&intersection_to_set2);

    // Should be only one char left in the final intersection
    let intersection_to_char = *final_intersection.next().unwrap();

    // a - z should end up as 1 - 26, A - Z should end up as 27 - 52
    return if intersection_to_char.is_lowercase() {
        let char_as_int = *intersection_to_char as u8 - 96;
        // println!("{} {}", intersection_to_char, char_as_int);
        char_as_int
    } else {
        // It is an uppercase letter
        let char_as_int = *intersection_to_char as u8 - 38;
        // println!("{} {}", intersection_to_char, char_as_int);
        char_as_int
    };
}
