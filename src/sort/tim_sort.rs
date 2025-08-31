use super::PerformanceCounter;
use std::cmp::min;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Minimum run size (typically 32-64)
    let min_run_size = calculate_min_run_size(n);

    // Sort individual runs of size min_run_size using insertion sort
    let mut i = 0;
    while i < n {
        let end = min(i + min_run_size, n);
        insertion_sort_range(arr, i, end, counter);
        i += min_run_size;
    }

    // Start merging runs of size min_run_size
    let mut size = min_run_size;
    while size < n {
        let mut start = 0;
        while start < n {
            let mid = start + size;
            let end = min(start + 2 * size, n);

            if mid < end {
                merge(arr, start, mid, end, counter);
            }
            start += 2 * size;
        }
        size *= 2;
    }
}

fn calculate_min_run_size(n: usize) -> usize {
    let mut r = 0;
    let mut n = n;
    while n >= 32 {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

fn insertion_sort_range(arr: &mut [i32], start: usize, end: usize, counter: &mut PerformanceCounter) {
    for i in (start + 1)..end {
        let key = arr[i];
        let mut j = i;

        while j > start && counter.compare(&arr[j - 1], &key) == std::cmp::Ordering::Greater {
            arr[j] = arr[j - 1];
            counter.swaps += 1;
            j -= 1;
        }

        arr[j] = key;
        if j != i {
            counter.swaps += 1;
        }
    }
}

fn merge(arr: &mut [i32], start: usize, mid: usize, end: usize, counter: &mut PerformanceCounter) {
    let left = arr[start..mid].to_vec();
    let right = arr[mid..end].to_vec();
    counter.allocate_memory(left.len() + right.len());

    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    while i < left.len() && j < right.len() {
        if counter.compare(&left[i], &right[j]) != std::cmp::Ordering::Greater {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        counter.swaps += 1;
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        counter.swaps += 1;
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        counter.swaps += 1;
        j += 1;
        k += 1;
    }
}