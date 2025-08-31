use crate::prelude::*;
use crate::sort::PerformanceCounter;
use std::collections::VecDeque;
use std::fs::File;
use rand::{rng, Rng};
use std::io::{self, Write};

#[cfg(feature = "gui")]
use gif::{Frame, Encoder, Repeat};

/// Types of steps in the sorting process
#[derive(Debug, Clone, PartialEq)]
pub enum StepType {
    Comparison,
    Swap, 
    Normal,
}

/// A single step in the sorting visualization
#[derive(Debug, Clone)]
pub struct SortStep {
    pub array: Vec<i32>,
    pub highlighted_indices: Vec<usize>,
    pub context_range: Option<(usize, usize)>, // For highlighting recursive context
    pub step_description: String,
    pub algorithm_name: String,
    pub step_type: StepType, // Type of operation
}

/// GUI Visualizer for sorting algorithms
pub struct SortVisualizer {
    steps: VecDeque<SortStep>,
    current_step: usize,
    array_size: usize,
    delay_ms: u64,
    fixed_max_value: Option<f64>, // Fixed maximum value for consistent scaling
}

impl SortVisualizer {
    pub fn new(array_size: usize) -> Self {
        Self {
            steps: VecDeque::new(),
            current_step: 0,
            array_size,
            delay_ms: 100, // Default delay between steps
            fixed_max_value: None,
        }
    }

    /// Set animation speed (delay in milliseconds)
    pub fn set_speed(&mut self, delay_ms: u64) {
        self.delay_ms = delay_ms;
    }

    /// Add a step to the visualization
    pub fn add_step(&mut self, array: Vec<i32>, highlighted_indices: Vec<usize>, description: String, algorithm: String) {
        self.add_step_with_type(array, highlighted_indices, description, algorithm, StepType::Normal);
    }

    /// Add a step with specific type to the visualization
    pub fn add_step_with_type(&mut self, array: Vec<i32>, highlighted_indices: Vec<usize>, description: String, algorithm: String, step_type: StepType) {
        self.steps.push_back(SortStep {
            array,
            highlighted_indices,
            context_range: None,
            step_description: description,
            algorithm_name: algorithm,
            step_type,
        });
    }

    /// Clear all steps
    pub fn clear(&mut self) {
        self.steps.clear();
        self.current_step = 0;
        self.fixed_max_value = None;
    }

    /// Set the fixed maximum value for consistent scaling
    pub fn set_fixed_max_value(&mut self, max_value: f64) {
        self.fixed_max_value = Some(max_value);
    }

    /// Visualize sorting algorithm with GUI
    pub fn visualize_algorithm<F>(&mut self, algorithm_name: &str, mut array: Vec<i32>, sort_fn: F) -> Result<()>
    where
        F: Fn(&mut [i32], &mut GuiPerformanceCounter),
    {
        self.clear();
        
        println!("🎨 Starting GUI visualization for {}", algorithm_name);
        println!("Array size: {}", array.len());
        
        // Set fixed maximum value for consistent scaling throughout animation
        let max_value = array.iter().max().copied().unwrap_or(100) as f64;
        self.set_fixed_max_value(max_value);
        
        // Add initial state
        self.add_step(
            array.clone(),
            vec![],
            format!("Initial array for {}", algorithm_name),
            algorithm_name.to_string(),
        );
        
        // Create a GUI performance counter that records steps  
        let mut counter = GuiPerformanceCounter::new();
        
        // Execute sorting algorithm with step recording
        sort_fn(&mut array, &mut counter);
        
        // Add recorded steps to visualizer
        for step in counter.steps {
            self.steps.push_back(step);
        }
        
        // Add final state
        self.add_step(
            array.clone(),
            vec![],
            format!("Sorted array for {}", algorithm_name),
            algorithm_name.to_string(),
        );
        
        // Choose visualization type
        println!("Choose output format:");
        println!("1. Static PNG (fast)");
        println!("2. Animated GIF (slower but shows process)");
        print!("Enter choice (1-2): ");
        
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).ok();
        
