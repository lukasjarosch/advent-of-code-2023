use day_06::calculate_winning_times;

fn main() {
    let input_filename = "input2";
    let input = std::fs::read_to_string(input_filename).unwrap();

    let distances: Vec<usize> = input
        .lines()
        .last()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let races: Vec<(usize, usize)> = input
        .lines()
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .zip(distances.into_iter())
        .collect();

    let mut margins: Vec<usize> = vec![];
    for (race_duration, distance_record) in races {
        margins.push(calculate_winning_times(race_duration, distance_record).len());
    }

    let result: usize = margins.iter().product();
    println! {"=> part 1 result: {result}"};

    // part 2

    let race_distance_record: usize = input
        .lines()
        .last()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();
    let race_time: usize = input
        .lines()
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    let winnings = calculate_winning_times(race_time, race_distance_record).len();
    println! {"=> part 2 result: {winnings}"};
}
