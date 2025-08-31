use crate::pathfinder::{Grid, Position, PerformanceCounter};
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

pub fn find_path(grid: &Grid) -> Result<(Vec<Position>, PerformanceCounter), String> {
    let mut counter = PerformanceCounter::new();
    let mut priority_queue = BinaryHeap::new();
    let mut distances: HashMap<Position, f64> = HashMap::new();
    let mut previous: HashMap<Position, Position> = HashMap::new();
    let mut visited: HashSet<Position> = HashSet::new();


    distances.insert(grid.start, 0.0);
    

    priority_queue.push(Node {
        position: grid.start,
        distance: 0.0,
    });
    
    counter.add_to_frontier();
    counter.allocate_memory(1);

    while let Some(current_node) = priority_queue.pop() {
        let current = current_node.position;
        

        if visited.contains(&current) {
            continue;
        }
        
        visited.insert(current);
        counter.explore_node();


        if current == grid.end {
            let path = reconstruct_path(&previous, current);
            return Ok((path, counter));
        }

        let current_distance = *distances.get(&current).unwrap_or(&f64::INFINITY);


        for neighbor in grid.get_neighbors(&current) {
            counter.compare();
            
            if visited.contains(&neighbor) {
                continue;
            }

            let edge_weight = 1.0;
            let new_distance = current_distance + edge_weight;
            let neighbor_distance = *distances.get(&neighbor).unwrap_or(&f64::INFINITY);

            if new_distance < neighbor_distance {
                distances.insert(neighbor, new_distance);
                previous.insert(neighbor, current);
                
                priority_queue.push(Node {
                    position: neighbor,
                    distance: new_distance,
                });
                
                counter.add_to_frontier();
                counter.allocate_memory(1);
            }
        }
    }


    Ok((Vec::new(), counter))
}

fn reconstruct_path(previous: &HashMap<Position, Position>, mut current: Position) -> Vec<Position> {
    let mut path = vec![current];
    
    while let Some(&parent) = previous.get(&current) {
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
    fn test_dijkstra_empty_grid() {
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
    fn test_dijkstra_with_obstacles() {
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
    fn test_dijkstra_no_path() {
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
    fn test_dijkstra_optimal_path() {

        let start = Position::new(0, 0);
        let end = Position::new(0, 3);
        let grid = Grid::new(4, 1, start, end);
        
        let result = find_path(&grid);
        assert!(result.is_ok());
        
        let (path, _) = result.unwrap();
        assert_eq!(path.len(), 4);
    }
}
