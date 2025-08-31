use crate::prelude::*;
use crate::models::PathfinderAlgorithm;
use crate::pathfinder::{Grid, Position, CellType, PerformanceCounter};
use crate::gui::pathfinder::{PathfinderVisualiser, GuiPerformanceCounter};
use rand::{rng, Rng};
use std::io::{self, Write};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run_pathfinder_visualisation(algorithm: &str, grid_size: (usize, usize)) -> Result<()> {
    let mut visualiser = PathfinderVisualiser::new(grid_size);
    

    let grid = create_test_grid(grid_size.0, grid_size.1, 0.25)?;
    
    match algorithm {
        "astar" | "a*" => {
            visualiser.visualise_algorithm("A*", grid, |grid, counter| {
                astar_with_gui(grid, counter)
            })?;
        },
        "dijkstra" => {
            visualiser.visualise_algorithm("Dijkstra", grid, |grid, counter| {
                dijkstra_with_gui(grid, counter)
            })?;
        },
        "breadth-first" | "bfs" => {
            visualiser.visualise_algorithm("Breadth-First Search", grid, |grid, counter| {
                breadth_first_with_gui(grid, counter)
            })?;
        },
        "depth-first" | "dfs" => {
            visualiser.visualise_algorithm("Depth-First Search", grid, |grid, counter| {
                depth_first_with_gui(grid, counter)
            })?;
        },
        "greedy-best-first" | "greedy" => {
            visualiser.visualise_algorithm("Greedy Best-First", grid, |grid, counter| {
                greedy_best_first_with_gui(grid, counter)
            })?;
        },
        _ => {
            return Err(Error::validation(format!("Unknown pathfinding algorithm: {}", algorithm)));
        }
    }
    
    Ok(())
}

pub fn run_all_pathfinder_visualisations(grid_size: (usize, usize)) -> Result<()> {
    println!("🎨 Running GUI visualisations for all 5 pathfinding algorithms!");
    
    println!("Choose output format:");
    println!("1. Static PNG (fast)");  
    println!("2. Animated GIF (slower but shows process)");
    print!("Enter choice (1-2): ");
    
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).ok();
    let use_gif = choice.trim() == "2";
    
    if use_gif {
        println!("📺 Will generate animated GIFs for all algorithms...");
    } else {
        println!("📷 Will generate static visualisations for all algorithms...");
    }
    
    let algorithms = vec![
        "A*", "Dijkstra", "Breadth-First Search", 
        "Depth-First Search", "Greedy Best-First"
    ];
    
    for (i, algorithm) in algorithms.iter().enumerate() {
        println!("🔄 Processing {}/{}: {}", i + 1, algorithms.len(), algorithm);
        

        let grid = create_test_grid(grid_size.0, grid_size.1, 0.25)?;
        
        let mut visualiser = PathfinderVisualiser::new(grid_size);
        
        match algorithm.as_ref() {
            "A*" => {
                visualiser.visualise_algorithm_with_choice("A*", grid, |grid, counter| {
                    astar_with_gui(grid, counter)
                }, use_gif)?;
            },
            "Dijkstra" => {
                visualiser.visualise_algorithm_with_choice("Dijkstra", grid, |grid, counter| {
                    dijkstra_with_gui(grid, counter)
                }, use_gif)?;
            },
            "Breadth-First Search" => {
                visualiser.visualise_algorithm_with_choice("Breadth-First Search", grid, |grid, counter| {
                    breadth_first_with_gui(grid, counter)
                }, use_gif)?;
            },
            "Depth-First Search" => {
                visualiser.visualise_algorithm_with_choice("Depth-First Search", grid, |grid, counter| {
                    depth_first_with_gui(grid, counter)
                }, use_gif)?;
            },
            "Greedy Best-First" => {
                visualiser.visualise_algorithm_with_choice("Greedy Best-First", grid, |grid, counter| {
                    greedy_best_first_with_gui(grid, counter)
                }, use_gif)?;
            },
            _ => {
                eprintln!("❌ Unknown algorithm: {}", algorithm);
                continue;
            }
        }
        
        println!("✅ Completed: {}\n", algorithm);
    }
    
    println!("🎉 All {} pathfinding algorithm visualisations completed!", algorithms.len());
    Ok(())
}

