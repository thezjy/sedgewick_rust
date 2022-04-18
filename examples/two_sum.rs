use std::{
    io::{self, Read},
    time::SystemTime,
};

use sedgewick::misc::{two_sum, two_sum_fast};

fn main() {
    let mut stdin = io::stdin().lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut integers: Vec<i32> = buf
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let t1 = SystemTime::now();
    println!("two sum: {}", two_sum(&mut integers));
    let d1 = t1.elapsed().unwrap().as_secs_f64();
    println!("time: {}", d1);

    let t2 = SystemTime::now();
    println!("two sum fast: {}", two_sum_fast(&mut integers));
    let d2 = t2.elapsed().unwrap().as_secs_f64();
    println!("time: {}", d2);

    println!("ratio: {}", d1 / d2);
}
