use crate::prelude::*;
use crate::tree_traversal::{TreeNode, PerformanceCounter};
use crate::gui::tree_traversal::{TreeTraversalVisualiser, GuiPerformanceCounter};

pub fn run_gui_visualisation(algorithm: &str, tree_depth: usize) -> Result<()> {
    let mut visualiser = TreeTraversalVisualiser::new(tree_depth);
    
    let tree = create_test_tree(5, 2)?;
    
    match algorithm {
        "preorder" | "pre" => {
            visualiser.visualise_algorithm("Pre-order Traversal", tree, |tree, counter| {
                preorder_with_gui(tree, counter)
            })?;
        },
        "inorder" | "in" => {
            visualiser.visualise_algorithm("In-order Traversal", tree, |tree, counter| {
                inorder_with_gui(tree, counter)
            })?;
        },
        "postorder" | "post" => {
            visualiser.visualise_algorithm("Post-order Traversal", tree, |tree, counter| {
                postorder_with_gui(tree, counter)
            })?;
        },
        "levelorder" | "level" | "bfs" => {
            visualiser.visualise_algorithm("Level-order Traversal", tree, |tree, counter| {
                levelorder_with_gui(tree, counter)
            })?;
        },
        _ => {
            return Err(Error::validation(format!("Unknown tree traversal algorithm: {}", algorithm)));
        }
    }
    
    Ok(())
}

pub fn run_all_tree_visualisations(tree_depth: usize, use_gif: bool) -> Result<()> {
    let algorithms = vec![
        "Pre-order Traversal",
        "In-order Traversal", 
        "Post-order Traversal",
        "Level-order Traversal",
    ];
    
    for (i, algorithm) in algorithms.iter().enumerate() {
        println!("🔄 Processing {}/{}: {}", i + 1, algorithms.len(), algorithm);
        
        let tree = create_test_tree(5, 2)?;
        let mut visualiser = TreeTraversalVisualiser::new(tree_depth);
        
        match algorithm {
            &"Pre-order Traversal" => {
                visualiser.visualise_algorithm_with_choice("Pre-order Traversal", tree, |tree, counter| {
                    preorder_with_gui(tree, counter)
                }, use_gif)?;
            },
            &"In-order Traversal" => {
                visualiser.visualise_algorithm_with_choice("In-order Traversal", tree, |tree, counter| {
                    inorder_with_gui(tree, counter)
                }, use_gif)?;
            },
            &"Post-order Traversal" => {
                visualiser.visualise_algorithm_with_choice("Post-order Traversal", tree, |tree, counter| {
                    postorder_with_gui(tree, counter)
                }, use_gif)?;
            },
            &"Level-order Traversal" => {
                visualiser.visualise_algorithm_with_choice("Level-order Traversal", tree, |tree, counter| {
                    levelorder_with_gui(tree, counter)
                }, use_gif)?;
            },
            _ => {
                eprintln!("❌ Unknown algorithm: {}", algorithm);
                continue;
            }
        }
        
        println!("✅ Completed: {}\n", algorithm);
    }
    
    println!("🎉 All {} tree traversal algorithm visualisations completed!", algorithms.len());
    Ok(())
}


pub fn run_custom_tree_visualisation(depth: usize, children_per_node: usize) -> Result<()> {
    println!("🌳 Custom Tree Visualisation");
    println!("Depth: {}, Children per node: {}", depth, children_per_node);
    
    let tree = create_test_tree(depth, children_per_node)?;
    let mut visualiser = TreeTraversalVisualiser::new(depth);
    
    let algorithms = vec![
        ("Pre-order", "preorder"),
        ("Level-order", "levelorder"),
    ];
    
    for (display_name, algorithm_key) in &algorithms {
        println!("🔄 Running {} on custom tree", display_name);
        
        let tree_clone = tree.clone();
        
        match *algorithm_key {
            "preorder" => {
                visualiser.visualise_algorithm_with_choice(*display_name, tree_clone, |tree, counter| {
                    preorder_with_gui(tree, counter)
                }, true)?;
            },
            "levelorder" => {
                visualiser.visualise_algorithm_with_choice(*display_name, tree_clone, |tree, counter| {
                    levelorder_with_gui(tree, counter)
                }, true)?;
            },
            _ => continue,
        }
    }
    
    Ok(())
}

