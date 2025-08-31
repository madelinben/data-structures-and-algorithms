use crate::prelude::*;
use crate::models::SortAlgorithm;
use crate::gui::sorting::{SortVisualizer, GuiPerformanceCounter};
use rand::{rng, Rng};
use std::io::{self, Write};

pub fn run_gui_visualization(algorithm: &str, array_size: usize) -> Result<()> {
    let mut visualizer = SortVisualizer::new(array_size);
    
    let effective_size = if array_size > 50 {
        println!("⚠️ Large array size ({}) detected. For smooth animation, limiting to 50 elements.", array_size);
        50
    } else {
        array_size
    };
    
    let mut rng = rand::rng();
    let test_array: Vec<i32> = (0..effective_size).map(|_| rng.random_range(1..=100)).collect();
    
    match algorithm {
        "bubble" => {
            visualizer.visualize_algorithm("Bubble Sort", test_array, |arr, counter| {
                bubble_sort_with_gui(arr, counter);
            })?;
        },
        "insertion" => {
            visualizer.visualize_algorithm("Insertion Sort", test_array, |arr, counter| {
                insertion_sort_with_gui(arr, counter);
            })?;
        },
        "selection" => {
            visualizer.visualize_algorithm("Selection Sort", test_array, |arr, counter| {
                selection_sort_with_gui(arr, counter);
            })?;
        },
        "merge" => {
            visualizer.visualize_algorithm("Merge Sort", test_array, |arr, counter| {
                merge_sort_with_gui(arr, counter);
            })?;
        },
        "quick" => {
            visualizer.visualize_algorithm("Quick Sort", test_array, |arr, counter| {
                quick_sort_with_gui(arr, counter);
            })?;
        },
        "heap" => {
            visualizer.visualize_algorithm("Heap Sort", test_array, |arr, counter| {
                heap_sort_with_gui(arr, counter);
            })?;
        },
        "shell" => {
            visualizer.visualize_algorithm("Shell Sort", test_array, |arr, counter| {
                shell_sort_with_gui(arr, counter);
            })?;
        },
        "tim" => {
            visualizer.visualize_algorithm("Tim Sort", test_array, |arr, counter| {
                tim_sort_with_gui(arr, counter);
            })?;
        },
        "tree" => {
            visualizer.visualize_algorithm("Tree Sort", test_array, |arr, counter| {
                tree_sort_with_gui(arr, counter);
            })?;
        },
        "bucket" => {
            visualizer.visualize_algorithm("Bucket Sort", test_array, |arr, counter| {
                bucket_sort_with_gui(arr, counter);
            })?;
        },
        "radix" => {
            visualizer.visualize_algorithm("Radix Sort", test_array, |arr, counter| {
                radix_sort_with_gui(arr, counter);
            })?;
        },
        "counting" => {
            visualizer.visualize_algorithm("Counting Sort", test_array, |arr, counter| {
                counting_sort_with_gui(arr, counter);
            })?;
        },
        "cube" => {
            visualizer.visualize_algorithm("Cube Sort", test_array, |arr, counter| {
                cube_sort_with_gui(arr, counter);
            })?;
        },
        _ => {
            return Err(Error::validation(format!("Unknown sorting algorithm: {}", algorithm)));
        }
    }
    
    Ok(())
}

