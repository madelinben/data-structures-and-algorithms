use crate::prelude::*;
use crate::pathfinder::{Grid, Position, CellType, PerformanceCounter};
use std::collections::{VecDeque, HashSet};
use std::fs::File;
use std::io::{self, Write};

#[cfg(feature = "gui")]
use gif::{Frame, Encoder, Repeat};

#[derive(Debug, Clone, PartialEq)]
pub enum PathfinderStepType {
    Exploring,
    InFrontier,
    Path,
    Normal,
}

#[derive(Debug, Clone)]
pub struct PathfinderStep {
    pub grid: Grid,
    pub current_position: Option<Position>,
    pub frontier_positions: Vec<Position>,
    pub explored_positions: HashSet<Position>,
    pub path_positions: Vec<Position>,
    pub step_description: String,
    pub algorithm_name: String,
    pub step_type: PathfinderStepType,
}

#[derive(Debug, Clone, Default)]
pub struct GuiPerformanceCounter {
    pub nodes_explored: usize,
    pub nodes_in_frontier: usize,
    pub comparisons: usize,
    pub memory_allocations: usize,
    pub steps: VecDeque<PathfinderStep>,
    pub current_grid: Option<Grid>,
    pub current_frontier: Vec<Position>,
    pub explored_set: HashSet<Position>,
}

impl GuiPerformanceCounter {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_grid(&mut self, grid: Grid) {
        self.current_grid = Some(grid);
    }
    
    pub fn explore_node(&mut self, position: Position, description: &str, algorithm: &str) {
        self.nodes_explored += 1;
        self.explored_set.insert(position);
        
        if let Some(ref grid) = self.current_grid {
            self.steps.push_back(PathfinderStep {
                grid: grid.clone(),
                current_position: Some(position),
                frontier_positions: self.current_frontier.clone(),
                explored_positions: self.explored_set.clone(),
                path_positions: vec![],
                step_description: description.to_string(),
                algorithm_name: algorithm.to_string(),
                step_type: PathfinderStepType::Exploring,
            });
        }
    }
    
    pub fn add_to_frontier(&mut self, position: Position, description: &str, algorithm: &str) {
        self.nodes_in_frontier += 1;
        self.current_frontier.push(position);
        
        if let Some(ref grid) = self.current_grid {
            self.steps.push_back(PathfinderStep {
                grid: grid.clone(),
                current_position: None,
                frontier_positions: self.current_frontier.clone(),
                explored_positions: self.explored_set.clone(),
                path_positions: vec![],
                step_description: description.to_string(),
                algorithm_name: algorithm.to_string(),
                step_type: PathfinderStepType::InFrontier,
            });
        }
    }
    
    pub fn remove_from_frontier(&mut self, position: Position) {
        self.current_frontier.retain(|&p| p != position);
    }
    
    pub fn compare(&mut self) {
        self.comparisons += 1;
    }
    
    pub fn allocate_memory(&mut self, _size: usize) {
        self.memory_allocations += 1;
    }
    
    pub fn record_final_path(&mut self, path: Vec<Position>, algorithm: &str) {
        if let Some(ref grid) = self.current_grid {
            self.steps.push_back(PathfinderStep {
                grid: grid.clone(),
                current_position: None,
                frontier_positions: vec![],
                explored_positions: self.explored_set.clone(),
                path_positions: path,
                step_description: "Final path found".to_string(),
                algorithm_name: algorithm.to_string(),
                step_type: PathfinderStepType::Path,
            });
        }
    }
}

pub struct PathfinderVisualiser {
    steps: VecDeque<PathfinderStep>,
    current_step: usize,
    grid_size: (usize, usize),
    delay_ms: u64,
}

impl PathfinderVisualiser {
    pub fn new(grid_size: (usize, usize)) -> Self {
        Self {
            steps: VecDeque::new(),
            current_step: 0,
            grid_size,
            delay_ms: 150,
        }
    }

    pub fn set_speed(&mut self, delay_ms: u64) {
        self.delay_ms = delay_ms;
    }

    pub fn add_step(&mut self, grid: Grid, current_pos: Option<Position>, frontier: Vec<Position>, 
                   explored: HashSet<Position>, path: Vec<Position>, description: String, algorithm: String) {
        self.steps.push_back(PathfinderStep {
            grid,
            current_position: current_pos,
            frontier_positions: frontier,
            explored_positions: explored,
            path_positions: path,
            step_description: description,
            algorithm_name: algorithm,
            step_type: PathfinderStepType::Normal,
        });
    }

    pub fn clear(&mut self) {
        self.steps.clear();
        self.current_step = 0;
    }

