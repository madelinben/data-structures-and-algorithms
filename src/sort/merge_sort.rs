use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mut aux = vec![0; n];
    counter.allocate_memory(n);
    
    merge_sort_recursive(arr, &mut aux, 0, n, counter);
}

fn merge_sort_recursive(
    arr: &mut [i32], 
    aux: &mut [i32], 
    left: usize, 
    right: usize, 
    counter: &mut PerformanceCounter
) {
    if right - left <= 1 {
        return;
    }
    
    let mid = left + (right - left) / 2;
    
    merge_sort_recursive(arr, aux, left, mid, counter);
    merge_sort_recursive(arr, aux, mid, right, counter);
    
    merge(arr, aux, left, mid, right, counter);
}

fn merge(
    arr: &mut [i32], 
    aux: &mut [i32], 
    left: usize, 
    mid: usize, 
    right: usize, 
    counter: &mut PerformanceCounter
) {
    for i in left..right {
        aux[i] = arr[i];
    }
    
    let mut i = left;
    let mut j = mid;
    let mut k = left;
    
    while i < mid && j < right {
        if counter.compare(&aux[i], &aux[j]) != std::cmp::Ordering::Greater {
            arr[k] = aux[i];
            i += 1;
        } else {
            arr[k] = aux[j];
            j += 1;
        }
        // Note: These are assignments, not swaps, so we don't count them as swaps
        k += 1;
    }
    
    while i < mid {
        arr[k] = aux[i];
        // Note: These are assignments, not swaps
        i += 1;
        k += 1;
    }
    
    while j < right {
        arr[k] = aux[j];
        // Note: These are assignments, not swaps  
        j += 1;
        k += 1;
    }
}

pub fn merge_sort_iterative(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mut aux = vec![0; n];
    counter.allocate_memory(n);
    
    let mut size = 1;
    while size < n {
        let mut left = 0;
        
        while left < n - size {
            let mid = left + size;
            let right = (left + 2 * size).min(n);
            
            merge(arr, &mut aux, left, mid, right, counter);
            left += 2 * size;
        }
        
        size *= 2;
    }
}

pub fn merge_sort_optimised(arr: &mut [i32], counter: &mut PerformanceCounter) {
    const INSERTION_SORT_THRESHOLD: usize = 16;
    
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    if n <= INSERTION_SORT_THRESHOLD {
        insertion_sort_simple(arr, counter);
        return;
    }
    
    let mut aux = vec![0; n];
    counter.allocate_memory(n);
    
    merge_sort_optimised_recursive(arr, &mut aux, 0, n, counter);
}

fn merge_sort_optimised_recursive(
    arr: &mut [i32], 
    aux: &mut [i32], 
    left: usize, 
    right: usize, 
    counter: &mut PerformanceCounter
) {
    const INSERTION_SORT_THRESHOLD: usize = 16;
    
    if right - left <= INSERTION_SORT_THRESHOLD {
        insertion_sort_range(arr, left, right, counter);
        return;
    }
    
    let mid = left + (right - left) / 2;
    
    merge_sort_optimised_recursive(arr, aux, left, mid, counter);
    merge_sort_optimised_recursive(arr, aux, mid, right, counter);
    
    if counter.compare(&arr[mid - 1], &arr[mid]) != std::cmp::Ordering::Greater {
        return;
    }
    
    merge(arr, aux, left, mid, right, counter);
}

fn insertion_sort_simple(arr: &mut [i32], counter: &mut PerformanceCounter) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        
        while j > 0 && counter.compare(&arr[j - 1], &key) == std::cmp::Ordering::Greater {
            arr[j] = arr[j - 1];
            counter.swaps += 1;
            j -= 1;
        }
        
        arr[j] = key;
    }
}

fn insertion_sort_range(arr: &mut [i32], left: usize, right: usize, counter: &mut PerformanceCounter) {
    for i in left + 1..right {
        let key = arr[i];
        let mut j = i;
        
        while j > left && counter.compare(&arr[j - 1], &key) == std::cmp::Ordering::Greater {
            arr[j] = arr[j - 1];
            counter.swaps += 1;
            j -= 1;
        }
        
        arr[j] = key;
    }
}

pub fn merge_sort_in_place(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    merge_sort_in_place_recursive(arr, 0, n, counter);
}

fn merge_sort_in_place_recursive(
    arr: &mut [i32], 
    left: usize, 
    right: usize, 
    counter: &mut PerformanceCounter
) {
    if right - left <= 1 {
        return;
    }
    
    let mid = left + (right - left) / 2;
    
    merge_sort_in_place_recursive(arr, left, mid, counter);
    merge_sort_in_place_recursive(arr, mid, right, counter);
    
    merge_in_place(arr, left, mid, right, counter);
}

fn merge_in_place(arr: &mut [i32], left: usize, mid: usize, right: usize, counter: &mut PerformanceCounter) {
    let mut start1 = left;
    let mut start2 = mid;
    
    while start1 < start2 && start2 < right {
        if counter.compare(&arr[start1], &arr[start2]) != std::cmp::Ordering::Greater {
            start1 += 1;
        } else {
            let value = arr[start2];
            let mut index = start2;
            
            while index != start1 {
                arr[index] = arr[index - 1];
                counter.swaps += 1;
                index -= 1;
            }
            
            arr[start1] = value;
            start1 += 1;
            start2 += 1;
        }
    }
}