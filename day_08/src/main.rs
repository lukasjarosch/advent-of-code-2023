use num::integer::lcm;
use std::{collections::HashMap, usize};

use day_08::{traverse_nodes, Node};
use regex::Regex;

fn main() {
    let input_filename = "input2";
    let input = std::fs::read_to_string(input_filename).unwrap();
    let re = Regex::new(r"(?P<start>\w+)\s+=\s+\((?P<left>\w+),\s+(?P<right>\w+)\)").unwrap();

    let directions: Vec<char> = input.lines().next().unwrap().chars().collect();

    let mut nodes: HashMap<Node, (Node, Node)> = HashMap::new();
    for line in input.lines().skip(1) {
        for capt in re.captures_iter(line) {
            let start: Node = Node(capt.name("start").unwrap().as_str().to_string());
            let left: Node = Node(capt.name("left").unwrap().as_str().to_string());
            let right: Node = Node(capt.name("right").unwrap().as_str().to_string());

            nodes.insert(start, (left, right));
        }
    }

    // part 1
    // let mut steps = 0;
    // let mut path: Vec<Node> = vec![Node("AAA".to_string())];
    // loop {
    //     let direction_index = steps % directions.len();
    //
    //     let current_node = path.iter().last().unwrap();
    //     let direction_nodes = nodes.get(&current_node).unwrap();
    //
    //     if *current_node == Node("ZZZ".to_string()) {
    //         println! {"FINISHED after {} steps", steps};
    //         break;
    //     }
    //
    //     let direction = directions.get(direction_index).unwrap();
    //     match direction {
    //         'L' => path.push(direction_nodes.0.clone()),
    //         'R' => path.push(direction_nodes.1.clone()),
    //         _ => panic!("unknown direction"),
    //     }
    //
    //     steps += 1;
    // }

    // part 2
    // NOTE: this will only work in 'release' mode or you'll run in a stack overflow because of the
    // recursion (TCE). Ideally I would've solved that part iteratively as well but I wanted to give it a
    // shot.
    let start_nodes: Vec<Node> = nodes
        .keys()
        .map(|node| node.clone())
        .filter(|node| node.is_start_node())
        .collect();
    let mut end_nodes: Vec<Node> = nodes
        .keys()
        .map(|node| node.clone())
        .filter(|node| node.is_end_node())
        .collect();

    let mut steps: Vec<usize> = vec![];
    for start_node in start_nodes {
        steps.push(traverse_nodes(
            &start_node,
            &mut end_nodes,
            &nodes,
            &directions,
            0,
        ));
    }

    let mut result: usize = steps.pop().unwrap();
    while steps.len() > 0 {
        result = lcm(result, steps.pop().unwrap())
    }

    println! {"=> Result is {result}"};
}
