// Embed the input file in the binary
const INPUT: &str = include_str!("data/day01.txt");

pub fn part1() -> u32 {
    INPUT
        .lines()
        .filter_map(|line| { // Filter out lines that don't contain digits
            // Find the first and last digits in the line
            let first_digit = line.chars().find(|c| c.is_digit(10))?;
            let last_digit = line.chars().rev().find(|c| c.is_digit(10))?;

            // Convert and concatenate the digits if found
            let first_value = first_digit.to_digit(10)?;
            let last_value = last_digit.to_digit(10)?;
            Some(first_value * 10 + last_value) // Return the result, if both digits are found. Otherwise, return None
        })
        .sum()
}

pub fn part2() -> u32 {
    const DIGIT_WORDS: [&str; 10] = [
        "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine"
    ];
    
    INPUT
        .lines()
        .map(|line| {
            let digit_list = find_all_digits(line, &DIGIT_WORDS);
            digit_list[0] * 10 + digit_list[digit_list.len() - 1]
        })
        .sum()
}

// Function to find all valid digits (both numeric and text-based) in a line
fn find_all_digits(line: &str, digit_words: &[&str]) -> Vec<u32> {
    let mut digits = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i].is_digit(10) {
            // If the character is a numeric digit, add it to the list
            if let Some(digit) = chars[i].to_digit(10) {
                digits.push(digit);
            }
        } else if chars[i].is_alphabetic() {
            // Check if the current position starts with a digit word
            let remaining_line = &line[i..];
            if let Some((index, &_word)) = digit_words
                .iter()
                .enumerate()
                .find(|&(_, &word)| remaining_line.starts_with(word))
            {
                // If a match is found, add the digit and skip the word length
                digits.push(index as u32);
            }
        }
        i += 1; // Move forward one step
    }

    digits
}