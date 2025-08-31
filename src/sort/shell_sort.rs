//! Shell Sort Algorithm - Basic Implementation
use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    let mut gap = n / 2;
    
    while gap > 0 {
        for i in gap..n {
            let key = arr[i];
            let mut j = i;
            
            while j >= gap && counter.compare(&arr[j - gap], &key) == std::cmp::Ordering::Greater {
                arr[j] = arr[j - gap];
                counter.swaps += 1;
                j -= gap;
            }
            arr[j] = key;
        }
        gap /= 2;
    }
}
