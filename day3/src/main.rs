use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("File not found.");
    let input_text: Vec<&str> = file_contents.lines().collect();

    let part_sum = find_all_parts(&input_text);
    println!("Total: {}", part_sum);
}

fn find_all_parts(vec2: &Vec<&str>) -> u32 {
    let mut all_parts_sum: u32 = 0;
    let mut curr_part: u32 = 0;
    let mut start_idx: usize = 0;
    let mut possible_gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for line_idx in 0..vec2.len() {
        let line_chars: Vec<char> = vec2[line_idx].chars().collect();
        for char_idx in 0..line_chars.len() {
            let char = line_chars[char_idx];
            if char.is_digit(10) {
                if curr_part == 0 {
                    start_idx = char_idx;
                }
                curr_part = (curr_part * 10) + char.to_digit(10).unwrap();
            } else if curr_part > 0 {
                if is_valid_partnum(
                    line_idx,
                    start_idx,
                    char_idx - 1,
                    curr_part,
                    vec2,
                    &mut possible_gears,
                ) {
                    all_parts_sum += curr_part;
                }
                curr_part = 0;
            }
        }
        if curr_part > 0 {
            if is_valid_partnum(
                line_idx,
                start_idx,
                line_chars.len() - 1,
                curr_part,
                vec2,
                &mut possible_gears,
            ) {
                all_parts_sum += curr_part;
            }
            curr_part = 0;
        }
    }

    let mut gear_sum: u32 = 0;
    for key in possible_gears.keys() {
        if possible_gears.get(key).unwrap().len() != 2 {
            continue;
        }
        let gear_vals = possible_gears.get(key).unwrap();
        gear_sum += gear_vals[0] * gear_vals[1];
    }
    println!("Gear sum: {gear_sum}");

    all_parts_sum
}

fn is_valid_partnum(
    line_idx: usize,
    start_idx: usize,
    end_idx: usize,
    value: u32,
    vec2: &Vec<&str>,
    possible_gears: &mut HashMap<(usize, usize), Vec<u32>>,
) -> bool {
    let min_row = max(0, (line_idx as i32) - 1) as usize;
    let max_row = min(vec2.len() - 1, line_idx + 1);
    let min_col = max(0, (start_idx as i32) - 1) as usize;
    let max_col = min(vec2[line_idx].len() - 1, end_idx + 1);

    for row in min_row..=max_row {
        let curr_row = vec2[row];

        for col in min_col..=max_col {
            if row == line_idx && (start_idx <= col && end_idx >= col) {
                continue;
            }
            let val: char = curr_row.chars().nth(col).unwrap();
            if val != '.' && val != '.' && val != '\n' && !val.is_digit(10) {
                if val == '*' {
                    possible_gears
                        .entry((row, col))
                        .and_modify(|e| e.push(value))
                        .or_insert(vec![value]);
                }
                return true;
            }
        }
    }
    return false;
}
