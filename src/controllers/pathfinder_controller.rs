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
            let choice = self.menu_display.show_pathfinder_menu()?;
            match choice {
                PathfinderMenuChoice::AlgorithmInfo => {
                    self.handle_algorithm_info().await?;
                }
                PathfinderMenuChoice::RunBenchmarks => {
                    self.handle_run_benchmarks().await?;
                    self.console.pause_for_input("Press Enter to continue...")?;
                }
                PathfinderMenuChoice::GuiVisualisation => {
                    self.handle_gui_visualisation().await?;
                    self.console.pause_for_input("Press Enter to continue...")?;
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
        Ok(())
    }


    async fn handle_gui_visualisation(&mut self) -> Result<()> {
        self.console.print_subheader("GUI Visualisation");
        
        println!("Choose algorithm to visualise:");
        println!("1. A*");
        println!("2. Dijkstra");
        println!("3. Breadth-First Search");
        println!("4. Depth-First Search");
        println!("5. Greedy Best-First");
        println!("a. All Algorithms");
        println!("b. Back");
        println!("\n💡 You can also type algorithm names like 'astar', 'dijkstra', 'bfs', 'dfs', etc.");
        
        let choice = self.input_handler.get_string("Enter choice (number or name)")?;
            
        if choice.to_lowercase() == "b" || choice.to_lowercase() == "back" {
            return Ok(());
        }
        
        let grid_size = self.get_grid_size_from_user()?;
        
        match PathfinderAlgorithm::from_str(&choice) {
            Some(PathfinderAlgorithm::All) => {
                use crate::gui::pathfinder_visualisation::run_all_pathfinder_visualisations;
                println!("🎬 Generating visualisations for all pathfinding algorithms...");
                run_all_pathfinder_visualisations(grid_size)?;
                self.console.print_success("All GUI visualisations completed!");
            }
            Some(algorithm) => {
                use crate::gui::pathfinder_visualisation::run_pathfinder_visualisation;
                println!("🎬 Generating visualisation for {}...", algorithm.display_name());
                run_pathfinder_visualisation(algorithm.as_str(), grid_size)?;
                self.console.print_success("GUI visualisation completed!");
            }
            None => {
                self.console.print_error("❌ Invalid choice. Please enter 1-5, 'a', or algorithm names like 'astar', 'dijkstra', etc.");
            }
        }
        
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
        
        let _metrics: Vec<crate::pathfinder::PathfindingMetrics> = match algorithm {
            PathfinderAlgorithm::All => {
                self.coordinator.run_benchmarks((config.grid_width, config.grid_height), config.iterations)?
            }
            PathfinderAlgorithm::AStar |
            PathfinderAlgorithm::Dijkstra |
            PathfinderAlgorithm::BreadthFirst |
            PathfinderAlgorithm::DepthFirst |
            PathfinderAlgorithm::GreedyBestFirst => {
                println!("🚧 Single algorithm benchmarking not yet fully implemented");
                Vec::new()
            }
        };
        
        Ok(())
    }
}
