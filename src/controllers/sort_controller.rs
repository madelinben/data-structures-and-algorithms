use crate::prelude::*;
use crate::sort::SortCoordinator;
use crate::gui::visualisation::{run_gui_visualisation, run_all_gui_visualisations};
use crate::models::{SortConfig, SortMenuChoice, SortAlgorithm};
use crate::views::{MenuDisplay, InputHandler, ConsoleView};

pub struct SortController {
    coordinator: SortCoordinator,
    console: ConsoleView,
    menu_display: MenuDisplay,
    input_handler: InputHandler,
}

impl SortController {
    pub fn new() -> Self {
        Self {
            coordinator: SortCoordinator::new(),
            console: ConsoleView::new(),
            menu_display: MenuDisplay::new(),
            input_handler: InputHandler::new(),
        }
    }
    
    pub async fn run_interactive(&mut self) -> Result<()> {
        loop {
            let choice = self.menu_display.show_sort_menu()?;
            match choice {
                SortMenuChoice::AlgorithmInfo => {
                    self.menu_display.show_algorithm_info();
                }
                SortMenuChoice::RunBenchmarks => {
                    self.handle_run_benchmarks().await?;
                    self.console.pause_for_input("Press Enter to continue...")?;
                }
                SortMenuChoice::GuiVisualisation => {
                    self.handle_gui_visualisation().await?;
                    self.console.pause_for_input("Press Enter to continue...")?;
                }
                SortMenuChoice::Back => {
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    pub async fn run_cli(&mut self, size: usize, iterations: usize) -> Result<()> {
        self.console.print_header("Sorting Algorithm Benchmarking System");
        
        self.coordinator.run_benchmarks(size, iterations)?;
        
        Ok(())
    }
    
    async fn handle_run_benchmarks(&mut self) -> Result<()> {
        self.console.print_subheader("Run Complete Benchmark Suite");
        
        let config = self.input_handler.get_sort_config()?;
        
        self.console.print_info(&format!("Running benchmarks with array size: {}, iterations: {}", 
            config.array_size, config.iterations));
        
        match self.coordinator.run_benchmarks(config.array_size, config.iterations) {
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
        
        let algorithm = match self.input_handler.get_sort_algorithm() {
            Ok(algo) => algo,
            Err(e) => {
                if e.to_string().contains("cancelled") {
                    return Ok(());
                }
                return Err(e);
            }
        };
        
        let size = self.input_handler.get_visualisation_size()?;
        
        match algorithm {
            SortAlgorithm::All => {
                if let Err(e) = run_all_gui_visualisations(size) {
                    self.console.print_error(&format!("GUI Error: {}", e));
                    return Err(e);
                }
                self.console.print_success("All GUI visualisations completed!");
            }
            _ => {
                if let Err(e) = run_gui_visualisation(algorithm.as_str(), size) {
                    self.console.print_error(&format!("GUI Error: {}", e));
                    return Err(e);
                }
                self.console.print_success("GUI visualisation completed!");
            }
        }
        
        Ok(())
    }
    
    async fn handle_gui_mode(&mut self, size: usize) -> Result<()> {
        match self.menu_display.show_gui_algorithm_menu() {
            Ok(choice) => {
                if choice == "back" {
                    return Ok(());
                }
                
                if choice == "all" {
                    if let Err(e) = run_all_gui_visualisations(size) {
                        self.console.print_error(&format!("GUI Error: {}", e));
                    } else {
                        self.console.print_success("All GUI visualisations completed!");
                    }
                } else {
                    if let Err(e) = run_gui_visualisation(&choice, size) {
                        self.console.print_error(&format!("GUI Error: {}", e));
                    } else {
                        self.console.print_success("GUI visualisation completed!");
                    }
                }
            }
            Err(e) => {
                self.console.print_error(&format!("Menu Error: {}", e));
            }
        }
        
        Ok(())
    }
}

impl Default for SortController {
    fn default() -> Self {
        Self::new()
    }
}
