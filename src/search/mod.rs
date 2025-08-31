//! Search Algorithms Module
//! 
//! This module provides various search algorithm implementations with performance analysis.
//! It loads word lists, shuffles them, and allows benchmarking different search strategies.

pub mod linear_search;
pub mod binary_search;
pub mod hash_search;
pub mod interpolation_search;
pub mod exponential_search;
pub mod jump_search;

use crate::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::prelude::*;
use rand::rng;
use prettytable::{Table, Row, Cell};

/// Performance metrics for search algorithms
#[derive(Debug, Clone)]
pub struct SearchMetrics {
    pub algorithm_name: String,
    pub target_found: bool,
    pub comparisons: usize,
    pub duration: Duration,
    pub theoretical_complexity: String,
    pub actual_complexity: f64,
}

/// Main search algorithm coordinator
pub struct SearchCoordinator {
    /// Original word list
    words: Vec<String>,
    /// Shuffled word list for linear searches
    shuffled_words: Vec<String>,
    /// Sorted word list for binary searches
    sorted_words: Vec<String>,
    /// Hash map for O(1) lookups
    word_map: HashMap<String, usize>,
}

impl SearchCoordinator {
    /// Create a new search coordinator
    pub fn new() -> Self {
        Self {
            words: Vec::new(),
            shuffled_words: Vec::new(),
            sorted_words: Vec::new(),
            word_map: HashMap::new(),
        }
    }

    /// Load words from file and prepare data structures
    pub async fn load_words(&mut self, file_path: &str) -> Result<()> {
        println!("Loading words from: {}", file_path);
        let content = tokio::fs::read_to_string(file_path).await
            .map_err(|e| Error::Generic(format!("Failed to read file {}: {}", file_path, e)))?;

        // Parse words from file
        self.words = content
            .lines()
            .map(|line| line.trim().to_lowercase())
            .filter(|line| !line.is_empty() && line.len() >= 2)
            .collect();

        if self.words.is_empty() {
            return Err(Error::Generic("No valid words found in file".to_string()));
        }

        // Create shuffled version
        self.shuffled_words = self.words.clone();
        let mut rng = rng();
        self.shuffled_words.shuffle(&mut rng);

        // Create sorted version  
        self.sorted_words = self.words.clone();
        self.sorted_words.sort_unstable();

        // Create hash map
        self.word_map = self.words
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect();

        println!("✓ Loaded {} words", self.words.len());
        println!("✓ Created shuffled array");
        println!("✓ Created sorted array");  
        println!("✓ Created hash map");

        Ok(())
    }

    /// Run comprehensive search benchmarks
    pub fn run_benchmarks(&self, target_word: &str, iterations: usize) -> Result<Vec<SearchMetrics>> {
        if self.words.is_empty() {
            return Err(Error::Generic("No words loaded. Load words first.".to_string()));
        }

        println!("\nRunning search benchmarks for target: '{}'", target_word);
        println!("Iterations per algorithm: {}", iterations);
        println!("{}", "=".repeat(60));

        let mut results = Vec::new();

        // Linear Search (on shuffled array)
        results.push(self.benchmark_linear_search(target_word, iterations)?);

        // Binary Search (on sorted array)
        results.push(self.benchmark_binary_search(target_word, iterations)?);

        // Hash Search (using HashMap)
        results.push(self.benchmark_hash_search(target_word, iterations)?);

        // Interpolation Search (on sorted array)
        results.push(self.benchmark_interpolation_search(target_word, iterations)?);

        // Jump Search (on sorted array)
        results.push(self.benchmark_jump_search(target_word, iterations)?);

        // Exponential Search (on sorted array)
        results.push(self.benchmark_exponential_search(target_word, iterations)?);

        self.display_results(&results);
        Ok(results)
    }

    /// Benchmark linear search
    fn benchmark_linear_search(&self, target: &str, iterations: usize) -> Result<SearchMetrics> {
        let mut total_comparisons = 0;
        let mut found_count = 0;
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let (found, comparisons) = linear_search::search(&self.shuffled_words, target);
            total_comparisons += comparisons;
            if found {
                found_count += 1;
            }
        }
        
