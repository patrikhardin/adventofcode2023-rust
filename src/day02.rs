const INPUT: &str = include_str!("data/day02.txt");

struct Round {
    n_red: usize,
    n_green: usize,
    n_blue: usize,
}

struct Game {
    index: usize,
    rounds: Vec<Round>,
    is_possible: bool,
}

fn parse_games(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    let game_lines = input.trim().lines();

    for line in game_lines {
        if let Some((game_info, rounds_info)) = line.split_once(':') {
            let game_index: usize = game_info.trim().strip_prefix("Game ").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
            let rounds_data: Vec<&str> = rounds_info.trim().split(';').collect::<Vec<&str>>();
            
            let mut rounds: Vec<Round> = Vec::new();
            let mut is_possible = true;  // Assume it's possible until proven otherwise

            for round_segment in rounds_data {
                let mut n_red: usize = 0;
                let mut n_green: usize = 0;
                let mut n_blue: usize = 0;

                let items: Vec<&str> = round_segment.split(',').collect::<Vec<&str>>();
                for item in items {
                    let parts: Vec<&str> = item.trim().split_whitespace().collect::<Vec<&str>>();
                    if parts.len() == 2 {
                        let count: usize = parts[0].parse().unwrap_or(0);
                        let color = parts[1].to_lowercase();

                        match color.as_str() {
                            "red" => n_red += count,
                            "green" => n_green += count,
                            "blue" => n_blue += count,
                            _ => {},
                        }
                    }
                }

                // Check if this round exceeds any of the color limits
                if n_red > 12 || n_green > 13 || n_blue > 14 {
                    is_possible = false;
                    break // No need to continue checking the rest of the round_segments
                }

                rounds.push(Round {
                    n_red,
                    n_green,
                    n_blue,
                });
            }

            games.push(Game {
                index: game_index,
                rounds: rounds,
                is_possible: is_possible,
            });
        }
    }
    games
}


pub fn part1() -> u32 {
    let games = parse_games(INPUT);

    let sum_idx_possible_games: u32 = games.iter()
        .filter(|game| game.is_possible)
        .map(|game| game.index as u32)
        .sum();

    sum_idx_possible_games
}