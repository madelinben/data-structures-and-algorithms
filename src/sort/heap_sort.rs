use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in (0..n / 2).rev() {
        heapify(arr, n, i, counter);
    }
    
    for i in (1..n).rev() {
        counter.swap(arr, 0, i);
        heapify(arr, i, 0, counter);
    }
}

fn heapify(arr: &mut [i32], n: usize, i: usize, counter: &mut PerformanceCounter) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    
    if left < n && counter.compare(&arr[left], &arr[largest]) == std::cmp::Ordering::Greater {
        largest = left;
    }
    
    if right < n && counter.compare(&arr[right], &arr[largest]) == std::cmp::Ordering::Greater {
        largest = right;
    }
    
    if largest != i {
        counter.swap(arr, i, largest);
        heapify(arr, n, largest, counter);
    }
}