const INPUT: &str = include_str!("data/day03.txt");

struct Part {
    x: usize,
    y: usize,
    len: usize,
    part_number: u32,
    adjacent_to_symbol: bool,
}

fn parse_schematic(schematic: Vec<Vec<char>>) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();

    // loop through the schematic
    for y in 0..schematic.len() {
        for x in 0..schematic[y].len() {
            let val = schematic[y][x];
            // skip empty spaces
            if val == '.' {
                continue;
            }
            // skip if previous value is numeric (i.e., is part of a number)
            if x > 0 && schematic[y][x - 1].is_ascii_digit() {
                continue;
            }
            // if value is numeric, parse the number into a Part
            else if val.is_ascii_digit() {
                // Find its length. Increase length until it's not a digit or end of line
                let mut len = 1;
                while x + len < schematic[y].len() && schematic[y][x + len].is_ascii_digit() {
                    len += 1;
                }

                // check neighbours for symbols
                let mut adjacent_to_symbol = false;

                for x_neighbor in x.saturating_sub(1)..=x + len {
                    for y_neighbor in y.saturating_sub(1)..=y + 1 {
                        // Ensure indices are within the bounds of the grid
                        if x_neighbor >= schematic[0].len() || y_neighbor >= schematic.len() {
                            continue;
                        }
                        // Skip coordinates that belong to the word itself
                        if x_neighbor >= x && x_neighbor < x + len && y_neighbor == y {
                            continue;
                        }
                        // Check if any neighboring cell contains a symbol
                        let cell: char = schematic[y_neighbor][x_neighbor];
                        if !(cell.is_ascii_digit() || cell == '.') {
                            adjacent_to_symbol = true;
                            break;
                        }
                    }
                }

                // construct the part
                let part = Part {
                    x,
                    y,
                    len,
                    part_number: schematic[y][x..x + len]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                    adjacent_to_symbol,
                };
                parts.push(part);
            }
        }
    }
    parts
}

pub fn part1() -> u32 {
    let schematic: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let parts = parse_schematic(schematic);

    parts
        .iter()
        .filter(|part| part.adjacent_to_symbol)
        .map(|part| part.part_number)
        .sum()
}

pub fn part2() -> u32 {
    todo!()
}
