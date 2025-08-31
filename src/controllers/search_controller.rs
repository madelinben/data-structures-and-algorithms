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
            match self.menu_display.show_search_menu()? {
                SearchMenuChoice::LoadWords => {
                    self.handle_load_words().await?;
                }
                SearchMenuChoice::ShowStats => {
                    self.handle_show_stats();
                }
                SearchMenuChoice::RunBenchmarks => {
                    self.handle_run_benchmarks().await?;
                }
                SearchMenuChoice::AnalyseArrayType => {
                    self.handle_analyse_array_type().await?;
                }
                SearchMenuChoice::Back => {
                    break;
                }
            }
            
            self.console.pause_for_input("Press Enter to continue...")?;
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
        self.console.print_subheader("Run Search Benchmarks");
        
        let target = self.input_handler.get_target_word()?;
        let iterations = self.console.get_number("Enter number of iterations", Some(100))?;
        
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
    
    async fn handle_analyse_array_type(&mut self) -> Result<()> {
        self.console.print_subheader("Analyse Specific Array Type");
        
        let array_type = self.input_handler.get_array_type_for_analysis()?;
        let size = self.console.get_number("Enter array size", Some(10000))?;
        
        match self.coordinator.analyse_array_type(&array_type, size) {
            Ok(_) => {
                self.console.print_success("Analysis completed!");
            }
            Err(e) => {
                self.console.print_error(&format!("Analysis failed: {}", e));
                return Err(e);
            }
        }
        
        Ok(())
    }
}

impl Default for SearchController {
    fn default() -> Self {
        Self::new()
    }
}
