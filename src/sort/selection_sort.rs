use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n - 1 {
        let mut min_idx = i;
        
        for j in i + 1..n {
            if counter.compare(&arr[j], &arr[min_idx]) == std::cmp::Ordering::Less {
                min_idx = j;
            }
        }
        
        if min_idx != i {
            counter.swap(arr, i, min_idx);
        }
    }
}

pub fn stable_selection_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n - 1 {
        let mut min_idx = i;
        
        for j in i + 1..n {
            if counter.compare(&arr[j], &arr[min_idx]) == std::cmp::Ordering::Less {
                min_idx = j;
            }
        }
        
        if min_idx != i {
            let min_val = arr[min_idx];
            
            for k in (i..min_idx).rev() {
                arr[k + 1] = arr[k];
                counter.swaps += 1;
            }
            
            arr[i] = min_val;
        }
    }
}

pub fn bidirectional_selection_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mut left = 0;
    let mut right = n - 1;
    
    while left < right {
        let mut min_idx = left;
        let mut max_idx = left;
        
        for i in left..=right {
            if counter.compare(&arr[i], &arr[min_idx]) == std::cmp::Ordering::Less {
                min_idx = i;
            }
            if counter.compare(&arr[i], &arr[max_idx]) == std::cmp::Ordering::Greater {
                max_idx = i;
            }
        }
        
        if min_idx != left {
            counter.swap(arr, left, min_idx);
            
            if max_idx == left {
                max_idx = min_idx;
            }
        }
        
        if max_idx != right {
            counter.swap(arr, right, max_idx);
        }
        
        left += 1;
        right -= 1;
    }
}

pub fn adaptive_selection_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n - 1 {
        let mut min_idx = i;
        let mut is_sorted = true;
        
        for j in i + 1..n {
            if counter.compare(&arr[j], &arr[min_idx]) == std::cmp::Ordering::Less {
                min_idx = j;
            }
            
            if j > i + 1 && counter.compare(&arr[j - 1], &arr[j]) == std::cmp::Ordering::Greater {
                is_sorted = false;
            }
        }
        
        if min_idx != i {
            counter.swap(arr, i, min_idx);
        }
        
        if is_sorted && min_idx == i {
            break;
        }
    }
}

pub fn heap_selection_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    if n < 20 {
        return sort(arr, counter);
    }
    
    for i in 0..n - 1 {
        let unsorted_start = i;
        let unsorted_size = n - i;
        
        build_min_heap(&mut arr[unsorted_start..], counter);
    }
}

fn build_min_heap(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in (0..n / 2).rev() {
        min_heapify(arr, i, n, counter);
    }
}

fn min_heapify(arr: &mut [i32], mut i: usize, heap_size: usize, counter: &mut PerformanceCounter) {
    loop {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut smallest = i;
        
        if left < heap_size && counter.compare(&arr[left], &arr[smallest]) == std::cmp::Ordering::Less {
            smallest = left;
        }
        
        if right < heap_size && counter.compare(&arr[right], &arr[smallest]) == std::cmp::Ordering::Less {
            smallest = right;
        }
        
        if smallest != i {
            counter.swap(arr, i, smallest);
            i = smallest;
        } else {
            break;
        }
    }
}