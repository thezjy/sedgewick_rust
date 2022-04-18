use core::num;
use std::collections::VecDeque;

use super::directed_graph::{Digraph, DirectedCycle};

pub struct DepthFirstOrder {
    pre: Vec<usize>,
    post: Vec<usize>,
    marked: Vec<bool>,
}

impl DepthFirstOrder {
    fn search(&mut self, g: &Digraph, v: usize) {
        self.pre.push(v);
        self.marked[v] = true;

        g.adj(v).for_each(|&w| {
            if !self.marked[w] {
                self.search(g, w);
            }
        });

        self.post.push(v);
    }

    pub fn new(g: &Digraph) -> Self {
        let num_vertices = g.num_vertices();

        let mut dfo = DepthFirstOrder {
            pre: Vec::with_capacity(num_vertices),
            post: Vec::with_capacity(num_vertices),
            marked: vec![false; num_vertices],
        };

        g.vertices().for_each(|v| {
            if !dfo.marked[v] {
                dfo.search(g, v);
            }
        });

        dfo
    }

    pub fn reverse_post(&self) -> Vec<usize> {
        let mut result = self.post.clone();

        result.reverse();

        result
    }
}

#[derive(Debug)]
struct Topological {
    order: Option<Vec<usize>>,
}

impl Topological {
    fn new(g: &Digraph) -> Self {
        let dc = DirectedCycle::new(g);

        let order = if dc.cycle.is_none() {
            let dfo = DepthFirstOrder::new(g);

            Some(dfo.reverse_post())
        } else {
            None
        };

        Topological { order }
    }
}

#[cfg(test)]
mod tests {

    use crate::graph::directed_symbol_graph::{create_symbol_digraph, SymbolDigraph};

    use super::*;

    #[test]
    fn courses() {
        let sg = create_symbol_digraph("./algs4-data/movies.txt", "/");

        let topo = Topological::new(sg.g());

        if let Some(order) = topo.order {
            order.iter().for_each(|&v| println!("{}", sg.name(v)));
        }
    }
}
