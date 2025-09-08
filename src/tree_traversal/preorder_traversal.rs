#![allow(dead_code)]

use crate::tree_traversal::{TreeNode, PerformanceCounter};

pub fn traverse<T: Clone>(root: &TreeNode<T>, counter: &mut PerformanceCounter) -> Vec<T> {
    traverse_recursive(root, counter)
}

pub fn traverse_recursive<T: Clone>(root: &TreeNode<T>, counter: &mut PerformanceCounter) -> Vec<T> {
    let mut result = Vec::new();
    preorder_recursive(root, &mut result, counter);
    result
}

fn preorder_recursive<T: Clone>(node: &TreeNode<T>, result: &mut Vec<T>, counter: &mut PerformanceCounter) {
    counter.nodes_visited += 1;
    result.push(node.value.clone());
    
    for child in &node.children {
        counter.comparisons += 1;
        preorder_recursive(child, result, counter);
    }
}

pub fn traverse_iterative<T: Clone>(root: &TreeNode<T>, counter: &mut PerformanceCounter) -> Vec<T> {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    stack.push(root);
    
    while let Some(node) = stack.pop() {
        counter.nodes_visited += 1;
        result.push(node.value.clone());
        
        for child in node.children.iter().rev() {
            counter.comparisons += 1;
            stack.push(child);
        }
    }
    
    result
}
