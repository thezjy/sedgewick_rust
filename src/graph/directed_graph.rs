use core::num;
use std::{collections::VecDeque, fmt::Debug, path::Path};

use super::parse_graph_params;

pub struct Digraph {
    adj_list: Vec<Vec<usize>>,
    num_edges: usize,
}

impl Digraph {
    pub fn vertices(&self) -> impl Iterator<Item = usize> {
        (0..self.num_vertices())
    }

    pub fn new(num_vertices: usize) -> Self {
        Digraph {
            adj_list: vec![vec![]; num_vertices],
            num_edges: 0,
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.adj_list.len()
    }

    pub fn num_edges(&self) -> usize {
        self.num_edges
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj_list[v].push(w);
        self.num_edges += 1;
    }

    /// rev because the textbook use linked-list stack
    pub fn adj(&self, v: usize) -> impl Iterator<Item = &usize> {
        self.adj_list[v].iter().rev()
    }

    pub fn reverse(&self) -> Self {
        let mut reversed_g = Self::new(self.num_vertices());

        (0..self.num_vertices()).for_each(|v| {
            self.adj(v).for_each(|&w| {
                reversed_g.add_edge(w, v);
            });
        });

        reversed_g
    }
}

impl Debug for Digraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} vertices, {} edges",
            self.num_vertices(),
            self.num_edges()
        );

        (0..self.num_vertices()).for_each(|v| {
            write!(f, "{}: ", v);

            self.adj(v).for_each(|w| {
                write!(f, "{} ", w);
            });

            write!(f, "\n");
        });

        Ok(())
    }
}

struct DirectedDFS {
    marked: Vec<bool>,
}

impl DirectedDFS {
    fn search(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;
        g.adj(v).for_each(|&w| {
            if !self.marked(w) {
                self.search(g, w);
            }
        });
    }

    fn new(g: &Digraph, s: usize) -> Self {
        let num_vertices = g.num_vertices();

        let mut dfs = DirectedDFS {
            marked: vec![false; num_vertices],
        };

        dfs.search(&g, s);

        dfs
    }

    fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }
}

#[derive(Debug)]
struct DirectedDFP {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
}

impl DirectedDFP {
    fn search(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;

        g.adj(v).for_each(|&w| {
            if !self.marked[w] {
                self.edge_to[w] = Some(v);
                self.search(g, w);
            }
        });
    }

    fn new(g: &Digraph, s: usize) -> Self {
        let num_vertices = g.num_vertices();

        let mut dfp = DirectedDFP {
            marked: vec![false; num_vertices],
            edge_to: vec![None; num_vertices],
        };

        dfp.search(g, s);

        dfp
    }

    fn path_to(&self, mut v: usize) -> Option<Vec<usize>> {
        if !self.marked[v] {
            return None;
        }

        let mut path = vec![];

        path.push(v);

        while let Some(from) = self.edge_to[v] {
            path.push(from);
            v = from;
        }

        path.reverse();

        Some(path)
    }
}

#[derive(Debug)]
struct DirectedBFP {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
}

impl DirectedBFP {
    fn search(&mut self, g: &Digraph, v: usize) {
        let mut unvisited = VecDeque::new();

        unvisited.push_back(v);

        while let Some(v) = unvisited.pop_front() {
            g.adj(v).for_each(|&w| {
                if !self.marked[w] {
                    self.marked[w] = true;
                    self.edge_to[w] = Some(v);
                    unvisited.push_back(w);
                }
            });
        }
    }

    fn new(g: &Digraph, s: usize) -> Self {
        let num_vertices = g.num_vertices();

        let mut bfp = DirectedBFP {
            marked: vec![false; num_vertices],
            edge_to: vec![None; num_vertices],
        };

        bfp.search(g, s);

        bfp
    }

    fn path_to(&self, mut v: usize) -> Option<Vec<usize>> {
        if !self.marked[v] {
            return None;
        }

        let mut path = vec![];

        path.push(v);

        while let Some(from) = self.edge_to[v] {
            path.push(from);
            v = from;
        }

        path.reverse();

        Some(path)
    }
}

#[derive(Debug)]
pub struct DirectedCycle {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    on_stack: Vec<bool>,
    pub cycle: Option<Vec<usize>>,
}

impl DirectedCycle {
    fn search(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;
        self.on_stack[v] = true;

        g.adj(v).for_each(|&w| {
            if self.cycle.is_some() {
                return;
            } else if !self.marked[w] {
                self.edge_to[w] = Some(v);
                self.search(g, w);
            } else if self.on_stack[w] {
                let mut cycle = vec![];

                let mut x = v;

                while x != w {
                    cycle.push(x);
                    x = self.edge_to[x].unwrap();
                }

                cycle.push(w);
                cycle.push(v);

                self.cycle = Some(cycle);
            }
        });

        self.on_stack[v] = false;
    }

    pub fn new(g: &Digraph) -> Self {
        let num_vertices = g.num_vertices();

        let mut dc = DirectedCycle {
            marked: vec![false; num_vertices],
            edge_to: vec![None; num_vertices],
            on_stack: vec![false; num_vertices],
            cycle: None,
        };

        g.vertices().for_each(|v| {
            if !dc.marked[v] {
                dc.search(g, v);
            }
        });

        dc
    }
}

pub fn create_digraph_from_path<P: AsRef<Path>>(path: P) -> Digraph {
    let (num_vertices, edges) = parse_graph_params(path);

    let mut g = Digraph::new(num_vertices);

    edges.into_iter().for_each(|(a, b)| {
        g.add_edge(a, b);
    });

    g
}

#[cfg(test)]
mod tests {
    use crate::graph::{print_path, undirected_graph::DepthFirstSearch};

    use super::*;

    #[test]
    fn create() {
        let g = create_digraph_from_path("./algs4-data/tinyDG.txt");
        dbg!(&g);
        dbg!(g.reverse());
    }

    #[test]
    fn dfs() {
        let mut g = create_digraph_from_path("./algs4-data/tinyDG.txt");

        let dfs = DirectedDFS::new(&g, 6);

        (0..g.num_vertices())
            .filter(|&v| dfs.marked(v))
            .for_each(|v| print!("{} ", v));

        println!();
    }

    #[test]
    fn path() {
        let mut g = create_digraph_from_path("./algs4-data/tinyDG.txt");

        let dfp = DirectedDFP::new(&g, 7);

        (0..g.num_vertices()).for_each(|v| {
            if let Some(path) = dfp.path_to(v) {
                print_path(&path);
            }
        });

        println!();

        let bfp = DirectedBFP::new(&g, 7);

        (0..g.num_vertices()).for_each(|v| {
            if let Some(path) = bfp.path_to(v) {
                print_path(&path);
            }
        });

        println!();
    }

    #[test]
    fn cycle() {
        let g = create_digraph_from_path("./algs4-data/mediumDG.txt");

        let dc = DirectedCycle::new(&g);

        if let Some(cycle) = dc.cycle {
            print_path(&cycle);
        } else {
            println!("no cycle");
        }
    }
}