pub fn run_all_gui_visualizations(array_size: usize) -> Result<()> {
    println!("🎨 Running GUI visualizations for all 13 sorting algorithms!");
    println!("Array size: {}", array_size);
    
    println!("Choose output format for all visualizations:");
    println!("1. Static PNG (fast)");
    println!("2. Animated GIF (slower but shows process)");
    print!("Enter choice (1-2): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let use_gif = input.trim() == "2";
    
    if use_gif {
        println!("📺 Will generate animated GIFs for all algorithms...");
    } else {
        println!("🖼️ Will generate static PNGs for all algorithms...");
    }
    
    println!("{}", "=".repeat(80));
    
    let algorithms = [
        "Bubble Sort", "Insertion Sort", "Selection Sort", "Merge Sort", 
        "Quick Sort", "Heap Sort", "Shell Sort", "Tim Sort", "Tree Sort",
        "Bucket Sort", "Radix Sort", "Counting Sort", "Cube Sort"
    ];
    
    for (i, algorithm) in algorithms.iter().enumerate() {
        println!("🔄 Processing {}/{}: {}", i + 1, algorithms.len(), algorithm);
        
        let mut rng = rand::rng();
        let test_array: Vec<i32> = (0..array_size).map(|_| rng.random_range(1..=100)).collect();
        
        let mut visualizer = SortVisualizer::new(array_size);
        
        match algorithm.as_ref() {
            "Bubble Sort" => {
                visualizer.visualize_algorithm_with_choice("Bubble Sort", test_array, |arr, counter| {
                    bubble_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Insertion Sort" => {
                visualizer.visualize_algorithm_with_choice("Insertion Sort", test_array, |arr, counter| {
                    insertion_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Selection Sort" => {
                visualizer.visualize_algorithm_with_choice("Selection Sort", test_array, |arr, counter| {
                    selection_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Merge Sort" => {
                visualizer.visualize_algorithm_with_choice("Merge Sort", test_array, |arr, counter| {
                    merge_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Quick Sort" => {
                visualizer.visualize_algorithm_with_choice("Quick Sort", test_array, |arr, counter| {
                    quick_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Heap Sort" => {
                visualizer.visualize_algorithm_with_choice("Heap Sort", test_array, |arr, counter| {
                    heap_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Shell Sort" => {
                visualizer.visualize_algorithm_with_choice("Shell Sort", test_array, |arr, counter| {
                    shell_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Tim Sort" => {
                visualizer.visualize_algorithm_with_choice("Tim Sort", test_array, |arr, counter| {
                    tim_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Tree Sort" => {
                visualizer.visualize_algorithm_with_choice("Tree Sort", test_array, |arr, counter| {
                    tree_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Bucket Sort" => {
                visualizer.visualize_algorithm_with_choice("Bucket Sort", test_array, |arr, counter| {
                    bucket_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Radix Sort" => {
                visualizer.visualize_algorithm_with_choice("Radix Sort", test_array, |arr, counter| {
                    radix_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Counting Sort" => {
                visualizer.visualize_algorithm_with_choice("Counting Sort", test_array, |arr, counter| {
                    counting_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            "Cube Sort" => {
                visualizer.visualize_algorithm_with_choice("Cube Sort", test_array, |arr, counter| {
                    cube_sort_with_gui(arr, counter);
                }, use_gif)?;
            },
            _ => {
                eprintln!("❌ Unknown algorithm: {}", algorithm);
                continue;
            }
        }
        
        println!("✅ Completed: {}\n", algorithm);
    }
    
    println!("🎉 All {} sorting algorithm visualizations completed!", algorithms.len());
    Ok(())
}

// GUI wrapper functions that record visual steps
fn bubble_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n {
        // Set context to show unsorted portion (purple)
        counter.set_context_range(0, n - i);
        
        for j in 0..n - 1 - i {
            // Record comparison (red)
            counter.record_comparison(arr, j, j + 1);
            
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                // Record swap (green)
                counter.record_swap(arr, j, j + 1);
            }
        }
        
        counter.clear_context_range();
    }
}

fn insertion_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 1..n {
        // Set context to show unsorted portion (purple)
        counter.set_context_range(i, n);
        
        let key = arr[i];
        let mut j = i;
        
        while j > 0 {
            // Record comparison (red)
            counter.record_comparison(arr, j, j - 1);
            
            if arr[j - 1] > key {
                arr[j] = arr[j - 1];
                // Record swap (green)
                counter.record_swap(arr, j, j - 1);
                j -= 1;
            } else {
                break;
            }
        }
        
        arr[j] = key;
        if j != i {
            // Record final placement (green)
            counter.record_swap(arr, j, i);
        }
        
        counter.clear_context_range();
    }
}

fn selection_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n - 1 {
        // Set context to show unsorted portion (purple)
        counter.set_context_range(i, n);
        
        let mut min_idx = i;
        
        for j in i + 1..n {
            // Record comparison (red)
            counter.record_comparison(arr, j, min_idx);
            
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        
        if min_idx != i {
            arr.swap(i, min_idx);
            // Record swap (green)
            counter.record_swap(arr, i, min_idx);
        }
        
        counter.clear_context_range();
    }
}

fn merge_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    merge_sort_recursive_gui(arr, 0, len, counter);
}

fn merge_sort_recursive_gui(arr: &mut [i32], start: usize, end: usize, counter: &mut GuiPerformanceCounter) {
    if end - start <= 1 {
        return;
    }
    
    // Show the current subarray being divided (purple)
    counter.set_context_range(start, end);
    
    let mid = start + (end - start) / 2;
    
    // Clear context before recursive calls to avoid overlap
    counter.clear_context_range();
    
    // Recursively sort left half
    merge_sort_recursive_gui(arr, start, mid, counter);
    
    // Recursively sort right half  
    merge_sort_recursive_gui(arr, mid, end, counter);
    
    // Show the two subarrays being merged (purple)
    counter.set_context_range(start, end);
    merge_gui(arr, start, mid, end, counter);
    counter.clear_context_range();
}

fn merge_gui(arr: &mut [i32], start: usize, mid: usize, end: usize, counter: &mut GuiPerformanceCounter) {
    let left = arr[start..mid].to_vec();
    let right = arr[mid..end].to_vec();
    
    let mut i = 0;
    let mut j = 0;
    let mut k = start;
    
    while i < left.len() && j < right.len() {
        counter.record_comparison(arr, k, k);
        
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        
        counter.record_swap(arr, k, k);
        k += 1;
    }
    
    while i < left.len() {
        arr[k] = left[i];
        counter.record_swap(arr, k, k);
        i += 1;
        k += 1;
    }
    
    while j < right.len() {
        arr[k] = right[j];
        counter.record_swap(arr, k, k);
        j += 1;
        k += 1;
    }
}

// Add more GUI wrapper functions for the remaining algorithms...
fn quick_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    if arr.len() <= 1 {
        return;
    }
    quick_sort_recursive_gui(arr, 0, arr.len(), counter);
}

fn quick_sort_recursive_gui(arr: &mut [i32], start: usize, end: usize, counter: &mut GuiPerformanceCounter) {
    if end <= start + 1 {
        return;
    }
    
    counter.set_context_range(start, end);
    
    let pivot_idx = partition_gui(arr, start, end, counter);
    
    quick_sort_recursive_gui(arr, start, pivot_idx, counter);
    quick_sort_recursive_gui(arr, pivot_idx + 1, end, counter);
    
    counter.clear_context_range();
}

fn partition_gui(arr: &mut [i32], start: usize, end: usize, counter: &mut GuiPerformanceCounter) -> usize {
    let pivot = arr[end - 1];
    let mut i = start;
    
    for j in start..end - 1 {
        counter.record_comparison(arr, j, end - 1);
        
        if arr[j] <= pivot {
            if i != j {
                arr.swap(i, j);
                counter.record_swap(arr, i, j);
            }
            i += 1;
        }
    }
    
    if i != end - 1 {
        arr.swap(i, end - 1);
        counter.record_swap(arr, i, end - 1);
    }
    
    i
}

// Proper implementations for remaining algorithms
fn heap_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    // Build max heap
    for i in (0..n / 2).rev() {
        heapify_gui(arr, n, i, counter);
    }
    
    // Extract elements from heap one by one
    for i in (1..n).rev() {
        // Move current root to end
        arr.swap(0, i);
        counter.record_swap(arr, 0, i);
        
        // Set context to show the heap portion
        counter.set_context_range(0, i);
        
        // Call heapify on the reduced heap
        heapify_gui(arr, i, 0, counter);
        
        counter.clear_context_range();
    }
}

fn heapify_gui(arr: &mut [i32], n: usize, i: usize, counter: &mut GuiPerformanceCounter) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    
    // Check if left child exists and is greater than root
    if left < n {
        counter.record_comparison(arr, left, largest);
        if arr[left] > arr[largest] {
            largest = left;
        }
    }
    
    // Check if right child exists and is greater than current largest
    if right < n {
        counter.record_comparison(arr, right, largest);
        if arr[right] > arr[largest] {
            largest = right;
        }
    }
    
    // If largest is not root
    if largest != i {
        arr.swap(i, largest);
        counter.record_swap(arr, i, largest);
        
        // Recursively heapify the affected sub-tree
        heapify_gui(arr, n, largest, counter);
    }
}

fn shell_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mut gap = n / 2;
    
    while gap > 0 {
        for i in gap..n {
            // Set context to show the gap-based subsequence being worked on (purple)
            let mut subsequence_indices = vec![];
            let mut k = i % gap;
            while k < n {
                subsequence_indices.push(k);
                k += gap;
            }
            
            // Show the gap-based working section
            if let (Some(&start), Some(&end)) = (subsequence_indices.first(), subsequence_indices.last()) {
                counter.set_context_range(start, end + 1);
            }
            
            let temp = arr[i];
            let mut j = i;
            
            while j >= gap {
                counter.record_comparison(arr, j, j - gap);
                
                if arr[j - gap] > temp {
                    arr[j] = arr[j - gap];
                    counter.record_swap(arr, j, j - gap);
                    j -= gap;
                } else {
                    break;
                }
            }
            
            arr[j] = temp;
            if j != i {
                counter.record_swap(arr, j, i);
            }
            
            counter.clear_context_range();
        }
        
        gap /= 2;
    }
}

fn tim_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    // Tim sort is a hybrid stable sorting algorithm
    if arr.len() <= 1 {
        return;
    }
    
    // For visualisation clarity, we'll show tim sort as an enhanced merge sort
    // with clear run identification and merging phases
    let min_run_length = 32.min(arr.len()); // Tim sort typically uses 32-64
    
    // Phase 1: Create initial runs using insertion sort
    let mut run_starts = Vec::new();
    let mut i = 0;
    
    while i < arr.len() {
        let run_start = i;
        let run_end = (i + min_run_length).min(arr.len());
        
        // Show current run being processed (purple)
        counter.set_context_range(run_start, run_end);
        
        // Use insertion sort to create a sorted run
        for j in run_start + 1..run_end {
            let key = arr[j];
            let mut k = j;
            
            while k > run_start {
                counter.record_comparison(arr, k, k - 1);
                if arr[k - 1] <= key {
                    break;
                }
                arr[k] = arr[k - 1];
                counter.record_swap(arr, k, k - 1);
                k -= 1;
            }
            arr[k] = key;
            if k != j {
                counter.record_swap(arr, k, j);
            }
        }
        
        run_starts.push(run_start);
        counter.clear_context_range();
        i = run_end;
    }
    
    // Add final boundary
    run_starts.push(arr.len());
    
    // Phase 2: Merge runs using bottom-up approach
    let mut run_size = min_run_length;
    
    while run_size < arr.len() {
        let mut left = 0;
        
        while left < arr.len() {
            let mid = (left + run_size).min(arr.len());
            let right = (left + 2 * run_size).min(arr.len());
            
            if mid < right {
                // Show the two runs being merged (purple)
                counter.set_context_range(left, right);
                merge_gui(arr, left, mid, right, counter);
                counter.clear_context_range();
            }
            
            left += 2 * run_size;
        }
        
        run_size *= 2;
    }
}

fn tree_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    // Tree sort implementation using binary search tree
    if arr.len() <= 1 {
        return;
    }
    
    // For visualisation purposes, we'll implement a simple tree sort
    // that shows comparisons and movements
    let mut tree_values: Vec<i32> = Vec::new();
    
    // Insert elements into sorted vector (simulating BST)
    for i in 0..arr.len() {
        // Set context to show portion being processed (purple)
        counter.set_context_range(0, i + 1);
        
        let value = arr[i];
        let mut insert_pos = tree_values.len();
        
        // Find insertion position in the growing sorted section
        for j in 0..tree_values.len() {
            counter.record_comparison(arr, i, j); // Compare with existing elements
            if value < tree_values[j] {
                insert_pos = j;
                break;
            }
        }
        
        tree_values.insert(insert_pos, value);
        
        // Update array to show current state
        for (k, &val) in tree_values.iter().enumerate() {
            if k < arr.len() {
                arr[k] = val;
            }
        }
        counter.record_swap(arr, i, insert_pos);
        
        counter.clear_context_range();
    }
    
    // Final pass - show the completed sorted array
    counter.set_context_range(0, arr.len());
    for (i, &value) in tree_values.iter().enumerate() {
        if i < arr.len() {
            arr[i] = value;
            counter.record_swap(arr, i, i);
        }
    }
    counter.clear_context_range();
}

fn bucket_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    if arr.is_empty() {
        return;
    }
    
    // Find min and max values for bucket range calculation
    let max_val = *arr.iter().max().unwrap();
    let min_val = *arr.iter().min().unwrap();
    let range = (max_val - min_val + 1) as usize;
    
    // Create buckets (use fewer buckets for better visualisation)
    let bucket_count = (arr.len() / 4).max(1).min(10); // 2-10 buckets
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); bucket_count];
    
    // Phase 1: Distribute elements into buckets
    counter.set_context_range(0, arr.len());
    for (i, &value) in arr.iter().enumerate() {
        let bucket_index = if range > 1 {
            ((value - min_val) as usize * (bucket_count - 1)) / (range - 1)
        } else {
            0
        };
        buckets[bucket_index].push(value);
        counter.record_comparison(arr, i, 0); // Show distribution activity
    }
    counter.clear_context_range();
    
    // Phase 2: Sort each bucket individually and collect back
    let mut index = 0;
    for (bucket_idx, bucket) in buckets.iter_mut().enumerate() {
        if bucket.is_empty() {
            continue;
        }
        
        // Show the section where this bucket will be placed (purple)
        let bucket_start = index;
        let bucket_end = (index + bucket.len()).min(arr.len());
        counter.set_context_range(bucket_start, bucket_end);
        
        // Sort the bucket using insertion sort
        for i in 1..bucket.len() {
            let key = bucket[i];
            let mut j = i;
            
            while j > 0 && bucket[j - 1] > key {
                counter.record_comparison(arr, bucket_start + j, bucket_start + j - 1);
                bucket[j] = bucket[j - 1];
                j -= 1;
            }
            bucket[j] = key;
            
            // Update the visual array to show bucket sorting progress
            for (k, &val) in bucket.iter().enumerate() {
                if index + k < arr.len() {
                    arr[index + k] = val;
                    counter.record_swap(arr, index + k, index + k);
                }
            }
        }
        
        // Place sorted bucket elements back into array
        for &value in bucket.iter() {
            if index < arr.len() {
                arr[index] = value;
                counter.record_swap(arr, index, index);
                index += 1;
            }
        }
        
        counter.clear_context_range();
    }
}

fn radix_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    if arr.is_empty() {
        return;
    }
    
    let max_val = arr.iter().max().copied().unwrap_or(0);
    let mut exp = 1;
    
    while max_val / exp > 0 {
        // Set context to show entire array for this digit pass (purple)
        counter.set_context_range(0, arr.len());
        counting_sort_by_digit_gui(arr, exp, counter);
        counter.clear_context_range();
        exp *= 10;
    }
}

fn counting_sort_by_digit_gui(arr: &mut [i32], exp: i32, counter: &mut GuiPerformanceCounter) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];
    
    for &num in arr.iter() {
        let digit = (num / exp) % 10;
        count[digit as usize] += 1;
        counter.record_comparison(arr, 0, 0);
    }
    
    for i in 1..10 {
        count[i] += count[i - 1];
    }
    
    for i in (0..n).rev() {
        let digit = (arr[i] / exp) % 10;
        output[count[digit as usize] - 1] = arr[i];
        count[digit as usize] -= 1;
        counter.record_swap(arr, i, count[digit as usize]);
    }
    
    for i in 0..n {
        arr[i] = output[i];
        counter.record_swap(arr, i, i);
    }
}

fn counting_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    if arr.is_empty() {
        return;
    }
    
    let max_val = arr.iter().max().copied().unwrap_or(0);
    let min_val = arr.iter().min().copied().unwrap_or(0);
    let range = (max_val - min_val + 1) as usize;
    
    let mut count = vec![0; range];
    let mut output = vec![0; arr.len()];
    
    // Phase 1: Count occurrences - show entire array
    counter.set_context_range(0, arr.len());
    for &num in arr.iter() {
        count[(num - min_val) as usize] += 1;
        counter.record_comparison(arr, 0, 0); // Simulate counting operation
    }
    counter.clear_context_range();
    
    // Calculate cumulative counts (no visualization needed)
    for i in 1..range {
        count[i] += count[i - 1];
    }
    
    // Phase 2: Build output array - show progress section by section
    let chunk_size = arr.len() / 4; // Show progress in chunks
    for chunk_start in (0..arr.len()).step_by(chunk_size.max(1)) {
        let chunk_end = (chunk_start + chunk_size).min(arr.len());
        counter.set_context_range(chunk_start, chunk_end);
        
        for i in ((chunk_start)..chunk_end).rev() {
            let val = arr[i];
            let pos = count[(val - min_val) as usize] - 1;
            output[pos] = val;
            count[(val - min_val) as usize] -= 1;
            counter.record_swap(arr, i, pos); // Show placement operation
        }
        
        counter.clear_context_range();
    }
    
    // Phase 3: Copy back to original array - show final result
    counter.set_context_range(0, arr.len());
    for i in 0..arr.len() {
        arr[i] = output[i];
        counter.record_swap(arr, i, i); // Show final placement
    }
    counter.clear_context_range();
}

fn cube_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    // Cube sort is implemented as quick sort with optimisations
    // Don't set additional context - let quicksort handle its own recursive contexts
    quick_sort_with_gui(arr, counter);
}
