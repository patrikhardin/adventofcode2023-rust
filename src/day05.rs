const INPUT: &str = include_str!("data/day05.txt");

struct Map {
    map_table: Vec<Vec<u64>>,
}

fn parse_inputs(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut maps: Vec<Map> = Vec::new();

    // Split input into blocks
    let blocks = input.split("\n\n").collect::<Vec<&str>>();

    // Parse seeds
    let seeds = blocks[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // Parse Maps
    for block in blocks.iter().skip(1) {
        let lines = block.lines();
        let map_table: Vec<Vec<u64>> = lines
            .skip(1)
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect()
            })
            .collect();

        maps.push(Map { map_table });
    }
    (seeds, maps)
}

/// Find the position of the seed value after propagating it through the maps
fn find_position(seed: u64, maps: &Vec<Map>) -> u64 {
    // Initialize the "next" value as the seed
    let mut next: u64 = seed;

    // Propagate the seed value through the maps
    for map in maps.iter() {
        for row in &map.map_table {
            // Extract the range values
            let destination_range_start = row[0];
            let source_range_start = row[1];
            let range_length = row[2];

            // Check if the "next" falls into the source ranges.
            // If so, compute the destination value
            // Next value is the same as the seed if no range is found
            if source_range_start <= next && next < source_range_start + range_length {
                // Compute the destination value
                next = destination_range_start + (next - source_range_start);
                break;
            }
        }
    }
    next
}

pub fn part1() -> u64 {
    let (seeds, maps) = parse_inputs(INPUT);
    let mut positions = Vec::new();
    for seed in seeds {
        positions.push(find_position(seed, &maps));
    }
    *positions.iter().min().unwrap()
}
