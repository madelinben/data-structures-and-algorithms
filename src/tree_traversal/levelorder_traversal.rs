#![allow(dead_code)]

use std::collections::VecDeque;
use crate::tree_traversal::{TreeNode, PerformanceCounter};

pub fn traverse<T: Clone>(root: &TreeNode<T>, counter: &mut PerformanceCounter) -> Vec<T> {
    traverse_recursive(root, counter)
}

pub fn traverse_recursive<T: Clone>(root: &TreeNode<T>, counter: &mut PerformanceCounter) -> Vec<T> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    
    queue.push_back(root);
    
    while let Some(node) = queue.pop_front() {
        counter.nodes_visited += 1;
        result.push(node.value.clone());
        
        for child in &node.children {
            counter.comparisons += 1;
            queue.push_back(child);
        }
    }
    
    result
}