fn create_test_tree(depth: usize, children_per_node: usize) -> Result<TreeNode<i32>> {
    if depth == 0 {
        return Ok(TreeNode::new(1));
    }
    
    fn create_tree_recursive(current_depth: usize, max_depth: usize, children_per_node: usize, value: i32) -> TreeNode<i32> {
        let mut node = TreeNode::new(value);
        
        if current_depth < max_depth {
            for i in 0..children_per_node {
                let child_value = value * 10 + (i as i32) + 1;
                let child = create_tree_recursive(current_depth + 1, max_depth, children_per_node, child_value);
                node.add_child(child);
            }
        }
        
        node
    }
    
    Ok(create_tree_recursive(1, depth, children_per_node, 1))
}

fn preorder_with_gui(tree: &TreeNode<i32>, counter: &mut GuiPerformanceCounter) -> (Vec<i32>, PerformanceCounter) {
    let perf_counter = PerformanceCounter::new();
    
    let mut result = Vec::new();
    preorder_traverse_with_steps(tree, counter, &mut result, &mut vec![]);
    
    (result, perf_counter)
}

fn inorder_with_gui(tree: &TreeNode<i32>, counter: &mut GuiPerformanceCounter) -> (Vec<i32>, PerformanceCounter) {
    let perf_counter = PerformanceCounter::new();
    
    let mut result = Vec::new();
    inorder_traverse_with_steps(tree, counter, &mut result, &mut vec![]);
    
    (result, perf_counter)
}

fn postorder_with_gui(tree: &TreeNode<i32>, counter: &mut GuiPerformanceCounter) -> (Vec<i32>, PerformanceCounter) {
    let perf_counter = PerformanceCounter::new();
    
    let mut result = Vec::new();
    postorder_traverse_with_steps(tree, counter, &mut result, &mut vec![]);
    
    (result, perf_counter)
}

fn levelorder_with_gui(tree: &TreeNode<i32>, counter: &mut GuiPerformanceCounter) -> (Vec<i32>, PerformanceCounter) {
    let perf_counter = PerformanceCounter::new();
    
    let mut result = Vec::new();
    levelorder_traverse_with_steps(tree, counter, &mut result);
    
    (result, perf_counter)
}


fn preorder_traverse_with_steps(node: &TreeNode<i32>, gui_counter: &mut GuiPerformanceCounter, 
                               result: &mut Vec<i32>, stack_context: &mut Vec<i32>) {
    gui_counter.add_step(
        get_full_tree(node),
        vec![node.value],
        stack_context.clone(),
        format!("Pre-order: Processing node {}", node.value),
        "Pre-order Traversal".to_string(),
    );
    
    result.push(node.value);
    
    gui_counter.add_step(
        get_full_tree(node),
        vec![], 
        stack_context.clone(),
        format!("Pre-order: Completed node {}", node.value),
        "Pre-order Traversal".to_string(),
    );
    
    for child in node.children.iter().rev() {
        stack_context.push(child.value);
    }
    
    if !node.children.is_empty() {
        gui_counter.add_step(
            get_full_tree(node),
            vec![], 
            stack_context.clone(),
            format!("Pre-order: Added children {:?} to stack", node.children.iter().map(|c| c.value).collect::<Vec<_>>()),
            "Pre-order Traversal".to_string(),
        );
    }
    
    for child in &node.children {
        stack_context.retain(|&x| x != child.value);
        preorder_traverse_with_steps(child, gui_counter, result, stack_context);
    }
}

