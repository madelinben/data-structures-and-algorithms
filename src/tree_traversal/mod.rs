pub mod preorder_traversal;
pub mod inorder_traversal;
pub mod postorder_traversal;
pub mod levelorder_traversal;

use crate::prelude::*;
use std::time::{Duration, Instant};
use prettytable::{Table, Row, Cell};

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<T> {
    pub value: T,
    pub children: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: Vec::new(),
        }
    }
    
    pub fn add_child(&mut self, child: TreeNode<T>) {
        self.children.push(child);
    }
    
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
    
    pub fn depth(&self) -> usize {
        if self.is_leaf() {
            1
        } else {
            1 + self.children.iter().map(|child| child.depth()).max().unwrap_or(0)
        }
    }
    
    pub fn count_nodes(&self) -> usize {
        1 + self.children.iter().map(|child| child.count_nodes()).sum::<usize>()
    }
    
    pub fn count_leaves(&self) -> usize {
        if self.is_leaf() {
            1
        } else {
            self.children.iter().map(|child| child.count_leaves()).sum()
        }
    }
}

#[derive(Debug, Clone)]
pub struct TreeTraversalMetrics {
    pub algorithm_name: String,
    pub tree_nodes: usize,
    pub tree_depth: usize,
    pub tree_leaves: usize,
    pub nodes_visited: usize,
    pub comparisons: usize,
    pub memory_allocations: usize,
    pub duration: Duration,
    pub theoretical_time_complexity: String,
    pub theoretical_space_complexity: String,
    pub actual_nodes_ratio: f64,
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceCounter {
    pub nodes_visited: usize,
    pub comparisons: usize,
    pub memory_allocations: usize,
    pub max_stack_depth: usize,
    pub current_stack_depth: usize,
}

impl PerformanceCounter {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    
    pub fn visit_node(&mut self) {
        self.nodes_visited += 1;
    }
    
    pub fn compare<T: PartialOrd>(&mut self, _a: &T, _b: &T) -> bool {
        self.comparisons += 1;
        true
    }
    
    pub fn allocate_memory(&mut self, _size: usize) {
        self.memory_allocations += 1;
    }
    
    pub fn push_stack(&mut self) {
        self.current_stack_depth += 1;
        if self.current_stack_depth > self.max_stack_depth {
            self.max_stack_depth = self.current_stack_depth;
        }
        self.allocate_memory(1);
    }
    
    pub fn pop_stack(&mut self) {
        if self.current_stack_depth > 0 {
            self.current_stack_depth -= 1;
        }
    }
}

pub struct TreeTraversalCoordinator {
    test_trees: Vec<TreeNode<i32>>,
}

impl TreeTraversalCoordinator {
    pub fn new() -> Self {
        let mut coordinator = Self {
            test_trees: Vec::new(),
        };
        coordinator.generate_test_trees();
        coordinator
    }
    
    fn generate_test_trees(&mut self) {
        self.test_trees.push(self.create_binary_tree(4));
        
        self.test_trees.push(self.create_nary_tree(3, 3));
        
        self.test_trees.push(self.create_nary_tree(2, 8));
        
        self.test_trees.push(self.create_unbalanced_tree(6));
        
        self.test_trees.push(self.create_binary_tree(6));
    }
    
    fn create_binary_tree(&self, depth: usize) -> TreeNode<i32> {
        self.create_complete_tree(depth, 2, 1)
    }
    
    fn create_nary_tree(&self, depth: usize, children_per_node: usize) -> TreeNode<i32> {
        self.create_complete_tree(depth, children_per_node, 1)
    }
    
    fn create_complete_tree(&self, depth: usize, children_per_node: usize, value: i32) -> TreeNode<i32> {
        let mut node = TreeNode::new(value);
        
        if depth > 1 {
            for i in 0..children_per_node {
                let child_value = value * 10 + (i as i32) + 1;
                node.add_child(self.create_complete_tree(depth - 1, children_per_node, child_value));
            }
        }
        
        node
    }
    
    fn create_unbalanced_tree(&self, nodes: usize) -> TreeNode<i32> {
        if nodes == 0 {
            return TreeNode::new(0);
        }
        
        let mut root = TreeNode::new(1);
        let current = &mut root;
        
        for i in 2..=nodes {
            let child = TreeNode::new(i as i32);
            current.children.push(child);
            
            if i < nodes {
                let _len = root.children.len();
                break;
            }
        }
        
        let mut node = TreeNode::new(1);
        for i in 2..=nodes {
            let mut child = TreeNode::new(i as i32);
            if i < nodes {
                child.add_child(TreeNode::new((i + 1) as i32));
            }
            node.add_child(child);
        }
        
        node
    }
    
    pub fn run_benchmarks(&self, iterations: usize) -> Result<Vec<TreeTraversalMetrics>> {
        println!("\nRunning tree traversal benchmarks");
        println!("Iterations per algorithm: {}", iterations);
        println!("{}", "=".repeat(80));
        
        let mut all_results = Vec::new();
        
        for (tree_idx, tree) in self.test_trees.iter().enumerate() {
            println!("\n🌳 Tree {} - Nodes: {}, Depth: {}, Leaves: {}", 
                tree_idx + 1, tree.count_nodes(), tree.depth(), tree.count_leaves());
            
            all_results.push(self.benchmark_algorithm("Pre-order", tree, iterations, |tree, counter| {
                preorder_traversal::traverse(tree, counter)
            })?);
            
            all_results.push(self.benchmark_algorithm("In-order", tree, iterations, |tree, counter| {
                inorder_traversal::traverse(tree, counter)
            })?);
            
            all_results.push(self.benchmark_algorithm("Post-order", tree, iterations, |tree, counter| {
                postorder_traversal::traverse(tree, counter)
            })?);
            
            all_results.push(self.benchmark_algorithm("Level-order", tree, iterations, |tree, counter| {
                levelorder_traversal::traverse(tree, counter)
            })?);
        }
        
        self.display_results(&all_results);
        Ok(all_results)
    }
    
