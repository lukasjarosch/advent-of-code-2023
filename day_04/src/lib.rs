use std::collections::HashSet;

#[derive(Debug)]
pub struct Card {
    number: u16,
    winning_numbers: Vec<u16>,
    my_numbers: Vec<u16>,
    matching_numbers: Vec<u16>,
}

impl Card {
    pub fn new(number: u16, winning_numbers: Vec<u16>, my_numbers: Vec<u16>) -> Card {
        let winning_set: HashSet<u16> = HashSet::from_iter(winning_numbers.iter().cloned());
        let my_set: HashSet<u16> = HashSet::from_iter(my_numbers.iter().cloned());
        let my_wins = winning_set
            .intersection(&my_set.clone())
            .map(|x| *x)
            .collect();

        Card {
            number,
            winning_numbers,
            my_numbers,
            matching_numbers: my_wins,
        }
    }

    pub fn points(&self) -> u32 {
        if self.matching_numbers.len() == 0 {
            return 0;
        }
        let base: u32 = 2;
        base.pow(self.matching_numbers.len() as u32 - 1)
    }
}
