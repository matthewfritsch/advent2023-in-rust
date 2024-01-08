
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("File not found.");
    let input_text: Vec<&str> = file_contents.lines().collect();
    let mut points: u32 = 0;
    for line in input_text {
        let no_card: &str = line.split(": ").skip(1).next().unwrap();
        let mut split = no_card.split(" | ");
        let (winners_str, card_str) = (split.next().unwrap(), split.next().unwrap());
        let winners: Vec<u8> = winners_str.split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect();
        let card: Vec<u8> = card_str.split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect();
        let matches: Vec<u8> = winners.into_iter().filter(|x| card.contains(x)).collect();
        if matches.len() == 0 {
            continue;
        }
        points += 2u32.pow(matches.len() as u32 - 1);
    }

    println!("Points -> {}", points);
}
