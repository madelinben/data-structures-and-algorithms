use crate::prelude::*;
use crate::views::ConsoleView;
use crate::models::{SearchConfig, SortConfig, BenchmarkParams};

pub struct InputHandler {
    console: ConsoleView,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            console: ConsoleView::new(),
        }
    }
    
    pub fn get_search_config(&self) -> Result<SearchConfig> {
        let mut config = SearchConfig::default();
        
        config.words_file = self.console.get_string(
            "Enter path to words file", 
            Some("data/words.txt")
        )?;
        
        config.target_word = match self.console.get_string("Enter target word to search for", None) {
            Ok(word) => Some(word.to_lowercase()),
            Err(_) => None,
        };
        
        config.iterations = self.console.get_number("Enter number of iterations", Some(100))?;
        
        self.validate_search_config(&config)?;
        Ok(config)
    }
    
    pub fn get_sort_config(&self) -> Result<SortConfig> {
        let mut config = SortConfig::default();
        
        config.array_size = self.console.get_number("Enter array size", Some(1000))?;
        config.iterations = self.console.get_number("Enter iterations", Some(10))?;
        config.gui_enabled = self.console.confirm("Enable GUI visualisation?", false)?;
        
        self.validate_sort_config(&config)?;
        Ok(config)
    }
    
    pub fn get_benchmark_params(&self) -> Result<BenchmarkParams> {
        let mut params = BenchmarkParams::default();
        
        params.size = self.console.get_number("Enter array size", Some(1000))?;
        params.iterations = self.console.get_number("Enter iterations", Some(10))?;
        params.array_type = self.console.get_string(
            "Enter array type (Random/Nearly Sorted/Reverse Sorted/etc)", 
            Some("Random")
        )?;
        
        self.validate_benchmark_params(&params)?;
        Ok(params)
    }
    
    pub fn get_target_word(&self) -> Result<String> {
        let word = self.console.get_input("Enter target word to search for: ")?;
        if word.trim().is_empty() {
            return Err(Error::input("Target word cannot be empty"));
        }
        Ok(word.trim().to_lowercase())
    }
    
    pub fn get_file_path(&self, prompt: &str, default: Option<&str>) -> Result<String> {
        let path = self.console.get_string(prompt, default)?;
        
        if !std::path::Path::new(&path).exists() {
            self.console.print_warning(&format!("File '{}' does not exist", path));
            if !self.console.confirm("Continue anyway?", false)? {
                return Err(Error::not_found(format!("File not found: {}", path)));
            }
        }
        
        Ok(path)
    }
    
    pub fn get_algorithm_name(&self) -> Result<String> {
        self.console.print_info("GUI Visualisation");
        println!("Select an algorithm to visualise:");
        println!("Available: bubble, insertion, selection, merge, quick");
        
        let algorithm = self.console.get_input("Enter algorithm name: ")?;
        
        if algorithm.trim().is_empty() {
            return Err(Error::input("No algorithm specified"));
        }
        
        let valid_algorithms = vec![
            "bubble", "insertion", "selection", "merge", "quick", 
            "heap", "shell", "tim", "tree", "bucket", "radix", 
            "counting", "cube"
        ];
        
        if !valid_algorithms.contains(&algorithm.trim()) {
            return Err(Error::validation(format!("Unknown algorithm: {}", algorithm.trim())));
        }
        
        Ok(algorithm.trim().to_string())
    }
    
    pub fn get_array_type_for_analysis(&self) -> Result<String> {
        let array_type = self.console.get_string(
            "Enter array type to analyse", 
            Some("Random")
        )?;
        
        let valid_types = vec!["random", "nearly sorted", "reverse sorted", "sorted", "short", "long", "common"];
        let input_lower = array_type.to_lowercase();
        
        let is_valid = valid_types.iter().any(|&valid_type| {
            input_lower.contains(valid_type)
        });
        
        if !is_valid {
            self.console.print_warning("Supported types: Random, Nearly Sorted, Reverse Sorted, Sorted, Short, Long, Common");
        }
        
        Ok(array_type)
    }
    
    pub fn get_visualisation_size(&self) -> Result<usize> {
        self.console.get_number("Enter array size for visualisation", Some(20))
    }
    
    pub fn get_positive_number(&self, prompt: &str, min: usize, max: usize) -> Result<usize> {
        loop {
            let full_prompt = format!("{} ({}-{}): ", prompt, min, max);
            let input = self.console.get_input(&full_prompt)?;
            
            match input.parse::<usize>() {
                Ok(value) => {
                    if value < min {
                        self.console.print_error(&format!("Value must be at least {}", min));
                    } else if value > max {
                        self.console.print_error(&format!("Value must be at most {}", max));
                    } else {
                        return Ok(value);
                    }
                }
                Err(_) => {
                    self.console.print_error("Invalid number format. Please enter a valid integer.");
                }
            }
        }
    }
    
    pub fn get_string(&self, prompt: &str) -> Result<String> {
        let input = self.console.get_input(&format!("{}: ", prompt))?;
        if input.trim().is_empty() {
            Err(Error::input("Input cannot be empty"))
        } else {
            Ok(input.trim().to_string())
        }
    }
    
    fn validate_search_config(&self, config: &SearchConfig) -> Result<()> {
        if config.iterations == 0 {
            return Err(Error::validation("Iterations must be greater than 0"));
        }
        
        if config.iterations > 10000 {
            self.console.print_warning("Large number of iterations may take a long time");
        }
        
        Ok(())
    }
    
    fn validate_sort_config(&self, config: &SortConfig) -> Result<()> {
        if config.array_size == 0 {
            return Err(Error::validation("Array size must be greater than 0"));
        }
        
        if config.iterations == 0 {
            return Err(Error::validation("Iterations must be greater than 0"));
        }
        
        if config.gui_enabled && config.array_size > 100 {
            self.console.print_warning("Large array sizes may result in slow GUI rendering");
        }
        
        Ok(())
    }
    
    fn validate_benchmark_params(&self, params: &BenchmarkParams) -> Result<()> {
        if params.size == 0 {
            return Err(Error::validation("Size must be greater than 0"));
        }
        
        if params.iterations == 0 {
            return Err(Error::validation("Iterations must be greater than 0"));
        }
        
        Ok(())
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}
