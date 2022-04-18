use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::graph::undirected_graph::Graph;

#[derive(Debug)]
pub struct SymbolGraph {
    g: Graph,
    names: Vec<String>,
    indices: HashMap<String, usize>,
}

impl SymbolGraph {
    pub fn new<'a>(edges: impl Clone + Iterator<Item = impl Iterator<Item = &'a str>>) -> Self {
        let mut indices = HashMap::new();

        let mut another_edges = edges.clone();

        edges.for_each(|vertices| {
            vertices.for_each(|v| {
                let len = indices.len();
                indices.entry(v.to_string()).or_insert(len);
            })
        });

        let mut names = vec![Default::default(); indices.len()];

        indices
            .iter()
            .for_each(|(name, index)| names[*index] = name.to_string());

        let mut g = Graph::new(indices.len());

        another_edges.clone().for_each(|mut vertices| {
            let first = vertices.next().unwrap();
            let v = indices[first];

            vertices.for_each(|w| {
                g.add_edge(v, indices[w]);
            })
        });

        SymbolGraph { g, names, indices }
    }

    pub fn index(&self, name: &str) -> usize {
        self.indices[name]
    }

    pub fn name(&self, index: usize) -> &str {
        &self.names[index]
    }

    pub fn contains(&self, name: &str) -> bool {
        self.indices.contains_key(name)
    }

    pub fn g(&self) -> &Graph {
        &self.g
    }
}

pub fn create_symbol_graph<T: AsRef<Path>>(path: T, delim: &str) -> SymbolGraph {
    let file = File::open(path).unwrap();

    let mut reader = BufReader::new(file);

    let mut input = String::new();

    reader.read_to_string(&mut input);

    let mut edges = input.lines().map(|l| l.split(&delim));

    let sg = SymbolGraph::new(edges);

    sg
}
