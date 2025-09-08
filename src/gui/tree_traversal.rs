use crate::tree_traversal::{TreeNode, PerformanceCounter};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct GuiPerformanceCounter {
    pub steps: VecDeque<TreeTraversalStep>,
}

#[derive(Debug, Clone)]
pub struct TreeTraversalStep {
    pub tree: TreeNode<i32>,
    pub current_nodes: Vec<i32>,
    pub context_nodes: Vec<i32>,
    pub description: String,
    pub algorithm_name: String,
}

impl GuiPerformanceCounter {
    pub fn new() -> Self {
        Self {
            steps: VecDeque::new(),
        }
    }
    
    pub fn add_step(
        &mut self, 
        tree: TreeNode<i32>, 
        current_nodes: Vec<i32>,
        context_nodes: Vec<i32>,
        description: String,
        algorithm_name: String,
    ) {
        let step = TreeTraversalStep {
            tree,
            current_nodes,
            context_nodes,
            description,
            algorithm_name,
        };
        self.steps.push_back(step);
    }
}

pub struct TreeTraversalVisualiser {
    pub steps: VecDeque<TreeTraversalStep>,
    pub current_step: usize,
}

impl TreeTraversalVisualiser {
    pub fn new(_tree_depth: usize) -> Self {
        Self {
            steps: VecDeque::new(),
            current_step: 0,
        }
    }
    
    pub fn clear(&mut self) {
        self.steps.clear();
        self.current_step = 0;
    }
    
    /// Visualise a tree traversal algorithm
    pub fn visualise_algorithm<F>(&mut self, algorithm_name: &str, tree: TreeNode<i32>, traverse_fn: F) -> crate::prelude::Result<()>
    where
        F: Fn(&TreeNode<i32>, &mut GuiPerformanceCounter) -> (Vec<i32>, PerformanceCounter),
    {
        self.clear();
        
        println!("🎨 Starting GUI visualisation for {}", algorithm_name);
        println!("Tree nodes: {}, depth: {}", tree.count_nodes(), tree.depth());
        
        // Initial step
        self.add_step(
            tree.clone(),
            vec![],
            vec![],
            format!("Initial tree for {}", algorithm_name),
            algorithm_name.to_string(),
        );
        
        let mut gui_counter = GuiPerformanceCounter::new();
        let (result, _counter) = traverse_fn(&tree, &mut gui_counter);
        
        // Add all recorded steps
        for step in gui_counter.steps {
            self.steps.push_back(step);
        }
        
        // Final step
        self.add_step(
            tree,
            result.clone(),
            vec![],
            format!("Traversal completed: {:?}", result),
            algorithm_name.to_string(),
        );
        
        self.choose_output_format()
    }
    
    /// Visualise with predetermined output choice
    pub fn visualise_algorithm_with_choice<F>(
        &mut self, 
        algorithm_name: &str, 
        tree: TreeNode<i32>, 
        traverse_fn: F,
        use_gif: bool
    ) -> crate::prelude::Result<()>
    where
        F: Fn(&TreeNode<i32>, &mut GuiPerformanceCounter) -> (Vec<i32>, PerformanceCounter),
    {
        self.clear();
        
        println!("🎨 Starting GUI visualisation for {}", algorithm_name);
        
        let mut gui_counter = GuiPerformanceCounter::new();
        let (_result, _counter) = traverse_fn(&tree, &mut gui_counter);
        
        for step in gui_counter.steps {
            self.steps.push_back(step);
        }
        
        // Always generate animated GIF
        self.render_animated_gif()
    }
    
    fn add_step(
        &mut self,
        tree: TreeNode<i32>,
        current_nodes: Vec<i32>,
        context_nodes: Vec<i32>,
        description: String,
        algorithm_name: String,
    ) {
        let step = TreeTraversalStep {
            tree,
            current_nodes,
            context_nodes,
            description,
            algorithm_name,
        };
        self.steps.push_back(step);
    }
    
