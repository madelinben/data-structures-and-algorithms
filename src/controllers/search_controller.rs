use crate::prelude::*;
use crate::search::SearchCoordinator;
use crate::models::{SearchConfig, SearchMenuChoice};
use crate::views::{MenuDisplay, InputHandler, ConsoleView};

pub struct SearchController {
    coordinator: SearchCoordinator,
    console: ConsoleView,
    menu_display: MenuDisplay,
    input_handler: InputHandler,
}

impl SearchController {
    pub fn new() -> Self {
        Self {
            coordinator: SearchCoordinator::new(),
            console: ConsoleView::new(),
            menu_display: MenuDisplay::new(),
            input_handler: InputHandler::new(),
        }
    }
    
    pub async fn run_interactive(&mut self) -> Result<()> {
        loop {
            let choice = self.menu_display.show_search_menu()?;
            match choice {
                SearchMenuChoice::AlgorithmInfo => {
                    self.handle_algorithm_info();
                }
                SearchMenuChoice::RunBenchmarks => {
                    self.handle_run_benchmarks().await?;
                    self.console.pause_for_input("Press Enter to continue...")?;
                }
                SearchMenuChoice::GuiVisualisation => {
                    self.handle_gui_visualisation().await?;
                    self.console.pause_for_input("Press Enter to continue...")?;
                }
                SearchMenuChoice::Back => {
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    pub async fn run_cli(&mut self, words_file: &str, target_word: Option<String>, iterations: usize) -> Result<()> {
        self.console.print_header("Search Algorithm Benchmarking System");
        
        self.coordinator.load_words(words_file).await?;
        self.console.print_success(&format!("Loaded words from: {}", words_file));
        
        let target = match target_word {
            Some(word) => word,
            None => {
                let stats = self.coordinator.get_stats();
                self.console.print_info(&stats);
                self.input_handler.get_target_word()?
            }
        };
        
        self.coordinator.run_benchmarks(&target, iterations)?;
        Ok(())
    }
    
    async fn handle_load_words(&mut self) -> Result<()> {
        self.console.print_subheader("Load Words File");
        
        let words_file = self.input_handler.get_file_path(
            "Enter path to words file",
            Some("data/words.txt")
        )?;
        
        match self.coordinator.load_words(&words_file).await {
            Ok(_) => {
                self.console.print_success("Words loaded successfully!");
                self.console.print_success("Created shuffled array");
                self.console.print_success("Created sorted array");
                self.console.print_success("Created hash map");
            }
            Err(e) => {
                self.console.print_error(&format!("Failed to load words: {}", e));
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    fn handle_show_stats(&self) {
        self.console.print_subheader("Dataset Statistics");
        
        let stats = self.coordinator.get_stats();
        if stats.contains("0") {
            self.console.print_warning("No words loaded. Please load words first.");
        } else {
            println!("\n{}", stats);
        }
    }
    
    async fn handle_run_benchmarks(&mut self) -> Result<()> {
        self.console.print_subheader("Run Complete Benchmark Suite");
        
        match self.coordinator.load_words("data/words.txt").await {
            Ok(_) => {
                self.console.print_success("Words loaded successfully!");
                let stats = self.coordinator.get_stats();
                if !stats.contains("0") {
                    println!("\n{}", stats);
                }
            }
            Err(e) => {
                self.console.print_warning(&format!("Failed to load default words file: {}", e));
                self.console.print_info("Please ensure data/words.txt exists or provide a custom file path");
                
                let words_file = self.input_handler.get_file_path(
                    "Enter path to words file",
                    Some("data/words.txt")
                )?;
                
                self.coordinator.load_words(&words_file).await?;
                self.console.print_success("Words loaded successfully!");
            }
        }
        
        let target = self.input_handler.get_target_word()?;
        let iterations = self.console.get_number("Enter number of iterations", Some(100))?;
        
        self.console.print_info(&format!("Running benchmarks for '{}' with {} iterations", target, iterations));
        
        match self.coordinator.run_benchmarks(&target, iterations) {
            Ok(_) => {
                self.console.print_success("Benchmarks completed!");
            }
            Err(e) => {
                self.console.print_error(&format!("Benchmark failed: {}", e));
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    async fn handle_gui_visualisation(&mut self) -> Result<()> {
        self.console.print_subheader("GUI Visualisation");
        
        self.console.print_info("Search algorithms don't have visual representations like sorting or pathfinding.");
        self.console.print_info("Instead, search performance can be observed through benchmark results.");
        self.console.print_info("Consider running the benchmark suite to see detailed performance metrics.");
        
        Ok(())
    }
    
    fn handle_algorithm_info(&self) {
        self.console.print_header("SEARCH ALGORITHMS IMPLEMENTED");
        
        let algorithms = vec![
            ("Linear Search", "O(n)", "O(1)", "Simple sequential scan", "Unsorted arrays"),
            ("Binary Search", "O(log n)", "O(1)", "Divide and conquer", "Sorted arrays only"),
            ("Hash Search", "O(1) avg", "O(n)", "Hash table lookup", "Hash-compatible data"),
            ("Interpolation Search", "O(log log n)", "O(1)", "Estimate position", "Uniformly distributed data"),
            ("Exponential Search", "O(log n)", "O(1)", "Find range then binary", "Sorted infinite arrays"),
            ("Jump Search", "O(√n)", "O(1)", "Block-wise jumping", "Sorted arrays"),
        ];
        
        println!("{:<20} {:<12} {:<12} {:<25} {:<25}", 
            "Algorithm", "Time", "Space", "Strategy", "Best Used For");
        println!("{}", "-".repeat(100));
        
        for (name, time, space, strategy, use_case) in algorithms {
            println!("{:<20} {:<12} {:<12} {:<25} {:<25}", 
                name, time, space, strategy, use_case);
        }
        
        println!("\n📝 Key Points:");
        println!("  • Time complexity assumes worst-case unless noted");
        println!("  • Hash search requires good hash function and collision handling");
        println!("  • Binary and interpolation require pre-sorted data");
        println!("  • Jump search optimal block size is √n");
        println!("  • n = array size");
    }
}

impl Default for SearchController {
    fn default() -> Self {
        Self::new()
    }
}
