use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    let n = arr.len();
    
    if n <= 16 {
        super::insertion_sort::sort(arr, counter);
    } else if n <= 100 {
        super::shell_sort::sort(arr, counter);
    } else {
        super::merge_sort::sort(arr, counter);
    }
}