//! Interpolation Search Algorithm
//! 
//! An improvement over binary search for uniformly distributed sorted arrays.
//! Estimates position based on the value being searched.
//! Time Complexity: O(log log n) for uniform distribution, O(n) worst case
//! Space Complexity: O(1)

/// Perform interpolation search on a sorted slice of strings
/// Returns (found, comparisons_made)
pub fn search(data: &[String], target: &str) -> (bool, usize) {
    if data.is_empty() {
        return (false, 0);
    }
    
    let mut low = 0;
    let mut high = data.len() - 1;
    let mut comparisons = 0;
    
    while low <= high && target >= data[low].as_str() && target <= data[high].as_str() {
        comparisons += 1;
        
        // If we're at the boundaries
        if low == high {
            if data[low] == target {
                return (true, comparisons);
            }
            return (false, comparisons);
        }
        
        // Estimate position using interpolation formula
        // For strings, we use the first character's ASCII value for estimation
        let target_val = target.chars().next().unwrap_or('\0') as usize;
        let low_val = data[low].chars().next().unwrap_or('\0') as usize;
        let high_val = data[high].chars().next().unwrap_or('\0') as usize;
        
        let pos = if high_val != low_val {
            low + (((target_val - low_val) * (high - low)) / (high_val - low_val))
        } else {
            low
        };
        
        // Ensure pos is within bounds
        let pos = pos.min(high).max(low);
        
        match data[pos].as_str().cmp(target) {
            std::cmp::Ordering::Equal => return (true, comparisons),
            std::cmp::Ordering::Less => low = pos + 1,
            std::cmp::Ordering::Greater => {
                if pos == 0 {
                    break;
                }
                high = pos - 1;
            }
        }
        
        // Prevent infinite loops
        if high >= data.len() {
            break;
        }
    }
    
    (false, comparisons)
}

/// Interpolation search with fallback to binary search
pub fn search_with_fallback(data: &[String], target: &str) -> (bool, usize) {
    // Try interpolation search first
    let (found, comparisons) = search(data, target);
    
    if found || comparisons < data.len().max(10) {
        (found, comparisons)
    } else {
        // Fallback to binary search if interpolation is performing poorly
        crate::search::binary_search::search(data, target)
    }
}

/// Interpolation search optimized for numeric strings
pub fn search_numeric_strings(data: &[String], target: &str) -> (bool, usize) {
    if data.is_empty() {
        return (false, 0);
    }
    
    let mut low = 0;
    let mut high = data.len() - 1;
    let mut comparisons = 0;
    
    // Try to parse strings as numbers for better interpolation
    let parse_as_num = |s: &str| -> Option<f64> {
        s.parse::<f64>().ok()
    };
    
    let target_num = parse_as_num(target);
    
    while low <= high {
        comparisons += 1;
        
        if low == high {
            if data[low] == target {
                return (true, comparisons);
            }
            return (false, comparisons);
        }
        
        let pos = if let (Some(target_val), Some(low_val), Some(high_val)) = 
            (target_num, parse_as_num(&data[low]), parse_as_num(&data[high])) {
            
            if high_val != low_val {
                let ratio = (target_val - low_val) / (high_val - low_val);
                low + ((high - low) as f64 * ratio) as usize
            } else {
                low
            }
        } else {
            // Fallback to middle if not numeric
            low + (high - low) / 2
        };
        
        let pos = pos.min(high).max(low);
        
        match data[pos].as_str().cmp(target) {
            std::cmp::Ordering::Equal => return (true, comparisons),
            std::cmp::Ordering::Less => low = pos + 1,
            std::cmp::Ordering::Greater => {
                if pos == 0 {
                    break;
                }
                high = pos - 1;
            }
        }
        
        if high >= data.len() {
            break;
        }
    }
    
    (false, comparisons)
}


