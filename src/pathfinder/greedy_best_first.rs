use crate::pathfinder::{Grid, Position, PerformanceCounter};
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

pub fn find_path(grid: &Grid) -> Result<(Vec<Position>, PerformanceCounter), String> {
    let mut counter = PerformanceCounter::new();
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut visited: HashSet<Position> = HashSet::new();


    open_set.push(Node {
        position: grid.start,
        heuristic: heuristic(&grid.start, &grid.end),
    });
    
    counter.add_to_frontier();
    counter.allocate_memory(1);

    while let Some(current_node) = open_set.pop() {
        let current = current_node.position;
        

        if visited.contains(&current) {
            continue;
        }
        
        visited.insert(current);
        counter.explore_node();


        if current == grid.end {
            let path = reconstruct_path(&came_from, current);
            return Ok((path, counter));
        }


        for neighbor in grid.get_neighbors(&current) {
            counter.compare();
            
            if !visited.contains(&neighbor) {
                came_from.insert(neighbor, current);
                
                open_set.push(Node {
                    position: neighbor,
                    heuristic: heuristic(&neighbor, &grid.end),
                });
                
                counter.add_to_frontier();
                counter.allocate_memory(1);
            }
        }
    }


    Ok((Vec::new(), counter))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pathfinder::CellType;

    #[test]
    fn test_greedy_best_first_empty_grid() {
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
    fn test_greedy_best_first_with_obstacles() {
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
    fn test_greedy_best_first_no_path() {
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
    fn test_heuristic_calculation() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(3, 4);
        
        let h = heuristic(&pos1, &pos2);
        assert_eq!(h, 7.0);
    }

    #[test]
    fn test_greedy_best_first_direct_path() {

        let start = Position::new(0, 0);
        let end = Position::new(0, 3);
        let grid = Grid::new(4, 1, start, end);
        
        let result = find_path(&grid);
        assert!(result.is_ok());
        
        let (path, _) = result.unwrap();
        assert_eq!(path.len(), 4);
        

        for (i, pos) in path.iter().enumerate() {
            assert_eq!(pos.row, 0);
            assert_eq!(pos.col, i);
        }
    }
}
