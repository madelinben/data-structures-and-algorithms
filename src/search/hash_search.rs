use std::collections::HashMap;

pub fn search(word_map: &HashMap<String, usize>, target: &str) -> bool {
    word_map.contains_key(target)
}

pub fn search_with_index(word_map: &HashMap<String, usize>, target: &str) -> Option<usize> {
    word_map.get(target).copied()
}

pub fn build_hash_map(data: &[String]) -> HashMap<String, usize> {
    data.iter()
        .enumerate()
        .map(|(i, word)| (word.clone(), i))
        .collect()
}

pub fn search_with_collision_analysis(word_map: &HashMap<String, usize>, target: &str) -> (bool, usize) {
    let found = word_map.contains_key(target);
    let estimated_probes = if found { 1 } else { 1 };
    (found, estimated_probes)
}

pub fn search_multiple(word_map: &HashMap<String, usize>, targets: &[&str]) -> Vec<(String, bool)> {
    targets
        .iter()
        .map(|&target| (target.to_string(), word_map.contains_key(target)))
        .collect()
}