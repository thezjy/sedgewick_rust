use std::{
    env,
    io::{stdin, BufRead},
};

use sedgewick::sort::priority_queue::*;

#[derive(Debug, Default)]
struct Transaction {
    text: String,
    amount: f64,
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(&other.amount)
    }
}
fn main() {
    let num_highest: usize = env::args().skip(1).next().unwrap().parse().unwrap();

    let mut stdin = stdin().lock();
    let mut buf = String::new();
    let mut min_pq = HeapMinPQ::new();

    loop {
        match stdin.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let parts: Vec<&str> = buf.split_ascii_whitespace().collect();

                min_pq.insert(Transaction {
                    text: buf.clone(),
                    amount: parts[2].parse().unwrap(),
                });

                if min_pq.size() > num_highest {
                    min_pq.delete_min();
                }

                buf.clear();
            }
            Err(_) => panic!(),
        }
    }

    let mut stack = vec![];
    while !min_pq.is_empty() {
        stack.push(min_pq.delete_min());
    }

    stack.iter().rev().for_each(|t| println!("{}", t.text));
}