fn inorder_traverse_with_steps(node: &TreeNode<i32>, gui_counter: &mut GuiPerformanceCounter, 
                             result: &mut Vec<i32>, stack_context: &mut Vec<i32>) {
    stack_context.push(node.value);
    gui_counter.add_step(
        get_full_tree(node),
        vec![], 
        stack_context.clone(),
        format!("In-order: Added node {} to stack (waiting for left subtree)", node.value),
        "In-order Traversal".to_string(),
    );
    
    if !node.children.is_empty() {
        inorder_traverse_with_steps(&node.children[0], gui_counter, result, stack_context);
    }
    
    stack_context.retain(|&x| x != node.value);
    gui_counter.add_step(
        get_full_tree(node),
        vec![node.value],
        stack_context.clone(),
        format!("In-order: Processing node {}", node.value),
        "In-order Traversal".to_string(),
    );
    
    result.push(node.value);
    
    gui_counter.add_step(
        get_full_tree(node),
        vec![], 
        stack_context.clone(),
        format!("In-order: Completed node {}", node.value),
        "In-order Traversal".to_string(),
    );
    
    if node.children.len() > 1 {
        inorder_traverse_with_steps(&node.children[1], gui_counter, result, stack_context);
    }
}

fn postorder_traverse_with_steps(node: &TreeNode<i32>, gui_counter: &mut GuiPerformanceCounter, 
                                result: &mut Vec<i32>, stack_context: &mut Vec<i32>) {
    stack_context.push(node.value);
    gui_counter.add_step(
        get_full_tree(node),
        vec![], 
        stack_context.clone(),
        format!("Post-order: Added node {} to stack (waiting for children)", node.value),
        "Post-order Traversal".to_string(),
    );
    
    for child in &node.children {
        postorder_traverse_with_steps(child, gui_counter, result, stack_context);
    }
    
    stack_context.retain(|&x| x != node.value);
    gui_counter.add_step(
        get_full_tree(node),
        vec![node.value],
        stack_context.clone(),
        format!("Post-order: Processing node {}", node.value),
        "Post-order Traversal".to_string(),
    );
    
    result.push(node.value);
    
    gui_counter.add_step(
        get_full_tree(node),
        vec![], 
        stack_context.clone(),
        format!("Post-order: Completed node {}", node.value),
        "Post-order Traversal".to_string(),
    );
}

fn levelorder_traverse_with_steps(tree: &TreeNode<i32>, gui_counter: &mut GuiPerformanceCounter, result: &mut Vec<i32>) {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    queue.push_back(tree);
    
    gui_counter.add_step(
        get_full_tree(tree),
        vec![],
        vec![tree.value],
        format!("Level-order: Added root {} to queue", tree.value),
        "Level-order Traversal".to_string(),
    );
    
    while let Some(current) = queue.pop_front() {
        let queue_contents: Vec<i32> = queue.iter().map(|node| node.value).collect();
        gui_counter.add_step(
            get_full_tree(tree),
            vec![current.value],
            queue_contents.clone(),
            format!("Level-order: Processing node {} from queue", current.value),
            "Level-order Traversal".to_string(),
        );
        
        result.push(current.value);
        
        for child in &current.children {
            queue.push_back(child);
        }
        
        if !current.children.is_empty() {
            let new_queue_contents: Vec<i32> = queue.iter().map(|node| node.value).collect();
            gui_counter.add_step(
                get_full_tree(tree),
                vec![],
                new_queue_contents,
                format!("Level-order: Added children {:?} to queue", current.children.iter().map(|c| c.value).collect::<Vec<_>>()),
                "Level-order Traversal".to_string(),
            );
        }
        
        let final_queue_contents: Vec<i32> = queue.iter().map(|node| node.value).collect();
        gui_counter.add_step(
            get_full_tree(tree),
            vec![],
            final_queue_contents,
            format!("Level-order: Completed node {} (visited)", current.value),
            "Level-order Traversal".to_string(),
        );
    }
}

fn get_full_tree(root: &TreeNode<i32>) -> TreeNode<i32> {
    root.clone()
}