fn create_test_grid(width: usize, height: usize, obstacle_percentage: f64) -> Result<Grid> {
    let start = Position::new(0, 0);
    let end = Position::new(height.saturating_sub(1), width.saturating_sub(1));
    let mut grid = Grid::new(width, height, start, end);
    

    if width < 3 || height < 3 {
        return Ok(grid); // Return empty grid for very small sizes
    }
    
    let mut rng = rand::rng();
    let total_cells = width * height;
    let obstacle_count = (total_cells as f64 * obstacle_percentage) as usize;
    

    let protected_positions = get_protected_positions(&grid);
    
    let mut obstacles_placed = 0;
    let mut attempts = 0;
    let max_attempts = total_cells * 3; // Prevent infinite loops
    
    while obstacles_placed < obstacle_count && attempts < max_attempts {
        attempts += 1;
        
        let row = rng.random_range(0..height);
        let col = rng.random_range(0..width);
        let pos = Position::new(row, col);
        

        if protected_positions.contains(&pos) || grid.cells[row][col] != CellType::Open {
            continue;
        }
        

        grid.add_obstacle(pos);
        

        if is_grid_connected(&grid) {
            obstacles_placed += 1;
        } else {

            grid.cells[row][col] = CellType::Open;
        }
    }
    

    if !is_grid_connected(&grid) {

        return create_simple_connected_grid(width, height, obstacle_percentage);
    }
    
    Ok(grid)
}

fn get_protected_positions(grid: &Grid) -> HashSet<Position> {
    let mut protected = HashSet::new();
    

    protected.insert(grid.start);
    for neighbor in get_eight_directional_neighbors(grid.start, grid.width, grid.height) {
        protected.insert(neighbor);
    }
    

    protected.insert(grid.end);
    for neighbor in get_eight_directional_neighbors(grid.end, grid.width, grid.height) {
        protected.insert(neighbor);
    }
    
    protected
}

fn get_eight_directional_neighbors(pos: Position, width: usize, height: usize) -> Vec<Position> {
    let mut neighbors = Vec::new();
    let row = pos.row as i32;
    let col = pos.col as i32;
    

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),  // top-left, top, top-right
        ( 0, -1),          ( 0, 1),  // left, right  
        ( 1, -1), ( 1, 0), ( 1, 1)   // bottom-left, bottom, bottom-right
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

fn is_grid_connected(grid: &Grid) -> bool {

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back(grid.start);
    visited.insert(grid.start);
    
    while let Some(current) = queue.pop_front() {
        if current == grid.end {
            return true; // Found path to end
        }
        

        for neighbor in grid.get_neighbors(&current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    
    false // Couldn't reach end from start
}

fn create_simple_connected_grid(width: usize, height: usize, obstacle_percentage: f64) -> Result<Grid> {
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
           !get_eight_directional_neighbors(pos, width, height).iter().any(|&p| guaranteed_path.contains(&p)) &&
           grid.cells[row][col] == CellType::Open {
            grid.add_obstacle(pos);
            obstacles_placed += 1;
        }
    }
    
    Ok(grid)
}


fn astar_with_gui(grid: &Grid, counter: &mut GuiPerformanceCounter) -> Result<(Vec<Position>, PerformanceCounter)> {
    use crate::pathfinder::astar;
    use std::collections::{BinaryHeap, HashMap};
    use std::cmp::Ordering;
    
    #[derive(Debug, Clone)]
    struct Node {
        position: Position,
        g_score: f64,
        f_score: f64,
        parent: Option<Position>,
    }
    
    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.position == other.position
        }
    }
    
    impl Eq for Node {}
    
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.f_score.partial_cmp(&self.f_score)
        }
    }
    
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap_or(Ordering::Equal)
        }
    }
    
    let mut perf_counter = PerformanceCounter::new();
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut g_score: HashMap<Position, f64> = HashMap::new();
    let mut f_score: HashMap<Position, f64> = HashMap::new();


    g_score.insert(grid.start, 0.0);
    f_score.insert(grid.start, heuristic(&grid.start, &grid.end));
    
    open_set.push(Node {
        position: grid.start,
        g_score: 0.0,
        f_score: heuristic(&grid.start, &grid.end),
        parent: None,
    });
    
    counter.add_to_frontier(grid.start, "Added start to frontier", "A*");
    perf_counter.add_to_frontier();
    perf_counter.allocate_memory(1);

    while let Some(current_node) = open_set.pop() {
        let current = current_node.position;
        counter.explore_node(current, &format!("Exploring node ({}, {})", current.row, current.col), "A*");
        counter.remove_from_frontier(current);
        perf_counter.explore_node();

        if current == grid.end {
            let path = reconstruct_path(&came_from, current);
            counter.record_final_path(path.clone(), "A*");
            return Ok((path, perf_counter));
        }

        for neighbor in grid.get_neighbors(&current) {
            counter.compare();
            perf_counter.compare();
            
            let tentative_g_score = g_score.get(&current).unwrap_or(&f64::INFINITY) + 1.0;
            let neighbor_g_score = *g_score.get(&neighbor).unwrap_or(&f64::INFINITY);

            if tentative_g_score < neighbor_g_score {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                
                let neighbor_f_score = tentative_g_score + heuristic(&neighbor, &grid.end);
                f_score.insert(neighbor, neighbor_f_score);

                let neighbor_node = Node {
                    position: neighbor,
                    g_score: tentative_g_score,
                    f_score: neighbor_f_score,
                    parent: Some(current),
                };

                open_set.push(neighbor_node);
                counter.add_to_frontier(neighbor, &format!("Added neighbor ({}, {}) to frontier", neighbor.row, neighbor.col), "A*");
                perf_counter.add_to_frontier();
                perf_counter.allocate_memory(1);
            }
        }
    }

    Ok((Vec::new(), perf_counter))
}

