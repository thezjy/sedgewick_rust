use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use super::parse_graph_params;

pub struct Graph {
    adj_list: Vec<Vec<usize>>,
    num_edges: usize,
}

impl Graph {
    pub fn new(num_vertices: usize) -> Self {
        Graph {
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
        self.adj_list[w].push(v);
        self.num_edges += 1;
    }

    pub fn adj(&self, v: usize) -> std::slice::Iter<usize> {
        self.adj_list[v].iter()
    }

    pub fn degree(&self, v: usize) -> usize {
        self.adj_list[v].len()
    }

    pub fn max_degree(&self) -> usize {
        let mut max_degree = 0;

        for i in 0..self.num_vertices() {
            let i_degree = self.degree(i);
            if i_degree > max_degree {
                max_degree = i_degree
            }
        }

        max_degree
    }

    fn average_degree(&self) -> usize {
        self.num_edges() * 2 / self.num_vertices()
    }
}

impl Debug for Graph {
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

pub trait Search {
    /// find vertices connected to a source vertex s
    fn new(g: &Graph, s: usize) -> Self;

    /// is v connected to s?
    fn marked(&self, v: usize) -> bool;

    /// how many vertices are connected to s?
    fn count(&self) -> usize;
}

pub struct DepthFirstSearch {
    marked: Vec<bool>,
    count: usize,
}

impl DepthFirstSearch {
    fn visit(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        self.count += 1;

        g.adj(v).for_each(|&w| {
            if !self.marked[w] {
                self.visit(&g, w);
            }
        });
    }
}

impl Search for DepthFirstSearch {
    fn new(g: &Graph, s: usize) -> Self {
        let mut dfs = DepthFirstSearch {
            marked: vec![false; g.num_vertices()],
            count: 0,
        };

        dfs.visit(&g, s);

        dfs
    }

    fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }

    fn count(&self) -> usize {
        self.count
    }
}

pub trait Paths {
    /// find paths in G from source s
    fn new(g: &Graph, s: usize) -> Self;

    /// path from s to v; null if no such path
    fn path_to(&self, v: usize) -> Option<Vec<usize>>;
}

#[derive(Debug)]
pub struct DepthFirstPaths {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    s: usize,
}

impl DepthFirstPaths {
    /// Can we implement a non-recursive depth-first visit with a
    /// stack instead of using the function call stack?
    fn visit(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;

        g.adj(v).for_each(|&w| {
            if !self.marked[w] {
                self.edge_to[w] = Some(v);
                self.visit(g, w);
            }
        });
    }
}

impl Paths for DepthFirstPaths {
    fn new(g: &Graph, s: usize) -> Self {
        let num_vertices = g.num_vertices();

        let mut dfp = DepthFirstPaths {
            marked: vec![false; num_vertices],
            edge_to: vec![None; num_vertices],
            s,
        };

        dfp.visit(g, s);

        dfp
    }

    fn path_to(&self, mut v: usize) -> Option<Vec<usize>> {
        if !self.marked[v] {
            return None;
        }

        let mut path = Vec::new();

        while let Some(from) = self.edge_to[v] {
            path.push(v);
            v = from;
        }

        path.push(self.s);

        path.reverse();

        Some(path)
    }
}

#[derive(Debug)]
pub struct BreadthFirstPaths {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    s: usize,
}

impl BreadthFirstPaths {
    fn visit(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        let mut queue = VecDeque::new();
        queue.push_back(v);

        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();

            g.adj(v).for_each(|&w| {
                if !self.marked[w] {
                    self.edge_to[w] = Some(v);
                    self.marked[w] = true;
                    queue.push_back(w);
                }
            });
        }
    }
}

impl Paths for BreadthFirstPaths {
    fn new(g: &Graph, s: usize) -> Self {
        let num_vertices = g.num_vertices();

        let mut dfp = BreadthFirstPaths {
            marked: vec![false; num_vertices],
            edge_to: vec![None; num_vertices],
            s,
        };

        dfp.visit(g, s);

        dfp
    }

    fn path_to(&self, mut v: usize) -> Option<Vec<usize>> {
        if !self.marked[v] {
            return None;
        }

        let mut path = Vec::new();

        while let Some(from) = self.edge_to[v] {
            path.push(v);
            v = from;
        }

        path.push(self.s);

        path.reverse();

        Some(path)
    }
}

fn create_graph_from_path<P: AsRef<Path>>(path: P) -> Graph {
    let (num_vertices, edges) = parse_graph_params(path);

    let mut g = Graph::new(num_vertices);

    edges.into_iter().for_each(|(a, b)| {
        g.add_edge(a, b);
    });

    g
}

#[cfg(test)]
mod tests {
    use std::io::stdin;

    use crate::graph::print_path;

    use super::*;

    #[test]
    fn graph() {
        let mut g = create_graph_from_path("./algs4-data/tinyG.txt");
        dbg!(g);
    }

    #[test]
    fn dfs() {
        let mut g = create_graph_from_path("./algs4-data/tinyG.txt");
        let dfs = DepthFirstSearch::new(&g, 0);

        (0..g.num_vertices())
            .filter(|v| dfs.marked(*v))
            .for_each(|v| print!("{} ", v));

        println!();

        if dfs.count() == g.num_vertices() {
            println!("connected");
        } else {
            println!("not connected");
        }
    }

    #[test]
    fn dfp() {
        let mut g = create_graph_from_path("./algs4-data/tinyG.txt");
        let s = 0;
        let dfp = DepthFirstPaths::new(&g, s);

        (0..g.num_vertices()).for_each(|v| {
            if let Some(path) = dfp.path_to(v) {
                print_path(&path);
            }
        });
    }

    #[test]
    fn bfp() {
        let mut g = create_graph_from_path("./algs4-data/tinyCG.txt");
        let s = 0;
        let bfp = BreadthFirstPaths::new(&g, s);

        (0..g.num_vertices()).for_each(|v| {
            if let Some(path) = bfp.path_to(v) {
                print_path(&path);
            }
        });
    }
}
