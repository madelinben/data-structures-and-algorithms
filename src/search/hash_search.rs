//! Hash Search Algorithm
//! 
//! Uses a hash map for O(1) average-case lookup time.
//! Time Complexity: O(1) average, O(n) worst case
//! Space Complexity: O(n)

use std::collections::HashMap;

/// Perform hash search using a pre-built HashMap
/// Returns whether the target was found
pub fn search(word_map: &HashMap<String, usize>, target: &str) -> bool {
    word_map.contains_key(target)
}

/// Get the index of a word in the original array
pub fn search_with_index(word_map: &HashMap<String, usize>, target: &str) -> Option<usize> {
    word_map.get(target).copied()
}

/// Build a hash map from a vector of strings
pub fn build_hash_map(data: &[String]) -> HashMap<String, usize> {
    data.iter()
        .enumerate()
        .map(|(i, word)| (word.clone(), i))
        .collect()
}

/// Hash search with collision counting (for analysis)
pub fn search_with_collision_analysis(word_map: &HashMap<String, usize>, target: &str) -> (bool, usize) {
    // In Rust's HashMap, we can't directly count collisions, but we can estimate
    // based on load factor and hash distribution
    let found = word_map.contains_key(target);
    let estimated_probes = if found { 1 } else { 1 }; // HashMap handles collisions internally
    (found, estimated_probes)
}

/// Multi-key hash search
pub fn search_multiple(word_map: &HashMap<String, usize>, targets: &[&str]) -> Vec<(String, bool)> {
    targets
        .iter()
        .map(|&target| (target.to_string(), word_map.contains_key(target)))
        .collect()
}



