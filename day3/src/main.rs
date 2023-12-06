use std::fs;
use std::cmp::min;
use std::cmp::max;

struct PartNumber {
    is_setup: bool,
    row_idx: usize,
    start_idx: usize,
    end_idx: usize,
    number: u32,
}

impl PartNumber {
    fn new() -> PartNumber{
        PartNumber {
            is_setup: false,
            row_idx: 0,
            start_idx: 0,
            end_idx: 0,
            number: 0,
        }
    }

    fn add_info(&mut self, num: u32, row: usize, colidx: usize) {
        if !self.is_setup {
            self.is_setup = true;
            self.row_idx = row;
            self.start_idx = colidx;
        }
        self.end_idx = colidx;
        self.number = self.number * 10 + num;
    }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("File not found.");

    let input_vec2: Vec<&str> = file_contents.lines().collect();

    let part_locs = find_part_locs(&input_vec2);
    for pl in part_locs {
        dbg!(pl.number);
    }
    // let part_sums = sum_of_parts(part_locs);
}

fn find_part_locs(vec2: &Vec<&str>) -> Vec<PartNumber> {
    let mut all_parts: Vec<PartNumber> = Vec::new();
    let mut curr_partnum = PartNumber::new();

    for line_idx in 0..vec2.len() {
        let line = vec2[line_idx];
        let line_chars: Vec<char> = line.chars().collect();
        for char_idx in 0..line.len() {
            let char = line_chars[char_idx];
            if char.is_digit(10) {
                curr_partnum.add_info(
                    char.to_digit(10).unwrap(),
                    line_idx,
                    char_idx
                );
            } else if curr_partnum.is_setup && is_valid_partnum(&curr_partnum, vec2) {
                all_parts.push(curr_partnum);
                curr_partnum = PartNumber::new();
            }
        }
    }

    all_parts
}

fn is_valid_partnum(partnum: &PartNumber, vec2: &Vec<&str>) -> bool {
    let min_row = max(0, (partnum.row_idx as i32) - 1);
    let max_row = min(vec2.len()-1, partnum.row_idx+1);
    let min_col = max(0, (partnum.start_idx as i32) - 1);
    let max_col = min(vec2[0].len()-1, partnum.end_idx+1);

    for row in min_row..=max_row {
        let curr_row = vec2[row];
        for col in min_col..=max_col {
            if row == partnum.row_idx && (partnum.start_idx <= col && partnum.end_idx >= col) {
                continue;
            }
            let val: char = curr_row.chars().nth(col).unwrap();
            if val != '.' && !val.is_digit(10) {
                return true;
            }
        }
    }
    return false;
}
