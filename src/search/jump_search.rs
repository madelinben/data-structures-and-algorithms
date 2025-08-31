pub fn search(data: &[String], target: &str) -> (bool, usize) {
    if data.is_empty() {
        return (false, 0);
    }
    
    let n = data.len();
    let jump_size = (n as f64).sqrt() as usize;
    let mut comparisons = 0;
    let mut prev = 0;
    
    while prev < n && data[(jump_size.min(n - 1)).min(prev + jump_size - 1)].as_str() < target {
        comparisons += 1;
        prev += jump_size;
        if prev >= n {
            break;
        }
    }
    
    let end = (prev + jump_size).min(n);
    for i in prev..end {
        comparisons += 1;
        if data[i] == target {
            return (true, comparisons);
        }
        if data[i].as_str() > target {
            break;
        }
    }
    
    (false, comparisons)
}

pub fn search_with_jump_size(data: &[String], target: &str, jump_size: usize) -> (bool, usize) {
    if data.is_empty() || jump_size == 0 {
        return (false, 0);
    }
    
    let n = data.len();
    let mut comparisons = 0;
    let mut prev = 0;
    
    while prev < n {
        let jump_index = (prev + jump_size - 1).min(n - 1);
        comparisons += 1;
        
        if data[jump_index].as_str() >= target {
            break;
        }
        prev += jump_size;
    }
    
    let end = (prev + jump_size).min(n);
    for i in prev..end {
        comparisons += 1;
        match data[i].as_str().cmp(target) {
            std::cmp::Ordering::Equal => return (true, comparisons),
            std::cmp::Ordering::Greater => break,
            std::cmp::Ordering::Less => continue,
        }
    }
    
    (false, comparisons)
}

pub fn search_adaptive(data: &[String], target: &str) -> (bool, usize) {
    if data.is_empty() {
        return (false, 0);
    }
    
    let n = data.len();
    let mut jump_size = (n as f64).sqrt() as usize;
    let mut comparisons = 0;
    let mut prev = 0;
    let mut consecutive_misses = 0;
    
    while prev < n {
        let jump_index = (prev + jump_size - 1).min(n - 1);
        comparisons += 1;
        
        match data[jump_index].as_str().cmp(target) {
            std::cmp::Ordering::Equal => return (true, comparisons),
            std::cmp::Ordering::Greater => break,
            std::cmp::Ordering::Less => {
                prev += jump_size;
                consecutive_misses += 1;
                
                if consecutive_misses > 3 && jump_size > 1 {
                    jump_size = (jump_size as f64 * 1.5) as usize;
                }
            }
        }
    }
    
    let end = (prev + jump_size).min(n);
    for i in prev..end {
        comparisons += 1;
        match data[i].as_str().cmp(target) {
            std::cmp::Ordering::Equal => return (true, comparisons),
            std::cmp::Ordering::Greater => break,
            std::cmp::Ordering::Less => continue,
        }
    }
    
    (false, comparisons)
}

pub fn calculate_optimal_jump_size(array_size: usize) -> usize {
    if array_size <= 1 {
        return 1;
    }
    (array_size as f64).sqrt() as usize
}