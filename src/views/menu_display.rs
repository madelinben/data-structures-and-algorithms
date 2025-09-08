use crate::prelude::*;
use crate::views::ConsoleView;
use crate::models::{MainMenuChoice, SearchMenuChoice, SortMenuChoice, PathfinderMenuChoice, TreeTraversalMenuChoice, SortAlgorithm};

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
            ("4", "Tree Traversal Algorithms (Pre-order, In-order, Post-order, Level-order)"),
            ("q", "Quit"),
        ];
        
        self.console.print_menu_options(&options);
        
        loop {
            let input = self.console.get_input("\nPlease select an option (1-4, or q to quit): ")?;
            
            match input.as_str() {
                "1" => return Ok(MainMenuChoice::Search),
                "2" => return Ok(MainMenuChoice::Sort),
                "3" => return Ok(MainMenuChoice::Pathfinder),
                "4" => return Ok(MainMenuChoice::TreeTraversal),
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
            ("1", "Algorithm Information"),
            ("2", "Run Complete Benchmark Suite (All Algorithms)"),
            ("3", "GUI Visualisation (Generate GIFs)"),
            ("b", "Back to Main Menu"),
        ];
        
        self.console.print_menu_options(&options);
        
        loop {
            let input = self.console.get_input("\nPlease select an option (1-3, or b to go back): ")?;
            
            match input.as_str() {
                "1" => return Ok(SearchMenuChoice::AlgorithmInfo),
                "2" => return Ok(SearchMenuChoice::RunBenchmarks),
                "3" => return Ok(SearchMenuChoice::GuiVisualisation),
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
            ("1", "Algorithm Information"),
            ("2", "Run Complete Benchmark Suite (All Algorithms)"),
            ("3", "GUI Visualisation (Generate GIFs)"),
            ("b", "Back to Main Menu"),
        ];
        
        self.console.print_menu_options(&options);
        
        loop {
            let input = self.console.get_input("\nPlease select an option (1-3, or b to go back): ")?;
            
            match input.as_str() {
                "1" => return Ok(SortMenuChoice::AlgorithmInfo),
                "2" => return Ok(SortMenuChoice::RunBenchmarks),
                "3" => return Ok(SortMenuChoice::GuiVisualisation),
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
        println!("13. Cube Sort           a. All Algorithms        b. Back");
        println!("\n💡 You can also type algorithm names like 'bubble', 'merge', 'quick', etc.");
        
        loop {
            let input = self.console.get_input("\nSelect algorithm (number or name): ")?;
            
            if input.to_lowercase() == "b" || input.to_lowercase() == "back" {
                return Ok("back".to_string());
            }
            
            match SortAlgorithm::from_str(&input) {
                Some(algorithm) => return Ok(algorithm.as_str().to_string()),
                None => {
                    self.console.print_error(&format!("Unknown algorithm: '{}'. Try numbers 1-13 or names like 'bubble', 'merge', etc.", input));
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
            ("1", "Algorithm Information"),
            ("2", "Run Complete Benchmark Suite (All Algorithms)"),
            ("3", "GUI Visualisation (Generate GIFs)"),
            ("b", "Back to Main Menu"),
        ];
        
        self.console.print_menu_options(&options);
        
        loop {
            let input = self.console.get_input("\nPlease select an option (1-3, or b to go back): ")?;
            
            match input.as_str() {
                "1" => return Ok(PathfinderMenuChoice::AlgorithmInfo),
                "2" => return Ok(PathfinderMenuChoice::RunBenchmarks),
                "3" => return Ok(PathfinderMenuChoice::GuiVisualisation),
                "b" | "B" | "back" => return Ok(PathfinderMenuChoice::Back),
                _ => {
                    self.console.print_error("Invalid option. Please try again.");
                }
            }
        }
    }
    
    pub fn show_tree_traversal_menu(&self) -> Result<TreeTraversalMenuChoice> {
        self.console.print_subheader("Tree Traversal Algorithm Benchmarking");
        
        let options = vec![
            ("1", "Algorithm Information"),
            ("2", "Run Complete Benchmark Suite (All Algorithms)"),
            ("3", "GUI Visualisation (Generate GIFs)"),
            ("b", "Back to Main Menu"),
        ];
        
        self.console.print_menu_options(&options);
        
        loop {
            let input = self.console.get_input("\nPlease select an option (1-3, or b to go back): ")?;
            
            match input.as_str() {
                "1" => return Ok(TreeTraversalMenuChoice::AlgorithmInfo),
                "2" => return Ok(TreeTraversalMenuChoice::RunBenchmarks),
                "3" => return Ok(TreeTraversalMenuChoice::GuiVisualisation),
                "b" | "B" | "back" => return Ok(TreeTraversalMenuChoice::Back),
                _ => {
                    self.console.print_error("Invalid option. Please try again.");
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
