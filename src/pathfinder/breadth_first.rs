use crate::pathfinder::{Grid, Position, PerformanceCounter};
use std::collections::{VecDeque, HashMap, HashSet};

pub fn find_path(grid: &Grid) -> Result<(Vec<Position>, PerformanceCounter), String> {
    let mut counter = PerformanceCounter::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();


    queue.push_back(grid.start);
    visited.insert(grid.start);
    counter.add_to_frontier();
    counter.allocate_memory(1);

    while let Some(current) = queue.pop_front() {
        counter.explore_node();


        if current == grid.end {
            let path = reconstruct_path(&came_from, current);
            return Ok((path, counter));
        }


        for neighbor in grid.get_neighbors(&current) {
            counter.compare();
            
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                came_from.insert(neighbor, current);
                queue.push_back(neighbor);
                
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
    fn test_bfs_empty_grid() {
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
    fn test_bfs_with_obstacles() {
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
    fn test_bfs_no_path() {
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
    fn test_bfs_shortest_path() {

        let start = Position::new(0, 0);
        let end = Position::new(0, 2);
        let grid = Grid::new(3, 1, start, end);
        
        let result = find_path(&grid);
        assert!(result.is_ok());
        
        let (path, _) = result.unwrap();
        assert_eq!(path.len(), 3);
    }
}
