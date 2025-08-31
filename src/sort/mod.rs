pub mod bubble_sort;
pub mod insertion_sort;
pub mod selection_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod heap_sort;
pub mod shell_sort;
pub mod tim_sort;
pub mod tree_sort;
pub mod bucket_sort;
pub mod radix_sort;
pub mod counting_sort;
pub mod cube_sort;
pub mod gui;

use crate::prelude::*;
use std::time::{Duration, Instant};
use rand::prelude::*;
use rand::rng;
use prettytable::{Table, Row, Cell};

#[derive(Debug, Clone)]
pub struct SortMetrics {
    pub algorithm_name: String,
    pub array_size: usize,
    pub comparisons: usize,
    pub swaps: usize,
    pub memory_allocations: usize,
    pub duration: Duration,
    pub theoretical_time_complexity: String,
    pub theoretical_space_complexity: String,
    pub actual_time_ratio: f64,
    pub space_efficiency: String,
    pub is_stable: bool,
    pub is_adaptive: bool,
    pub is_in_place: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceCounter {
    pub comparisons: usize,
    pub swaps: usize,
    pub memory_allocations: usize,
}

impl PerformanceCounter {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn reset(&mut self) {
        self.comparisons = 0;
        self.swaps = 0;
        self.memory_allocations = 0;
    }
    
    pub fn compare<T: PartialOrd>(&mut self, a: &T, b: &T) -> std::cmp::Ordering {
        self.comparisons += 1;
        a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
    }
    
    pub fn swap<T>(&mut self, arr: &mut [T], i: usize, j: usize) {
        self.swaps += 1;
        arr.swap(i, j);
    }
    
    pub fn allocate_memory(&mut self, _size: usize) {
        self.memory_allocations += 1;
    }
}

pub struct SortCoordinator {
    last_results: Vec<SortMetrics>,
}

impl SortCoordinator {
    pub fn new() -> Self {
        Self {
            last_results: Vec::new(),
        }
    }

    pub fn generate_random_array(&self, size: usize, min_val: i32, max_val: i32) -> Vec<i32> {
        let mut rng = rng();
        (0..size)
            .map(|_| rng.random_range(min_val..=max_val))
            .collect()
    }

    pub fn generate_test_arrays(&self, size: usize) -> Vec<(String, Vec<i32>)> {
        let mut arrays = Vec::new();
        let mut rng = rng();
        
        arrays.push((
            "Random".to_string(),
            self.generate_random_array(size, 1, size as i32 * 10)
        ));
        
        let mut nearly_sorted: Vec<i32> = (1..=size as i32).collect();
        let swaps = size / 10;
        for _ in 0..swaps {
            let i = rng.random_range(0..size);
            let j = rng.random_range(0..size);
            nearly_sorted.swap(i, j);
        }
        arrays.push(("Nearly Sorted".to_string(), nearly_sorted));
        
        let mut reverse_sorted: Vec<i32> = (1..=size as i32).collect();
        reverse_sorted.reverse();
        arrays.push(("Reverse Sorted".to_string(), reverse_sorted));
        
        let sorted: Vec<i32> = (1..=size as i32).collect();
        arrays.push(("Already Sorted".to_string(), sorted));
        
        let duplicates: Vec<i32> = (0..size).map(|_| rng.random_range(1..=10)).collect();
        arrays.push(("Many Duplicates".to_string(), duplicates));
        
        let few_unique: Vec<i32> = (0..size).map(|_| rng.random_range(1..=5)).collect();
        arrays.push(("Few Unique".to_string(), few_unique));
        
        arrays
    }

    pub fn run_benchmarks(&mut self, array_size: usize, iterations: usize) -> Result<Vec<SortMetrics>> {
        println!("Running sorting benchmarks...");
        println!("Array size: {}", array_size);
        println!("Iterations per algorithm: {}", iterations);
        println!("{}", "=".repeat(80));

        let mut results = Vec::new();

        let test_array = self.generate_random_array(array_size, 1, array_size as i32 * 10);
        
        results.push(self.benchmark_algorithm("Bubble Sort", &test_array, iterations, bubble_sort::sort)?);
        results.push(self.benchmark_algorithm("Insertion Sort", &test_array, iterations, insertion_sort::sort)?);
        results.push(self.benchmark_algorithm("Selection Sort", &test_array, iterations, selection_sort::sort)?);
        results.push(self.benchmark_algorithm("Merge Sort", &test_array, iterations, merge_sort::sort)?);
        results.push(self.benchmark_algorithm("Quick Sort", &test_array, iterations, quick_sort::sort)?);
        results.push(self.benchmark_algorithm("Heap Sort", &test_array, iterations, heap_sort::sort)?);
        results.push(self.benchmark_algorithm("Shell Sort", &test_array, iterations, shell_sort::sort)?);
        results.push(self.benchmark_algorithm("Tim Sort", &test_array, iterations, tim_sort::sort)?);
        results.push(self.benchmark_algorithm("Tree Sort", &test_array, iterations, tree_sort::sort)?);
        results.push(self.benchmark_algorithm("Bucket Sort", &test_array, iterations, bucket_sort::sort)?);
        results.push(self.benchmark_algorithm("Radix Sort", &test_array, iterations, radix_sort::sort)?);
        results.push(self.benchmark_algorithm("Counting Sort", &test_array, iterations, counting_sort::sort)?);
        results.push(self.benchmark_algorithm("Cube Sort", &test_array, iterations, cube_sort::sort)?);

        self.last_results = results.clone();
        self.display_results(&results);
        Ok(results)
    }

