use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    super::merge_sort::sort(arr, counter);
}