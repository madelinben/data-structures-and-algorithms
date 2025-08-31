//! Exponential Search Algorithm
//! 
//! Finds the range where the element might be present, then uses binary search.
//! Time Complexity: O(log n)
//! Space Complexity: O(1)

/// Perform exponential search on a sorted slice of strings
/// Returns (found, comparisons_made)
pub fn search(data: &[String], target: &str) -> (bool, usize) {
    if data.is_empty() {
        return (false, 0);
    }
    
    let n = data.len();
    let mut comparisons = 0;
    
    // Check if target is at first position
    comparisons += 1;
    if data[0] == target {
        return (true, comparisons);
    }
    
    // Find range for binary search by repeated doubling
    let mut bound = 1;
    while bound < n && data[bound].as_str() < target {
        comparisons += 1;
        bound *= 2;
    }
    
    // Perform binary search in the identified range
    let left = bound / 2;
    let right = bound.min(n - 1);
    
    let (found, binary_comparisons) = binary_search_range(data, target, left, right + 1);
    (found, comparisons + binary_comparisons)
}

/// Binary search within a specified range
fn binary_search_range(data: &[String], target: &str, left: usize, right: usize) -> (bool, usize) {
    let mut left = left;
    let mut right = right;
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

/// Exponential search with custom growth factor
pub fn search_with_growth_factor(data: &[String], target: &str, growth_factor: usize) -> (bool, usize) {
    if data.is_empty() || growth_factor < 2 {
        return (false, 0);
    }
    
    let n = data.len();
    let mut comparisons = 0;
    
    // Check if target is at first position
    comparisons += 1;
    if data[0] == target {
        return (true, comparisons);
    }
    
    // Find range using custom growth factor
    let mut bound = 1;
    while bound < n && data[bound].as_str() < target {
        comparisons += 1;
        bound *= growth_factor;
    }
    
    // Binary search in the identified range
    let left = bound / growth_factor;
    let right = bound.min(n - 1);
    
    let (found, binary_comparisons) = binary_search_range(data, target, left, right + 1);
    (found, comparisons + binary_comparisons)
}

/// Exponential search optimized for large arrays
pub fn search_optimized(data: &[String], target: &str) -> (bool, usize) {
    if data.is_empty() {
        return (false, 0);
    }
    
    let n = data.len();
    let mut comparisons = 0;
    
    // Quick check for boundary conditions
    comparisons += 1;
    if data[0] == target {
        return (true, comparisons);
    }
    
    if n > 1 {
        comparisons += 1;
        if data[n - 1].as_str() < target {
            return (false, comparisons); // Target is beyond array
        }
    }
    
    // Exponential search with optimizations
    let mut bound = 1;
    while bound < n {
        comparisons += 1;
        match data[bound].as_str().cmp(target) {
            std::cmp::Ordering::Equal => return (true, comparisons),
            std::cmp::Ordering::Greater => break,
            std::cmp::Ordering::Less => bound *= 2,
        }
    }
    
    // Binary search in the range
    let left = bound / 2;
    let right = bound.min(n);
    
    let (found, binary_comparisons) = binary_search_range(data, target, left, right);
    (found, comparisons + binary_comparisons)
}

/// Unbounded exponential search (when array size is unknown)
pub fn search_unbounded(data: &[String], target: &str, max_safe_index: Option<usize>) -> (bool, usize) {
    let max_index = max_safe_index.unwrap_or(data.len());
    let mut comparisons = 0;
    
    if data.is_empty() {
        return (false, comparisons);
    }
    
    // Check first element
    comparisons += 1;
    if data[0] == target {
        return (true, comparisons);
    }
    
    // Find upper bound
    let mut bound = 1;
    while bound < max_index && bound < data.len() && data[bound].as_str() < target {
        comparisons += 1;
        bound *= 2;
    }
    
    // Binary search in identified range
    let left = bound / 2;
    let right = bound.min(data.len());
    
    let (found, binary_comparisons) = binary_search_range(data, target, left, right);
    (found, comparisons + binary_comparisons)
}


