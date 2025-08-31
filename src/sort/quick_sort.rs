//! Quick Sort Algorithm - Basic Implementation
//! More comprehensive implementation coming soon

use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    if arr.len() <= 1 {
        return;
    }
    
    quicksort(arr, 0, arr.len() - 1, counter);
}

fn quicksort(arr: &mut [i32], low: usize, high: usize, counter: &mut PerformanceCounter) {
    if low < high {
        let pi = partition(arr, low, high, counter);
        
        if pi > 0 {
            quicksort(arr, low, pi - 1, counter);
        }
        quicksort(arr, pi + 1, high, counter);
    }
}

fn partition(arr: &mut [i32], low: usize, high: usize, counter: &mut PerformanceCounter) -> usize {
    let pivot = arr[high];
    let mut i = low;
    
    for j in low..high {
        if counter.compare(&arr[j], &pivot) != std::cmp::Ordering::Greater {
            counter.swap(arr, i, j);
            i += 1;
        }
    }
    
    counter.swap(arr, i, high);
    i
}
