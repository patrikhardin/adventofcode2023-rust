const INPUT: &str = include_str!("data/day04.txt");

struct Card {
    card_number: u32,
    winning_numbers: Vec<u32>,
    had_numbers: Vec<u32>,
    num_matched_numbers: usize,
}

fn parse_cards(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    let card_lines = input.trim().lines();

    for line in card_lines {
        let (card_info, numbers) = line.split_once(':').unwrap();
        let (winning_numbers_text, had_numbers_text) = numbers.split_once('|').unwrap();

        let card_number = card_info
            .trim()
            .strip_prefix("Card ")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);

        let winning_numbers = winning_numbers_text
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let had_numbers = had_numbers_text
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();

        let num_matched_numbers = winning_numbers
            .iter()
            .filter(|&num| had_numbers.contains(num))
            .count();

        let card = Card {
            card_number,
            winning_numbers,
            had_numbers,
            num_matched_numbers,
        };

        cards.push(card);
    }
    cards
}

pub fn part1() -> u32 {
    let cards = parse_cards(INPUT);
    let mut sum: u32 = 0;
    for card in cards.iter() {
        if card.num_matched_numbers > 0 {
            // avoid underflow
            sum += 2u32.pow(card.num_matched_numbers as u32 - 1);
        }
    }
    sum
}
