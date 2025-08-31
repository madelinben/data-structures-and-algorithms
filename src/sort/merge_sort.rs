//! Merge Sort Algorithm
//! 
//! Divide-and-conquer algorithm that divides array into halves,
//! sorts them separately, then merges the sorted halves.
//! Time Complexity: O(n log n)
//! Space Complexity: O(n)
//! Stable: Yes
//! Adaptive: No
//! In-place: No

use super::PerformanceCounter;

/// Standard merge sort implementation
pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    // Allocate auxiliary array once
    let mut aux = vec![0; n];
    counter.allocate_memory(n);
    
    merge_sort_recursive(arr, &mut aux, 0, n, counter);
}

/// Recursive merge sort implementation
fn merge_sort_recursive(
    arr: &mut [i32], 
    aux: &mut [i32], 
    left: usize, 
    right: usize, 
    counter: &mut PerformanceCounter
) {
    if right - left <= 1 {
        return;
    }
    
    let mid = left + (right - left) / 2;
    
    // Recursively sort left and right halves
    merge_sort_recursive(arr, aux, left, mid, counter);
    merge_sort_recursive(arr, aux, mid, right, counter);
    
    // Merge the sorted halves
    merge(arr, aux, left, mid, right, counter);
}

/// Merge two sorted halves
fn merge(
    arr: &mut [i32], 
    aux: &mut [i32], 
    left: usize, 
    mid: usize, 
    right: usize, 
    counter: &mut PerformanceCounter
) {
    // Copy data to auxiliary array
    for i in left..right {
        aux[i] = arr[i];
    }
    
    let mut i = left;  // Left subarray index
    let mut j = mid;   // Right subarray index
    let mut k = left;  // Merged array index
    
    // Merge the two halves
    while i < mid && j < right {
        if counter.compare(&aux[i], &aux[j]) != std::cmp::Ordering::Greater {
            arr[k] = aux[i];
            i += 1;
        } else {
            arr[k] = aux[j];
            j += 1;
        }
        counter.swaps += 1; // Count assignments as swaps
        k += 1;
    }
    
    // Copy remaining elements
    while i < mid {
        arr[k] = aux[i];
        counter.swaps += 1;
        i += 1;
        k += 1;
    }
    
    while j < right {
        arr[k] = aux[j];
        counter.swaps += 1;
        j += 1;
        k += 1;
    }
}

/// Bottom-up merge sort (iterative)
pub fn merge_sort_iterative(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mut aux = vec![0; n];
    counter.allocate_memory(n);
    
    let mut size = 1;
    while size < n {
        let mut left = 0;
        
        while left < n - size {
            let mid = left + size;
            let right = (left + 2 * size).min(n);
            
            merge(arr, &mut aux, left, mid, right, counter);
            left += 2 * size;
        }
        
        size *= 2;
    }
}

/// Optimized merge sort with insertions sort for small arrays
pub fn merge_sort_optimized(arr: &mut [i32], counter: &mut PerformanceCounter) {
    const INSERTION_SORT_THRESHOLD: usize = 16;
    
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    if n <= INSERTION_SORT_THRESHOLD {
        // Use insertion sort for small arrays
        insertion_sort_simple(arr, counter);
        return;
    }
    
    let mut aux = vec![0; n];
    counter.allocate_memory(n);
    
    merge_sort_optimized_recursive(arr, &mut aux, 0, n, counter);
}

/// Recursive optimized merge sort
fn merge_sort_optimized_recursive(
    arr: &mut [i32], 
    aux: &mut [i32], 
    left: usize, 
    right: usize, 
    counter: &mut PerformanceCounter
) {
    const INSERTION_SORT_THRESHOLD: usize = 16;
    
    if right - left <= INSERTION_SORT_THRESHOLD {
        insertion_sort_range(arr, left, right, counter);
        return;
    }
    
    let mid = left + (right - left) / 2;
    
    merge_sort_optimized_recursive(arr, aux, left, mid, counter);
    merge_sort_optimized_recursive(arr, aux, mid, right, counter);
    
    // Skip merge if already sorted
    if counter.compare(&arr[mid - 1], &arr[mid]) != std::cmp::Ordering::Greater {
        return;
    }
    
    merge(arr, aux, left, mid, right, counter);
}

