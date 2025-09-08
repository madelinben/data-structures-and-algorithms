use crate::prelude::*;
use crate::views::{MenuDisplay, ConsoleView, InputHandler};
use crate::tree_traversal::TreeTraversalCoordinator;
use crate::models::{TreeTraversalMenuChoice, TreeTraversalAlgorithm};

use crate::gui::tree_traversal_visualisation;

pub struct TreeTraversalController {
    coordinator: TreeTraversalCoordinator,
    console: ConsoleView,
    menu_display: MenuDisplay,
    input_handler: InputHandler,
}

impl TreeTraversalController {
    pub fn new() -> Self {
        Self {
            coordinator: TreeTraversalCoordinator::new(),
            console: ConsoleView::new(),
            menu_display: MenuDisplay::new(),
            input_handler: InputHandler::new(),
        }
    }
    
    pub async fn run_interactive(&mut self) -> Result<()> {
        loop {
            let choice = self.menu_display.show_tree_traversal_menu()?;
            match choice {
                TreeTraversalMenuChoice::AlgorithmInfo => {
                    self.handle_algorithm_info().await?;
                }
                TreeTraversalMenuChoice::RunBenchmarks => {
                    self.handle_run_benchmarks().await?;
                    self.console.pause_for_input("Press Enter to continue...")?;
                }
                TreeTraversalMenuChoice::GuiVisualisation => {
                    self.handle_gui_visualisation().await?;
                    self.console.pause_for_input("Press Enter to continue...")?;
                }
                TreeTraversalMenuChoice::Back => {
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_run_benchmarks(&mut self) -> Result<()> {
        self.console.print_subheader("Run Complete Benchmark Suite");
        
        let iterations = self.console.get_number("Enter number of iterations", Some(1000))?;
        
        self.console.print_info(&format!("Running benchmarks with {} iterations per algorithm", iterations));
        
        match self.coordinator.run_benchmarks(iterations) {
            Ok(results) => {
                self.console.print_success(&format!("Benchmarks completed! {} results generated.", results.len()));
            }
            Err(e) => {
                self.console.print_error(&format!("Benchmark failed: {}", e));
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    
    async fn handle_tree_configuration(&mut self) -> Result<()> {
        self.console.print_header("Tree Configuration & Analysis");
        
        println!("🌳 Current test trees in coordinator:");
        let analysis_results = self.coordinator.run_benchmarks(1)?;
        
        let mut tree_stats: std::collections::HashMap<(usize, usize), usize> = std::collections::HashMap::new();
        
        for result in &analysis_results {
            {
                let key = (result.tree_nodes, result.tree_depth);
                tree_stats.insert(key, result.tree_leaves);
            }
        }
        
        for (i, ((nodes, depth), leaves)) in tree_stats.iter().enumerate() {
            println!("   Tree {}: {} nodes, {} depth, {} leaves", i + 1, nodes, depth, leaves);
        }
        
        println!("\n🔍 Tree Analysis Options:");
        println!("1. Analyze tree properties (depth, width, balance)");
        println!("2. Compare traversal performance on different tree shapes");
        
        let choice = self.console.get_input("Select analysis type (1-2): ")?;
        
        match choice.trim() {
            "1" => self.analyze_tree_properties().await?,
            "2" => self.compare_tree_shapes().await?,
            _ => {
                self.console.print_error("Invalid choice.");
            }
        }
        
        self.console.pause_for_input("Press Enter to continue...")?;
        Ok(())
    }
    
    async fn analyze_tree_properties(&mut self) -> Result<()> {
        println!("\n📈 Tree Properties Analysis");
        
        let results = self.coordinator.run_benchmarks(100)?;
        
        let mut tree_groups: std::collections::HashMap<(usize, usize), Vec<_>> = std::collections::HashMap::new();
        
        for result in &results {
            {
                let key = (result.tree_nodes, result.tree_depth);
                tree_groups.entry(key).or_insert_with(Vec::new).push(result);
            }
        }
        
        for (tree_id, ((nodes, depth), tree_results)) in tree_groups.iter().enumerate() {
            println!("\n🌳 Tree {} Analysis:", tree_id + 1);
            println!("   Nodes: {}, Depth: {}", nodes, depth);
            
            if let Some(first_result) = tree_results.first() {
                println!("   Leaves: {}", first_result.tree_leaves);
                let branching_factor = if *depth > 1 { 
                    (*nodes as f64).powf(1.0 / (*depth as f64 - 1.0)) 
                } else { 
                    0.0 
                };
                println!("   Avg branching factor: {:.2}", branching_factor);
                
                let balance_ratio = first_result.tree_leaves as f64 / *nodes as f64;
                println!("   Balance ratio (leaves/nodes): {:.2}", balance_ratio);
            }
            
            if let (Some(fastest), Some(slowest)) = (
                tree_results.iter().min_by_key(|r| r.duration),
                tree_results.iter().max_by_key(|r| r.duration)
            ) {
                println!("   Fastest: {} ({} μs)", fastest.algorithm_name, fastest.duration.as_micros());
                println!("   Slowest: {} ({} μs)", slowest.algorithm_name, slowest.duration.as_micros());
            }
        }
        
        Ok(())
    }
    
    async fn compare_tree_shapes(&mut self) -> Result<()> {
        println!("\n📊 Tree Shape Performance Comparison");
        
        let results = self.coordinator.run_benchmarks(500)?;
        
        let mut algorithm_performance: std::collections::HashMap<String, Vec<&crate::tree_traversal::TreeTraversalMetrics>> = std::collections::HashMap::new();
        
        for result in &results {
            {
                let algo_name = result.algorithm_name.split(" (Tree").next().unwrap_or(&result.algorithm_name).to_string();
                algorithm_performance.entry(algo_name).or_insert_with(Vec::new).push(result);
            }
        }
        
        for (algorithm, performances) in &algorithm_performance {
            println!("\n🔍 {} Performance Across Tree Shapes:", algorithm);
            
            let avg_ratio: f64 = performances.iter().map(|p| p.actual_nodes_ratio).sum::<f64>() / performances.len() as f64;
            let avg_duration: u128 = performances.iter().map(|p| p.duration.as_micros()).sum::<u128>() / performances.len() as u128;
            
            println!("   Average node visitation ratio: {:.3}", avg_ratio);
            println!("   Average execution time: {} μs", avg_duration);
            
            let variance: f64 = performances.iter()
                .map(|p| (p.actual_nodes_ratio - avg_ratio).powi(2))
                .sum::<f64>() / performances.len() as f64;
            let std_dev = variance.sqrt();
            
            println!("   Performance consistency (std dev): {:.3}", std_dev);
            
            if std_dev < 0.1 {
                println!("   ✅ Consistent across tree shapes");
            } else {
                println!("   ⚠️  Performance varies significantly with tree shape");
            }
        }
        
        Ok(())
    }
    
    async fn handle_gui_visualisation(&mut self) -> Result<()> {
        self.console.print_subheader("GUI Visualisation");
        
        println!("Select tree traversal algorithm:");
        println!("1. Pre-order (DFS)      2. In-order (DFS)");
        println!("3. Post-order (DFS)     4. Level-order (BFS)");
        println!("a. All Algorithms       b. Back");
        println!("\n💡 You can also type algorithm names like 'preorder', 'inorder', 'levelorder', etc.");
        
        let choice = self.console.get_input("Enter choice (number or name): ")?;
        
        if choice.to_lowercase() == "b" || choice.to_lowercase() == "back" {
            return Ok(());
        }
        
        let tree_depth = self.console.get_number("Enter tree depth", Some(4))?;
        
        match TreeTraversalAlgorithm::from_str(&choice) {
            Some(TreeTraversalAlgorithm::All) => {
                if let Err(e) = tree_traversal_visualisation::run_all_tree_visualisations(tree_depth, true) {
                    self.console.print_error(&format!("GUI Error: {}", e));
                    return Err(e);
                }
                self.console.print_success("All GUI visualisations completed!");
            }
            Some(algorithm) => {
                if let Err(e) = tree_traversal_visualisation::run_gui_visualisation(algorithm.as_str(), tree_depth) {
                    self.console.print_error(&format!("GUI Error: {}", e));
                    return Err(e);
                }
                self.console.print_success("GUI visualisation completed!");
            }
            None => {
                self.console.print_error(&format!("Unknown algorithm: '{}'. Try numbers 1-4 or names like 'preorder', 'inorder', etc.", choice));
            }
        }
        
        Ok(())
    }
    
    
    async fn handle_algorithm_info(&mut self) -> Result<()> {
        self.console.print_header("Tree Traversal Algorithm Information");
        
        println!("📚 Tree Traversal Algorithms Overview\n");
        
        println!("🌲 Depth-First Search (DFS) Algorithms:");
        println!("   📍 Pre-order (Root-Left-Right):");
        println!("      • Time: O(n), Space: O(h) where h is height");
        println!("      • Use cases: Tree copying, prefix expression evaluation, file system traversal");
        println!("      • Visits root before children - good for creating copies or processing hierarchies");
        
        println!("\n   📍 In-order (Left-Root-Right):");
        println!("      • Time: O(n), Space: O(h) where h is height");
        println!("      • Use cases: Binary Search Tree sorting, validation, finding kth element");
        println!("      • For BSTs: produces sorted sequence, essential for ordered operations");
        
        println!("\n   📍 Post-order (Left-Right-Root):");
        println!("      • Time: O(n), Space: O(h) where h is height");
        println!("      • Use cases: Tree deletion, expression evaluation, dependency resolution");
        println!("      • Visits children before root - safe for deletion and bottom-up processing");
        
        println!("\n🌊 Breadth-First Search (BFS) Algorithm:");
        println!("   📍 Level-order:");
        println!("      • Time: O(n), Space: O(w) where w is maximum width");
        println!("      • Use cases: Level-by-level processing, finding shortest path, serialization");
        println!("      • Uses queue, visits all nodes at current level before moving deeper");
        
        println!("\n🎯 Greedy Algorithms (Depth-Limited Variants):");
        println!("   📍 Greedy Pre/In/Post-order:");
        println!("      • Time: O(b^d), Space: O(d) where b=branching factor, d=depth limit");
        println!("      • Use case: Database queries with depth limits (like Prisma includes)");
        println!("      • Stops traversal at specified depth - prevents expensive deep queries");
        
        println!("\n   📍 Greedy Level-order:");
        println!("      • Time: O(b^d), Space: O(b^d) where b=branching factor, d=depth limit");
        println!("      • Use case: Limited breadth-first exploration, UI tree expansion");
        println!("      • Controls memory usage by limiting exploration depth");
        
        println!("\n💡 Performance Insights:");
        println!("   • DFS uses recursion (implicit stack) or explicit stack");
        println!("   • BFS uses queue for level-by-level processing");
        println!("   • Greedy variants optimize for scenarios with expensive deep traversal");
        println!("   • Tree shape affects performance: balanced vs skewed trees");
        println!("   • Memory usage varies: DFS O(height), BFS O(width)");
        
        println!("\n🗃️  Database Context (Prisma-style queries):");
        println!("   • Standard algorithms: fetch all related data (expensive)");
        println!("   • Greedy algorithms: limit include depth (cost optimization)");
        println!("   • Depth 1: immediate children only");
        println!("   • Depth 2: children + grandchildren");
        println!("   • Higher depths: exponential cost increase with unions/joins");
        
        self.console.pause_for_input("\nPress Enter to continue...")?;
        Ok(())
    }
}
