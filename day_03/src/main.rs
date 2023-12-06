use day_03::Matrix;

fn main() {
    let input_filename = "input2";
    let input = std::fs::read_to_string(input_filename).unwrap();

    let mut schematic = Matrix::new(&input);

    println!("{schematic}");

    let mut result: u32 = 0;

    for number in schematic.numbers() {
        // println! {"{:?}", number};
        if !schematic.number_has_adjacent_symbol(number.0, number.1) {
            continue;
        }
        result += number.0 as u32;
    }

    println!("\n=> Result is {result}");
}
