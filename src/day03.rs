const INPUT: &str = include_str!("data/day03.txt");

struct Part {
    part_number: u32,
    adjacent_to_symbol: bool,
    gear_coordinates: Option<(usize, usize)>,
}

struct Gear {
    x: usize,
    y: usize,
    part_1_index: Option<usize>,
    part_2_index: Option<usize>,
}

fn parse_schematic(schematic: Vec<Vec<char>>) -> (Vec<Part>, Vec<Gear>) {
    let mut parts: Vec<Part> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();

    // loop through the schematic
    for y in 0..schematic.len() {
        for x in 0..schematic[y].len() {
            let cell = schematic[y][x];
            // construct the gear
            if cell == '*' {
                let gear = Gear {
                    x,
                    y,
                    part_1_index: None,
                    part_2_index: None,
                };
                gears.push(gear);
            }
            // skip empty spaces
            if cell == '.' {
                continue;
            }
            // skip if value to the left is numeric (i.e., current cell is part of a number)
            if x > 0 && schematic[y][x - 1].is_ascii_digit() {
                continue;
            }
            // if value is numeric, parse the number into a Part
            else if cell.is_ascii_digit() {
                // Find its length. Increase length until it's not a digit or end of line
                let mut len = 1;
                while x + len < schematic[y].len() && schematic[y][x + len].is_ascii_digit() {
                    len += 1;
                }

                // check neighbours for symbols and gears
                let mut adjacent_to_symbol = false;
                let mut gear_coordinates: Option<(usize, usize)> = None;

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
                        let neighbor_cell: char = schematic[y_neighbor][x_neighbor];
                        if neighbor_cell == '*' {
                            gear_coordinates = Some((x_neighbor, y_neighbor));
                            adjacent_to_symbol = true;
                            break;
                        }
                        if !(neighbor_cell.is_ascii_digit() || neighbor_cell == '.') {
                            adjacent_to_symbol = true;
                            break;
                        }
                    }
                }

                // construct the part
                let part = Part {
                    part_number: schematic[y][x..x + len]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                    adjacent_to_symbol,
                    gear_coordinates,
                };
                parts.push(part);
            }
        }
    }

    (parts, gears)
}

pub fn part1() -> u32 {
    let schematic: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let (parts, _) = parse_schematic(schematic);

    parts
        .iter()
        .filter(|part| part.adjacent_to_symbol)
        .map(|part| part.part_number)
        .sum()
}

pub fn part2() -> u32 {
    let schematic: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let (parts, mut gears) = parse_schematic(schematic);

    // Loop through the gears and find the two parts that are adjacent to the gear
    for gear in gears.iter_mut() {
        for (index, part) in parts
            .iter()
            .enumerate()
            .filter(|(_, part)| part.gear_coordinates.is_some())
        {
            if let Some(coord) = part.gear_coordinates {
                if coord == (gear.x, gear.y) {
                    if gear.part_1_index.is_none() {
                        gear.part_1_index = Some(index);
                    } else if gear.part_2_index.is_none() {
                        gear.part_2_index = Some(index);
                    } else {
                        // If a third neighbor is found, invalidate the gear by setting indices to `None`
                        gear.part_1_index = None;
                        gear.part_2_index = None;
                        break;
                    }
                }
            }
        }
    }

    // Calculate the sum for gears that have exactly two neighbors
    gears
        .iter()
        .filter_map(|gear| {
            if let (Some(part_1_idx), Some(part_2_idx)) = (gear.part_1_index, gear.part_2_index) {
                let part_1 = &parts[part_1_idx];
                let part_2 = &parts[part_2_idx];
                Some(part_1.part_number * part_2.part_number)
            } else {
                None
            }
        })
        .sum()
}
