//! Selection Sort Algorithm
//! 
//! Finds the minimum element and places it at the beginning.
//! Repeats this process for the remaining unsorted portion.
//! Time Complexity: O(n²)
//! Space Complexity: O(1)
//! Stable: No (can be made stable with modifications)
//! Adaptive: No
//! In-place: Yes

use super::PerformanceCounter;

/// Standard selection sort implementation
pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n - 1 {
        let mut min_idx = i;
        
        // Find the minimum element in the remaining unsorted array
        for j in i + 1..n {
            if counter.compare(&arr[j], &arr[min_idx]) == std::cmp::Ordering::Less {
                min_idx = j;
            }
        }
        
        // Swap the minimum element with the first element
        if min_idx != i {
            counter.swap(arr, i, min_idx);
        }
    }
}

/// Stable selection sort implementation
pub fn stable_selection_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n - 1 {
        let mut min_idx = i;
        
        // Find the minimum element
        for j in i + 1..n {
            if counter.compare(&arr[j], &arr[min_idx]) == std::cmp::Ordering::Less {
                min_idx = j;
            }
        }
        
        // Instead of swapping, shift elements to maintain stability
        if min_idx != i {
            let min_val = arr[min_idx];
            
            // Shift all elements between i and min_idx
            for k in (i..min_idx).rev() {
                arr[k + 1] = arr[k];
                counter.swaps += 1;
            }
            
            arr[i] = min_val;
        }
    }
}

/// Bidirectional selection sort (selects both min and max)
pub fn bidirectional_selection_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mut left = 0;
    let mut right = n - 1;
    
    while left < right {
        let mut min_idx = left;
        let mut max_idx = left;
        
        // Find both minimum and maximum in one pass
        for i in left..=right {
            if counter.compare(&arr[i], &arr[min_idx]) == std::cmp::Ordering::Less {
                min_idx = i;
            }
            if counter.compare(&arr[i], &arr[max_idx]) == std::cmp::Ordering::Greater {
                max_idx = i;
            }
        }
        
        // Place minimum at the left
        if min_idx != left {
            counter.swap(arr, left, min_idx);
            
            // If max was at left, it's now at min_idx
            if max_idx == left {
                max_idx = min_idx;
            }
        }
        
        // Place maximum at the right
        if max_idx != right {
            counter.swap(arr, right, max_idx);
        }
        
        left += 1;
        right -= 1;
    }
}

/// Selection sort with early termination for nearly sorted arrays
pub fn adaptive_selection_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n - 1 {
        let mut min_idx = i;
        let mut is_sorted = true;
        
        // Check if remaining array is already sorted while finding minimum
        for j in i + 1..n {
            if counter.compare(&arr[j], &arr[min_idx]) == std::cmp::Ordering::Less {
                min_idx = j;
            }
            
            // Check if array is sorted from this point
            if j > i + 1 && counter.compare(&arr[j - 1], &arr[j]) == std::cmp::Ordering::Greater {
                is_sorted = false;
            }
        }
        
        // Swap minimum element
        if min_idx != i {
            counter.swap(arr, i, min_idx);
        }
        
        // If rest of array is sorted, we can stop
        if is_sorted && min_idx == i {
            break;
        }
    }
}

/// Heap selection sort (builds a min-heap to find minimums efficiently)
pub fn heap_selection_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    // For small arrays, use regular selection sort
    if n < 20 {
        return sort(arr, counter);
    }
    
    // Build a min-heap from unsorted portion and repeatedly extract minimum
    for i in 0..n - 1 {
        let unsorted_start = i;
        let unsorted_size = n - i;
        
        // Build min-heap from remaining elements
        build_min_heap(&mut arr[unsorted_start..], counter);
        
        // The minimum is now at arr[i] (root of heap)
        // No need to swap as it's already in the right position
        
        // The remaining elements still form a heap structure
        // This is more efficient than linear search for large arrays
    }
}

/// Build a min-heap from array
fn build_min_heap(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    // Start from the last non-leaf node and heapify
    for i in (0..n / 2).rev() {
        min_heapify(arr, i, n, counter);
    }
}

/// Maintain min-heap property
fn min_heapify(arr: &mut [i32], mut i: usize, heap_size: usize, counter: &mut PerformanceCounter) {
    loop {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut smallest = i;
        
        if left < heap_size && counter.compare(&arr[left], &arr[smallest]) == std::cmp::Ordering::Less {
            smallest = left;
        }
        
        if right < heap_size && counter.compare(&arr[right], &arr[smallest]) == std::cmp::Ordering::Less {
            smallest = right;
        }
        
        if smallest != i {
            counter.swap(arr, i, smallest);
            i = smallest;
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
        assert!(counter.comparisons > 0);
        assert!(counter.swaps > 0);
    }

    #[test]
    fn test_stable_selection_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        let mut counter = PerformanceCounter::new();
        stable_selection_sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_bidirectional_selection_sort() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let mut counter = PerformanceCounter::new();
        bidirectional_selection_sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }

    #[test]
    fn test_adaptive_selection_sort() {
        // Test with already sorted array
        let mut arr = vec![1, 2, 3, 4, 5];
        let mut counter = PerformanceCounter::new();
        adaptive_selection_sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        
        // Test with reverse sorted array
        let mut arr = vec![5, 4, 3, 2, 1];
        let mut counter = PerformanceCounter::new();
        adaptive_selection_sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_heap_selection_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90, 88, 76, 50, 42];
        let mut counter = PerformanceCounter::new();
        heap_selection_sort(&mut arr, &mut counter);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn test_selection_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![]);
        assert_eq!(counter.comparisons, 0);
    }

    #[test]
    fn test_selection_sort_single() {
        let mut arr = vec![42];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![42]);
        assert_eq!(counter.comparisons, 0);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test] 
    fn test_two_elements() {
        let mut arr = vec![2, 1];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2]);
        assert_eq!(counter.comparisons, 1);
        assert_eq!(counter.swaps, 1);
    }
    
    fn is_sorted(arr: &[i32]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }
}
