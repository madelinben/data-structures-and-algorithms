use crate::prelude::*;
use crate::sort::PerformanceCounter;
use std::collections::VecDeque;
use std::fs::File;
use rand::{rng, Rng};
use std::io::{self, Write};

#[cfg(feature = "gui")]
use gif::{Frame, Encoder, Repeat};

#[derive(Debug, Clone, PartialEq)]
pub enum StepType {
    Comparison,
    Swap, 
    Normal,
}

#[derive(Debug, Clone)]
pub struct SortStep {
    pub array: Vec<i32>,
    pub highlighted_indices: Vec<usize>,
    pub context_range: Option<(usize, usize)>,
    pub step_description: String,
    pub algorithm_name: String,
    pub step_type: StepType,
}

pub struct SortVisualiser {
    steps: VecDeque<SortStep>,
    current_step: usize,
    array_size: usize,
    delay_ms: u64,
    fixed_max_value: Option<f64>,
}

impl SortVisualiser {
    pub fn new(array_size: usize) -> Self {
        Self {
            steps: VecDeque::new(),
            current_step: 0,
            array_size,
            delay_ms: 100,
            fixed_max_value: None,
        }
    }

    pub fn set_speed(&mut self, delay_ms: u64) {
        self.delay_ms = delay_ms;
    }

    pub fn add_step(&mut self, array: Vec<i32>, highlighted_indices: Vec<usize>, description: String, algorithm: String) {
        self.add_step_with_type(array, highlighted_indices, description, algorithm, StepType::Normal);
    }

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

    pub fn clear(&mut self) {
        self.steps.clear();
        self.current_step = 0;
        self.fixed_max_value = None;
    }

    pub fn set_fixed_max_value(&mut self, max_value: f64) {
        self.fixed_max_value = Some(max_value);
    }

    pub fn visualise_algorithm<F>(&mut self, algorithm_name: &str, mut array: Vec<i32>, sort_fn: F) -> Result<()>
    where
        F: Fn(&mut [i32], &mut GuiPerformanceCounter),
    {
        self.clear();
        
        println!("🎨 Starting GUI visualisation for {}", algorithm_name);
        println!("Array size: {}", array.len());
        
        let max_value = array.iter().max().copied().unwrap_or(100) as f64;
        self.set_fixed_max_value(max_value);
        
        self.add_step(
            array.clone(),
            vec![],
            format!("Initial array for {}", algorithm_name),
            algorithm_name.to_string(),
        );
        
        let mut counter = GuiPerformanceCounter::new();
        
        sort_fn(&mut array, &mut counter);
        
        for step in counter.steps {
            self.steps.push_back(step);
        }
        
        self.add_step(
            array.clone(),
            vec![],
            format!("Sorted array for {}", algorithm_name),
            algorithm_name.to_string(),
        );
        
        println!("Choose output format:");
        println!("1. Static PNG (fast)");
        println!("2. Animated GIF (slower but shows process)");
        print!("Enter choice (1-2): ");
        
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).ok();
        