    fn benchmark_algorithm<F>(
        &self,
        name: &str,
        tree: &TreeNode<i32>,
        iterations: usize,
        traverse_fn: F,
    ) -> Result<TreeTraversalMetrics>
    where
        F: Fn(&TreeNode<i32>, &mut PerformanceCounter) -> Vec<i32>,
    {
        let mut total_nodes_visited = 0;
        let mut total_comparisons = 0;
        let mut total_memory = 0;
        let mut total_stack_depth = 0;
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let mut counter = PerformanceCounter::new();
            let _result = traverse_fn(tree, &mut counter);
            
            total_nodes_visited += counter.nodes_visited;
            total_comparisons += counter.comparisons;
            total_memory += counter.memory_allocations;
            total_stack_depth += counter.max_stack_depth;
        }
        
        let duration = start.elapsed() / iterations as u32;
        let avg_nodes_visited = total_nodes_visited / iterations;
        let avg_comparisons = total_comparisons / iterations;
        let avg_memory = total_memory / iterations;
        let _avg_stack_depth = total_stack_depth / iterations;
        
        let (time_complexity, space_complexity) = get_algorithm_complexity(name);
        let actual_nodes_ratio = avg_nodes_visited as f64 / tree.count_nodes() as f64;
        
        Ok(TreeTraversalMetrics {
            algorithm_name: format!("{} (Tree {})", name, 1),
            tree_nodes: tree.count_nodes(),
            tree_depth: tree.depth(),
            tree_leaves: tree.count_leaves(),
            nodes_visited: avg_nodes_visited,
            comparisons: avg_comparisons,
            memory_allocations: avg_memory,
            duration,
            theoretical_time_complexity: time_complexity,
            theoretical_space_complexity: space_complexity,
            actual_nodes_ratio,
        })
    }
    
    fn display_results(&self, results: &[TreeTraversalMetrics]) {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Algorithm"),
            Cell::new("Nodes"),
            Cell::new("Depth"),
            Cell::new("Visited"),
            Cell::new("Ratio"),
            Cell::new("Comparisons"),
            Cell::new("Duration (μs)"),
            Cell::new("Time Complex"),
            Cell::new("Space Complex"),
        ]));
        
        for metrics in results {
            table.add_row(Row::new(vec![
                Cell::new(&metrics.algorithm_name),
                Cell::new(&metrics.tree_nodes.to_string()),
                Cell::new(&metrics.tree_depth.to_string()),
                Cell::new(&metrics.nodes_visited.to_string()),
                Cell::new(&format!("{:.2}", metrics.actual_nodes_ratio)),
                Cell::new(&metrics.comparisons.to_string()),
                Cell::new(&metrics.duration.as_micros().to_string()),
                Cell::new(&metrics.theoretical_time_complexity),
                Cell::new(&metrics.theoretical_space_complexity),
            ]));
        }
        
        table.printstd();
        
        println!("\n📊 Summary Statistics:");
        let standard_results = results;
        
        if !standard_results.is_empty() {
            let avg_standard_ratio: f64 = standard_results.iter()
                .map(|r| r.actual_nodes_ratio)
                .sum::<f64>() / standard_results.len() as f64;
            println!("   Standard algorithms average node visitation ratio: {:.2}", avg_standard_ratio);
        }
        
    }
}

fn get_algorithm_complexity(name: &str) -> (String, String) {
    match name {
        "Pre-order" | "In-order" | "Post-order" => ("O(n)".to_string(), "O(h)".to_string()),
        "Level-order" => ("O(n)".to_string(), "O(w)".to_string()),
        _ => ("O(n)".to_string(), "O(h)".to_string()),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_creation() {
        let node = TreeNode::new(42);
        assert_eq!(node.value, 42);
        assert!(node.is_leaf());
        assert_eq!(node.depth(), 1);
        assert_eq!(node.count_nodes(), 1);
    }

    #[test]
    fn test_tree_node_with_children() {
        let mut root = TreeNode::new(1);
        root.add_child(TreeNode::new(2));
        root.add_child(TreeNode::new(3));
        
        assert_eq!(root.children.len(), 2);
        assert!(!root.is_leaf());
        assert_eq!(root.depth(), 2);
        assert_eq!(root.count_nodes(), 3);
        assert_eq!(root.count_leaves(), 2);
    }

    #[test]
    fn test_performance_counter() {
        let mut counter = PerformanceCounter::new();
        counter.visit_node();
        counter.push_stack();
        counter.compare(&1, &2);
        
        assert_eq!(counter.nodes_visited, 1);
        assert_eq!(counter.comparisons, 1);
        assert_eq!(counter.current_stack_depth, 1);
        assert_eq!(counter.max_stack_depth, 1);
    }
}
