use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub words_file: String,
    pub target_word: Option<String>,
    pub iterations: usize,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            words_file: "data/words.txt".to_string(),
            target_word: None,
            iterations: 100,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SortConfig {
    pub array_size: usize,
    pub iterations: usize,
    pub gui_enabled: bool,
}

impl Default for SortConfig {
    fn default() -> Self {
        Self {
            array_size: 1000,
            iterations: 10,
            gui_enabled: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub search: SearchConfig,
    pub sort: SortConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            search: SearchConfig::default(),
            sort: SortConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkParams {
    pub size: usize,
    pub iterations: usize,
    pub array_type: String,
}

impl Default for BenchmarkParams {
    fn default() -> Self {
        Self {
            size: 1000,
            iterations: 10,
            array_type: "Random".to_string(),
        }
    }
}
