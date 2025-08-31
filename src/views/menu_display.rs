use crate::prelude::*;
use crate::views::ConsoleView;
use crate::models::{MainMenuChoice, SearchMenuChoice, SortMenuChoice, PathfinderMenuChoice};

pub struct MenuDisplay {
    console: ConsoleView,
}

impl MenuDisplay {
    pub fn new() -> Self {
        Self {
            console: ConsoleView::new(),
        }
    }
    
    pub fn show_main_menu(&self) -> Result<MainMenuChoice> {
        self.console.print_header("Data Structures and Algorithms in Rust");
        
        let options = vec![
            ("1", "Search Algorithms (Linear, Binary, Hash, etc.)"),
            ("2", "Sorting Algorithms (13+ algorithms with benchmarking)"),
            ("3", "Pathfinding Algorithms (A*, Dijkstra, BFS, DFS, etc.)"),
            ("q", "Quit"),
        ];
        
        self.console.print_menu_options(&options);
        
        loop {
            let input = self.console.get_input("\nPlease select an option (1-3, or q to quit): ")?;
            
            match input.as_str() {
                "1" => return Ok(MainMenuChoice::Search),
                "2" => return Ok(MainMenuChoice::Sort),
                "3" => return Ok(MainMenuChoice::Pathfinder),
                "q" | "Q" | "quit" => return Ok(MainMenuChoice::Quit),
                _ => {
                    self.console.print_error("Invalid option. Please try again.");
                    self.console.pause_for_input("Press Enter to continue...")?;
                }
            }
        }
    }
    
    pub fn show_search_menu(&self) -> Result<SearchMenuChoice> {
        self.console.print_subheader("Search Algorithm Benchmarking");
        
        let options = vec![
            ("1", "Load Words File"),
            ("2", "Show Dataset Statistics"),
            ("3", "Run Search Benchmarks"),
            ("4", "Analyse Specific Array Type"),
            ("b", "Back to Main Menu"),
        ];
        
        self.console.print_menu_options(&options);
        
        loop {
            let input = self.console.get_input("\nPlease select an option (1-4, or b to go back): ")?;
            
            match input.as_str() {
                "1" => return Ok(SearchMenuChoice::LoadWords),
                "2" => return Ok(SearchMenuChoice::ShowStats),
                "3" => return Ok(SearchMenuChoice::RunBenchmarks),
                "4" => return Ok(SearchMenuChoice::AnalyseArrayType),
                "b" | "B" | "back" => return Ok(SearchMenuChoice::Back),
                _ => {
                    self.console.print_error("Invalid option. Please try again.");
                }
            }
        }
    }
    
    pub fn show_sort_menu(&self) -> Result<SortMenuChoice> {
        self.console.print_subheader("Sorting Algorithm Benchmarking");
        
        let options = vec![
            ("1", "Run Complete Benchmark Suite (13+ algorithms)"),
            ("2", "Analyse Specific Array Type"),
            ("3", "GUI Visualisation"),
            ("4", "Algorithm Information"),
            ("b", "Back to Main Menu"),
        ];
        
        self.console.print_menu_options(&options);
        
        loop {
            let input = self.console.get_input("\nPlease select an option (1-4, or b to go back): ")?;
            
            match input.as_str() {
                "1" => return Ok(SortMenuChoice::RunBenchmarks),
                "2" => return Ok(SortMenuChoice::AnalyseArrayType),
                "3" => return Ok(SortMenuChoice::GuiVisualisation),
                "4" => return Ok(SortMenuChoice::AlgorithmInfo),
                "b" | "B" | "back" => return Ok(SortMenuChoice::Back),
                _ => {
                    self.console.print_error("Invalid option. Please try again.");
                }
            }
        }
    }
    