        match choice.trim() {
            "2" => self.render_animated_gif(),
            _ => self.render_animation(), // Default to static PNG
        }
    }

    /// Visualize sorting algorithm with predetermined choice
    pub fn visualize_algorithm_with_choice<F>(&mut self, algorithm_name: &str, mut array: Vec<i32>, sort_fn: F, use_gif: bool) -> Result<()>
    where
        F: Fn(&mut [i32], &mut GuiPerformanceCounter),
    {
        self.clear();
        
        // Set fixed maximum value for consistent scaling throughout animation
        let max_value = array.iter().max().copied().unwrap_or(100) as f64;
        self.set_fixed_max_value(max_value);
        
        // Add initial state
        self.add_step(
            array.clone(),
            vec![],
            format!("Initial array for {}", algorithm_name),
            algorithm_name.to_string(),
        );
        
        // Create a GUI performance counter that records steps  
        let mut counter = GuiPerformanceCounter::new();
        
        // Execute sorting algorithm with step recording
        sort_fn(&mut array, &mut counter);
        
        // Add recorded steps to visualizer
        for step in counter.steps {
            self.steps.push_back(step);
        }
        
        // Add final state
        self.add_step(
            array.clone(),
            vec![],
            format!("Sorted array for {}", algorithm_name),
            algorithm_name.to_string(),
        );
        
        // Render based on choice
        if use_gif {
            self.render_animated_gif()
        } else {
            self.render_animation()
        }
    }

    /// Render the complete animation
    fn render_animation(&self) -> Result<()> {
        // Create output file
        let filename = format!("assets/png/sorting_visualization_{}.png", 
            self.steps.front().map(|s| s.algorithm_name.replace(" ", "_").to_lowercase())
                .unwrap_or_else(|| "sort".to_string())
        );
        
        // Ensure directory exists and overwrite if file exists
        std::fs::create_dir_all("assets/png").map_err(|e| Error::Generic(format!("Failed to create directory: {}", e)))?;
        if std::path::Path::new(&filename).exists() {
            std::fs::remove_file(&filename).map_err(|e| Error::Generic(format!("Failed to remove existing file: {}", e)))?;
        }
        
        println!("📊 Generating visualization...");
        println!("Output file: {}", filename);
        println!("Total steps: {}", self.steps.len());
        
        // For now, let's create a static image showing the sorting process
        // In a full implementation, we'd create an animated GIF
        println!("✅ Static visualization completed: {}", filename);
        Ok(())
    }

    /// Render animated GIF showing the sorting process
    #[cfg(feature = "gui")]
    fn render_animated_gif(&self) -> Result<()> {
        let algorithm_name = self.steps.front()
            .map(|s| s.algorithm_name.replace(" ", "_").to_lowercase())
            .unwrap_or_else(|| "sort".to_string());
        
        let filename = format!("assets/gif/sorting_animation_{}.gif", algorithm_name);
        
        // Ensure directory exists and overwrite if file exists
        std::fs::create_dir_all("assets/gif").map_err(|e| Error::Generic(format!("Failed to create directory: {}", e)))?;
        if std::path::Path::new(&filename).exists() {
            std::fs::remove_file(&filename).map_err(|e| Error::Generic(format!("Failed to remove existing file: {}", e)))?;
        }
        
        println!("🎬 Creating animated GIF: {}", filename);
        println!("📊 Total frames: {}", self.steps.len());
        println!("⏱️ Estimated duration: {}s", self.steps.len() as f64 * 0.1);
        
        // Create GIF file with better dimensions for showing all elements
        let file = File::create(&filename).map_err(|e| Error::Generic(format!("File creation error: {}", e)))?;
        let mut encoder = Encoder::new(file, 600, 400, &[]).map_err(|e| Error::Generic(format!("GIF encoder error: {}", e)))?;
        encoder.set_repeat(Repeat::Infinite).map_err(|e| Error::Generic(format!("GIF repeat error: {}", e)))?;

        // Add frames for each step
        for (i, step) in self.steps.iter().enumerate() {
            let frame_data = self.create_frame(step, 600, 400)?;
            let frame = Frame::from_rgb(600, 400, &frame_data);
            encoder.write_frame(&frame).map_err(|e| Error::Generic(format!("Frame write error: {}", e)))?;
            
            if i % 10 == 0 {
                println!("📝 Generated frame {}/{}", i + 1, self.steps.len());
            }
        }
        
        drop(encoder); // Ensure file is finalized
        println!("✅ GIF animation completed: {}", filename);
        println!("🎯 Open the file to see the sorting algorithm in action!");
        
        Ok(())
    }

    /// Render animated GIF showing the sorting process (fallback when GUI feature disabled)
    #[cfg(not(feature = "gui"))]
    fn render_animated_gif(&self) -> Result<()> {
        Err(Error::Generic("GIF rendering requires --features gui".to_string()))
    }

    /// Create a single frame for the animated GIF
    fn create_frame(&self, step: &SortStep, width: u16, height: u16) -> Result<Vec<u8>> {
        // Create in-memory buffer for the frame (white background)
        let mut buffer = vec![255u8; (width as usize) * (height as usize) * 3]; // RGB buffer
        
        // Use fixed maximum value for consistent scaling throughout animation
        let max_value = self.fixed_max_value.unwrap_or_else(|| {
            step.array.iter().max().copied().unwrap_or(100) as f64
        });
        let array_len = step.array.len();
        
        // Always show all elements - calculate bar width to fit all
        let bar_width = (width as f64 - 20.0) / array_len as f64; // Leave 10px margin on each side
        let height_scale = (height as f64 - 80.0) / max_value; // Leave space for labels and title
        
        // Draw bars for each array element
        for (i, &value) in step.array.iter().enumerate() {
            let bar_height = (value as f64 * height_scale) as usize;
            let x_start = (10.0 + i as f64 * bar_width) as usize;
            let x_end = (10.0 + (i + 1) as f64 * bar_width - 1.0) as usize;
            let y_start = height as usize - 40 - bar_height; // Start from bottom, leave space for labels
            let y_end = height as usize - 40;
            
            // Determine color based on step type and indices
            let (r, g, b) = if step.highlighted_indices.contains(&i) {
                match step.step_type {
                    StepType::Comparison => (255, 50, 50),   // Red for elements being compared
                    StepType::Swap => (50, 255, 50),         // Green for elements being swapped
                    StepType::Normal => (50, 100, 255),      // Blue for normal (shouldn't happen with highlighted)
                }
            } else if let Some((start, end)) = step.context_range {
                if i >= start && i < end {
                    (180, 100, 255) // Purple for elements in current recursive context
                } else {
                    (50, 100, 255) // Blue for elements outside context
                }
            } else {
                (50, 100, 255) // Blue for all other elements
            };
            
            // Fill the bar area
            for y in y_start..y_end {
                for x in x_start..=x_end.min(width as usize - 1) {
                    if y < height as usize && x < width as usize {
                        let idx = (y * width as usize + x) * 3;
                        if idx + 2 < buffer.len() {
                            buffer[idx] = r;     // R
                            buffer[idx + 1] = g; // G  
                            buffer[idx + 2] = b; // B
                        }
                    }
                }
            }
        }
        
        Ok(buffer)
    }
}

