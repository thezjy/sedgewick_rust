use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use itertools::Itertools;

pub mod directed_graph;
pub mod directed_symbol_graph;
pub mod strong_connected;
pub mod topological;
pub mod undirected_graph;
pub mod undirected_symbol_graph;

pub fn parse_graph_params<P: AsRef<Path>>(path: P) -> (usize, Vec<(usize, usize)>) {
    let mut input = String::new();
    File::open(path).unwrap().read_to_string(&mut input);

    let mut args = input.split_ascii_whitespace();

    let num_vertices = args.next().unwrap().parse().unwrap();

    let edges = args
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .tuples()
        .collect();

    (num_vertices, edges)
}

pub fn print_path(path: &[usize]) {
    println!(
        "{} to {}: {}",
        path.first().unwrap(),
        path.last().unwrap(),
        path.iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("-")
    );
}
