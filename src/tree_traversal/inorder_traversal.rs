#![allow(dead_code)]

use crate::tree_traversal::{TreeNode, PerformanceCounter};

pub fn traverse<T: Clone>(root: &TreeNode<T>, counter: &mut PerformanceCounter) -> Vec<T> {
    traverse_recursive(root, counter)
}

pub fn traverse_recursive<T: Clone>(root: &TreeNode<T>, counter: &mut PerformanceCounter) -> Vec<T> {
    let mut result = Vec::new();
    inorder_recursive(root, &mut result, counter);
    result
}


fn inorder_recursive<T: Clone>(node: &TreeNode<T>, result: &mut Vec<T>, counter: &mut PerformanceCounter) {
    let children_count = node.children.len();
    
    if children_count == 0 {
        counter.nodes_visited += 1;
        result.push(node.value.clone());
        return;
    }
    
    // Process first half of children
    let mid = children_count / 2;
    for i in 0..mid {
        counter.comparisons += 1;
        inorder_recursive(&node.children[i], result, counter);
    }
    
    // Process root
    counter.nodes_visited += 1;
    result.push(node.value.clone());
    
    // Process second half of children
    for i in mid..children_count {
        counter.comparisons += 1;
        inorder_recursive(&node.children[i], result, counter);
    }
}
