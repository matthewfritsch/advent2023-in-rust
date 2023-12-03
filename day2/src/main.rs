use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("File not found.");

    let sum = valid_indexes(file_contents);
    println!("{sum}");
}

fn valid_indexes(file_contents: String) -> u32 {
    let mut sum: u32 = 0;
    let r_max: u16 = 12;
    let g_max: u16 = 13;
    let b_max: u16 = 14;

    for line in file_contents.lines() {
        sum += u32::from(process_game(line, r_max, g_max, b_max));
    }

    sum
}

fn process_game(line: &str, r: u16, g: u16, b: u16) -> u16 {
    let game_and_info: Vec<&str> = line.split(':').collect();
    if game_and_info.len() < 2 {
        return 0
    }
    let game_number = game_and_info[0]
        .split(' ')
        .collect::<Vec<&str>>()[1]
        .parse::<u16>()
        .expect("Was hoping for a game number here...");

    for stage in game_and_info[1].split(';').collect::<Vec<&str>>() {
        if !stage_is_valid(stage, r, g, b) {
            return 0;
        }
    }

    game_number
}

fn stage_is_valid(stage: &str, r: u16, g: u16, b: u16) -> bool {
    let mut r_qty: u16 = 0;
    let mut g_qty: u16 = 0;
    let mut b_qty: u16 = 0;
    for ball_shown in stage.split(',').collect::<Vec<&str>>() {
        let qty_and_colour: Vec<&str> = ball_shown.split(' ').collect();
        let qty = qty_and_colour[1].parse::<u8>().unwrap();
        let colour = qty_and_colour[2];

        match colour {
            "red" => r_qty += u16::from(qty),
            "green" => g_qty += u16::from(qty),
            "blue" => b_qty += u16::from(qty),
            _ => return false
        }
    }
    r_qty <= r && g_qty <= g && b_qty <= b
}