    pub fn visualise_algorithm<F>(&mut self, algorithm_name: &str, grid: Grid, pathfind_fn: F) -> Result<()>
    where
        F: Fn(&Grid, &mut GuiPerformanceCounter) -> Result<(Vec<Position>, PerformanceCounter)>,
    {
        self.clear();
        
        println!("🎨 Starting GUI visualisation for {}", algorithm_name);
        println!("Grid size: {}x{}", grid.width, grid.height);
        
        self.add_step(
            grid.clone(),
            None,
            vec![],
            HashSet::new(),
            vec![],
            format!("Initial grid for {}", algorithm_name),
            algorithm_name.to_string(),
        );
        
        let mut gui_counter = GuiPerformanceCounter::new();
        gui_counter.set_grid(grid.clone());
        
        match pathfind_fn(&grid, &mut gui_counter) {
            Ok((path, _counter)) => {

                for step in gui_counter.steps {
                    self.steps.push_back(step);
                }
                

                if !path.is_empty() {
                    self.add_step(
                        grid.clone(),
                        None,
                        vec![],
                        gui_counter.explored_set.clone(),
                        path,
                        format!("Path found for {}", algorithm_name),
                        algorithm_name.to_string(),
                    );
                } else {
                    self.add_step(
                        grid.clone(),
                        None,
                        vec![],
                        gui_counter.explored_set.clone(),
                        vec![],
                        format!("No path found for {}", algorithm_name),
                        algorithm_name.to_string(),
                    );
                }
            }
            Err(e) => {
                println!("❌ Algorithm failed: {}", e);
                return Err(Error::Generic(format!("Pathfinding failed: {}", e)));
            }
        }
        
        println!("Choose output format:");
        println!("1. Static PNG (fast)");
        println!("2. Animated GIF (slower but shows process)");
        print!("Enter choice (1-2): ");
        
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).ok();
        
