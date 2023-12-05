use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref MATCH_COLORS_REGEX: Regex = Regex::new(
        r"((?P<blue>\d+)\sblue)(,\s)?|((?P<green>\d+)\sgreen)(,\s)?|((?P<red>\d+)\sred)(,\s)?"
    )
    .unwrap();
}

fn draw_from_string(input: &str) -> (u8, u8, u8) {
    // for each capture (red, gree, blue) attempt to parse the value into an u8; otherwise assume 0
    let captures: Vec<(u8, u8, u8)> = MATCH_COLORS_REGEX
        .captures_iter(input)
        .map(|capt| {
            let red: u8 = capt.name("red").map_or(0, |c| c.as_str().parse().unwrap());
            let green: u8 = capt
                .name("green")
                .map_or(0, |c| c.as_str().parse().unwrap());
            let blue: u8 = capt.name("blue").map_or(0, |c| c.as_str().parse().unwrap());
            (red, green, blue)
        })
        .collect();

    // The captures vector holds tuples for each color and looks like this: [(0,1,0), (2,0,0), (0,0,0)]
    // Now just fold the vector by each max value yielding a single (R,G,B) tuple
    let result = captures.iter().fold((0, 0, 0), |acc, tuple| {
        (acc.0.max(tuple.0), acc.1.max(tuple.1), acc.2.max(tuple.2))
    });

    result
}

fn main() {
    let input_filename = "input2";
    let input = std::fs::read_to_string(input_filename).unwrap();

    let re_split_game_id = Regex::new(r"Game\s(?P<game_id>\d+):\s(?P<draws>[\w\d\s,;]+)$").unwrap();

    // Holds the final extracted state combining game_id with the respective draws: [(game_id), [(1,2,0), (0,0,1)]]
    // The draws are always (red, green, blue).
    let mut game_draws: Vec<(u8, Vec<(u8, u8, u8)>)> = Vec::new();

    for line in input.lines() {
        let game = re_split_game_id.captures(line).map(|cap| {
            let game_id: u8 = cap.name("game_id").unwrap().as_str().parse().unwrap();
            let draws_raw: &str = cap.name("draws").unwrap().as_str();

            let draws: Vec<(u8, u8, u8)> = draws_raw
                .split(";")
                .map(|draw| draw_from_string(draw))
                .collect();

            // println! {"{game_id} {:?}", draws};

            (game_id, draws)
        });

        match game {
            Some(game) => game_draws.push(game),
            None => continue,
        }
    }

    // println! {"{:?}", game_draws};

    let check_red = 12;
    let check_green = 13;
    let check_blue = 14;
    let mut result_part1: u16 = 0;

    // Part 1: Filter the game state by invalid games and add the IDs of valid games
    for (game_id, draws) in &game_draws {
        let invalid_draws: Vec<(u8, u8, u8)> = draws
            .iter()
            .map(|draw| *draw)
            .filter(|draw| draw.0 > check_red || draw.1 > check_green || draw.2 > check_blue)
            .collect();

        if invalid_draws.len() > 0 {
            println! {"Game #{game_id} has invalid draws {:?}", draws};
        } else {
            println! {"Game #{game_id} is valid"};
            result_part1 += *game_id as u16;
        }
    }
    println! {"=> Result for part 1 is: {result_part1}"};

    // Part 2: Find the minimum cube count for each game
    let mut result_part2: u32 = 0;
    for (game_id, draws) in &game_draws {
        let min_cube_count = draws.iter().fold((0, 0, 0), |acc, tuple| {
            (acc.0.max(tuple.0), acc.1.max(tuple.1), acc.2.max(tuple.2))
        });

        println!(
            "Game #{game_id} requires at least {} red, {} green and {} blue cubes to work",
            min_cube_count.0, min_cube_count.1, min_cube_count.2
        );

        let power: u32 =
            min_cube_count.0 as u32 * min_cube_count.1 as u32 * min_cube_count.2 as u32;
        println! {"{:?}", power};

        result_part2 += power;
    }

    println!("=> Result for part 2 is: {result_part2}");
}