    fn benchmark_algorithm<F>(
        &self, 
        name: &str, 
        original_array: &[i32], 
        iterations: usize,
        sort_fn: F
    ) -> Result<SortMetrics> 
    where 
        F: Fn(&mut [i32], &mut PerformanceCounter),
    {
        let mut total_comparisons = 0;
        let mut total_swaps = 0;
        let mut total_memory = 0;
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let mut test_array = original_array.to_vec();
            let mut counter = PerformanceCounter::new();
            
            sort_fn(&mut test_array, &mut counter);
            
            total_comparisons += counter.comparisons;
            total_swaps += counter.swaps;
            total_memory += counter.memory_allocations;
            
            if !is_sorted(&test_array) {
                return Err(Error::Generic(format!("{} failed to sort array correctly", name)));
            }
        }
        
        let duration = start.elapsed() / iterations as u32;
        let avg_comparisons = total_comparisons / iterations;
        let avg_swaps = total_swaps / iterations;
        let avg_memory = total_memory / iterations;
        
        let (time_complexity, space_complexity, is_stable, is_adaptive, is_in_place) = get_algorithm_properties(name);
        
        let theoretical_time = calculate_theoretical_time_complexity(name, original_array.len());
        let actual_time_ratio = avg_comparisons as f64 / theoretical_time;
        
        Ok(SortMetrics {
            algorithm_name: name.to_string(),
            array_size: original_array.len(),
            comparisons: avg_comparisons,
            swaps: avg_swaps,
            memory_allocations: avg_memory,
            duration,
            theoretical_time_complexity: time_complexity,
            theoretical_space_complexity: space_complexity,
            actual_time_ratio,
            space_efficiency: if avg_memory == 0 { "O(1)".to_string() } else { format!("~{}", avg_memory) },
            is_stable,
            is_adaptive,
            is_in_place,
        })
    }

    fn display_results(&self, results: &[SortMetrics]) {
        println!("\n{}", "=".repeat(120));
        println!("SORTING ALGORITHM PERFORMANCE ANALYSIS");
        println!("{}", "=".repeat(120));
        
        let mut table = Table::new();
        
        table.add_row(Row::new(vec![
            Cell::new("Algorithm"),
            Cell::new("Size"),
            Cell::new("Comparisons"),
            Cell::new("Swaps"),
            Cell::new("Time (μs)"),
            Cell::new("Big O Time"),
            Cell::new("Big O Space"),
            Cell::new("Ratio"),
            Cell::new("Stable"),
            Cell::new("In-Place"),
            Cell::new("Efficiency"),
        ]));

        for metric in results {
            let efficiency = classify_efficiency(metric.actual_time_ratio);
            
            table.add_row(Row::new(vec![
                Cell::new(&metric.algorithm_name),
                Cell::new(&format!("{}", metric.array_size)),
                Cell::new(&format!("{}", metric.comparisons)),
                Cell::new(&format!("{}", metric.swaps)),
                Cell::new(&format!("{:.2}", metric.duration.as_micros())),
                Cell::new(&metric.theoretical_time_complexity),
                Cell::new(&metric.theoretical_space_complexity),
                Cell::new(&format!("{:.2}", metric.actual_time_ratio)),
                Cell::new(&format!("{}", metric.is_stable)),
                Cell::new(&format!("{}", metric.is_in_place)),
                Cell::new(efficiency),
            ]));
        }

        println!("{}", table);
        
        self.display_summary_statistics(results);
    }
    
