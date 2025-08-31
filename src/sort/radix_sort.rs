//! Radix Sort Algorithm - Basic Implementation
use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    if arr.is_empty() {
        return;
    }
    
    let max_val = *arr.iter().max().unwrap();
    let mut exp = 1;
    
    while max_val / exp > 0 {
        counting_sort_by_digit(arr, exp, counter);
        exp *= 10;
    }
}

fn counting_sort_by_digit(arr: &mut [i32], exp: i32, counter: &mut PerformanceCounter) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];
    counter.allocate_memory(n + 10);
    
    // Count occurrences
    for &value in arr.iter() {
        count[((value / exp) % 10) as usize] += 1;
        counter.comparisons += 1;
    }
    
    // Build cumulative count
    for i in 1..10 {
        count[i] += count[i - 1];
    }
    
    // Build output array
    for i in (0..n).rev() {
        let digit = ((arr[i] / exp) % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = arr[i];
        counter.swaps += 1;
    }
    
    // Copy back to original array
    for i in 0..n {
        arr[i] = output[i];
        counter.swaps += 1;
    }
}