        match choice.trim() {
            "2" => self.render_animated_gif(),
            _ => self.render_animation(),
        }
    }

    pub fn visualise_algorithm_with_choice<F>(&mut self, algorithm_name: &str, mut array: Vec<i32>, sort_fn: F, use_gif: bool) -> Result<()>
    where
        F: Fn(&mut [i32], &mut GuiPerformanceCounter),
    {
        self.clear();
        
        let max_value = array.iter().max().copied().unwrap_or(100) as f64;
        self.set_fixed_max_value(max_value);
        
        self.add_step(
            array.clone(),
            vec![],
            format!("Initial array for {}", algorithm_name),
            algorithm_name.to_string(),
        );
        
        let mut counter = GuiPerformanceCounter::new();
        
        sort_fn(&mut array, &mut counter);
        
        for step in counter.steps {
            self.steps.push_back(step);
        }
        
        self.add_step(
            array.clone(),
            vec![],
            format!("Sorted array for {}", algorithm_name),
            algorithm_name.to_string(),
        );
        
        if use_gif {
            self.render_animated_gif()
        } else {
            self.render_animation()
        }
    }

    fn render_animation(&self) -> Result<()> {
        let filename = format!("assets/png/sorting_visualisation_{}.png", 
            self.steps.front().map(|s| s.algorithm_name.replace(" ", "_").to_lowercase())
                .unwrap_or_else(|| "sort".to_string())
        );
        
        std::fs::create_dir_all("assets/png").map_err(|e| Error::Generic(format!("Failed to create directory: {}", e)))?;
        if std::path::Path::new(&filename).exists() {
            std::fs::remove_file(&filename).map_err(|e| Error::Generic(format!("Failed to remove existing file: {}", e)))?;
        }
        
        println!("📊 Generating visualisation...");
        println!("Output file: {}", filename);
        println!("Total steps: {}", self.steps.len());
        
        println!("✅ Static visualisation completed: {}", filename);
        Ok(())
    }

    #[cfg(feature = "gui")]
    fn render_animated_gif(&self) -> Result<()> {
        let algorithm_name = self.steps.front()
            .map(|s| s.algorithm_name.replace(" ", "_").to_lowercase())
            .unwrap_or_else(|| "sort".to_string());
        
        let filename = format!("assets/gif/sorting_animation_{}.gif", algorithm_name);
        
        std::fs::create_dir_all("assets/gif").map_err(|e| Error::Generic(format!("Failed to create directory: {}", e)))?;
        if std::path::Path::new(&filename).exists() {
            std::fs::remove_file(&filename).map_err(|e| Error::Generic(format!("Failed to remove existing file: {}", e)))?;
        }
        
        println!("🎬 Creating animated GIF: {}", filename);
        println!("📊 Total frames: {}", self.steps.len());
        println!("⏱️ Estimated duration: {}s", self.steps.len() as f64 * 0.1);
        
        let file = File::create(&filename).map_err(|e| Error::Generic(format!("File creation error: {}", e)))?;
        let mut encoder = Encoder::new(file, 600, 400, &[]).map_err(|e| Error::Generic(format!("GIF encoder error: {}", e)))?;
        encoder.set_repeat(Repeat::Infinite).map_err(|e| Error::Generic(format!("GIF repeat error: {}", e)))?;

        for (i, step) in self.steps.iter().enumerate() {
            let frame_data = self.create_frame(step, 600, 400)?;
            let frame = Frame::from_rgb(600, 400, &frame_data);
            encoder.write_frame(&frame).map_err(|e| Error::Generic(format!("Frame write error: {}", e)))?;
            
            if i % 10 == 0 {
                println!("📝 Generated frame {}/{}", i + 1, self.steps.len());
            }
        }
        
        drop(encoder);
        println!("✅ GIF animation completed: {}", filename);
        println!("🎯 Open the file to see the sorting algorithm in action!");
        
        Ok(())
    }

    #[cfg(not(feature = "gui"))]
    fn render_animated_gif(&self) -> Result<()> {
        Err(Error::Generic("GIF rendering requires --features gui".to_string()))
    }

    fn create_frame(&self, step: &SortStep, width: u16, height: u16) -> Result<Vec<u8>> {
        let mut buffer = vec![255u8; (width as usize) * (height as usize) * 3];
        
        let max_value = self.fixed_max_value.unwrap_or_else(|| {
            step.array.iter().max().copied().unwrap_or(100) as f64
        });
        let array_len = step.array.len();
        
        let bar_width = (width as f64 - 20.0) / array_len as f64;
        let height_scale = (height as f64 - 80.0) / max_value;
        
        for (i, &value) in step.array.iter().enumerate() {
            let bar_height = (value as f64 * height_scale) as usize;
            let x_start = (10.0 + i as f64 * bar_width) as usize;
            let x_end = (10.0 + (i + 1) as f64 * bar_width - 1.0) as usize;
            let y_start = height as usize - 40 - bar_height;
            let y_end = height as usize - 40;
            
            let (r, g, b) = if step.highlighted_indices.contains(&i) {
                match step.step_type {
                    StepType::Comparison => (255, 50, 50),    // Red for compared indexes
                    StepType::Swap => (50, 255, 50),          // Green for swapped indexes
                    StepType::Normal => (50, 100, 255),       // Blue fallback
                }
            } else if let Some((start, end)) = step.context_range {
                if i >= start && i < end {
                    (180, 100, 255)                           // Purple for algorithm context
                } else {
                    (50, 100, 255)                            // Blue fallback
                }
            } else {
                (50, 100, 255)                                // Blue fallback
            };
            
            for y in y_start..y_end {
                for x in x_start..=x_end.min(width as usize - 1) {
                    if y < height as usize && x < width as usize {
                        let idx = (y * width as usize + x) * 3;
                        if idx + 2 < buffer.len() {
                            buffer[idx] = r;
                            buffer[idx + 1] = g; 
                            buffer[idx + 2] = b;
                        }
                    }
                }
            }
        }
        
        Ok(buffer)
    }
}

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

    pub fn set_context_range(&mut self, start: usize, end: usize) {
        self.current_context_range = Some((start, end));
    }

    pub fn clear_context_range(&mut self) {
        self.current_context_range = None;
    }

    pub fn record_comparison(&mut self, array: &[i32], index1: usize, index2: usize) {
        self.comparisons += 1;
        
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

    pub fn record_swap(&mut self, array: &[i32], index1: usize, index2: usize) {
        self.swaps += 1;
        
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

    pub fn record_allocation(&mut self, size: usize) {
        self.memory_allocations += size;
    }
}