        match choice.trim() {
            "2" => self.render_animated_gif(),
            _ => self.render_static_output(),
        }
    }

    pub fn visualise_algorithm_with_choice<F>(&mut self, algorithm_name: &str, grid: Grid, pathfind_fn: F, use_gif: bool) -> Result<()>
    where
        F: Fn(&Grid, &mut GuiPerformanceCounter) -> Result<(Vec<Position>, PerformanceCounter)>,
    {
        self.clear();
        
        self.add_step(
            grid.clone(),
            None,
            vec![],
            HashSet::new(),
            vec![],
            format!("Initial grid for {}", algorithm_name),
            algorithm_name.to_string(),
        );
        
        let mut gui_counter = GuiPerformanceCounter::new();
        gui_counter.set_grid(grid.clone());
        
        match pathfind_fn(&grid, &mut gui_counter) {
            Ok((path, _counter)) => {
                for step in gui_counter.steps {
                    self.steps.push_back(step);
                }
                
                if !path.is_empty() {
                    self.add_step(
                        grid.clone(),
                        None,
                        vec![],
                        gui_counter.explored_set.clone(),
                        path,
                        format!("Path found for {}", algorithm_name),
                        algorithm_name.to_string(),
                    );
                } else {
                    self.add_step(
                        grid.clone(),
                        None,
                        vec![],
                        gui_counter.explored_set.clone(),
                        vec![],
                        format!("No path found for {}", algorithm_name),
                        algorithm_name.to_string(),
                    );
                }
            }
            Err(e) => {
                println!("❌ Algorithm failed: {}", e);
                return Err(Error::Generic(format!("Pathfinding failed: {}", e)));
            }
        }
        
        if use_gif {
            self.render_animated_gif()
        } else {
            self.render_static_output()
        }
    }

    fn render_static_output(&self) -> Result<()> {
        println!("📊 Pathfinding steps: {}", self.steps.len());
        
        if let Some(last_step) = self.steps.back() {
            println!("🎯 Final result:");
            if !last_step.path_positions.is_empty() {
                println!("✅ Path found with {} steps", last_step.path_positions.len());
                println!("📍 Explored {} nodes", last_step.explored_positions.len());
            } else {
                println!("❌ No path found");
                println!("📍 Explored {} nodes", last_step.explored_positions.len());
            }
        }

        Ok(())
    }

    #[cfg(feature = "gui")]
    fn render_animated_gif(&self) -> Result<()> {
        let algorithm_name = self.steps.front()
            .map(|s| s.algorithm_name.replace(" ", "_").replace("*", "star").to_lowercase())
            .unwrap_or_else(|| "pathfinder".to_string());
        
        let filename = format!("assets/gif/pathfinding_animation_{}.gif", algorithm_name);
        
        std::fs::create_dir_all("assets/gif").map_err(|e| Error::Generic(format!("Failed to create directory: {}", e)))?;
        if std::path::Path::new(&filename).exists() {
            std::fs::remove_file(&filename).map_err(|e| Error::Generic(format!("Failed to remove existing file: {}", e)))?;
        }
        
        println!("🎬 Creating animated GIF: {}", filename);
        println!("📊 Total frames: {}", self.steps.len());
        println!("⏱️ Estimated duration: {}s", self.steps.len() as f64 * 0.15);
        
        let file = File::create(&filename).map_err(|e| Error::Generic(format!("File creation error: {}", e)))?;
        let mut encoder = Encoder::new(file, 600, 600, &[]).map_err(|e| Error::Generic(format!("GIF encoder error: {}", e)))?;
        encoder.set_repeat(Repeat::Infinite).map_err(|e| Error::Generic(format!("GIF repeat error: {}", e)))?;

        for (i, step) in self.steps.iter().enumerate() {
            let frame_data = self.create_frame(step, 600, 600)?;
            let frame = Frame::from_rgb(600, 600, &frame_data);
            encoder.write_frame(&frame).map_err(|e| Error::Generic(format!("Frame write error: {}", e)))?;
            
            if i % 5 == 0 {
                println!("📝 Generated frame {}/{}", i + 1, self.steps.len());
            }
        }
        
        drop(encoder);
        println!("✅ GIF animation completed: {}", filename);
        println!("🎯 Open the file to see the pathfinding algorithm in action!");
        
        Ok(())
    }

    #[cfg(not(feature = "gui"))]
    fn render_animated_gif(&self) -> Result<()> {
        Err(Error::Generic("GIF rendering requires --features gui".to_string()))
    }

    fn create_frame(&self, step: &PathfinderStep, width: u16, height: u16) -> Result<Vec<u8>> {
        let mut buffer = vec![255u8; (width as usize) * (height as usize) * 3];
        
        let grid_width = step.grid.width;
        let grid_height = step.grid.height;
        

        let margin = 20;
        let available_width = width as usize - 2 * margin;
        let available_height = height as usize - 2 * margin;
        
        let cell_width = available_width / grid_width;
        let cell_height = available_height / grid_height;
        let cell_size = cell_width.min(cell_height);
        

        let start_x = margin + (available_width - grid_width * cell_size) / 2;
        let start_y = margin + (available_height - grid_height * cell_size) / 2;
        

        for row in 0..grid_height {
            for col in 0..grid_width {
                let pos = Position::new(row, col);
                let x = start_x + col * cell_size;
                let y = start_y + row * cell_size;
                

                let (r, g, b) = if step.path_positions.contains(&pos) {
                    (50, 255, 50)  // Green for path
                } else if step.current_position == Some(pos) {
                    (255, 50, 50)  // Red for current exploration
                } else if step.frontier_positions.contains(&pos) {
                    (180, 100, 255)  // Purple for frontier
                } else if step.explored_positions.contains(&pos) {
                    (200, 200, 200)  // Light gray for explored
                } else if step.grid.cells[row][col] == CellType::Blocked {
                    (0, 0, 0)      // Black for obstacles
                } else if step.grid.cells[row][col] == CellType::Start {
                    (0, 200, 0)    // Dark green for start
                } else if step.grid.cells[row][col] == CellType::End {
                    (200, 0, 0)    // Dark red for end
                } else {
                    (100, 150, 255)  // Light blue for open cells
                };
                

                for dy in 0..cell_size {
                    for dx in 0..cell_size {
                        let px = x + dx;
                        let py = y + dy;
                        
                        if px < width as usize && py < height as usize {
                            let pixel_idx = (py * width as usize + px) * 3;
                            if pixel_idx + 2 < buffer.len() {
                                buffer[pixel_idx] = r;
                                buffer[pixel_idx + 1] = g;
                                buffer[pixel_idx + 2] = b;
                            }
                        }
                    }
                }
                

                let border_colour = (0, 0, 0);
                

                for dx in 0..cell_size {
                    for border_y in [y, y + cell_size - 1] {
                        let px = x + dx;
                        let py = border_y;
                        
                        if px < width as usize && py < height as usize {
                            let pixel_idx = (py * width as usize + px) * 3;
                            if pixel_idx + 2 < buffer.len() {
                                buffer[pixel_idx] = border_colour.0;
                                buffer[pixel_idx + 1] = border_colour.1;
                                buffer[pixel_idx + 2] = border_colour.2;
                            }
                        }
                    }
                }
                

                for dy in 0..cell_size {
                    for border_x in [x, x + cell_size - 1] {
                        let px = border_x;
                        let py = y + dy;
                        
                        if px < width as usize && py < height as usize {
                            let pixel_idx = (py * width as usize + px) * 3;
                            if pixel_idx + 2 < buffer.len() {
                                buffer[pixel_idx] = border_colour.0;
                                buffer[pixel_idx + 1] = border_colour.1;
                                buffer[pixel_idx + 2] = border_colour.2;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(buffer)
    }
}