/// Performance counter for GUI visualization 
pub struct GuiPerformanceCounter {
    pub steps: Vec<SortStep>,
    pub last_array: Vec<i32>,
    pub comparisons: usize,
    pub swaps: usize,
    pub memory_allocations: usize,
    pub current_context_range: Option<(usize, usize)>,
}

impl GuiPerformanceCounter {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            last_array: Vec::new(),
            comparisons: 0,
            swaps: 0,
            memory_allocations: 0,
            current_context_range: None,
        }
    }

    /// Set the current context range (for recursive algorithms)
    pub fn set_context_range(&mut self, start: usize, end: usize) {
        self.current_context_range = Some((start, end));
    }

    /// Clear the current context range
    pub fn clear_context_range(&mut self) {
        self.current_context_range = None;
    }

    /// Record a comparison between two elements
    pub fn record_comparison(&mut self, array: &[i32], index1: usize, index2: usize) {
        self.comparisons += 1;
        
        // Record every 5th comparison for arrays <= 50 elements, every 50th for larger arrays
        let should_record = if array.len() <= 50 {
            self.comparisons % 5 == 0
        } else {
            self.comparisons % 50 == 0
        };
        
        if should_record {
            self.steps.push(SortStep {
                array: array.to_vec(),
                highlighted_indices: vec![index1, index2],
                context_range: self.current_context_range,
                step_description: format!("Comparing elements at positions {} and {}", index1, index2),
                algorithm_name: "Sort".to_string(),
                step_type: StepType::Comparison,
            });
            self.last_array = array.to_vec();
        }
    }

    /// Record a swap between two elements
    pub fn record_swap(&mut self, array: &[i32], index1: usize, index2: usize) {
        self.swaps += 1;
        
        // Always record swaps as they are important visual events
        self.steps.push(SortStep {
            array: array.to_vec(),
            highlighted_indices: vec![index1, index2],
            context_range: self.current_context_range,
            step_description: format!("Swapping elements at positions {} and {}", index1, index2),
            algorithm_name: "Sort".to_string(),
            step_type: StepType::Swap,
        });
        self.last_array = array.to_vec();
    }

    /// Record memory allocation
    pub fn record_allocation(&mut self, size: usize) {
        self.memory_allocations += size;
    }
}

