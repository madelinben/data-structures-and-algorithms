pub fn search(data: &[String], target: &str) -> (bool, usize) {
    if data.is_empty() {
        return (false, 0);
    }
    
    let n = data.len();
    let mut comparisons = 0;
    
    comparisons += 1;
    if data[0] == target {
        return (true, comparisons);
    }
    
    let mut bound = 1;
    while bound < n && data[bound].as_str() < target {
        comparisons += 1;
        bound *= 2;
    }
    
    let left = bound / 2;
    let right = bound.min(n - 1);
    
    let (found, binary_comparisons) = binary_search_range(data, target, left, right + 1);
    (found, comparisons + binary_comparisons)
}

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

pub fn search_with_growth_factor(data: &[String], target: &str, growth_factor: usize) -> (bool, usize) {
    if data.is_empty() || growth_factor < 2 {
        return (false, 0);
    }
    
    let n = data.len();
    let mut comparisons = 0;
    
    comparisons += 1;
    if data[0] == target {
        return (true, comparisons);
    }
    
    let mut bound = 1;
    while bound < n && data[bound].as_str() < target {
        comparisons += 1;
        bound *= growth_factor;
    }
    
    let left = bound / growth_factor;
    let right = bound.min(n - 1);
    
    let (found, binary_comparisons) = binary_search_range(data, target, left, right + 1);
    (found, comparisons + binary_comparisons)
}

pub fn search_optimised(data: &[String], target: &str) -> (bool, usize) {
    if data.is_empty() {
        return (false, 0);
    }
    
    let n = data.len();
    let mut comparisons = 0;
    
    comparisons += 1;
    if data[0] == target {
        return (true, comparisons);
    }
    
    if n > 1 {
        comparisons += 1;
        if data[n - 1].as_str() < target {
            return (false, comparisons);
        }
    }
    
    let mut bound = 1;
    while bound < n {
        comparisons += 1;
        match data[bound].as_str().cmp(target) {
            std::cmp::Ordering::Equal => return (true, comparisons),
            std::cmp::Ordering::Greater => break,
            std::cmp::Ordering::Less => bound *= 2,
        }
    }
    
    let left = bound / 2;
    let right = bound.min(n);
    
    let (found, binary_comparisons) = binary_search_range(data, target, left, right);
    (found, comparisons + binary_comparisons)
}

pub fn search_unbounded(data: &[String], target: &str, max_safe_index: Option<usize>) -> (bool, usize) {
    let max_index = max_safe_index.unwrap_or(data.len());
    let mut comparisons = 0;
    
    if data.is_empty() {
        return (false, comparisons);
    }
    
    comparisons += 1;
    if data[0] == target {
        return (true, comparisons);
    }
    
    let mut bound = 1;
    while bound < max_index && bound < data.len() && data[bound].as_str() < target {
        comparisons += 1;
        bound *= 2;
    }
    
    let left = bound / 2;
    let right = bound.min(data.len());
    
    let (found, binary_comparisons) = binary_search_range(data, target, left, right);
    (found, comparisons + binary_comparisons)
}