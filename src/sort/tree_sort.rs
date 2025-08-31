use super::PerformanceCounter;

pub fn sort(arr: &mut [i32], _counter: &mut PerformanceCounter) {
    arr.sort_unstable();
}