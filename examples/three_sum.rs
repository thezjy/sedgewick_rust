use std::{
    io::{self, Read},
    time::SystemTime,
};

use sedgewick::misc::three_sum_binary;

fn main() {
    let mut stdin = io::stdin().lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut integers: Vec<i32> = buf
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let t2 = SystemTime::now();
    println!("three sum binary: {}", three_sum_binary(&mut integers));
    let d2 = t2.elapsed().unwrap().as_secs_f64();
    println!("time: {}", d2);
}
