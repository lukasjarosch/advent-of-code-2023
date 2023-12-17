use std::{fs::File, io::Write};

use day_10::{Direction, Field, Tile};
use petgraph::{dot::Dot, matrix_graph::NodeIndex, visit::Dfs, Graph};

fn main() -> Result<(), std::io::Error> {
    let input_filename = "input2";
    let input = std::fs::read_to_string(input_filename).unwrap();

    let len = input.lines().into_iter().next().unwrap().len();
    let height = input.lines().count();

    let mut field: Field = Field(vec![vec![Tile::Ground; len]; height]);
    for (i, line) in input.lines().enumerate() {
        field[i] = line.chars().map(|c| Tile::from_char(c)).collect();
    }

    println! {"{:?}", field};

    let mut start_node_index = NodeIndex::new(0);
    let mut graph = Graph::<char, Direction>::new();
    for (i, row) in field.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let field = field[i][j];
            let idx = graph.add_node(field.value());
            if field == Tile::StartPosition {
                start_node_index = idx;
            }
        }
    }

    let mut current_tile_index = 0;
    for (i, row) in field.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let current_tile = field[i][j];
            let current_node_index = NodeIndex::new(current_tile_index);

            // if there is a tile north of the current tile, add an edge between
            // the two nodes if it can connect with the current tile
            if let Some(north) = i.checked_sub(1) {
                let tile_north = field[north][j];
                if Tile::can_connect(current_tile, tile_north, Direction::North) {
                    let node_north_index = NodeIndex::new(field.tile_index(north, j));
                    graph.add_edge(current_node_index, node_north_index, Direction::North);
                }
            }
            if let Some(east) = if (j + 1) < len { Some(j + 1) } else { None } {
                let tile_east = field[i][east];
                if Tile::can_connect(current_tile, tile_east, Direction::East) {
                    let node_east_index = NodeIndex::new(field.tile_index(i, east));
                    graph.add_edge(current_node_index, node_east_index, Direction::East);
                }
            }
            if let Some(south) = if (i + 1) < height { Some(i + 1) } else { None } {
                let tile_south = field[south][j];
                if Tile::can_connect(current_tile, tile_south, Direction::South) {
                    let node_south_index = NodeIndex::new(field.tile_index(south, j));
                    graph.add_edge(current_node_index, node_south_index, Direction::South);
                }
            }
            if let Some(west) = j.checked_sub(1) {
                let tile_west = field[i][west];
                if Tile::can_connect(current_tile, tile_west, Direction::West) {
                    let node_west_index = NodeIndex::new(field.tile_index(i, west));
                    graph.add_edge(current_node_index, node_west_index, Direction::West);
                }
            }

            current_tile_index += 1;
        }
    }

    // perform DFS starting from the startnode
    // Because there is only one loop, this will always give the correct result.
    let mut dfs = Dfs::new(&graph, start_node_index);
    let mut steps = 0;
    while let Some(_) = dfs.next(&graph) {
        steps += 1;
    }

    let result = (steps as f64) / (2 as f64);
    println! {"{:?}", steps};
    println! {"{:?}", result.ceil()};

    let mut dot_file = File::create("/tmp/graph.dot").unwrap();
    dot_file
        .write_fmt(format_args!("{:?}", Dot::with_config(&graph, &[])))
        .unwrap();

    Ok(())
}
