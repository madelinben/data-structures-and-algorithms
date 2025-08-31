use crate::pathfinder::{Grid, Position, PerformanceCounter};
use std::collections::{HashMap, HashSet};

pub fn find_path(grid: &Grid) -> Result<(Vec<Position>, PerformanceCounter), String> {
    let mut counter = PerformanceCounter::new();
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    if dfs_recursive(grid, grid.start, grid.end, &mut visited, &mut path, &mut counter) {
        Ok((path, counter))
    } else {
        Ok((Vec::new(), counter))
    }
}

fn dfs_recursive(
    grid: &Grid,
    current: Position,
    target: Position,
    visited: &mut HashSet<Position>,
    path: &mut Vec<Position>,
    counter: &mut PerformanceCounter,
) -> bool {
    counter.explore_node();
    visited.insert(current);
    path.push(current);

    
    if current == target {
        return true;
    }


    for neighbor in grid.get_neighbors(&current) {
        counter.compare();
        
        if !visited.contains(&neighbor) {
            counter.allocate_memory(1);
            
            if dfs_recursive(grid, neighbor, target, visited, path, counter) {
                return true;
            }
        }
    }


    path.pop();
    false
}


pub fn find_path_iterative(grid: &Grid) -> Result<(Vec<Position>, PerformanceCounter), String> {
    let mut counter = PerformanceCounter::new();
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();


    stack.push(grid.start);
    counter.add_to_frontier();
    counter.allocate_memory(1);

    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);
        counter.explore_node();


        if current == grid.end {
            let path = reconstruct_path(&came_from, current);
            return Ok((path, counter));
        }


        let mut neighbors = grid.get_neighbors(&current);
        neighbors.reverse();
        
        for neighbor in neighbors {
            counter.compare();
            
            if !visited.contains(&neighbor) {
                stack.push(neighbor);
                came_from.insert(neighbor, current);
                counter.add_to_frontier();
                counter.allocate_memory(1);
            }
        }
    }


    Ok((Vec::new(), counter))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pathfinder::CellType;

    #[test]
    fn test_dfs_empty_grid() {
        let start = Position::new(0, 0);
        let end = Position::new(2, 2);
        let grid = Grid::new(3, 3, start, end);
        
        let result = find_path(&grid);
        assert!(result.is_ok());
        
        let (path, _) = result.unwrap();
        assert!(!path.is_empty());
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], end);
    }

    #[test]
    fn test_dfs_with_obstacles() {
        let start = Position::new(0, 0);
        let end = Position::new(2, 2);
        let mut grid = Grid::new(3, 3, start, end);
        

        grid.add_obstacle(Position::new(1, 1));
        
        let result = find_path(&grid);
        assert!(result.is_ok());
        
        let (path, _) = result.unwrap();
        assert!(!path.is_empty());
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], end);
        

        for pos in &path {
            assert_ne!(grid.cells[pos.row][pos.col], CellType::Blocked);
        }
    }

    #[test]
    fn test_dfs_no_path() {
        let start = Position::new(0, 0);
        let end = Position::new(2, 2);
        let mut grid = Grid::new(3, 3, start, end);
        

        grid.add_obstacle(Position::new(0, 1));
        grid.add_obstacle(Position::new(1, 0));
        grid.add_obstacle(Position::new(1, 1));
        
        let result = find_path(&grid);
        assert!(result.is_ok());
        
        let (path, _) = result.unwrap();
        assert!(path.is_empty());
    }

    #[test]
    fn test_dfs_iterative_empty_grid() {
        let start = Position::new(0, 0);
        let end = Position::new(2, 2);
        let grid = Grid::new(3, 3, start, end);
        
        let result = find_path_iterative(&grid);
        assert!(result.is_ok());
        
        let (path, _) = result.unwrap();
        assert!(!path.is_empty());
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], end);
    }
}
