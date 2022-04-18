#![allow(unused)]
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

use dashmap::DashMap;
use fnv::FnvHashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rb_tree::RBMap;
use sedgewick::symbol_table::{
    hash::separate_chaining::SeparateChainingHashST, ordered_vec::OrderedVecST, SymbolTable,
};

fn count_frequency_with_red_black_tree<'a, T>(words: T, min_len: usize)
where
    T: Iterator<Item = &'a str>,
{
    let mut table = RBMap::new();

    words.for_each(|word| {
        if word.len() >= min_len {
            *table.entry(word).or_insert(0) += 1;
        }
    });

    let mut max_key = "";
    table.insert(max_key, 0);

    for k in table.keys() {
        if table.get(k) > table.get(&max_key) {
            max_key = k;
        }
    }
    println!("{} {}", max_key, table.get(&max_key).unwrap());
}

fn count_frequency_with_fnvhashmap<'a, T>(words: T, min_len: usize)
where
    T: Iterator<Item = &'a str>,
{
    let mut table = FnvHashMap::with_capacity_and_hasher(299594, Default::default());

    words.for_each(|word| {
        if word.len() >= min_len {
            *table.entry(word).or_insert(0) += 1;
        }
    });

    let mut max_key = "";
    table.insert(max_key, 0);

    for k in table.keys() {
        if table.get(k) > table.get(max_key) {
            max_key = k;
        }
    }
    println!("{} {}", max_key, table.get(max_key).unwrap());
}

fn count_frequency_with_hashmap<'a, T>(words: T, min_len: usize)
where
    T: Iterator<Item = &'a str>,
{
    let mut table = HashMap::with_capacity(299594);

    words.for_each(|word| {
        if word.len() >= min_len {
            *table.entry(word).or_insert(0) += 1;
        }
    });

    let mut max_key = "";
    table.insert(max_key, 0);

    for k in table.keys() {
        if table.get(k) > table.get(max_key) {
            max_key = k;
        }
    }
    println!("{} {}", max_key, table.get(max_key).unwrap());
}

fn count_frequency_with_btreemap<'a, T>(words: T, min_len: usize)
where
    T: Iterator<Item = &'a str>,
{
    let mut table = BTreeMap::new();

    words.for_each(|word| {
        if word.len() >= min_len {
            *table.entry(word).or_insert(0) += 1;
        }
    });

    let mut max_key = "";
    table.insert(max_key, 0);

    for k in table.keys() {
        if table.get(k) > table.get(max_key) {
            max_key = k;
        }
    }
    println!("{} {}", max_key, table.get(max_key).unwrap());
}

fn count_frequency_with_dashmap(words: Vec<&str>, min_len: usize) {
    let mut table = DashMap::new();

    words.par_iter().for_each(|word| {
        if word.len() >= min_len {
            *table.entry(*word).or_insert(0) += 1;
        }
    });

    let mut max_key = "";
    table.insert(max_key, 0);

    table.iter().for_each(|x| {
        if x.value() > table.get(max_key).unwrap().value() {
            max_key = x.key();
        }
    });

    println!("{} {}", max_key, table.get(max_key).unwrap().value());
}

fn count_frequency<'a, T>(table: &mut dyn SymbolTable<String, u16>, words: T, min_len: usize)
where
    T: Iterator<Item = &'a str>,
{
    words.for_each(|word| {
        if word.len() >= min_len {
            let key = word.to_string();

            match table.get_mut(&key) {
                Some(count) => *count += 1,
                None => table.put(key, 1),
            }
        }
    });

    let mut max = "".to_string();

    table.put(max.clone(), 0);

    for k in table.keys() {
        if table.get(&k) > table.get(&max) {
            max = k.to_owned();
        }
    }
    println!("{} {}", max, table.get(&max).unwrap());
}

fn main() {
    let min_len: usize = 8;

    let file = File::open("./algs4-data/leipzig1M.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    reader.read_to_string(&mut buf).unwrap();

    let words = buf.split_ascii_whitespace();
    let words1 = words.clone();

    let now = Instant::now();
    count_frequency_with_hashmap(words1, min_len);
    dbg!(now.elapsed());

    let words: Vec<&str> = words.collect();
    let now = Instant::now();
    count_frequency_with_dashmap(words, min_len);
    dbg!(now.elapsed());
}