fn dijkstra_with_gui(grid: &Grid, counter: &mut GuiPerformanceCounter) -> Result<(Vec<Position>, PerformanceCounter)> {
    use std::collections::{BinaryHeap, HashMap, HashSet};
    use std::cmp::Ordering;
    
    #[derive(Debug, Clone)]
    struct Node {
        position: Position,
        distance: f64,
    }
    
    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.position == other.position
        }
    }
    
    impl Eq for Node {}
    
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.distance.partial_cmp(&self.distance)
        }
    }
    
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap_or(Ordering::Equal)
        }
    }
    
    let mut perf_counter = PerformanceCounter::new();
    let mut priority_queue = BinaryHeap::new();
    let mut distances: HashMap<Position, f64> = HashMap::new();
    let mut previous: HashMap<Position, Position> = HashMap::new();
    let mut visited: HashSet<Position> = HashSet::new();

    distances.insert(grid.start, 0.0);
    
    priority_queue.push(Node {
        position: grid.start,
        distance: 0.0,
    });
    
    counter.add_to_frontier(grid.start, "Added start to queue", "Dijkstra");
    perf_counter.add_to_frontier();
    perf_counter.allocate_memory(1);

    while let Some(current_node) = priority_queue.pop() {
        let current = current_node.position;
        
        if visited.contains(&current) {
            continue;
        }
        
        visited.insert(current);
        counter.explore_node(current, &format!("Exploring node ({}, {})", current.row, current.col), "Dijkstra");
        counter.remove_from_frontier(current);
        perf_counter.explore_node();

        if current == grid.end {
            let path = reconstruct_path(&previous, current);
            counter.record_final_path(path.clone(), "Dijkstra");
            return Ok((path, perf_counter));
        }

        let current_distance = *distances.get(&current).unwrap_or(&f64::INFINITY);

        for neighbor in grid.get_neighbors(&current) {
            counter.compare();
            perf_counter.compare();
            
            if visited.contains(&neighbor) {
                continue;
            }

            let new_distance = current_distance + 1.0;
            let neighbor_distance = *distances.get(&neighbor).unwrap_or(&f64::INFINITY);

            if new_distance < neighbor_distance {
                distances.insert(neighbor, new_distance);
                previous.insert(neighbor, current);
                
                priority_queue.push(Node {
                    position: neighbor,
                    distance: new_distance,
                });
                
                counter.add_to_frontier(neighbor, &format!("Added neighbor ({}, {}) to queue", neighbor.row, neighbor.col), "Dijkstra");
                perf_counter.add_to_frontier();
                perf_counter.allocate_memory(1);
            }
        }
    }

    Ok((Vec::new(), perf_counter))
}

fn breadth_first_with_gui(grid: &Grid, counter: &mut GuiPerformanceCounter) -> Result<(Vec<Position>, PerformanceCounter)> {
    use std::collections::{VecDeque, HashMap, HashSet};
    
    let mut perf_counter = PerformanceCounter::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();

    queue.push_back(grid.start);
    visited.insert(grid.start);
    counter.add_to_frontier(grid.start, "Added start to queue", "BFS");
    perf_counter.add_to_frontier();
    perf_counter.allocate_memory(1);

    while let Some(current) = queue.pop_front() {
        counter.explore_node(current, &format!("Exploring node ({}, {})", current.row, current.col), "BFS");
        counter.remove_from_frontier(current);
        perf_counter.explore_node();

        if current == grid.end {
            let path = reconstruct_path(&came_from, current);
            counter.record_final_path(path.clone(), "BFS");
            return Ok((path, perf_counter));
        }

        for neighbor in grid.get_neighbors(&current) {
            counter.compare();
            perf_counter.compare();
            
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                came_from.insert(neighbor, current);
                queue.push_back(neighbor);
                
                counter.add_to_frontier(neighbor, &format!("Added neighbor ({}, {}) to queue", neighbor.row, neighbor.col), "BFS");
                perf_counter.add_to_frontier();
                perf_counter.allocate_memory(1);
            }
        }
    }

    Ok((Vec::new(), perf_counter))
}

