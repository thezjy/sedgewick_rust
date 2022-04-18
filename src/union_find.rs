use core::num;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use rand::{thread_rng, Rng};

pub trait UnionFind {
    fn new(count: usize) -> Self;

    fn union(&mut self, p: usize, q: usize);

    fn find(&mut self, p: usize) -> usize;

    /// number of components
    fn count(&self) -> usize;
}

#[derive(Debug)]
pub struct QuickFind {
    ids: Vec<usize>,
    count: usize,
}

impl UnionFind for QuickFind {
    fn new(count: usize) -> Self {
        QuickFind {
            ids: (0..count).collect(),
            count,
        }
    }

    fn union(&mut self, p: usize, q: usize) {
        let p_id = self.find(p);
        let q_id = self.find(q);

        if p_id != q_id {
            self.ids.iter_mut().for_each(|id| {
                if *id == p_id {
                    *id = q_id;
                }
            });

            self.count -= 1;
        }
    }

    fn find(&mut self, p: usize) -> usize {
        self.ids[p]
    }

    fn count(&self) -> usize {
        self.count
    }
}

#[derive(Debug)]
pub struct QuickUnion {
    pub ids: Vec<usize>,
    pub count: usize,
    compressed: bool,
}

impl UnionFind for QuickUnion {
    fn new(count: usize) -> Self {
        QuickUnion {
            ids: (0..count).collect(),
            count,
            compressed: true,
        }
    }

    fn union(&mut self, p: usize, q: usize) {
        let p_id = self.find(p);
        let q_id = self.find(q);

        if p_id != q_id {
            self.ids[p_id] = q_id;
            self.count -= 1;
        }
    }

    fn find(&mut self, mut p: usize) -> usize {
        let mut root = p;
        while root != self.ids[root] {
            root = self.ids[root]
        }

        if self.compressed {
            while p != root {
                let p_id = self.ids[p];
                self.ids[p] = root;
                p = p_id;
            }
        }

        root
    }

    fn count(&self) -> usize {
        self.count
    }
}

pub struct WeightedQuickUnion {
    pub ids: Vec<usize>,
    pub size: Vec<usize>,
    count: usize,
    compressed: bool,
}

impl UnionFind for WeightedQuickUnion {
    fn new(count: usize) -> Self {
        WeightedQuickUnion {
            ids: (0..count).collect(),
            size: (0..count).map(|_| 1).collect(),
            count,
            compressed: true,
        }
    }

    fn union(&mut self, p: usize, q: usize) {
        let p_id = self.find(p);
        let q_id = self.find(q);

        if p_id != q_id {
            let p_size = self.size[p_id];
            let q_size = self.size[q_id];

            if p_size < q_size {
                self.ids[p_id] = q_id;
                self.size[q_id] += p_size;
            } else {
                self.ids[q_id] = p_id;
                self.size[p_id] += q_size;
            }

            self.count -= 1;
        }
    }

    fn find(&mut self, mut p: usize) -> usize {
        let mut root = p;
        while root != self.ids[root] {
            root = self.ids[root]
        }

        if self.compressed {
            while p != root {
                let p_id = self.ids[p];
                self.ids[p] = root;
                p = p_id;
            }
        }

        root
    }

    fn count(&self) -> usize {
        self.count
    }
}

pub struct UnionFindParams {
    pub count: usize,
    pub pairs: Vec<(usize, usize)>,
}

pub fn parse_count(s: &str) -> usize {
    s.trim().parse().unwrap()
}

fn parse_pq_pair(s: &str) -> (usize, usize) {
    let pair = s
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
    (pair[0], pair[1])
}

pub fn parse_params_from_file<P: AsRef<Path>>(path: P) -> UnionFindParams {
    let file = File::open(path).unwrap();
    let mut file = BufReader::new(file);
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    let lines = &mut input.lines();

    let count = parse_count(lines.next().unwrap());

    let pairs = lines.map(|line| parse_pq_pair(line)).collect();

    UnionFindParams { count, pairs }
}

pub fn union_find<U: UnionFind>(uf: &mut U, pairs: &[(usize, usize)]) -> usize {
    pairs.iter().for_each(|(p, q)| uf.union(*p, *q));

    return uf.count();
}

pub fn erdos_renyi<U: UnionFind>(uf: &mut U, count: usize) -> usize {
    let mut rng = thread_rng();

    let mut num_connections = 0;

    while uf.count() != 1 {
        uf.union(rng.gen_range(0..count), rng.gen_range(0..count));
        num_connections += 1;
    }

    num_connections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_find() {
        let params = parse_params_from_file("algs4-data/mediumUF.txt");
        let mut qf = QuickFind::new(params.count);
        union_find(&mut qf, &params.pairs);
        assert_eq!(qf.count(), 3);
    }

    #[test]
    fn quick_union() {
        let params = parse_params_from_file("algs4-data/mediumUF.txt");
        let mut qu = QuickUnion::new(params.count);
        union_find(&mut qu, &params.pairs);
        assert_eq!(qu.count(), 3);
    }

    #[test]
    fn weighted_quick_union() {
        let params = parse_params_from_file("algs4-data/largeUF.txt");
        let mut wqu = WeightedQuickUnion::new(params.count);
        union_find(&mut wqu, &params.pairs);
        assert_eq!(wqu.count(), 6);
    }

    #[test]
    fn test_erdos_renyi() {
        let inputs = [1_000, 10_000, 100_000, 1_000_000];
        let results = inputs.map(|count| {
            let mut uf = WeightedQuickUnion::new(count);
            erdos_renyi(&mut uf, count);
        });

        println!("inputs: {:?}", &inputs);
        println!("results: {:?}", &results);
    }
}
