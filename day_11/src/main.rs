// use std::{fs::File, io::Write};

use day_11::{Direction, Space};
// use petgraph::{algo::dijkstra, dot::Dot, matrix_graph::NodeIndex, Graph};

fn main() {
    let input_filename = "input1";
    let input = std::fs::read_to_string(input_filename).unwrap();

    let mut space = Space::new(input);
    println! {"{:?}", space};
    space.expand();
    println! {"{:?}", space};

    let mut sum = 0;
    for pair in space.galaxy_pairs() {
        sum += space.distance(pair.0.position, pair.1.position);
    }

    println! {"{:?}", sum};
}
