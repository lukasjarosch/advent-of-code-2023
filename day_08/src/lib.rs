use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node(pub String);

impl Node {
    pub fn is_start_node(&self) -> bool {
        self.ends_with("A")
    }
    pub fn is_end_node(&self) -> bool {
        self.ends_with("Z")
    }
}

impl Deref for Node {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn traverse_nodes(
    current_node: &Node,
    end_nodes: &mut Vec<Node>,
    nodes: &HashMap<Node, (Node, Node)>,
    directions: &Vec<char>,
    steps: usize,
) -> usize {
    if let Some(end_node_idnex) = end_nodes.iter().position(|node| node == current_node) {
        end_nodes.remove(end_node_idnex);
        return steps;
    }

    println! {"{:?}", current_node};

    let direction_nodes = nodes.get(current_node).unwrap();
    let direction_index = steps % directions.len();

    match directions.get(direction_index).unwrap() {
        'L' => traverse_nodes(&direction_nodes.0, end_nodes, nodes, directions, steps + 1),
        'R' => traverse_nodes(&direction_nodes.1, end_nodes, nodes, directions, steps + 1),
        _ => panic!("invalid direction"),
    }
}
