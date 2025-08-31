//! Binary Search Algorithm
//! 
//! Searches a sorted list by repeatedly dividing the search interval in half.
//! Time Complexity: O(log n)
//! Space Complexity: O(1)

/// Perform binary search on a sorted slice of strings
/// Returns (found, comparisons_made)
pub fn search(data: &[String], target: &str) -> (bool, usize) {
    let mut left = 0;
    let mut right = data.len();
    let mut comparisons = 0;
    
    while left < right {
        let mid = left + (right - left) / 2;
        comparisons += 1;
        
        match data[mid].as_str().cmp(target) {
            std::cmp::Ordering::Equal => return (true, comparisons),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    
    (false, comparisons)
}

/// Recursive binary search implementation
pub fn search_recursive(data: &[String], target: &str) -> (bool, usize) {
    fn binary_search_recursive(data: &[String], target: &str, left: usize, right: usize, comparisons: &mut usize) -> bool {
        if left >= right {
            return false;
        }
        
        let mid = left + (right - left) / 2;
        *comparisons += 1;
        
        match data[mid].as_str().cmp(target) {
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Less => binary_search_recursive(data, target, mid + 1, right, comparisons),
            std::cmp::Ordering::Greater => binary_search_recursive(data, target, left, mid, comparisons),
        }
    }
    
    let mut comparisons = 0;
    let found = binary_search_recursive(data, target, 0, data.len(), &mut comparisons);
    (found, comparisons)
}

/// Binary search that returns the insertion point if not found
pub fn search_with_insertion_point(data: &[String], target: &str) -> (Option<usize>, usize, usize) {
    let mut left = 0;
    let mut right = data.len();
    let mut comparisons = 0;
    
    while left < right {
        let mid = left + (right - left) / 2;
        comparisons += 1;
        
        match data[mid].as_str().cmp(target) {
            std::cmp::Ordering::Equal => return (Some(mid), comparisons, mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    
    (None, comparisons, left) // left is the insertion point
}


