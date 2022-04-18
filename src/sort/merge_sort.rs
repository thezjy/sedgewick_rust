use std::{
    cmp::{max, min},
    fmt::{Debug, Display},
};

pub fn bottom_up_merge_sort<T: PartialOrd + Copy + Debug>(list: &mut [T]) {
    let len = list.len();
    let max_index = len - 1;
    let mut aux = list.to_vec();

    let mut size = 1;
    while size < len {
        let mut low = 0;
        while low < len - size {
            let high = min(max_index, low + 2 * size - 1);
            let mid = low + size - 1;

            merge_with_aux(list, &mut aux, low, mid, high);

            low += 2 * size;
        }

        size *= 2;
    }
}

pub fn merge_sort_with_aux<T: PartialOrd + Copy + Debug>(list: &mut [T]) {
    let mut aux = list.to_vec();

    do_merge_sort(list, &mut aux, 0, list.len() - 1);
}

fn do_merge_sort<T: PartialOrd + Copy + Debug>(
    list: &mut [T],
    aux: &mut [T],
    low: usize,
    high: usize,
) {
    if (high <= low) {
        return;
    }

    let mid = low + (high - low) / 2;

    do_merge_sort(list, aux, low, mid);
    do_merge_sort(list, aux, mid + 1, high);

    merge_with_aux(list, aux, low, mid, high);
}

fn merge_with_aux<T: PartialOrd + Copy + Debug>(
    list: &mut [T],
    aux: &mut [T],
    low: usize,
    mid: usize,
    high: usize,
) {
    let mut i = low;
    let mut j = mid + 1;

    for k in i..=high {
        aux[k] = list[k];
    }

    let mut k = i;

    while i <= mid && j <= high {
        if (aux[j] < aux[i]) {
            list[k] = aux[j];
            j += 1;
        } else {
            list[k] = aux[i];
            i += 1;
        }
        k += 1;
    }

    if i <= mid {
        list[k..=high].copy_from_slice(&aux[i..=mid]);
    }

    if j <= high {
        list[k..=high].copy_from_slice(&aux[j..=high]);
    }
}

pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        // 1
        return;
    }

    merge_sort(&mut arr[..mid]); // 2
    merge_sort(&mut arr[mid..]);

    // Create an array to store intermediate result.
    let mut ret = arr.to_vec(); // 3

    // Merge the two piles.
    merge(&arr[..mid], &arr[mid..], &mut ret[..]); // 4

    // Copy back the result back to original array.
    arr.copy_from_slice(&ret); // 5
}

fn merge<T: PartialOrd + Copy>(arr1: &[T], arr2: &[T], ret: &mut [T]) {
    let mut left = 0; // Head of left pile.             // 1
    let mut right = 0; // Head of right pile.
    let mut index = 0;

    // Compare element and insert back to result array.
    while left < arr1.len() && right < arr2.len() {
        // 2
        if arr1[left] <= arr2[right] {
            // 3
            ret[index] = arr1[left];
            index += 1;
            left += 1;
        } else {
            ret[index] = arr2[right];
            index += 1;
            right += 1;
        }
    }

    // Copy the reset elements to returned array.
    // `memcpy` may be more performant than for-loop assignment.
    if left < arr1.len() {
        // 4
        ret[index..].copy_from_slice(&arr1[left..]);
    }
    if right < arr2.len() {
        ret[index..].copy_from_slice(&arr2[right..]);
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn top_down() {
        let mut rng = thread_rng();
        let mut l1: Vec<u32> = (0..13).map(|_| rng.gen_range(0..10)).collect();
        let mut l2 = l1.clone();

        l1.sort_by(|a, b| a.partial_cmp(b).unwrap());

        merge_sort_with_aux(&mut l2);

        assert_eq!(l1, l2);
    }

    #[test]
    fn bottom_up() {
        let mut rng = thread_rng();
        let mut l1: Vec<u32> = (0..100).map(|_| rng.gen_range(0..10)).collect();
        let mut l2 = l1.clone();

        l1.sort_by(|a, b| a.partial_cmp(b).unwrap());

        bottom_up_merge_sort(&mut l2);

        assert_eq!(l1, l2);
    }
}