    fn choose_output_format(&self) -> crate::prelude::Result<()> {
        println!("🎬 Generating animated GIF visualization...");
        self.render_animated_gif()
    }
    
    
    fn render_animated_gif(&self) -> crate::prelude::Result<()> {
        use gif::{Frame, Encoder, Repeat};
        use std::fs::File;
        use crate::prelude::*;

        let algorithm_name = self.steps.front()
            .map(|s| s.algorithm_name.replace(" ", "_").replace("-", "_").to_lowercase())
            .unwrap_or_else(|| "tree_traversal".to_string());
        
        // Determine if this is a greedy algorithm
        let is_greedy = self.steps.front()
            .map(|s| s.algorithm_name.to_lowercase().contains("greedy"))
            .unwrap_or(false);
            
        let (directory, filename) = if is_greedy {
            let clean_name = algorithm_name.replace("greedy_", "");
            ("assets/gif/tree_traversal/greedy", format!("{}.gif", clean_name))
        } else {
            ("assets/gif/tree_traversal", format!("{}.gif", algorithm_name))
        };
        
        let full_path = format!("{}/{}", directory, filename);
        
        std::fs::create_dir_all(directory).map_err(|e| Error::Generic(format!("Failed to create directory: {}", e)))?;
        if std::path::Path::new(&full_path).exists() {
            std::fs::remove_file(&full_path).map_err(|e| Error::Generic(format!("Failed to remove existing file: {}", e)))?;
        }
        
        println!("🎬 Creating animated GIF: {}", full_path);
        println!("📊 Total frames: {}", self.steps.len());
        println!("⏱️ Estimated duration: {}s", self.steps.len() as f64 * 0.2);
        
        let file = File::create(&full_path).map_err(|e| Error::Generic(format!("File creation error: {}", e)))?;
        let mut encoder = Encoder::new(file, 800, 600, &[]).map_err(|e| Error::Generic(format!("GIF encoder error: {}", e)))?;
        encoder.set_repeat(Repeat::Infinite).map_err(|e| Error::Generic(format!("GIF repeat error: {}", e)))?;

        for (i, step) in self.steps.iter().enumerate() {
            let frame_data = self.create_frame_with_index(step, 800, 600, i)?;
            let frame = Frame::from_rgb(800, 600, &frame_data);
            encoder.write_frame(&frame).map_err(|e| Error::Generic(format!("Frame write error: {}", e)))?;
            
            if i % 3 == 0 {
                println!("📝 Generated frame {}/{}", i + 1, self.steps.len());
            }
        }
        
        drop(encoder);
        println!("✅ GIF animation completed: {}", full_path);
        println!("🎯 Open the file to see the tree traversal algorithm in action!");
        
        Ok(())
    }


    // Removed create_frame - replaced by create_frame_with_index

    fn create_frame_with_index(&self, step: &TreeTraversalStep, width: u16, height: u16, current_step_index: usize) -> crate::prelude::Result<Vec<u8>> {
        let mut frame_data = vec![255u8; (width as usize) * (height as usize) * 3]; // RGB format
        
        let w = width as usize;
        let h = height as usize;
        
        // White background (clean, no blue header)
        for y in 0..h {
            for x in 0..w {
                let idx = (y * w + x) * 3;
                frame_data[idx] = 255;     // R - white background
                frame_data[idx + 1] = 255; // G
                frame_data[idx + 2] = 255; // B
            }
        }
        
        // Create a proper binary tree layout (5 layers deep)
        self.draw_binary_tree_with_index(&mut frame_data, &step.tree, &step.current_nodes, &step.description, w, h, current_step_index);
        
        Ok(frame_data)
    }

    // Removed draw_binary_tree - replaced by draw_binary_tree_with_index

    fn draw_binary_tree_with_index(&self, frame_data: &mut [u8], _tree: &crate::tree_traversal::TreeNode<i32>, 
                                  current_nodes: &[i32], description: &str, width: usize, height: usize, current_step_index: usize) {
        // Create a mapping of ALL possible nodes in a complete 5-layer binary tree
        let mut node_positions = std::collections::HashMap::new();
        
        // Calculate positions for a 5-layer binary tree
        let layers = 5;
        let node_size = 40; // Bigger square nodes
        let layer_height = (height - 100) / layers; // Leave margin at top/bottom
        let start_y = 50;
        
        // Build the complete binary tree structure (ALL nodes, always visible)
        self.build_complete_binary_tree_positions(&mut node_positions, width, start_y, layer_height, layers);
        
        // Draw all connections first (behind nodes)
        self.draw_complete_tree_connections(frame_data, &node_positions, width, height, node_size);
        
        // Draw all nodes with appropriate colors (maintaining state persistence)
        for (&node_value, &(x, y, _layer)) in &node_positions {
            let color = self.get_node_color_with_persistence(node_value, current_nodes, description, current_step_index);
            self.draw_square_node(frame_data, x, y, node_size, width, height, node_value, color);
        }
    }
    
