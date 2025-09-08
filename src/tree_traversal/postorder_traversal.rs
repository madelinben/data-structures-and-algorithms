use crate::tree_traversal::{TreeNode, PerformanceCounter};

pub fn traverse<T: Clone>(root: &TreeNode<T>, counter: &mut PerformanceCounter) -> Vec<T> {
    traverse_recursive(root, counter)
}

pub fn traverse_recursive<T: Clone>(root: &TreeNode<T>, counter: &mut PerformanceCounter) -> Vec<T> {
    let mut result = Vec::new();
    postorder_recursive(root, &mut result, counter);
    result
}

fn postorder_recursive<T: Clone>(node: &TreeNode<T>, result: &mut Vec<T>, counter: &mut PerformanceCounter) {
    for child in &node.children {
        counter.comparisons += 1;
        postorder_recursive(child, result, counter);
    }
    
    counter.nodes_visited += 1;
    result.push(node.value.clone());
}
