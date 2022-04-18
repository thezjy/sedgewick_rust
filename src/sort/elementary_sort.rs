use std::mem::swap;

use rand::{thread_rng, Rng};

pub fn selection_sort<T: PartialOrd>(list: &mut [T]) {
    let len = list.len();

    for i in 0..len {
        let mut min = i;

        for j in (i + 1)..len {
            if list[j] < list[min] {
                min = j;
            }
        }

        list.swap(i, min);
    }
}

pub fn insertion_sort(list: &mut [f64]) {
    let len = list.len();

    for i in 1..len {
        let mut j = i;
        while j > 0 && list[j - 1] > list[j] {
            list.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut rng = thread_rng();
        let mut l1: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();
        let mut l2 = l1.clone();
        let mut l3 = l1.clone();
        l1.sort_by(|a, b| a.partial_cmp(b).unwrap());
        selection_sort(&mut l2);
        insertion_sort(&mut l3);
        assert_eq!(l1, l2);
        assert_eq!(l1, l3);
    }
}
