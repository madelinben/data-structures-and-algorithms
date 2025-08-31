use crate::pathfinder::{Grid, Position, PerformanceCounter};
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

pub fn find_path(grid: &Grid) -> Result<(Vec<Position>, PerformanceCounter), String> {
    let mut counter = PerformanceCounter::new();
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
    
    counter.add_to_frontier();
    counter.allocate_memory(1);

    while let Some(current_node) = open_set.pop() {
        let current = current_node.position;
        counter.explore_node();


        if current == grid.end {
            let path = reconstruct_path(&came_from, current);
            return Ok((path, counter));
        }


        for neighbor in grid.get_neighbors(&current) {
            counter.compare();
            
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
    fn test_astar_empty_grid() {
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
    fn test_astar_with_obstacles() {
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
    fn test_astar_no_path() {
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
}
