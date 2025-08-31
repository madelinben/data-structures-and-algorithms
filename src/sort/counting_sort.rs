//! Counting Sort Algorithm - Basic Implementation
use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    if arr.is_empty() {
        return;
    }
    
    let max_val = *arr.iter().max().unwrap();
    let min_val = *arr.iter().min().unwrap();
    let range = (max_val - min_val + 1) as usize;
    
    let mut count = vec![0; range];
    counter.allocate_memory(range);
    
    // Count each element
    for &value in arr.iter() {
        count[(value - min_val) as usize] += 1;
        counter.comparisons += 1;
    }
    
    // Reconstruct sorted array
    let mut index = 0;
    for (i, &freq) in count.iter().enumerate() {
        for _ in 0..freq {
            arr[index] = i as i32 + min_val;
            counter.swaps += 1;
            index += 1;
        }
    }
}
