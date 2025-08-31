pub mod astar;
pub mod dijkstra;
pub mod breadth_first;
pub mod depth_first;
pub mod greedy_best_first;

use crate::prelude::*;
use std::time::{Duration, Instant};
use std::collections::{VecDeque, HashMap, HashSet};
use rand::prelude::*;
use prettytable::{Table, Row, Cell};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Open,
    Blocked,
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        let dr = (self.row as i32 - other.row as i32) as f64;
        let dc = (self.col as i32 - other.col as i32) as f64;
        (dr * dr + dc * dc).sqrt()
    }

    pub fn manhattan_distance_to(&self, other: &Position) -> usize {
        let dr = (self.row as i32 - other.row as i32).abs() as usize;
        let dc = (self.col as i32 - other.col as i32).abs() as usize;
        dr + dc
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<CellType>>,
    pub start: Position,
    pub end: Position,
}

impl Grid {
    pub fn new(width: usize, height: usize, start: Position, end: Position) -> Self {
        let mut cells = vec![vec![CellType::Open; width]; height];
        

        if start.row < height && start.col < width {
            cells[start.row][start.col] = CellType::Start;
        }
        if end.row < height && end.col < width {
            cells[end.row][end.col] = CellType::End;
        }

        Self {
            width,
            height,
            cells,
            start,
            end,
        }
    }

    pub fn add_obstacle(&mut self, pos: Position) {
        if pos.row < self.height && pos.col < self.width {

            if pos != self.start && pos != self.end {
                self.cells[pos.row][pos.col] = CellType::Blocked;
            }
        }
    }

    pub fn get_neighbors(&self, pos: &Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let row = pos.row as i32;
        let col = pos.col as i32;


        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        for (dr, dc) in directions {
            let new_row = row + dr;
            let new_col = col + dc;
            
            if new_row >= 0 && new_row < self.height as i32 &&
               new_col >= 0 && new_col < self.width as i32 {
                let new_pos = Position::new(new_row as usize, new_col as usize);
                

                if self.cells[new_pos.row][new_pos.col] != CellType::Blocked {
                    neighbors.push(new_pos);
                }
            }
        }

        neighbors
    }

    pub fn is_valid_position(&self, pos: &Position) -> bool {
        pos.row < self.height && pos.col < self.width && 
        self.cells[pos.row][pos.col] != CellType::Blocked
    }
}

#[derive(Debug, Clone)]
pub struct PathfindingMetrics {
    pub algorithm_name: String,
    pub path_found: bool,
    pub path_length: usize,
    pub nodes_explored: usize,
    pub nodes_in_frontier: usize,
    pub duration: Duration,
    pub theoretical_complexity: String,
    pub grid_size: (usize, usize),
    pub obstacle_count: usize,
    pub path: Vec<Position>,
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceCounter {
    pub nodes_explored: usize,
    pub nodes_in_frontier: usize,
    pub comparisons: usize,
    pub memory_allocations: usize,
}

impl PerformanceCounter {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn explore_node(&mut self) {
        self.nodes_explored += 1;
    }
    
    pub fn add_to_frontier(&mut self) {
        self.nodes_in_frontier += 1;
    }
    
    pub fn compare(&mut self) {
        self.comparisons += 1;
    }
    
    pub fn allocate_memory(&mut self, _size: usize) {
        self.memory_allocations += 1;
    }
}

pub struct PathfinderCoordinator {
    grids: Vec<Grid>,
}

impl PathfinderCoordinator {
    pub fn new() -> Self {
        Self {
            grids: Vec::new(),
        }
    }

    pub fn generate_test_grids(&mut self, grid_size: (usize, usize), obstacle_percentage: f64) -> Result<()> {
        let (width, height) = grid_size;
        

        self.grids.clear();
        

        self.grids.push(self.create_empty_grid(width, height)?);
        self.grids.push(self.create_random_obstacles_grid(width, height, obstacle_percentage)?);
        self.grids.push(self.create_maze_like_grid(width, height)?);
        
        Ok(())
    }

    fn create_empty_grid(&self, width: usize, height: usize) -> Result<Grid> {
        let start = Position::new(0, 0);
        let end = Position::new(height.saturating_sub(1), width.saturating_sub(1));
        Ok(Grid::new(width, height, start, end))
    }

