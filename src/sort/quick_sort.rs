use std::fmt::{Debug, Display};

use rand::prelude::*;

use super::elementary_sort::selection_sort;

pub fn quick_sort<T: PartialOrd + Copy + Debug>(list: &mut [T]) {
    let mut rng = thread_rng();
    list.shuffle(&mut rng);

    do_sort(list, 0, (list.len() - 1) as isize);
}

fn do_sort<T: PartialOrd + Copy + Debug>(list: &mut [T], low: isize, high: isize) {
    if high - low < 32 {
        selection_sort(&mut list[(low as usize)..=(high as usize)]);
        return;
    }

    let pivot = partition(list, low, high);

    do_sort(list, low, pivot - 1);
    do_sort(list, pivot + 1, high);
}

fn partition<T: PartialOrd + Copy + Debug>(list: &mut [T], low: isize, high: isize) -> isize {
    let mut i = low;
    let mut j = high + 1;
    let first = list[low as usize];

    loop {
        loop {
            i += 1;

            if list[i as usize] >= first || i == high {
                break;
            }
        }

        loop {
            j -= 1;

            if list[j as usize] <= first || j == low {
                break;
            }
        }

        if i >= j {
            break;
        }

        list.swap(i as usize, j as usize);
    }

    list.swap(low as usize, j as usize);

    j
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut rng = thread_rng();
        let mut l1: Vec<u32> = (0..100).map(|_| rng.gen_range(0..100)).collect();
        let mut l2 = l1.clone();

        l1.sort_by(|a, b| a.partial_cmp(b).unwrap());

        quick_sort(&mut l2);

        assert_eq!(l1, l2);
    }
}