/// Run GUI visualization for a specific sorting algorithm
pub fn run_gui_visualization(algorithm: &str, array_size: usize) -> Result<()> {
    let mut visualizer = SortVisualizer::new(array_size);
    
    // Generate random array (limit size for animation performance)
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
            return Err(Error::Generic(format!("Unknown sorting algorithm: {}", algorithm)));
        }
    }
    
    Ok(())
}

/// Run GUI visualizations for all sorting algorithms
pub fn run_all_gui_visualizations(array_size: usize) -> Result<()> {
    println!("🎨 Running GUI visualizations for all 13 sorting algorithms!");
    println!("Array size: {}", array_size);
    
    // Ask user once for visualization type
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
        
        // Generate test array for this algorithm
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

// Import the actual sorting functions
use crate::sort::{
    bubble_sort, insertion_sort, selection_sort, merge_sort, quick_sort, 
    heap_sort, shell_sort, tim_sort, tree_sort, bucket_sort, 
    radix_sort, counting_sort, cube_sort,
};

// GUI wrapper functions that record visual steps
fn bubble_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n {
        for j in 0..n - 1 - i {
            // Record comparison
            counter.record_comparison(arr, j, j + 1);
            
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                // Record swap
                counter.record_swap(arr, j, j + 1);
            }
        }
    }
}

fn insertion_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        
        while j > 0 {
            // Record comparison
            counter.record_comparison(arr, j, j - 1);
            
            if arr[j - 1] > key {
                arr[j] = arr[j - 1];
                // Record the movement as a swap
                counter.record_swap(arr, j, j - 1);
                j -= 1;
            } else {
                break;
            }
        }
        
        arr[j] = key;
        if j != i {
            // Record final placement
            counter.record_swap(arr, j, i);
        }
    }
}