    fn create_random_obstacles_grid(&self, width: usize, height: usize, obstacle_percentage: f64) -> Result<Grid> {
        let start = Position::new(0, 0);
        let end = Position::new(height.saturating_sub(1), width.saturating_sub(1));
        let mut grid = Grid::new(width, height, start, end);
        

        if width < 3 || height < 3 {
            return Ok(grid);
        }
        
        let mut rng = rand::rng();
        let total_cells = width * height;
        let obstacle_count = (total_cells as f64 * obstacle_percentage) as usize;
        

        let protected_positions = self.get_protected_positions(&grid);
        
        let mut obstacles_placed = 0;
        let mut attempts = 0;
        let max_attempts = total_cells * 3;
        
        while obstacles_placed < obstacle_count && attempts < max_attempts {
            attempts += 1;
            
            let row = rng.random_range(0..height);
            let col = rng.random_range(0..width);
            let pos = Position::new(row, col);
            

            if protected_positions.contains(&pos) || grid.cells[row][col] != CellType::Open {
                continue;
            }
            

            grid.add_obstacle(pos);
            

            if self.is_grid_connected(&grid) {
                obstacles_placed += 1;
            } else {
                grid.cells[row][col] = CellType::Open;
            }
        }
        

        if !self.is_grid_connected(&grid) {
            return self.create_simple_connected_grid(width, height, obstacle_percentage);
        }
        
        Ok(grid)
    }

    fn create_maze_like_grid(&self, width: usize, height: usize) -> Result<Grid> {
        let start = Position::new(0, 0);
        let end = Position::new(height.saturating_sub(1), width.saturating_sub(1));
        let mut grid = Grid::new(width, height, start, end);
        

        for row in 1..height-1 {
            for col in 1..width-1 {
                if row % 2 == 0 && col % 2 == 0 {
                    let pos = Position::new(row, col);
                    if pos != start && pos != end {
                        grid.add_obstacle(pos);
                    }
                }
            }
        }
        
        Ok(grid)
    }

    pub fn run_benchmarks(&mut self, grid_size: (usize, usize), iterations: usize) -> Result<Vec<PathfindingMetrics>> {
        let mut all_metrics = Vec::new();

        self.generate_test_grids(grid_size, 0.3)?;

        println!("Running pathfinding benchmarks...");
        println!("Grid size: {}x{}", grid_size.0, grid_size.1);
        println!("Iterations per algorithm: {}", iterations);
        println!();


        let algorithms = vec![
            "A*",
            "Dijkstra",
            "Breadth-First Search",
            "Depth-First Search", 
            "Greedy Best-First"
        ];

        for algorithm in algorithms {
            let metrics = self.benchmark_algorithm(algorithm, iterations)?;
            all_metrics.extend(metrics);
        }

        self.display_benchmark_results(&all_metrics)?;
        Ok(all_metrics)
    }

    fn get_protected_positions(&self, grid: &Grid) -> HashSet<Position> {
        let mut protected = HashSet::new();
        

        protected.insert(grid.start);
        for neighbor in self.get_eight_directional_neighbors(grid.start, grid.width, grid.height) {
            protected.insert(neighbor);
        }
        

        protected.insert(grid.end);
        for neighbor in self.get_eight_directional_neighbors(grid.end, grid.width, grid.height) {
            protected.insert(neighbor);
        }
        
        protected
    }

