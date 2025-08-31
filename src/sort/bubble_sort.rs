//! Bubble Sort Algorithm
//! 
//! Simple sorting algorithm that repeatedly steps through the list,
//! compares adjacent elements and swaps them if they're in wrong order.
//! Time Complexity: O(n²)
//! Space Complexity: O(1)
//! Stable: Yes
//! Adaptive: Yes (can detect sorted arrays)
//! In-place: Yes

use super::PerformanceCounter;

/// Basic bubble sort implementation
pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n {
        let mut swapped = false;
        
        // Last i elements are already sorted
        for j in 0..n - 1 - i {
            if counter.compare(&arr[j], &arr[j + 1]) == std::cmp::Ordering::Greater {
                counter.swap(arr, j, j + 1);
                swapped = true;
            }
        }
        
        // If no swapping occurred, array is sorted
        if !swapped {
            break; // Adaptive behavior
        }
    }
}

/// Optimized bubble sort with early termination
pub fn sort_optimized(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mut end = n;
    
    while end > 1 {
        let mut new_end = 0;
        
        for i in 1..end {
            if counter.compare(&arr[i - 1], &arr[i]) == std::cmp::Ordering::Greater {
                counter.swap(arr, i - 1, i);
                new_end = i; // Remember the last swap position
            }
        }
        
        end = new_end; // Elements after new_end are sorted
    }
}

/// Cocktail shaker sort (bidirectional bubble sort)
pub fn cocktail_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mut start = 0;
    let mut end = n - 1;
    let mut swapped = true;
    
    while swapped && start < end {
        swapped = false;
        
        // Forward pass
        for i in start..end {
            if counter.compare(&arr[i], &arr[i + 1]) == std::cmp::Ordering::Greater {
                counter.swap(arr, i, i + 1);
                swapped = true;
            }
        }
        
        if !swapped {
            break;
        }
        
        end -= 1;
        swapped = false;
        
        // Backward pass
        for i in (start..end).rev() {
            if counter.compare(&arr[i], &arr[i + 1]) == std::cmp::Ordering::Greater {
                counter.swap(arr, i, i + 1);
                swapped = true;
            }
        }
        
        start += 1;
    }
}




    #[test]
    fn test_cocktail_sort() {
        let mut arr = vec![5, 1, 4, 2, 8, 0, 2];
        let mut counter = PerformanceCounter::new();
        cocktail_sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![0, 1, 2, 2, 4, 5, 8]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut counter = PerformanceCounter::new();
        sort(&mut arr, &mut counter);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }


