use super::PerformanceCounter;

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32, counter: &mut PerformanceCounter) {
        match counter.compare(&value, &self.value) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                match self.left {
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                        counter.allocate_memory(1);
                    }
                    Some(ref mut left) => {
                        left.insert(value, counter);
                    }
                }
            }
            std::cmp::Ordering::Greater => {
                match self.right {
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                        counter.allocate_memory(1);
                    }
                    Some(ref mut right) => {
                        right.insert(value, counter);
                    }
                }
            }
        }
    }

    fn inorder_traversal(&self, result: &mut Vec<i32>) {
        if let Some(ref left) = self.left {
            left.inorder_traversal(result);
        }
        result.push(self.value);
        if let Some(ref right) = self.right {
            right.inorder_traversal(result);
        }
    }
}

pub fn sort(arr: &mut [i32], counter: &mut PerformanceCounter) {
    if arr.is_empty() {
        return;
    }

    let mut root = TreeNode::new(arr[0]);
    counter.allocate_memory(1);

    for &value in arr.iter().skip(1) {
        root.insert(value, counter);
    }

    let mut sorted_values = Vec::new();
    root.inorder_traversal(&mut sorted_values);

    for (i, value) in sorted_values.into_iter().enumerate() {
        if i < arr.len() {
            arr[i] = value;
            counter.swaps += 1;
        }
    }
}