    /// Draw connections for complete binary tree structure
    fn draw_complete_tree_connections(&self, frame_data: &mut [u8], 
                                    positions: &std::collections::HashMap<i32, (usize, usize, usize)>, 
                                    width: usize, height: usize, node_size: usize) {
        // Draw connections for complete binary tree structure
        let connections = vec![
            // Layer 0 -> Layer 1
            (1, 11), (1, 12),
            // Layer 1 -> Layer 2  
            (11, 111), (11, 112), (12, 121), (12, 122),
            // Layer 2 -> Layer 3
            (111, 1111), (111, 1112), (112, 1121), (112, 1122),
            (121, 1211), (121, 1212), (122, 1221), (122, 1222),
            // Layer 3 -> Layer 4
            (1111, 11111), (1111, 11112), (1112, 11121), (1112, 11122),
            (1121, 11211), (1121, 11212), (1122, 11221), (1122, 11222),
            (1211, 12111), (1211, 12112), (1212, 12121), (1212, 12122),
            (1221, 12211), (1221, 12212), (1222, 12221), (1222, 12222),
        ];
        
        for (parent, child) in connections {
            if let (Some(&(parent_x, parent_y, _)), Some(&(child_x, child_y, _))) = 
               (positions.get(&parent), positions.get(&child)) {
                self.draw_line_center_to_center(frame_data, 
                    parent_x, parent_y + node_size/2,    // Bottom of parent
                    child_x, child_y - node_size/2,      // Top of child
                    width, height);
            }
        }
    }

    // Removed old build_binary_positions method - replaced by build_complete_binary_tree_positions

    /// Build a complete 5-layer binary tree structure for consistent visualization
    fn build_complete_binary_tree_positions(&self, positions: &mut std::collections::HashMap<i32, (usize, usize, usize)>, 
                                          width: usize, start_y: usize, layer_height: usize, max_layers: usize) {
        // Generate all possible nodes for a complete 5-layer binary tree
        // Layer 0: 1 (root)
        // Layer 1: 11, 12  
        // Layer 2: 111, 112, 121, 122
        // Layer 3: 1111, 1112, 1121, 1122, 1211, 1212, 1221, 1222
        // Layer 4: 11111, 11112, 11121, 11122, 11211, 11212, 11221, 11222, 12111, 12112, 12121, 12122, 12211, 12212, 12221, 12222
        
        for layer in 0..max_layers {
            let nodes_in_layer = 2_usize.pow(layer as u32);
            let layer_width = width - 100; // Margins
            let x_spacing = if nodes_in_layer == 1 { 
                layer_width / 2 
            } else { 
                layer_width / (nodes_in_layer + 1) 
            };
            let y = start_y + layer * layer_height;
            
            for position_in_layer in 0..nodes_in_layer {
                // Calculate node value based on layer and position
                let node_value = self.calculate_node_value_for_position(layer, position_in_layer);
                
                let x = if nodes_in_layer == 1 {
                    50 + layer_width / 2  // Center the root node
                } else {
                    50 + (position_in_layer + 1) * x_spacing
                };
                
                positions.insert(node_value, (x, y, layer));
            }
        }
    }
    
    /// Calculate node value based on layer and position for complete binary tree
    fn calculate_node_value_for_position(&self, layer: usize, position_in_layer: usize) -> i32 {
        match layer {
            0 => 1,  // Root
            1 => if position_in_layer == 0 { 11 } else { 12 },  // Left: 11, Right: 12
            2 => match position_in_layer {
                0 => 111, 1 => 112, 2 => 121, 3 => 122,
                _ => 111  // Fallback
            },
            3 => match position_in_layer {
                0 => 1111, 1 => 1112, 2 => 1121, 3 => 1122,
                4 => 1211, 5 => 1212, 6 => 1221, 7 => 1222,
                _ => 1111  // Fallback
            },
            4 => match position_in_layer {
                0 => 11111, 1 => 11112, 2 => 11121, 3 => 11122,
                4 => 11211, 5 => 11212, 6 => 11221, 7 => 11222,
                8 => 12111, 9 => 12112, 10 => 12121, 11 => 12122,
                12 => 12211, 13 => 12212, 14 => 12221, 15 => 12222,
                _ => 11111  // Fallback
            },
            _ => 1  // Fallback for any additional layers
        }
    }