    fn display_summary_statistics(&self, results: &[SortMetrics]) {
        println!("\n{}", "=".repeat(60));
        println!("SUMMARY STATISTICS");
        println!("{}", "=".repeat(60));
        
        if let Some(fastest) = results.iter().min_by_key(|m| m.duration) {
            println!("🏆 Fastest Algorithm: {} ({:.2}μs)", 
                fastest.algorithm_name, fastest.duration.as_micros());
        }
        
        if let Some(fewest_comparisons) = results.iter().min_by_key(|m| m.comparisons) {
            println!("🎯 Fewest Comparisons: {} ({} comparisons)", 
                fewest_comparisons.algorithm_name, fewest_comparisons.comparisons);
        }
        
        if let Some(fewest_swaps) = results.iter().min_by_key(|m| m.swaps) {
            println!("🔄 Fewest Swaps: {} ({} swaps)", 
                fewest_swaps.algorithm_name, fewest_swaps.swaps);
        }
        
        let stable_count = results.iter().filter(|m| m.is_stable).count();
        let in_place_count = results.iter().filter(|m| m.is_in_place).count();
        let adaptive_count = results.iter().filter(|m| m.is_adaptive).count();
        
        println!("\n📊 Algorithm Properties:");
        println!("   Stable: {}/{}", stable_count, results.len());
        println!("   In-Place: {}/{}", in_place_count, results.len());
        println!("   Adaptive: {}/{}", adaptive_count, results.len());
        
        let mut time_complexities = std::collections::HashMap::new();
        for result in results {
            *time_complexities.entry(&result.theoretical_time_complexity).or_insert(0) += 1;
        }
        
        println!("\n⏱️ Time Complexity Distribution:");
        
        let mut complexity_vec: Vec<_> = time_complexities.into_iter().collect();
        complexity_vec.sort_by(|a, b| {
            let order = |complexity: &str| match complexity {
                "O(1)" => 0,
                "O(log n)" => 1,
                "O(n)" => 2,
                "O(n log n)" => 3,
                "O(n^1.25)" => 4,
                "O(n²)" => 5,
                "O(n³)" => 6,
                "O(n + k)" => 7,
                "O(d × n)" => 8,
                "O(2^n)" => 9,
                _ => 10,
            };
            order(a.0).cmp(&order(b.0))
        });
        
        for (complexity, count) in complexity_vec {
            println!("   {}: {} algorithms", complexity, count);
        }
    }

    pub fn analyse_array_type(&mut self, array_type: &str, size: usize) -> Result<()> {
        let test_arrays = self.generate_test_arrays(size);
        
        if let Some((_, array)) = test_arrays.iter().find(|(name, _)| name == array_type) {
            println!("\nAnalysing performance on {} array (size: {})", array_type, size);
            
            let algorithms = vec![
                ("Quick Sort", quick_sort::sort as fn(&mut [i32], &mut PerformanceCounter)),
                ("Merge Sort", merge_sort::sort as fn(&mut [i32], &mut PerformanceCounter)),
                ("Tim Sort", tim_sort::sort as fn(&mut [i32], &mut PerformanceCounter)),
                ("Insertion Sort", insertion_sort::sort as fn(&mut [i32], &mut PerformanceCounter)),
            ];
            
            let mut results = Vec::new();
            for (name, sort_fn) in algorithms {
                results.push(self.benchmark_algorithm(name, array, 10, sort_fn)?);
            }
            
            self.display_results(&results);
        } else {
            return Err(Error::Generic(format!("Unknown array type: {}", array_type)));
        }
        
        Ok(())
    }
}

fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn get_algorithm_properties(name: &str) -> (String, String, bool, bool, bool) {
    match name {
        "Bubble Sort" => ("O(n²)".to_string(), "O(1)".to_string(), true, true, true),
        "Insertion Sort" => ("O(n²)".to_string(), "O(1)".to_string(), true, true, true),
        "Selection Sort" => ("O(n²)".to_string(), "O(1)".to_string(), false, false, true),
        "Merge Sort" => ("O(n log n)".to_string(), "O(n)".to_string(), true, false, false),
        "Quick Sort" => ("O(n log n)".to_string(), "O(log n)".to_string(), false, false, true),
        "Heap Sort" => ("O(n log n)".to_string(), "O(1)".to_string(), false, false, true),
        "Shell Sort" => ("O(n^1.25)".to_string(), "O(1)".to_string(), false, true, true),
        "Tim Sort" => ("O(n log n)".to_string(), "O(n)".to_string(), true, true, false),
        "Tree Sort" => ("O(n log n)".to_string(), "O(n)".to_string(), true, false, false),
        "Bucket Sort" => ("O(n + k)".to_string(), "O(n + k)".to_string(), true, false, false),
        "Radix Sort" => ("O(d × n)".to_string(), "O(n + k)".to_string(), true, false, false),
        "Counting Sort" => ("O(n + k)".to_string(), "O(k)".to_string(), true, false, false),
        "Cube Sort" => ("O(n log n)".to_string(), "O(n)".to_string(), false, false, false),
        _ => ("Unknown".to_string(), "Unknown".to_string(), false, false, false),
    }
}

fn calculate_theoretical_time_complexity(name: &str, n: usize) -> f64 {
    let n_f = n as f64;
    match name {
        "Bubble Sort" | "Insertion Sort" | "Selection Sort" => n_f * n_f,
        "Merge Sort" | "Quick Sort" | "Heap Sort" | "Tim Sort" | "Tree Sort" | "Cube Sort" => n_f * n_f.log2(),
        "Shell Sort" => n_f.powf(1.25),
        "Bucket Sort" | "Radix Sort" | "Counting Sort" => n_f,
        _ => n_f,
    }
}

fn classify_efficiency(ratio: f64) -> &'static str {
    if ratio <= 1.2 {
        "Excellent"
    } else if ratio <= 2.0 {
        "Very Good"
    } else if ratio <= 3.0 {
        "Good"
    } else if ratio <= 5.0 {
        "Fair"
    } else {
        "Poor"
    }
}

impl Default for SortCoordinator {
    fn default() -> Self {
        Self::new()
    }
}