use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 1..n {
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

pub fn binary_insertion_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 1..n {
        let key = arr[i];
        let insertion_point = binary_search_insertion_point(&arr[..i], key, counter);
        
        for j in (insertion_point..i).rev() {
            arr[j + 1] = arr[j];
            counter.swaps += 1;
        }
        
        arr[insertion_point] = key;
    }
}

fn binary_search_insertion_point(arr: &[i32], key: i32, counter: &mut PerformanceCounter) -> usize {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        if counter.compare(&arr[mid], &key) == std::cmp::Ordering::Less {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    
    left
}

pub fn insertion_sort_with_sentinel(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mut min_idx = 0;
    for i in 1..n {
        if counter.compare(&arr[i], &arr[min_idx]) == std::cmp::Ordering::Less {
            min_idx = i;
        }
    }
    
    if min_idx != 0 {
        counter.swap(arr, 0, min_idx);
    }
    
    for i in 2..n {
        let key = arr[i];
        let mut j = i;
        
        while counter.compare(&arr[j - 1], &key) == std::cmp::Ordering::Greater {
            arr[j] = arr[j - 1];
            counter.swaps += 1;
            j -= 1;
        }
        
        arr[j] = key;
    }
}

pub fn shell_sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mut gap = n / 2;
    
    while gap > 0 {
        for i in gap..n {
            let key = arr[i];
            let mut j = i;
            
            while j >= gap && counter.compare(&arr[j - gap], &key) == std::cmp::Ordering::Greater {
                arr[j] = arr[j - gap];
                counter.swaps += 1;
                j -= gap;
            }
            
            arr[j] = key;
        }
        
        gap /= 2;
    }
}

pub fn insertion_sort_small(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    
    match n {
        0 | 1 => return,
        2 => {
            if counter.compare(&arr[0], &arr[1]) == std::cmp::Ordering::Greater {
                counter.swap(arr, 0, 1);
            }
        }
        3 => {
            if counter.compare(&arr[0], &arr[1]) == std::cmp::Ordering::Greater {
                counter.swap(arr, 0, 1);
            }
            if counter.compare(&arr[1], &arr[2]) == std::cmp::Ordering::Greater {
                counter.swap(arr, 1, 2);
                if counter.compare(&arr[0], &arr[1]) == std::cmp::Ordering::Greater {
                    counter.swap(arr, 0, 1);
                }
            }
        }
        _ => sort(arr, counter),
    }
}