    // Removed old connection drawing methods - replaced by draw_complete_tree_connections
    
    // This method is for potential future use when we have direct tree access
    fn draw_tree_connections_recursive(&self, frame_data: &mut [u8], 
                                     tree: &crate::tree_traversal::TreeNode<i32>,
                                     positions: &std::collections::HashMap<i32, (usize, usize, usize)>, 
                                     width: usize, height: usize, node_size: usize) {
        if let Some(&(parent_x, parent_y, _)) = positions.get(&tree.value) {
            // Draw connections to all children
            for child in &tree.children {
                if let Some(&(child_x, child_y, _)) = positions.get(&child.value) {
                    // Draw line from parent to child (center to center)
                    self.draw_line_center_to_center(frame_data, 
                        parent_x, parent_y + node_size/2,  // Bottom of parent node
                        child_x, child_y - node_size/2,    // Top of child node
                        width, height);
                }
                
                // Recursively draw connections for child's subtree
                self.draw_tree_connections_recursive(frame_data, child, positions, width, height, node_size);
            }
        }
    }

    /// Get the appropriate color for a node with state persistence (visited nodes stay green)
    fn get_node_color_with_persistence(&self, node_value: i32, current_nodes: &[i32], description: &str, current_step_index: usize) -> (u8, u8, u8) {
        let desc_lower = description.to_lowercase();
        let _node_str = node_value.to_string(); // Not needed anymore since we use actual step data
        
        // 1. RED: Check if this node is currently being processed (highest priority)
        if current_nodes.contains(&node_value) {
            return (220, 50, 50);   // Red - currently selected/being processed
        }
        
        // 2. GREEN: Check if this node has been completed/visited (persistent)
        // First check if it was visited in any previous step (only up to current frame)
        if self.has_node_been_visited(node_value, current_step_index) {
            return (50, 200, 50);   // Green - completed/visited (persistent)
        }
        
        // Check if this node was RED in current step and is now completing
        // IMPORTANT: We don't immediately mark as GREEN just from description
        // A node only becomes GREEN in the step AFTER it was RED
        
        // 3. PURPLE: Check if this node is in algorithm's context (stack/queue)
        // Use the context_nodes from the step data (most reliable)
        if let Some(current_step) = self.steps.get(current_step_index) {
            if current_step.context_nodes.contains(&node_value) {
                return (150, 100, 200); // Purple - in algorithm's context
            }
        }
        
        // Fallback: Check description for context patterns
        if desc_lower.contains("stack") || desc_lower.contains("queue") || 
           desc_lower.contains("added") {
            // Use precise matching to avoid false positives
            if desc_lower.contains(&format!("[{}", node_value)) || 
               desc_lower.contains(&format!(" {}", node_value)) ||
               desc_lower.contains(&format!("{},", node_value)) ||
               desc_lower.contains(&format!("{}]", node_value)) {
                return (150, 100, 200); // Purple - in algorithm's context
            }
        }
        
        // 4. BLUE: Default unvisited state
        (100, 150, 200) // Blue - default/unvisited
    }

    /// Check if a node has been PROCESSED (was RED) in any previous step (up to current frame)
    fn has_node_been_visited(&self, node_value: i32, current_step_index: usize) -> bool {
        // Only check steps UP TO the current frame, not future steps
        // A node is "visited" ONLY if it was RED (currently processing) in a previous step
        for (index, step) in self.steps.iter().enumerate() {
            if index >= current_step_index {
                break; // Don't look at future steps
            }
            
            // Check if this node was RED (currently being processed) in this previous step
            if step.current_nodes.contains(&node_value) {
                return true; // This node was processed (RED) in a previous step
            }
        }
        false
    }

    // Removed old color method - replaced by get_node_color_with_persistence

