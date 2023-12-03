use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("File not found.");

    let mut sum: u32 = 0;

    for line in contents.lines() {
        let mut seenfirst = false;
        let mut first = 0;
        let mut last = 0;
        for char in line.chars() {
            if char.is_digit(10) {
                if !seenfirst {
                    seenfirst = true;
                    first = char.to_digit(10).unwrap();
                }
                last = char.to_digit(10).unwrap();
            }
        }
        sum += (first * 10) + last;
    }
    println!("{sum}");
}