        let duration = start.elapsed();
        let avg_comparisons = total_comparisons / iterations;
        
        Ok(SearchMetrics {
            algorithm_name: "Linear Search".to_string(),
            target_found: found_count > 0,
            comparisons: avg_comparisons,
            duration,
            theoretical_complexity: "O(n)".to_string(),
            actual_complexity: avg_comparisons as f64 / self.shuffled_words.len() as f64,
        })
    }

    /// Benchmark binary search
    fn benchmark_binary_search(&self, target: &str, iterations: usize) -> Result<SearchMetrics> {
        let mut total_comparisons = 0;
        let mut found_count = 0;
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let (found, comparisons) = binary_search::search(&self.sorted_words, target);
            total_comparisons += comparisons;
            if found {
                found_count += 1;
            }
        }
        
        let duration = start.elapsed();
        let avg_comparisons = total_comparisons / iterations;
        
        Ok(SearchMetrics {
            algorithm_name: "Binary Search".to_string(),
            target_found: found_count > 0,
            comparisons: avg_comparisons,
            duration,
            theoretical_complexity: "O(log n)".to_string(),
            actual_complexity: avg_comparisons as f64 / (self.sorted_words.len() as f64).log2(),
        })
    }

    /// Benchmark hash search
    fn benchmark_hash_search(&self, target: &str, iterations: usize) -> Result<SearchMetrics> {
        let mut found_count = 0;
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let found = hash_search::search(&self.word_map, target);
            if found {
                found_count += 1;
            }
        }
        
        let duration = start.elapsed();
        
        Ok(SearchMetrics {
            algorithm_name: "Hash Search".to_string(),
            target_found: found_count > 0,
            comparisons: 1, // Hash lookup is essentially O(1)
            duration,
            theoretical_complexity: "O(1)".to_string(),
            actual_complexity: 1.0,
        })
    }

    /// Benchmark interpolation search
    fn benchmark_interpolation_search(&self, target: &str, iterations: usize) -> Result<SearchMetrics> {
        let mut total_comparisons = 0;
        let mut found_count = 0;
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let (found, comparisons) = interpolation_search::search(&self.sorted_words, target);
            total_comparisons += comparisons;
            if found {
                found_count += 1;
            }
        }
        
        let duration = start.elapsed();
        let avg_comparisons = total_comparisons / iterations;
        
        Ok(SearchMetrics {
            algorithm_name: "Interpolation Search".to_string(),
            target_found: found_count > 0,
            comparisons: avg_comparisons,
            duration,
            theoretical_complexity: "O(log log n)".to_string(),
            actual_complexity: avg_comparisons as f64 / (self.sorted_words.len() as f64).log2().log2(),
        })
    }

    /// Benchmark jump search
    fn benchmark_jump_search(&self, target: &str, iterations: usize) -> Result<SearchMetrics> {
        let mut total_comparisons = 0;
        let mut found_count = 0;
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let (found, comparisons) = jump_search::search(&self.sorted_words, target);
            total_comparisons += comparisons;
            if found {
                found_count += 1;
            }
        }
        
        let duration = start.elapsed();
        let avg_comparisons = total_comparisons / iterations;
        
        Ok(SearchMetrics {
            algorithm_name: "Jump Search".to_string(),
            target_found: found_count > 0,
            comparisons: avg_comparisons,
            duration,
            theoretical_complexity: "O(√n)".to_string(),
            actual_complexity: avg_comparisons as f64 / (self.sorted_words.len() as f64).sqrt(),
        })
    }

    /// Benchmark exponential search
    fn benchmark_exponential_search(&self, target: &str, iterations: usize) -> Result<SearchMetrics> {
        let mut total_comparisons = 0;
        let mut found_count = 0;
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let (found, comparisons) = exponential_search::search(&self.sorted_words, target);
            total_comparisons += comparisons;
            if found {
                found_count += 1;
            }
        }
        
        let duration = start.elapsed();
        let avg_comparisons = total_comparisons / iterations;
        
        Ok(SearchMetrics {
            algorithm_name: "Exponential Search".to_string(),
            target_found: found_count > 0,
            comparisons: avg_comparisons,
            duration,
            theoretical_complexity: "O(log n)".to_string(),
            actual_complexity: avg_comparisons as f64 / (self.sorted_words.len() as f64).log2(),
        })
    }

    /// Display benchmark results in a formatted table
    fn display_results(&self, results: &[SearchMetrics]) {
        let mut table = Table::new();
        
        table.add_row(Row::new(vec![
            Cell::new("Algorithm"),
            Cell::new("Found"),
            Cell::new("Comparisons"),
            Cell::new("Duration (μs)"),
            Cell::new("Big O"),
            Cell::new("Actual/Theoretical"),
            Cell::new("Efficiency"),
        ]));

        for metric in results {
            let efficiency = if metric.actual_complexity <= 1.5 {
                "Excellent"
            } else if metric.actual_complexity <= 3.0 {
                "Good"  
            } else {
                "Needs Optimization"
            };

            table.add_row(Row::new(vec![
                Cell::new(&metric.algorithm_name),
                Cell::new(&format!("{}", metric.target_found)),
                Cell::new(&format!("{}", metric.comparisons)),
                Cell::new(&format!("{:.2}", metric.duration.as_micros())),
                Cell::new(&metric.theoretical_complexity),
                Cell::new(&format!("{:.2}", metric.actual_complexity)),
                Cell::new(efficiency),
            ]));
        }

        println!("\n{}", table);

        // Summary statistics
        if let Some(fastest) = results.iter().min_by_key(|m| m.duration) {
            println!("🏆 Fastest Algorithm: {} ({:.2}μs)", 
                fastest.algorithm_name, fastest.duration.as_micros());
        }

        if let Some(most_efficient) = results.iter().min_by_key(|m| m.comparisons) {
            println!("🎯 Most Efficient: {} ({} comparisons)", 
                most_efficient.algorithm_name, most_efficient.comparisons);
        }
    }

    /// Analyze search performance on a specific word pattern type
    pub fn analyze_array_type(&self, pattern_type: &str, size: usize) -> Result<()> {
        println!("\nAnalyzing search performance on {} pattern (size: {})", pattern_type, size);
        
        // For search algorithms, we can analyze different word patterns
        let test_words = match pattern_type.to_lowercase().as_str() {
            "short" => self.words.iter().filter(|w| w.len() <= 5).take(size).cloned().collect::<Vec<_>>(),
            "long" => self.words.iter().filter(|w| w.len() > 10).take(size).cloned().collect::<Vec<_>>(),
            "common" => self.words.iter().take(size).cloned().collect::<Vec<_>>(),
            "random" => {
                let mut rng = rng();
                let mut words = self.words.clone();
                words.shuffle(&mut rng);
                words.into_iter().take(size).collect()
            }
            _ => {
                return Err(Error::Generic(format!("Unknown pattern type: {}. Try 'short', 'long', 'common', or 'random'", pattern_type)));
            }
        };

        if test_words.is_empty() {
            return Err(Error::Generic("No words available for analysis. Load words first.".to_string()));
        }

        // Pick a target word from the test set
        let target_word = &test_words[test_words.len() / 2];
        println!("Target word: '{}'", target_word);
        
        // Create temporary coordinator with test data
        let mut temp_coord = SearchCoordinator::new();
        temp_coord.words = test_words.clone();
        temp_coord.shuffled_words = test_words.clone();
        temp_coord.shuffled_words.shuffle(&mut rng());
        temp_coord.sorted_words = test_words.clone();
        temp_coord.sorted_words.sort_unstable();
        temp_coord.word_map = test_words.iter().enumerate().map(|(i, w)| (w.clone(), i)).collect();

        // Run benchmarks
        temp_coord.run_benchmarks(target_word, 50)?;
        
        Ok(())
    }

    /// Get statistics about loaded data
    pub fn get_stats(&self) -> String {
        format!(
            "Dataset Statistics:\n  Words loaded: {}\n  Shuffled array size: {}\n  Sorted array size: {}\n  Hash map size: {}",
            self.words.len(),
            self.shuffled_words.len(), 
            self.sorted_words.len(),
            self.word_map.len()
        )
    }
}

impl Default for SearchCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
