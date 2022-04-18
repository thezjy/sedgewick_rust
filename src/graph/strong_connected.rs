use super::{directed_graph::Digraph, topological::DepthFirstOrder};

#[derive(Debug)]
pub struct SCC {
    pub count: usize,
    id: Vec<usize>,
    marked: Vec<bool>,
}

impl SCC {
    fn search(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;
        self.id[v] = self.count;

        g.adj(v).for_each(|&w| {
            if !self.marked[w] {
                self.search(g, w);
            }
        });
    }

    pub fn new(g: &Digraph) -> Self {
        let num_vertices = g.num_vertices();
        let mut scc = SCC {
            count: 0,
            id: vec![0; num_vertices],
            marked: vec![false; num_vertices],
        };

        DepthFirstOrder::new(&g.reverse())
            .reverse_post()
            .iter()
            .for_each(|&v| {
                if !scc.marked[v] {
                    scc.search(g, v);
                    scc.count += 1;
                }
            });

        scc
    }

    fn strongly_connected(&self, v: usize, w: usize) -> bool {
        self.id[v] == self.id[w]
    }

    fn id(&self, v: usize) -> usize {
        self.id[v]
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::graph::directed_graph::create_digraph_from_path;

    use super::*;

    #[test]
    fn scc() {
        let g = create_digraph_from_path("./algs4-data/tinyDG.txt");
        let scc = SCC::new(&g);
        println!("{} components", scc.count);

        let mut components = vec![vec![]; scc.count];

        g.vertices().for_each(|v| {
            components[scc.id(v)].push(v);
        });

        components.iter().for_each(|component| {
            println!("{}", component.iter().map(ToString::to_string).join(" "))
        });
    }
}