/// Simple insertion sort for small arrays
fn insertion_sort_simple(arr: &mut [i32], counter: &mut PerformanceCounter) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        
        while j > 0 && counter.compare(&arr[j - 1], &key) == std::cmp::Ordering::Greater {
            arr[j] = arr[j - 1];
            counter.swaps += 1;
            j -= 1;
        }
        
        arr[j] = key;
    }
}

/// Insertion sort for a range
fn insertion_sort_range(arr: &mut [i32], left: usize, right: usize, counter: &mut PerformanceCounter) {
    for i in left + 1..right {
        let key = arr[i];
        let mut j = i;
        
        while j > left && counter.compare(&arr[j - 1], &key) == std::cmp::Ordering::Greater {
            arr[j] = arr[j - 1];
            counter.swaps += 1;
            j -= 1;
        }
        
        arr[j] = key;
    }
}

/// In-place merge sort (uses O(1) extra space but is more complex)
pub fn merge_sort_in_place(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    merge_sort_in_place_recursive(arr, 0, n, counter);
}

/// Recursive in-place merge sort
fn merge_sort_in_place_recursive(
    arr: &mut [i32], 
    left: usize, 
    right: usize, 
    counter: &mut PerformanceCounter
) {
    if right - left <= 1 {
        return;
    }
    
    let mid = left + (right - left) / 2;
    
    merge_sort_in_place_recursive(arr, left, mid, counter);
    merge_sort_in_place_recursive(arr, mid, right, counter);
    
    merge_in_place(arr, left, mid, right, counter);
}

/// In-place merge (rotates elements to avoid extra space)
fn merge_in_place(arr: &mut [i32], left: usize, mid: usize, right: usize, counter: &mut PerformanceCounter) {
    let mut start1 = left;
    let mut start2 = mid;
    
    while start1 < start2 && start2 < right {
        if counter.compare(&arr[start1], &arr[start2]) != std::cmp::Ordering::Greater {
            start1 += 1;
        } else {
            let value = arr[start2];
            let mut index = start2;
            
            // Shift elements
            while index != start1 {
                arr[index] = arr[index - 1];
                counter.swaps += 1;
                index -= 1;
            }
            
            arr[start1] = value;
            start1 += 1;
            start2 += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
        assert!(counter.comparisons > 0);
        assert!(counter.swaps > 0);
        assert!(counter.memory_allocations > 0);
    }

    #[test]
    fn test_merge_sort_iterative() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        let mut counter = PerformanceCounter::new();
        merge_sort_iterative(&mut arr, &mut counter);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_merge_sort_optimized() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90, 88, 76, 50, 42, 15, 3, 99, 55, 23];
        let mut counter = PerformanceCounter::new();
        merge_sort_optimized(&mut arr, &mut counter);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn test_merge_sort_in_place() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        let mut counter = PerformanceCounter::new();
        merge_sort_in_place(&mut arr, &mut counter);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
        assert_eq!(counter.memory_allocations, 0); // Should use no extra space
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![]);
        assert_eq!(counter.comparisons, 0);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![42]);
        assert_eq!(counter.comparisons, 0);
    }

    #[test]
    fn test_two_elements() {
        let mut arr = vec![2, 1];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2]);
    }

    #[test]
    fn test_large_array() {
        let mut arr: Vec<i32> = (0..1000).rev().collect();
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert!(is_sorted(&arr));
        assert_eq!(arr.len(), 1000);
        assert_eq!(arr[0], 0);
        assert_eq!(arr[999], 999);
    }

    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }
}