    pub fn show_gui_algorithm_menu(&self) -> Result<String> {
        self.console.print_info("GUI Visualisation Mode Enabled!");
        println!("Select an algorithm to visualise:\n");
        
        println!("Available algorithms for visualisation:");
        println!("1. Bubble Sort          2. Insertion Sort       3. Selection Sort");
        println!("4. Merge Sort           5. Quick Sort            6. Heap Sort");
        println!("7. Shell Sort           8. Tim Sort              9. Tree Sort");
        println!("10. Bucket Sort         11. Radix Sort           12. Counting Sort");
        println!("13. Cube Sort           a. All Algorithms        q. Quit");
        
        loop {
            let input = self.console.get_input("\nSelect algorithm (1-13, 'a', or 'q'): ")?;
            
            match input.as_str() {
                "1" => return Ok("bubble".to_string()),
                "2" => return Ok("insertion".to_string()),
                "3" => return Ok("selection".to_string()),
                "4" => return Ok("merge".to_string()),
                "5" => return Ok("quick".to_string()),
                "6" => return Ok("heap".to_string()),
                "7" => return Ok("shell".to_string()),
                "8" => return Ok("tim".to_string()),
                "9" => return Ok("tree".to_string()),
                "10" => return Ok("bucket".to_string()),
                "11" => return Ok("radix".to_string()),
                "12" => return Ok("counting".to_string()),
                "13" => return Ok("cube".to_string()),
                "a" | "all" => return Ok("all".to_string()),
                "q" => return Err(Error::input("User quit GUI selection")),
                _ => {
                    self.console.print_error("Invalid choice. Please enter 1-13, 'a', or 'q'.");
                }
            }
        }
    }
    
    pub fn show_algorithm_info(&self) {
        self.console.print_header("SORTING ALGORITHMS IMPLEMENTED");
        
        let algorithms = vec![
            ("Bubble Sort", "O(n²)", "O(1)", "Yes", "Yes", "Yes"),
            ("Insertion Sort", "O(n²)", "O(1)", "Yes", "Yes", "Yes"),
            ("Selection Sort", "O(n²)", "O(1)", "No", "No", "Yes"),
            ("Merge Sort", "O(n log n)", "O(n)", "Yes", "No", "No"),
            ("Quick Sort", "O(n log n)", "O(log n)", "No", "No", "Yes"),
            ("Heap Sort", "O(n log n)", "O(1)", "No", "No", "Yes"),
            ("Shell Sort", "O(n^1.25)", "O(1)", "No", "Yes", "Yes"),
            ("Tim Sort", "O(n log n)", "O(n)", "Yes", "Yes", "No"),
            ("Tree Sort", "O(n log n)", "O(n)", "Yes", "No", "No"),
            ("Bucket Sort", "O(n + k)", "O(n + k)", "Yes", "No", "No"),
            ("Radix Sort", "O(d × n)", "O(n + k)", "Yes", "No", "No"),
            ("Counting Sort", "O(n + k)", "O(k)", "Yes", "No", "No"),
            ("Cube Sort", "O(n log n)", "O(n)", "No", "No", "No"),
        ];
        
        println!("{:<15} {:<12} {:<12} {:<8} {:<10} {:<10}", 
            "Algorithm", "Time", "Space", "Stable", "Adaptive", "In-Place");
        println!("{}", "-".repeat(80));
        
        for (name, time, space, stable, adaptive, in_place) in algorithms {
            println!("{:<15} {:<12} {:<12} {:<8} {:<10} {:<10}", 
                name, time, space, stable, adaptive, in_place);
        }
        
        println!("\n📝 Legend:");
        println!("  • Stable: Maintains relative order of equal elements");
        println!("  • Adaptive: Performs better on partially sorted data");  
        println!("  • In-Place: Uses O(1) extra memory");
        println!("  • n = array size, k = range of input, d = number of digits");
    }
    
    pub fn show_pathfinder_menu(&self) -> Result<PathfinderMenuChoice> {
        self.console.print_subheader("Pathfinding Algorithm Benchmarking");
        
        let options = vec![
            ("1", "Run Pathfinding Benchmarks (All Algorithms)"),
            ("2", "Configure Grid Settings"),
            ("3", "GUI Visualization (Generate GIFs)"),
            ("4", "Algorithm Information"),
            ("b", "Back to Main Menu"),
        ];
        
        self.console.print_menu_options(&options);
        
        loop {
            let input = self.console.get_input("\nPlease select an option (1-4, or b to go back): ")?;
            
            match input.as_str() {
                "1" => return Ok(PathfinderMenuChoice::RunBenchmarks),
                "2" => return Ok(PathfinderMenuChoice::ConfigureGrid),
                "3" => return Ok(PathfinderMenuChoice::GuiVisualization),
                "4" => return Ok(PathfinderMenuChoice::AlgorithmInfo),
                "b" | "B" | "back" => return Ok(PathfinderMenuChoice::Back),
                _ => {
                    self.console.print_error("Invalid option. Please try again.");
                    self.console.pause_for_input("Press Enter to continue...")?;
                }
            }
        }
    }
}

impl Default for MenuDisplay {
    fn default() -> Self {
        Self::new()
    }
}