    /// Draw a square node with the given color
    fn draw_square_node(&self, frame_data: &mut [u8], x: usize, y: usize, size: usize, 
                       width: usize, height: usize, _value: i32, color: (u8, u8, u8)) {
        let (r, g, b) = color;
        let half_size = size / 2;
        
        // Draw filled square
        for dy in 0..size {
            for dx in 0..size {
                let nx = x + dx - half_size;
                let ny = y + dy - half_size;
                if nx < width && ny < height {
                    let idx = (ny * width + nx) * 3;
                    frame_data[idx] = r;
                    frame_data[idx + 1] = g;
                    frame_data[idx + 2] = b;
                }
            }
        }
        
        // Draw black border
        self.draw_square_border(frame_data, x - half_size, y - half_size, size, width, height);
        
        // No text rendering - clean nodes without characters
    }

    /// Draw border around square node
    fn draw_square_border(&self, frame_data: &mut [u8], x: usize, y: usize, size: usize, width: usize, height: usize) {
        let border_color = (0, 0, 0); // Black border
        let (r, g, b) = border_color;
        
        // Top and bottom borders
        for dx in 0..size {
            if x + dx < width {
                // Top border
                if y < height {
                    let idx = (y * width + x + dx) * 3;
                    frame_data[idx] = r;
                    frame_data[idx + 1] = g;
                    frame_data[idx + 2] = b;
                }
                // Bottom border
                if y + size - 1 < height {
                    let idx = ((y + size - 1) * width + x + dx) * 3;
                    frame_data[idx] = r;
                    frame_data[idx + 1] = g;
                    frame_data[idx + 2] = b;
                }
            }
        }
        
        // Left and right borders
        for dy in 0..size {
            if y + dy < height {
                // Left border
                if x < width {
                    let idx = ((y + dy) * width + x) * 3;
                    frame_data[idx] = r;
                    frame_data[idx + 1] = g;
                    frame_data[idx + 2] = b;
                }
                // Right border
                if x + size - 1 < width {
                    let idx = ((y + dy) * width + x + size - 1) * 3;
                    frame_data[idx] = r;
                    frame_data[idx + 1] = g;
                    frame_data[idx + 2] = b;
                }
            }
        }
    }

    /// Draw a line from center of one node to center of another
    fn draw_line_center_to_center(&self, frame_data: &mut [u8], x1: usize, y1: usize, x2: usize, y2: usize, 
                                  width: usize, height: usize) {
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();
        let steps = dx.max(dy).max(1);

        let x_inc = (x2 as f32 - x1 as f32) / steps as f32;
        let y_inc = (y2 as f32 - y1 as f32) / steps as f32;

        for i in 0..=steps {
            let x = (x1 as f32 + i as f32 * x_inc) as usize;
            let y = (y1 as f32 + i as f32 * y_inc) as usize;
            
            if x < width && y < height {
                let idx = (y * width + x) * 3;
                frame_data[idx] = 60;      // R - dark gray line
                frame_data[idx + 1] = 60;  // G
                frame_data[idx + 2] = 60;  // B
            }
        }
    }

    // Removed draw_node_text and draw_text_overlay methods - clean visualization without text clutter
    
    fn render_tree_ascii(&self, tree: &TreeNode<i32>, highlighted: &[i32]) {
        println!("🌳 Tree Structure:");
        self.print_tree_recursive(tree, 0, highlighted, "");
        println!();
    }
    
    fn print_tree_recursive(&self, node: &TreeNode<i32>, depth: usize, highlighted: &[i32], prefix: &str) {
        let is_highlighted = highlighted.contains(&node.value);
        let node_symbol = if is_highlighted { "→" } else { " " };
        let node_color = if is_highlighted { "🔍" } else { "○" };
        
        println!("{}{} {} {}", prefix, node_symbol, node_color, node.value);
        
        for (i, child) in node.children.iter().enumerate() {
            let new_prefix = format!("{}{}",  
                prefix,
                if i == node.children.len() - 1 { "  " } else { "│ " }
            );
            
            let branch = if i == node.children.len() - 1 { "└─" } else { "├─" };
            print!("{}{}", prefix, branch);
            
            self.print_tree_recursive(child, depth + 1, highlighted, &new_prefix);
        }
    }
}

