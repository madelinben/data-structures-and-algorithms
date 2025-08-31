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

pub fn search_sorted_early_exit(data: &[String], target: &str) -> (bool, usize) {
    let mut comparisons = 0;
    
    for item in data {
        comparisons += 1;
        match item.as_str().cmp(target) {
            std::cmp::Ordering::Equal => return (true, comparisons),
            std::cmp::Ordering::Greater => return (false, comparisons),
            std::cmp::Ordering::Less => continue,
        }
    }
    
    (false, comparisons)
}