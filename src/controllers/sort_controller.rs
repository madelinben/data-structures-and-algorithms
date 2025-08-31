use crate::prelude::*;
use crate::sort::SortCoordinator;
use crate::gui::visualisation::{run_gui_visualization, run_all_gui_visualizations};
use crate::models::{SortConfig, SortMenuChoice};
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
            match self.menu_display.show_sort_menu()? {
                SortMenuChoice::RunBenchmarks => {
                    self.handle_run_benchmarks().await?;
                }
                SortMenuChoice::AnalyseArrayType => {
                    self.handle_analyse_array_type().await?;
                }
                SortMenuChoice::GuiVisualisation => {
                    self.handle_gui_visualisation().await?;
                }
                SortMenuChoice::AlgorithmInfo => {
                    self.menu_display.show_algorithm_info();
                }
                SortMenuChoice::Back => {
                    break;
                }
            }
            
            if !matches!(self.menu_display.show_sort_menu(), Ok(SortMenuChoice::AlgorithmInfo)) {
                self.console.pause_for_input("Press Enter to continue...")?;
            }
        }
        
        Ok(())
    }
    
    pub async fn run_cli(&mut self, size: usize, iterations: usize, gui_enabled: bool) -> Result<()> {
        self.console.print_header("Sorting Algorithm Benchmarking System");
        
        if gui_enabled {
            self.handle_gui_mode(size).await?;
        } else {
            self.coordinator.run_benchmarks(size, iterations)?;
        }
        
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
    
    async fn handle_analyse_array_type(&mut self) -> Result<()> {
        self.console.print_subheader("Analyse Specific Array Type");
        
        let array_type = self.input_handler.get_array_type_for_analysis()?;
        let size = self.console.get_number("Enter array size", Some(1000))?;
        
        self.console.print_info(&format!("Analysing performance on {} array (size: {})", array_type, size));
        
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
    
    async fn handle_gui_visualisation(&mut self) -> Result<()> {
        self.console.print_subheader("GUI Visualisation");
        
        let algorithm = self.input_handler.get_algorithm_name()?;
        let size = self.input_handler.get_visualisation_size()?;
        
        if let Err(e) = run_gui_visualization(&algorithm, size) {
            self.console.print_error(&format!("GUI Error: {}", e));
            return Err(e);
        }
        
        self.console.print_success("GUI visualisation completed!");
        Ok(())
    }
    
    async fn handle_gui_mode(&mut self, size: usize) -> Result<()> {
        self.console.print_info("GUI Visualisation Mode Enabled!");
        
        loop {
            match self.menu_display.show_gui_algorithm_menu() {
                Ok(choice) => {
                    if choice == "all" {
                        if let Err(e) = run_all_gui_visualizations(size) {
                            self.console.print_error(&format!("GUI Error: {}", e));
                        } else {
                            self.console.print_success("All GUI visualisations completed!");
                        }
                        break;
                    } else {
                        if let Err(e) = run_gui_visualization(&choice, size) {
                            self.console.print_error(&format!("GUI Error: {}", e));
                        } else {
                            self.console.print_success("GUI visualisation completed!");
                        }
                        break;
                    }
                }
                Err(e) => {
                    if e.to_string().contains("User quit") {
                        self.console.print_goodbye();
                        break;
                    } else {
                        return Err(e);
                    }
                }
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
