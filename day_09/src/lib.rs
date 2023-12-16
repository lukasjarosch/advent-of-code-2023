pub fn history_diff(input: &Vec<isize>) -> Vec<isize> {
    let mut diff = vec![];
    for (a, b) in input.iter().zip(input.iter().skip(1)) {
        diff.push(b - a);
    }

    diff
}

pub fn is_zero_vec(input: &Vec<isize>) -> bool {
    if let Some(_) = input.iter().find(|x| **x != 0) {
        return false;
    }
    return true;
}

pub fn mutate_until_zero(history: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut permutations: Vec<Vec<isize>> = vec![history.clone()];

    loop {
        let current = &permutations.iter().last().unwrap();
        if is_zero_vec(current) {
            break;
        }
        permutations.push(history_diff(current));
    }

    permutations
}

pub fn predict_values_right(permutations: &Vec<Vec<isize>>) -> Vec<isize> {
    let mut predictions: Vec<isize> = vec![0; permutations.len()];
    let mut previous_prediction: isize = 0;

    for (i, permutation) in permutations.iter().enumerate().rev() {
        predictions[i] = permutation.last().unwrap() + previous_prediction;
        previous_prediction = predictions[i];
    }

    predictions
}

pub fn predict_values_left(permutations: &Vec<Vec<isize>>) -> Vec<isize> {
    let mut predictions: Vec<isize> = vec![0; permutations.len()];
    let mut previous_prediction: isize = 0;

    for (i, permutation) in permutations.iter().enumerate().rev() {
        predictions[i] = permutation.first().unwrap() - previous_prediction;
        previous_prediction = predictions[i];
    }

    predictions
}
