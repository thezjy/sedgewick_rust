use std::{
    env::args,
    time::{Duration, Instant},
};

use rand::prelude::*;
use sedgewick::sort::{
    merge_sort::{bottom_up_merge_sort, merge_sort, merge_sort_with_aux},
    quick_sort::quick_sort,
};

type Sort = fn(&mut [usize]);

struct SortAlgorithm {
    name: &'static str,
    sort: Sort,
}

#[derive(Debug)]
pub struct TimeResult {
    pub name: &'static str,
    pub duration: Duration,
}

fn time_sort(sort: &Sort, list: &mut [usize]) -> Duration {
    let instant = Instant::now();

    sort(list);

    instant.elapsed()
}

fn time_all_sort(
    sort_algorithms: &Vec<SortAlgorithm>,
    list_len: usize,
    sample_count: usize,
) -> Vec<TimeResult> {
    let mut result: Vec<TimeResult> = sort_algorithms
        .iter()
        .map(|item| TimeResult {
            name: item.name,
            duration: Duration::ZERO,
        })
        .collect();

    let mut rng = thread_rng();

    (0..sample_count).for_each(|_| {
        let random_list: Vec<usize> = (0..list_len)
            .map(|_| rng.gen_range(0..(list_len / 10)))
            .collect();

        sort_algorithms
            .iter()
            .enumerate()
            .for_each(|(i, SortAlgorithm { name: _name, sort })| {
                result[i].duration += time_sort(sort, &mut random_list.clone());
            });
    });

    result
}

fn main() {
    let args: Vec<String> = args().collect();

    let list_len: usize = args[1].parse().unwrap();
    let sample_count: usize = args[2].parse().unwrap();

    let sort_algorithms = vec![
        SortAlgorithm {
            name: "top-down merge sort",
            sort: merge_sort_with_aux,
        },
        // SortAlgorithm {
        //     name: "insertion sort",
        //     sort: insertion_sort,
        // },
        SortAlgorithm {
            name: "top-down merge sort without aux",
            sort: merge_sort,
        },
        SortAlgorithm {
            name: "bottom-up merge sort",
            sort: bottom_up_merge_sort,
        },
        SortAlgorithm {
            name: "quick sort",
            sort: quick_sort,
        },
        // SortAlgorithm {
        //     name: "selection sort",
        //     sort: selection_sort,
        // },
    ];

    let mut result = time_all_sort(&sort_algorithms, list_len, sample_count);

    result.sort_by_key(|item| item.duration);

    println!("{:#?}", result);

    let comparisons: Vec<String> = result
        .windows(2)
        .map(|pair| {
            let a = &pair[0];
            let b = &pair[1];
            format!(
                "{} is {:.2} times faster than {}",
                a.name,
                b.duration.as_secs_f64() / a.duration.as_secs_f64(),
                b.name
            )
        })
        .collect();

    println!("{:#?}", comparisons);
}
