//! Insertion Sort Algorithm
//! 
//! Builds the final sorted array one item at a time.
//! Efficient for small data sets and nearly sorted arrays.
//! Time Complexity: O(n²) worst case, O(n) best case
//! Space Complexity: O(1)
//! Stable: Yes
//! Adaptive: Yes
//! In-place: Yes

use super::PerformanceCounter;

/// Standard insertion sort implementation
pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        
        // Move elements greater than key one position ahead
        while j > 0 && counter.compare(&arr[j - 1], &key) == std::cmp::Ordering::Greater {
            arr[j] = arr[j - 1];
            counter.swaps += 1; // Count as swap (actually a move)
            j -= 1;
        }
        
        arr[j] = key;
    }
}

/// Binary insertion sort - uses binary search to find insertion position
pub fn binary_insertion_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 1..n {
        let key = arr[i];
        let insertion_point = binary_search_insertion_point(&arr[..i], key, counter);
        
        // Shift elements to make room
        for j in (insertion_point..i).rev() {
            arr[j + 1] = arr[j];
            counter.swaps += 1;
        }
        
        arr[insertion_point] = key;
    }
}

/// Find insertion point using binary search
fn binary_search_insertion_point(arr: &[i32], key: i32, counter: &mut PerformanceCounter) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        if counter.compare(&arr[mid], &key) == std::cmp::Ordering::Less {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    left
}

/// Insertion sort with sentinel - optimized version
pub fn insertion_sort_with_sentinel(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    // Find minimum element and move to first position (sentinel)
    let mut min_idx = 0;
    for i in 1..n {
        if counter.compare(&arr[i], &arr[min_idx]) == std::cmp::Ordering::Less {
            min_idx = i;
        }
    }
    
    if min_idx != 0 {
        counter.swap(arr, 0, min_idx);
    }
    
    // Now we can use the sentinel to avoid boundary checks
    for i in 2..n {
        let key = arr[i];
        let mut j = i;
        
        // No need to check j > 0 because sentinel guarantees we'll stop
        while counter.compare(&arr[j - 1], &key) == std::cmp::Ordering::Greater {
            arr[j] = arr[j - 1];
            counter.swaps += 1;
            j -= 1;
        }
        
        arr[j] = key;
    }
}

/// Shell sort (advanced insertion sort with gaps)
pub fn shell_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    // Start with a large gap, then reduce
    let mut gap = n / 2;
    
    while gap > 0 {
        // Perform gapped insertion sort
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

/// Insertion sort optimized for small arrays
pub fn insertion_sort_small(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    
    // Use different strategies based on size
    match n {
        0 | 1 => return,
        2 => {
            if counter.compare(&arr[0], &arr[1]) == std::cmp::Ordering::Greater {
                counter.swap(arr, 0, 1);
            }
        }
        3 => {
            // Optimized 3-element sort
            if counter.compare(&arr[0], &arr[1]) == std::cmp::Ordering::Greater {
                counter.swap(arr, 0, 1);
            }
            if counter.compare(&arr[1], &arr[2]) == std::cmp::Ordering::Greater {
                counter.swap(arr, 1, 2);
                if counter.compare(&arr[0], &arr[1]) == std::cmp::Ordering::Greater {
                    counter.swap(arr, 0, 1);
                }
            }
        }
        _ => sort(arr, counter), // Standard insertion sort for larger arrays
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
        assert!(counter.comparisons > 0);
        assert!(counter.swaps > 0);
    }

    #[test]
    fn test_insertion_sort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        // Should be very efficient on sorted data
        assert!(counter.swaps < arr.len());
    }

    #[test]
    fn test_binary_insertion_sort() {
        let mut arr = vec![5, 2, 4, 6, 1, 3];
        let mut counter = PerformanceCounter::new();
        binary_insertion_sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_insertion_sort_with_sentinel() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let mut counter = PerformanceCounter::new();
        insertion_sort_with_sentinel(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_shell_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        let mut counter = PerformanceCounter::new();
        shell_sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_small_array_optimizations() {
        // Test 2 elements
        let mut arr = vec![2, 1];
        let mut counter = PerformanceCounter::new();
        insertion_sort_small(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2]);
        
        // Test 3 elements
        let mut arr = vec![3, 1, 2];
        let mut counter = PerformanceCounter::new();
        insertion_sort_small(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2, 3]);
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
    fn test_single_element() {
        let mut arr = vec![42];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![42]);
        assert_eq!(counter.comparisons, 0);
    }

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![]);
        assert_eq!(counter.comparisons, 0);
    }
}