    fn get_eight_directional_neighbors(&self, pos: Position, width: usize, height: usize) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let row = pos.row as i32;
        let col = pos.col as i32;
        

        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1)
        ];
        
        for (dr, dc) in directions {
            let new_row = row + dr;
            let new_col = col + dc;
            
            if new_row >= 0 && new_row < height as i32 && 
               new_col >= 0 && new_col < width as i32 {
                neighbors.push(Position::new(new_row as usize, new_col as usize));
            }
        }
        
        neighbors
    }

    fn is_grid_connected(&self, grid: &Grid) -> bool {

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(grid.start);
        visited.insert(grid.start);
        
        while let Some(current) = queue.pop_front() {
            if current == grid.end {
                return true;
            }
            

            for neighbor in grid.get_neighbors(&current) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        
        false
    }

    fn create_simple_connected_grid(&self, width: usize, height: usize, obstacle_percentage: f64) -> Result<Grid> {
        let start = Position::new(0, 0);
        let end = Position::new(height.saturating_sub(1), width.saturating_sub(1));
        let mut grid = Grid::new(width, height, start, end);
        

        let mut current = start;
        let mut guaranteed_path = HashSet::new();
        

        while current != end {
            guaranteed_path.insert(current);
            
            if current.col < end.col {
                current = Position::new(current.row, current.col + 1);
            } else if current.row < end.row {
                current = Position::new(current.row + 1, current.col);
            } else {
                break;
            }
        }
        guaranteed_path.insert(end);
        

        let mut rng = rand::rng();
        let total_cells = width * height;
        let obstacle_count = ((total_cells - guaranteed_path.len()) as f64 * obstacle_percentage * 0.5) as usize;
        
        let mut obstacles_placed = 0;
        while obstacles_placed < obstacle_count {
            let row = rng.random_range(0..height);
            let col = rng.random_range(0..width);
            let pos = Position::new(row, col);
            

            if !guaranteed_path.contains(&pos) && 
               !self.get_eight_directional_neighbors(pos, width, height).iter().any(|&p| guaranteed_path.contains(&p)) &&
               grid.cells[row][col] == CellType::Open {
                grid.add_obstacle(pos);
                obstacles_placed += 1;
            }
        }
        
        Ok(grid)
    }

    fn benchmark_algorithm(&self, algorithm_name: &str, iterations: usize) -> Result<Vec<PathfindingMetrics>> {
        let mut results = Vec::new();

        for grid in &self.grids {
            let mut total_duration = Duration::default();
            let mut successful_runs = 0;
            let mut last_result = None;

            for _ in 0..iterations {
                let start_time = Instant::now();
                
                let result = match algorithm_name {
                    "A*" => astar::find_path(grid),
                    "Dijkstra" => dijkstra::find_path(grid),
                    "Breadth-First Search" => breadth_first::find_path(grid),
                    "Depth-First Search" => depth_first::find_path(grid),
                    "Greedy Best-First" => greedy_best_first::find_path(grid),
                    _ => return Err(Error::NotFound(format!("Unknown algorithm: {}", algorithm_name))),
                };
                
                let duration = start_time.elapsed();
                total_duration += duration;
                
                if let Ok(path_result) = result {
                    if !path_result.0.is_empty() {
                        successful_runs += 1;
                        last_result = Some(path_result);
                    }
                }
            }

            if successful_runs > 0 {
                let avg_duration = total_duration / successful_runs as u32;
                
                if let Some((path, counter)) = last_result {
                    let obstacle_count = grid.cells.iter()
                        .flatten()
                        .filter(|&&cell| cell == CellType::Blocked)
                        .count();

                    let metrics = PathfindingMetrics {
                        algorithm_name: algorithm_name.to_string(),
                        path_found: !path.is_empty(),
                        path_length: path.len(),
                        nodes_explored: counter.nodes_explored,
                        nodes_in_frontier: counter.nodes_in_frontier,
                        duration: avg_duration,
                        theoretical_complexity: self.get_theoretical_complexity(algorithm_name),
                        grid_size: (grid.width, grid.height),
                        obstacle_count,
                        path,
                    };
                    results.push(metrics);
                }
            }
        }

        Ok(results)
    }

    fn get_theoretical_complexity(&self, algorithm_name: &str) -> String {
        match algorithm_name {
            "A*" => "O(b^d)".to_string(),
            "Dijkstra" => "O((V + E) log V)".to_string(),
            "Breadth-First Search" => "O(V + E)".to_string(),
            "Depth-First Search" => "O(V + E)".to_string(),
            "Greedy Best-First" => "O(b^m)".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    fn display_benchmark_results(&self, metrics: &[PathfindingMetrics]) -> Result<()> {
        println!();
        println!("============================================================================");
        println!("PATHFINDING ALGORITHM PERFORMANCE ANALYSIS");
        println!("============================================================================");

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Algorithm"),
            Cell::new("Grid Size"),
            Cell::new("Path Found"),
            Cell::new("Path Length"),
            Cell::new("Nodes Explored"),
            Cell::new("Time (μs)"),
            Cell::new("Big O"),
            Cell::new("Obstacles"),
        ]));

        for metric in metrics {
            table.add_row(Row::new(vec![
                Cell::new(&metric.algorithm_name),
                Cell::new(&format!("{}x{}", metric.grid_size.0, metric.grid_size.1)),
                Cell::new(&metric.path_found.to_string()),
                Cell::new(&metric.path_length.to_string()),
                Cell::new(&metric.nodes_explored.to_string()),
                Cell::new(&format!("{:.2}", metric.duration.as_micros())),
                Cell::new(&metric.theoretical_complexity),
                Cell::new(&metric.obstacle_count.to_string()),
            ]));
        }

        table.printstd();
        println!();
        Ok(())
    }
}
