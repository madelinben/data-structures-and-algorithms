#[derive(Debug, Clone, PartialEq)]
pub enum MainMenuChoice {
    Search,
    Sort,
    Quit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchMenuChoice {
    LoadWords,
    ShowStats,
    RunBenchmarks,
    AnalyseArrayType,
    Back,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortMenuChoice {
    RunBenchmarks,
    AnalyseArrayType,
    GuiVisualisation,
    AlgorithmInfo,
    Back,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortAlgorithm {
    Bubble,
    Insertion,
    Selection,
    Merge,
    Quick,
    Heap,
    Shell,
    Tim,
    Tree,
    Bucket,
    Radix,
    Counting,
    Cube,
    All,
}

impl SortAlgorithm {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1" | "bubble" => Some(Self::Bubble),
            "2" | "insertion" => Some(Self::Insertion),
            "3" | "selection" => Some(Self::Selection),
            "4" | "merge" => Some(Self::Merge),
            "5" | "quick" => Some(Self::Quick),
            "6" | "heap" => Some(Self::Heap),
            "7" | "shell" => Some(Self::Shell),
            "8" | "tim" => Some(Self::Tim),
            "9" | "tree" => Some(Self::Tree),
            "10" | "bucket" => Some(Self::Bucket),
            "11" | "radix" => Some(Self::Radix),
            "12" | "counting" => Some(Self::Counting),
            "13" | "cube" => Some(Self::Cube),
            "a" | "all" => Some(Self::All),
            _ => None,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bubble => "bubble",
            Self::Insertion => "insertion", 
            Self::Selection => "selection",
            Self::Merge => "merge",
            Self::Quick => "quick",
            Self::Heap => "heap",
            Self::Shell => "shell",
            Self::Tim => "tim",
            Self::Tree => "tree",
            Self::Bucket => "bucket",
            Self::Radix => "radix",
            Self::Counting => "counting",
            Self::Cube => "cube",
            Self::All => "all",
        }
    }
}
