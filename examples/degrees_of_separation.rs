use std::{
    env::args,
    io::{stdin, BufRead},
};

use sedgewick::graph::{
    undirected_graph::{BreadthFirstPaths, Paths},
    undirected_symbol_graph::create_symbol_graph,
};

fn main() {
    let mut args = args().skip(1);
    let path = args.next().unwrap();
    let separator = args.next().unwrap();
    let source = args.next().unwrap();
    dbg!(&source);

    let sg = create_symbol_graph(path, &separator);
    let g = sg.g();
    let s = sg.index(&source);
    let bfs = BreadthFirstPaths::new(g, s);

    let mut stdin = stdin().lock();
    let mut buf = String::new();

    loop {
        match stdin.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let w = sg.index(&buf.trim());

                if let Some(path) = bfs.path_to(w) {
                    path.iter().for_each(|&v| {
                        println!("   {}", sg.name(v));
                    });
                };

                buf.clear();
            }
            Err(_) => panic!(),
        }
    }
}
