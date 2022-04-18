pub fn binary_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
where
    T: Ord,
{
    let mut size = arr.len(); // 1
    if size == 0 {
        return Err(0);
    }
    let mut base = 0_usize;

    while size > 1 {
        // 2
        // mid: [base..size)
        let half = size / 2; // 2.1
        let mid = base + half;
        if arr[mid] <= *target {
            // 2.2
            base = mid
        }
        size -= half; // 2.3
    }

    if arr[base] == *target {
        // 3
        Ok(base)
    } else {
        Err(base + (arr[base] < *target) as usize)
    }
}
