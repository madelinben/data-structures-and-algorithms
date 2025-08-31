//! Bucket Sort Algorithm - Basic Implementation
use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    if arr.is_empty() {
        return;
    }
    
    // Find max value to determine range
    let max_val = *arr.iter().max().unwrap();
    let min_val = *arr.iter().min().unwrap();
    let range = (max_val - min_val + 1) as usize;
    
    // Create buckets
    let bucket_count = (range / 10).max(1).min(arr.len());
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); bucket_count];
    counter.allocate_memory(bucket_count);
    
    // Distribute elements into buckets
    for &value in arr.iter() {
        let bucket_index = ((value - min_val) as usize * (bucket_count - 1)) / range.max(1);
        buckets[bucket_index].push(value);
    }
    
    // Sort individual buckets and concatenate
    let mut index = 0;
    for bucket in buckets.iter_mut() {
        bucket.sort_unstable();
        for &value in bucket.iter() {
            arr[index] = value;
            counter.swaps += 1;
            index += 1;
        }
    }
}
