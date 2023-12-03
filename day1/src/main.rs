use std::fs;
use std::cmp::min;
use std::cmp::max;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File not found.");

    let num_words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero"];
    let mut sum: u32 = 0;

    for line in contents.lines() {
        let first = get_front(line, &num_words);
        let last = get_back(line, &num_words);
        let value = u32::from((first * 10) + last);
        sum += value;
    }
    println!("{sum}");
}

fn num_word_to_digit(num_word: &str) -> u8 {
    match num_word {
        "one"   => return 1,
        "two"   => return 2,
        "three" => return 3,
        "four"  => return 4,
        "five"  => return 5,
        "six"   => return 6,
        "seven" => return 7,
        "eight" => return 8,
        "nine"  => return 9,
        _       => return 0,
    }
}

fn get_front(line: &str, num_words: &Vec<&str>) -> u8 {
    get_num(line, num_words, false)
}

fn get_back(line: &str, num_words: &Vec<&str>) -> u8 {
    get_num(line, num_words, true)
}

fn get_num(line: &str, num_words: &Vec<&str>, reverse: bool) -> u8 {
    let mut start = 0;
    let indexes: Vec<usize> = if reverse {
        start = line.len() - 1;
        (0..line.len()).rev().collect()
    } else {
        (0..line.len()).collect()
    };

    let line_chars: Vec<char> = line.chars().collect();
    for idx in indexes {
        let char = line_chars[idx];
        let substr = line.get(min(start, idx)..max(start, idx)+1).unwrap();
        if char.is_digit(10) {
            return char.to_digit(10).unwrap() as u8;
        }
        for num_word in num_words {
            if substr.contains(num_word) {
                return num_word_to_digit(&num_word);
            }
        }
    }
    return 0
}
