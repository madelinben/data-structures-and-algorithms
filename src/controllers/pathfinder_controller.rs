use crate::prelude::*;
use crate::pathfinder::PathfinderCoordinator;
use crate::models::{PathfinderConfig, PathfinderMenuChoice, PathfinderAlgorithm};
use crate::views::{MenuDisplay, InputHandler, ConsoleView};

pub struct PathfinderController {
    coordinator: PathfinderCoordinator,
    console: ConsoleView,
    menu_display: MenuDisplay,
    input_handler: InputHandler,
}

impl PathfinderController {
    pub fn new() -> Self {
        Self {
            coordinator: PathfinderCoordinator::new(),
            console: ConsoleView::new(),
            menu_display: MenuDisplay::new(),
            input_handler: InputHandler::new(),
        }
    }
    
    pub async fn run_interactive(&mut self) -> Result<()> {
        loop {
            match self.menu_display.show_pathfinder_menu()? {
                PathfinderMenuChoice::RunBenchmarks => {
                    self.handle_run_benchmarks().await?;
                }
                PathfinderMenuChoice::ConfigureGrid => {
                    self.handle_configure_grid().await?;
                }
                PathfinderMenuChoice::GuiVisualization => {
                    self.handle_gui_visualization().await?;
                }
                PathfinderMenuChoice::AlgorithmInfo => {
                    self.handle_algorithm_info().await?;
                }
                PathfinderMenuChoice::Back => {
                    break;
                }
            }
        }
        Ok(())
    }

    async fn handle_run_benchmarks(&mut self) -> Result<()> {
        println!("🔍 Pathfinding Algorithm Benchmarks");
        println!("====================================");


        let grid_size = self.get_grid_size_from_user()?;
        let iterations = self.get_iterations_from_user()?;


        let _metrics = self.coordinator.run_benchmarks(grid_size, iterations)?;
        
        println!("✅ Benchmarks completed!");
        self.console.wait_for_enter("Press Enter to continue...");
        Ok(())
    }

    async fn handle_configure_grid(&mut self) -> Result<()> {
        println!("🎛️  Grid Configuration");
        println!("===================");
        println!("Current configuration:");
        println!("- Grid size: 20x20");
        println!("- Obstacle percentage: 30%");
        println!();
        
        let width = self.input_handler.get_positive_number("Grid width", 5, 50)?;
        let height = self.input_handler.get_positive_number("Grid height", 5, 50)?;
        let obstacle_percentage = self.get_obstacle_percentage_from_user()?;
        

        self.coordinator.generate_test_grids((width, height), obstacle_percentage)?;
        
        println!("✅ Grid configuration updated!");
        println!("New settings:");
        println!("- Grid size: {}x{}", width, height);
        println!("- Obstacle percentage: {:.0}%", obstacle_percentage * 100.0);
        
        self.console.wait_for_enter("Press Enter to continue...");
        Ok(())
    }

    async fn handle_gui_visualization(&mut self) -> Result<()> {
        println!("🎨 Pathfinding Visualization");
        println!("===========================");
        
        #[cfg(feature = "gui")]
        {
            println!("Choose algorithm to visualize:");
            println!("1. A*");
            println!("2. Dijkstra");
            println!("3. Breadth-First Search");
            println!("4. Depth-First Search");
            println!("5. Greedy Best-First");
            println!("a. All algorithms");
            
            let choice = self.input_handler.get_string("Enter choice")?;
            let grid_size = self.get_grid_size_from_user()?;
            
            match PathfinderAlgorithm::from_str(&choice) {
                Some(PathfinderAlgorithm::All) => {
                    use crate::gui::pathfinder_visualisation::run_all_pathfinder_visualizations;
                    println!("🎬 Generating visualizations for all pathfinding algorithms...");
                    run_all_pathfinder_visualizations(grid_size)?;
                }
                Some(algorithm) => {
                    use crate::gui::pathfinder_visualisation::run_pathfinder_visualization;
                    println!("🎬 Generating visualization for {}...", algorithm.display_name());
                    run_pathfinder_visualization(algorithm.as_str(), grid_size)?;
                }
                None => {
                    println!("❌ Invalid algorithm choice");
                }
            }
        }
        
        #[cfg(not(feature = "gui"))]
        {
            println!("❌ GUI features not enabled");
            println!("Build with --features gui to enable visualization");
        }
        
        self.console.wait_for_enter("Press Enter to continue...");
        Ok(())
    }

    async fn handle_algorithm_info(&mut self) -> Result<()> {
        println!("📚 Pathfinding Algorithm Information");
        println!("===================================");
        println!();
        
        println!("🌟 A* (A-Star)");
        println!("   - Uses heuristic + actual cost");
        println!("   - Guarantees shortest path (optimal)");
        println!("   - Time complexity: O(b^d)");
        println!("   - Space complexity: O(b^d)");
        println!();
        
        println!("🚀 Dijkstra's Algorithm");
        println!("   - Explores uniformly in all directions");
        println!("   - Guarantees shortest path (optimal)");
        println!("   - Time complexity: O((V + E) log V)");
        println!("   - Space complexity: O(V)");
        println!();
        
        println!("📊 Breadth-First Search (BFS)");
        println!("   - Explores level by level");
        println!("   - Guarantees shortest path (unweighted)");
        println!("   - Time complexity: O(V + E)");
        println!("   - Space complexity: O(V)");
        println!();
        
        println!("🌲 Depth-First Search (DFS)");
        println!("   - Goes deep before exploring alternatives");
        println!("   - Does NOT guarantee shortest path");
        println!("   - Time complexity: O(V + E)");
        println!("   - Space complexity: O(V)");
        println!();
        
        println!("🎯 Greedy Best-First");
        println!("   - Uses only heuristic (no actual cost)");
        println!("   - Fast but NOT optimal");
        println!("   - Time complexity: O(b^m)");
        println!("   - Space complexity: O(b^m)");
        println!();
        
        println!("Legend:");
        println!("  V = number of vertices (grid cells)");
        println!("  E = number of edges (connections)");
        println!("  b = branching factor");
        println!("  d = depth of solution");
        println!("  m = maximum depth");
        
        self.console.wait_for_enter("Press Enter to continue...");
        Ok(())
    }

    fn get_grid_size_from_user(&mut self) -> Result<(usize, usize)> {
        println!("Grid size configuration:");
        let width = self.input_handler.get_positive_number("Grid width", 5, 50)?;
        let height = self.input_handler.get_positive_number("Grid height", 5, 50)?;
        Ok((width, height))
    }

    fn get_iterations_from_user(&mut self) -> Result<usize> {
        self.input_handler.get_positive_number("Number of iterations per algorithm", 1, 100)
    }

    fn get_obstacle_percentage_from_user(&mut self) -> Result<f64> {
        let percentage = self.input_handler.get_positive_number("Obstacle percentage", 0, 80)?;
        Ok(percentage as f64 / 100.0)
    }

    pub async fn run_single_algorithm(&mut self, algorithm: PathfinderAlgorithm, config: PathfinderConfig) -> Result<()> {
        println!("Running {} pathfinding algorithm...", algorithm.display_name());
        
        self.coordinator.generate_test_grids((config.grid_width, config.grid_height), config.obstacle_percentage)?;
        
        let _metrics = match algorithm {
            PathfinderAlgorithm::All => {
                self.coordinator.run_benchmarks((config.grid_width, config.grid_height), config.iterations)?
            }
            _ => {

                println!("🚧 Single algorithm benchmarking not yet fully implemented");
                Vec::new()
            }
        };
        
        Ok(())
    }
}