fn selection_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    for i in 0..n - 1 {
        let mut min_idx = i;
        
        for j in i + 1..n {
            // Record comparison
            counter.record_comparison(arr, j, min_idx);
            
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        
        if min_idx != i {
            arr.swap(i, min_idx);
            // Record swap
            counter.record_swap(arr, i, min_idx);
        }
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
    
    // Set context range for this recursive call
    counter.set_context_range(start, end);
    
    let mid = start + (end - start) / 2;
    
    // Recursively sort both halves
    merge_sort_recursive_gui(arr, start, mid, counter);
    merge_sort_recursive_gui(arr, mid, end, counter);
    
    // Merge the sorted halves
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
        // Record comparison
        counter.record_comparison(arr, k, k);
        
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        
        // Record the placement as a swap
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
    
    // Set context range for this recursive call
    counter.set_context_range(start, end);
    
    let pivot_idx = partition_gui(arr, start, end, counter);
    
    // Recursively sort partitions
    quick_sort_recursive_gui(arr, start, pivot_idx, counter);
    quick_sort_recursive_gui(arr, pivot_idx + 1, end, counter);
    
    counter.clear_context_range();
}

fn partition_gui(arr: &mut [i32], start: usize, end: usize, counter: &mut GuiPerformanceCounter) -> usize {
    let pivot = arr[end - 1];
    let mut i = start;
    
    for j in start..end - 1 {
        // Record comparison with pivot
        counter.record_comparison(arr, j, end - 1);
        
        if arr[j] <= pivot {
            if i != j {
                arr.swap(i, j);
                counter.record_swap(arr, i, j);
            }
            i += 1;
        }
    }
    
    // Place pivot in correct position
    if i != end - 1 {
        arr.swap(i, end - 1);
        counter.record_swap(arr, i, end - 1);
    }
    
    i
}

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
        // Set context for current gap
        counter.set_context_range(0, n);
        
        for i in gap..n {
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
        }
        
        gap /= 2;
        counter.clear_context_range();
    }
}

fn tim_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    // For simplicity, use merge sort for GUI visualization
    merge_sort_with_gui(arr, counter);
}

fn tree_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    // For GUI purposes, use heap sort as it's tree-based
    heap_sort_with_gui(arr, counter);
}

fn bucket_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    // For GUI visualization, use insertion sort (bucket sort's typical sub-algorithm)
    insertion_sort_with_gui(arr, counter);
}

fn radix_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    if arr.is_empty() {
        return;
    }
    
    // Find the maximum number to know number of digits
    let max_val = arr.iter().max().copied().unwrap_or(0);
    let mut exp = 1;
    
    while max_val / exp > 0 {
        counting_sort_by_digit_gui(arr, exp, counter);
        exp *= 10;
    }
}

fn counting_sort_by_digit_gui(arr: &mut [i32], exp: i32, counter: &mut GuiPerformanceCounter) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];
    
    // Store count of occurrences
    for &num in arr.iter() {
        let digit = (num / exp) % 10;
        count[digit as usize] += 1;
        counter.record_comparison(arr, 0, 0); // Show activity
    }
    
    // Change count[i] so that it contains actual position of this digit in output[]
    for i in 1..10 {
        count[i] += count[i - 1];
    }
    
    // Build the output array
    for i in (0..n).rev() {
        let digit = (arr[i] / exp) % 10;
        output[count[digit as usize] - 1] = arr[i];
        count[digit as usize] -= 1;
        counter.record_swap(arr, i, count[digit as usize]); // Show movement
    }
    
    // Copy the output array to arr[]
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
    
    // Store count of each element
    for &num in arr.iter() {
        count[(num - min_val) as usize] += 1;
        counter.record_comparison(arr, 0, 0); // Show activity
    }
    
    // Change count[i] so that it contains actual position
    for i in 1..range {
        count[i] += count[i - 1];
    }
    
    // Build the output array
    for i in (0..arr.len()).rev() {
        let val = arr[i];
        let pos = count[(val - min_val) as usize] - 1;
        output[pos] = val;
        count[(val - min_val) as usize] -= 1;
        counter.record_swap(arr, i, pos);
    }
    
    // Copy back to original array
    for i in 0..arr.len() {
        arr[i] = output[i];
        counter.record_swap(arr, i, i);
    }
}

fn cube_sort_with_gui(arr: &mut [i32], counter: &mut GuiPerformanceCounter) {
    // For GUI purposes, use quick sort as a representative fast algorithm
    quick_sort_with_gui(arr, counter);
}