use std::cmp::min;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("File not found.");
    let cards = get_cards(file_contents.lines().collect());

    println!("Cards -> {}", cards);
}

fn get_cards(lines: Vec<&str>) -> u32 {
    let mut card_counts = vec![1; lines.len()];
    let mut curr_card = 1;
    for line in lines {
        println!("Working with '{}'", line);
        let curr_card_qty = card_counts[curr_card - 1];
        if curr_card_qty == 0 {
            break;
        }

        let no_card: &str = line.split(": ").skip(1).next().unwrap();
        let mut split = no_card.split(" | ");
        let (winners_str, card_str) = (split.next().unwrap(), split.next().unwrap());
        let winners: Vec<u8> = winners_str
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        let card: Vec<u8> = card_str
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        let matches = winners
            .into_iter()
            .filter(|x| card.contains(x))
            .collect::<Vec<u8>>()
            .len();
        println!("Found {} matches", matches);
        println!(
            "Setting values from {} to {} with {}",
            curr_card,
            curr_card + min(matches, card_counts.len() - 1),
            curr_card_qty
        );
        for card_idx in 1..=min(matches, card_counts.len() - 1) {
            card_counts[curr_card + card_idx - 1] += curr_card_qty;
        }
        dbg!(&card_counts);
        curr_card += 1;
    }

    return card_counts.iter().sum();
}
