use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n {
        let mut swapped = false;
        
        for j in 0..n - 1 - i {
            if counter.compare(&arr[j], &arr[j + 1]) == std::cmp::Ordering::Greater {
                counter.swap(arr, j, j + 1);
                swapped = true;
            }
        }
        
        if !swapped {
            break;
        }
    }
}

pub fn sort_optimised(arr: &mut [i32], counter: &mut PerformanceCounter) {
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
                new_end = i;
            }
        }
        
        end = new_end;
    }
}

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
        
        for i in (start..end).rev() {
            if counter.compare(&arr[i], &arr[i + 1]) == std::cmp::Ordering::Greater {
                counter.swap(arr, i, i + 1);
                swapped = true;
            }
        }
        
        start += 1;
    }
}