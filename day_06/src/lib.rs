pub fn calculate_winning_times(race_duration: usize, distance_record: usize) -> Vec<usize> {
    let mut winning_times: Vec<usize> = vec![];

    for hold_button_time in 0..=race_duration {
        let time_delta = race_duration - hold_button_time;
        let distance = hold_button_time * time_delta;

        if distance.gt(&distance_record) {
            winning_times.push(hold_button_time);
        }
    }

    winning_times
}
