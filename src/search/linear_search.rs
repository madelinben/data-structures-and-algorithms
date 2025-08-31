//! Linear Search Algorithm
//! 
//! Searches through a list sequentially until the target is found or the end is reached.
//! Time Complexity: O(n)
//! Space Complexity: O(1)

/// Perform linear search on a slice of strings
/// Returns (found, comparisons_made)
pub fn search(data: &[String], target: &str) -> (bool, usize) {
    let mut comparisons = 0;
    
    for item in data {
        comparisons += 1;
        if item == target {
            return (true, comparisons);
        }
    }
    
    (false, comparisons)
}

/// Linear search with early termination for sorted data
pub fn search_sorted_early_exit(data: &[String], target: &str) -> (bool, usize) {
    let mut comparisons = 0;
    
    for item in data {
        comparisons += 1;
        match item.as_str().cmp(target) {
            std::cmp::Ordering::Equal => return (true, comparisons),
            std::cmp::Ordering::Greater => return (false, comparisons), // Early exit
            std::cmp::Ordering::Less => continue,
        }
    }
    
    (false, comparisons)
}


