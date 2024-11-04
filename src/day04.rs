use std::collections::HashMap;

const INPUT: &str = include_str!("data/day04.txt");

#[derive(Clone)]
struct Card {
    card_number: u32,
    matched_numbers: Vec<u32>,
}

fn parse_cards(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in input.trim().lines() {
        // Split line into card info and numbers
        let (card_info, numbers) = line.split_once(':').unwrap();
        let (winning_numbers_text, had_numbers_text) = numbers.split_once('|').unwrap();

        // Parse card number
        let card_number = card_info
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        // Parse winning and had numbers
        let winning_numbers: Vec<u32> = winning_numbers_text
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let had_numbers: Vec<u32> = had_numbers_text
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        // Compute matching numbers as owned `u32` values
        let matched_numbers: Vec<u32> = winning_numbers
            .iter()
            .filter(|&&num| had_numbers.contains(&num))
            .cloned()
            .collect();

        // Create the Card instance
        let card = Card {
            card_number,
            matched_numbers,
        };

        cards.push(card);
    }

    cards
}

pub fn part1() -> u32 {
    let cards = parse_cards(INPUT);
    let mut sum: u32 = 0;
    for card in cards.iter() {
        let n = card.matched_numbers.len();
        if n > 0 {
            // avoid underflow
            sum += 2u32.pow(n as u32 - 1);
        }
    }
    sum
}

pub fn part2() -> u32 {
    let cards = parse_cards(INPUT);
    let mut m: HashMap<u32, u32> = HashMap::new(); // HashMap to count card instances

    for card in cards {
        let num_matches = card.matched_numbers.len();
        let key = card.card_number as u32;

        // Initialize the count for the current card if not present
        m.entry(key).or_insert(1);

        // Retrieve a copy of parent_count to avoid borrow issues
        let parent_count = *m.get(&key).unwrap();

        // Update counts for child cards
        for n in (key + 1)..=(key + num_matches as u32) {
            let child_count = m.entry(n).or_insert(1);
            *child_count += parent_count;
        }
    }

    m.values().sum()
}