fn depth_first_with_gui(grid: &Grid, counter: &mut GuiPerformanceCounter) -> Result<(Vec<Position>, PerformanceCounter)> {
    use std::collections::{HashMap, HashSet};
    
    let mut perf_counter = PerformanceCounter::new();
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    if dfs_recursive_gui(grid, grid.start, grid.end, &mut visited, &mut path, &mut perf_counter, counter) {
        counter.record_final_path(path.clone(), "DFS");
        Ok((path, perf_counter))
    } else {
        Ok((Vec::new(), perf_counter))
    }
}

fn dfs_recursive_gui(
    grid: &Grid,
    current: Position,
    target: Position,
    visited: &mut HashSet<Position>,
    path: &mut Vec<Position>,
    perf_counter: &mut PerformanceCounter,
    counter: &mut GuiPerformanceCounter,
) -> bool {
    counter.explore_node(current, &format!("Exploring node ({}, {}) [DFS]", current.row, current.col), "DFS");
    perf_counter.explore_node();
    visited.insert(current);
    path.push(current);

    if current == target {
        return true;
    }

    for neighbor in grid.get_neighbors(&current) {
        counter.compare();
        perf_counter.compare();
        
        if !visited.contains(&neighbor) {
            counter.add_to_frontier(neighbor, &format!("Adding neighbor ({}, {}) to stack", neighbor.row, neighbor.col), "DFS");
            perf_counter.allocate_memory(1);
            
            if dfs_recursive_gui(grid, neighbor, target, visited, path, perf_counter, counter) {
                return true;
            }
        }
    }

    path.pop();
    false
}

fn greedy_best_first_with_gui(grid: &Grid, counter: &mut GuiPerformanceCounter) -> Result<(Vec<Position>, PerformanceCounter)> {
    use std::collections::{BinaryHeap, HashMap, HashSet};
    use std::cmp::Ordering;
    
    #[derive(Debug, Clone)]
    struct Node {
        position: Position,
        heuristic: f64,
    }
    
    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.position == other.position
        }
    }
    
    impl Eq for Node {}
    
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.heuristic.partial_cmp(&self.heuristic)
        }
    }
    
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap_or(Ordering::Equal)
        }
    }
    
    let mut perf_counter = PerformanceCounter::new();
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut visited: HashSet<Position> = HashSet::new();

    open_set.push(Node {
        position: grid.start,
        heuristic: heuristic(&grid.start, &grid.end),
    });
    
    counter.add_to_frontier(grid.start, "Added start to frontier", "Greedy");
    perf_counter.add_to_frontier();
    perf_counter.allocate_memory(1);

    while let Some(current_node) = open_set.pop() {
        let current = current_node.position;
        
        if visited.contains(&current) {
            continue;
        }
        
        visited.insert(current);
        counter.explore_node(current, &format!("Exploring node ({}, {}) [Greedy]", current.row, current.col), "Greedy");
        counter.remove_from_frontier(current);
        perf_counter.explore_node();

        if current == grid.end {
            let path = reconstruct_path(&came_from, current);
            counter.record_final_path(path.clone(), "Greedy");
            return Ok((path, perf_counter));
        }

        for neighbor in grid.get_neighbors(&current) {
            counter.compare();
            perf_counter.compare();
            
            if !visited.contains(&neighbor) {
                came_from.insert(neighbor, current);
                
                open_set.push(Node {
                    position: neighbor,
                    heuristic: heuristic(&neighbor, &grid.end),
                });
                
                counter.add_to_frontier(neighbor, &format!("Added neighbor ({}, {}) to frontier", neighbor.row, neighbor.col), "Greedy");
                perf_counter.add_to_frontier();
                perf_counter.allocate_memory(1);
            }
        }
    }

    Ok((Vec::new(), perf_counter))
}

fn heuristic(from: &Position, to: &Position) -> f64 {
    let dx = (from.col as i32 - to.col as i32).abs() as f64;
    let dy = (from.row as i32 - to.row as i32).abs() as f64;
    dx + dy
}

fn reconstruct_path(came_from: &HashMap<Position, Position>, mut current: Position) -> Vec<Position> {
    let mut path = vec![current];
    
    while let Some(&parent) = came_from.get(&current) {
        current = parent;
        path.push(current);
    }
    
    path.reverse();
    path
}
