use std::fs;

struct GameResults {
    sum: u32,
    powers: u32,
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("File not found.");

    let game_results = get_results(file_contents);

    println!("Sum: {}", game_results.sum);
    println!("Powers: {}", game_results.powers);
}

fn get_results(file_contents: String) -> GameResults {
    GameResults {
        sum: valid_indexes(file_contents.clone()),
        powers: all_powers_per_game(file_contents.clone()),
    }
}

fn valid_indexes(file_contents: String) -> u32 {
    let mut sum: u32 = 0;
    let r_max: u16 = 12;
    let g_max: u16 = 13;
    let b_max: u16 = 14;

    for line in file_contents.lines() {
        sum += u32::from(valid_game_id(line, r_max, g_max, b_max));
    }

    sum
}

fn all_powers_per_game(file_contents: String) -> u32 {
    let mut sum: u32 = 0;
    for line in file_contents.lines() {
        sum += min_items_per_game(line);
    }

    sum
}

fn valid_game_id(line: &str, r: u16, g: u16, b: u16) -> u16 {
    let game_and_info: Vec<&str> = line.split(':').collect();
    if game_and_info.len() < 2 {
        return 0;
    }
    let game_number = game_and_info[0].split(' ').collect::<Vec<&str>>()[1]
        .parse::<u16>()
        .expect("Was hoping for a game number here...");

    for stage in game_and_info[1].split(';').collect::<Vec<&str>>() {
        if !stage_is_valid(stage, r, g, b) {
            return 0;
        }
    }

    game_number
}

fn min_items_per_game(game: &str) -> u32 {
    let mut sums = 0;

    let game_and_info: Vec<&str> = game.split(':').collect();

    if game_and_info.len() < 2 {
        return 0;
    }

    let mut min_r = 0;
    let mut min_g = 0;
    let mut min_b = 0;

    for stage in game_and_info[1].split(';').collect::<Vec<&str>>() {
        for ball_shown in stage.split(',').collect::<Vec<&str>>() {
            let qty_and_colour: Vec<&str> = ball_shown.split(' ').collect();
            let qty = qty_and_colour[1].parse::<u32>().unwrap();
            let colour = qty_and_colour[2];

            if colour == "red" && qty > min_r {
                min_r = qty;
            } else if colour == "green" && qty > min_g {
                min_g = qty;
            } else if colour == "blue" && qty > min_b {
                min_b = qty;
            }
        }
    }
    sums += (min_r * min_g) * min_b;
    return sums;
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
            _ => return false,
        }
    }
    r_qty <= r && g_qty <= g && b_qty <= b
}
