//! Cube Sort Algorithm - Basic Implementation (Custom Algorithm)
use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    // Cube sort: A hybrid approach combining multiple techniques
    let n = arr.len();
    
    if n <= 16 {
        // Use insertion sort for small arrays
        super::insertion_sort::sort(arr, counter);
    } else if n <= 100 {
        // Use shell sort for medium arrays  
        super::shell_sort::sort(arr, counter);
    } else {
        // Use merge sort for large arrays
        super::merge_sort::sort(arr, counter);
    }
}
