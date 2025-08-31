#![allow(unused)]

use crate::prelude::*;
use std::io::{self, Write};
use clap::{Command, Arg, ArgMatches};

mod error;
mod prelude;
mod utils;
mod search;
mod sort;

use search::SearchCoordinator;
use sort::SortCoordinator;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = create_cli().get_matches();
    
    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            run_search_algorithms(sub_matches).await?;
        }
        Some(("sort", sub_matches)) => {
            run_sort_algorithms(sub_matches).await?;
        }
        _ => {
            run_interactive_menu().await?;
        }
    }

    Ok(())
}

fn create_cli() -> Command {
    Command::new("Data Structures and Algorithms")
        .version("0.1.0")
        .about("A Rust project for exploring data structures and algorithms")
        .subcommand(
            Command::new("search")
                .about("Search Algorithm Benchmarking System")
                .arg(
                    Arg::new("words")
                        .short('w')
                        .long("words")
                        .value_name("FILE")
                        .help("Path to words file for search benchmarking")
                        .default_value("data/words.txt")
                )
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .value_name("WORD")
                        .help("Target word to search for")
                )
                .arg(
                    Arg::new("iterations")
                        .short('i')
                        .long("iterations")
                        .value_name("NUM")
                        .help("Number of iterations for benchmarking")
                        .default_value("100")
                )
        )
        .subcommand(
            Command::new("sort")
                .about("Sorting Algorithm Benchmarking System")
                .arg(
                    Arg::new("size")
                        .short('s')
                        .long("size")
                        .value_name("SIZE")
                        .help("Array size for sorting benchmarks")
                        .default_value("1000")
                )
                .arg(
                    Arg::new("iterations")
                        .short('i')
                        .long("iterations")
                        .value_name("NUM")
                        .help("Number of iterations for benchmarking")
                        .default_value("10")
                )
                .arg(
                    Arg::new("gui")
                        .long("gui")
                        .help("Enable GUI visualization")
                        .action(clap::ArgAction::SetTrue)
                )
        )
}

async fn run_interactive_menu() -> Result<()> {
    loop {
        print_main_menu();
        let choice = get_user_input("Please select an option (2-3, or q to quit): ")?;

        match choice.trim() {
            "1" => {
                run_search_interactive().await?;
            }
            "2" => {
                run_sort_interactive().await?;
            }
            "q" | "Q" | "quit" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
                println!("Press Enter to continue...");
                let _ = get_user_input("")?;
            }
        }
    }
    
    Ok(())
}

fn print_main_menu() {
    println!("\n{}", "=".repeat(60));
    println!("    Data Structures and Algorithms in Rust");
    println!("{}", "=".repeat(60));
    println!("1. Search Algorithms (Linear, Binary, Hash, etc.)");
    println!("2. Sorting Algorithms (13+ algorithms with benchmarking)");
    println!("q. Quit");
    println!("{}", "=".repeat(60));
}

async fn run_search_algorithms(matches: &ArgMatches) -> Result<()> {
    let words_file = matches.get_one::<String>("words").unwrap();
    let iterations: usize = matches.get_one::<String>("iterations").unwrap().parse()
        .map_err(|_| Error::Generic("Invalid iterations number".to_string()))?;
    
    println!("🔍 Search Algorithm Benchmarking System");
    println!("{}", "=".repeat(50));
    
    let mut coordinator = SearchCoordinator::new();
    coordinator.load_words(words_file).await?;
    
    let target_word = if let Some(target) = matches.get_one::<String>("target") {
        target.clone()
    } else {
        let stats = coordinator.get_stats();
        println!("{}", stats);
        let input = get_user_input("Enter target word to search for: ")?;
        input.trim().to_lowercase()
    };
    
    coordinator.run_benchmarks(&target_word, iterations)?;
    Ok(())
}

async fn run_sort_algorithms(matches: &ArgMatches) -> Result<()> {
    let size: usize = matches.get_one::<String>("size").unwrap().parse()
        .map_err(|_| Error::Generic("Invalid size number".to_string()))?;
    let iterations: usize = matches.get_one::<String>("iterations").unwrap().parse()
        .map_err(|_| Error::Generic("Invalid iterations number".to_string()))?;
    let gui_enabled = matches.get_flag("gui");
    
    println!("📊 Sorting Algorithm Benchmarking System");
    println!("{}", "=".repeat(50));
    
    if gui_enabled {
        println!("🎨 GUI Visualization Mode Enabled!");
        println!("Select an algorithm to visualize:\n");
        
        loop {
            println!("Available algorithms for visualization:");
            println!("1. Bubble Sort          2. Insertion Sort       3. Selection Sort");
            println!("4. Merge Sort           5. Quick Sort            6. Heap Sort");
            println!("7. Shell Sort           8. Tim Sort              9. Tree Sort");
            println!("10. Bucket Sort         11. Radix Sort           12. Counting Sort");
            println!("13. Cube Sort           a. All Algorithms        q. Quit");
            print!("\nSelect algorithm (1-13, 'a', or 'q'): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            
            match input {
                "1" => {
                    if let Err(e) = sort::gui::run_gui_visualization("bubble", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "2" => {
                    if let Err(e) = sort::gui::run_gui_visualization("insertion", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "3" => {
                    if let Err(e) = sort::gui::run_gui_visualization("selection", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "4" => {
                    if let Err(e) = sort::gui::run_gui_visualization("merge", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "5" => {
                    if let Err(e) = sort::gui::run_gui_visualization("quick", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "6" => {
                    if let Err(e) = sort::gui::run_gui_visualization("heap", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "7" => {
                    if let Err(e) = sort::gui::run_gui_visualization("shell", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "8" => {
                    if let Err(e) = sort::gui::run_gui_visualization("tim", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "9" => {
                    if let Err(e) = sort::gui::run_gui_visualization("tree", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "10" => {
                    if let Err(e) = sort::gui::run_gui_visualization("bucket", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "11" => {
                    if let Err(e) = sort::gui::run_gui_visualization("radix", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "12" => {
                    if let Err(e) = sort::gui::run_gui_visualization("counting", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "13" => {
                    if let Err(e) = sort::gui::run_gui_visualization("cube", size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "a" | "all" => {
                    if let Err(e) = sort::gui::run_all_gui_visualizations(size) {
                        eprintln!("❌ GUI Error: {}", e);
                    }
                    break;
                },
                "q" => {
                    println!("👋 Goodbye!");
                    return Ok(());
                },
                _ => {
                    println!("❌ Invalid choice. Please enter 1-13, 'a', or 'q'.");
                }
            }
        }
        return Ok(());
    }
    
    let mut coordinator = SortCoordinator::new();
    coordinator.run_benchmarks(size, iterations)?;
    Ok(())
}

async fn run_search_interactive() -> Result<()> {
    let mut coordinator = SearchCoordinator::new();
    
    loop {
        print_search_menu();
        let choice = get_user_input("Please select an option (1-4, or b to go back): ")?;

        match choice.trim() {
            "1" => {
                let path = get_user_input("Enter path to words file (default: data/words.txt): ")?;
                let words_file = if path.trim().is_empty() { "data/words.txt" } else { path.trim() };
                
                match coordinator.load_words(words_file).await {
                    Ok(_) => println!("✓ Words loaded successfully!"),
                    Err(e) => println!("✗ Failed to load words: {}", e),
                }
            }
            "2" => {
                let stats = coordinator.get_stats();
                if stats.contains("0") {
                    println!("⚠️  No words loaded. Please load words first.");
                } else {
                    println!("\n{}", stats);
                }
            }
            "3" => {
                let target = get_user_input("Enter target word to search for: ")?;
                let iterations_str = get_user_input("Enter number of iterations (default: 100): ")?;
                let iterations = if iterations_str.trim().is_empty() { 
                    100 
                } else { 
                    iterations_str.trim().parse().unwrap_or(100) 
                };
                
                match coordinator.run_benchmarks(&target.trim().to_lowercase(), iterations) {
                    Ok(_) => println!("\n✓ Benchmarks completed!"),
                    Err(e) => println!("✗ Benchmark failed: {}", e),
                }
            }
            "4" => {
                let array_type = get_user_input("Enter array type to analyze (Random/Nearly Sorted/etc): ")?;
                let size_str = get_user_input("Enter array size (default: 10000): ")?;
                let size = if size_str.trim().is_empty() { 10000 } else { size_str.trim().parse().unwrap_or(10000) };
                
                match coordinator.analyze_array_type(&array_type.trim(), size) {
                    Ok(_) => println!("\n✓ Analysis completed!"),
                    Err(e) => println!("✗ Analysis failed: {}", e),
                }
            }
            "b" | "B" | "back" => break,
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
        
        if choice.trim() != "b" && choice.trim() != "B" && choice.trim() != "back" {
            println!("\nPress Enter to continue...");
            let _ = get_user_input("")?;
        }
    }
    
    Ok(())
}

async fn run_sort_interactive() -> Result<()> {
    let mut coordinator = SortCoordinator::new();
    
    loop {
        print_sort_menu();
        let choice = get_user_input("Please select an option (1-4, or b to go back): ")?;

        match choice.trim() {
            "1" => {
                let size_str = get_user_input("Enter array size (default: 1000): ")?;
                let size = if size_str.trim().is_empty() { 1000 } else { size_str.trim().parse().unwrap_or(1000) };
                
                let iterations_str = get_user_input("Enter iterations (default: 10): ")?;
                let iterations = if iterations_str.trim().is_empty() { 10 } else { iterations_str.trim().parse().unwrap_or(10) };
                
                match coordinator.run_benchmarks(size, iterations) {
                    Ok(_) => println!("\n✓ Benchmarks completed!"),
                    Err(e) => println!("✗ Benchmark failed: {}", e),
                }
            }
            "2" => {
                let array_type = get_user_input("Enter array type (Random/Nearly Sorted/Reverse Sorted/etc): ")?;
                let size_str = get_user_input("Enter array size (default: 1000): ")?;
                let size = if size_str.trim().is_empty() { 1000 } else { size_str.trim().parse().unwrap_or(1000) };
                
                match coordinator.analyze_array_type(&array_type.trim(), size) {
                    Ok(_) => println!("\n✓ Analysis completed!"),
                    Err(e) => println!("✗ Analysis failed: {}", e),
                }
            }
            "3" => {
                println!("🎨 GUI Visualization");
                println!("Select an algorithm to visualize:");
                println!("Available: bubble, insertion, selection, merge, quick");
                print!("Enter algorithm name: ");
                io::stdout().flush().unwrap();
                
                let mut algorithm = String::new();
                io::stdin().read_line(&mut algorithm).unwrap();
                let algorithm = algorithm.trim();
                
                if !algorithm.is_empty() {
                    let size_str = get_user_input("Enter array size for visualization (default: 20): ")?;
                    let size = if size_str.trim().is_empty() { 20 } else { size_str.trim().parse().unwrap_or(20) };
                    
                    if let Err(e) = sort::gui::run_gui_visualization(algorithm, size) {
                        eprintln!("❌ Error: {}", e);
                    }
                } else {
                    println!("❌ No algorithm specified.");
                }
                
                match coordinator.run_benchmarks(20, 1) {
                    Ok(_) => println!("✓ Console benchmark completed!"),
                    Err(e) => println!("✗ Benchmark failed: {}", e),
                }
            }
            "4" => {
                print_algorithm_info();
            }
            "b" | "B" | "back" => break,
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
        
        if choice.trim() != "b" && choice.trim() != "B" && choice.trim() != "back" {
            println!("\nPress Enter to continue...");
            let _ = get_user_input("")?;
        }
    }
    
    Ok(())
}

fn print_search_menu() {
    println!("\n{}", "=".repeat(60));
    println!("        Search Algorithm Benchmarking");
    println!("{}", "=".repeat(60));
    println!("1. Load Words File");
    println!("2. Show Dataset Statistics"); 
    println!("3. Run Search Benchmarks");
    println!("4. Analyze Specific Array Type");
    println!("b. Back to Main Menu");
    println!("{}", "=".repeat(60));
}

fn print_sort_menu() {
    println!("\n{}", "=".repeat(60));
    println!("        Sorting Algorithm Benchmarking");
    println!("{}", "=".repeat(60));
    println!("1. Run Complete Benchmark Suite (13+ algorithms)");
    println!("2. Analyze Specific Array Type");
    println!("3. GUI Visualization");
    println!("4. Algorithm Information");
    println!("b. Back to Main Menu");
    println!("{}", "=".repeat(60));
}

fn print_algorithm_info() {
    println!("\n{}", "=".repeat(80));
    println!("                    SORTING ALGORITHMS IMPLEMENTED");
    println!("{}", "=".repeat(80));
    
    let algorithms = vec![
        ("Bubble Sort", "O(n²)", "O(1)", "Yes", "Yes", "Yes"),
        ("Insertion Sort", "O(n²)", "O(1)", "Yes", "Yes", "Yes"),
        ("Selection Sort", "O(n²)", "O(1)", "No", "No", "Yes"),
        ("Merge Sort", "O(n log n)", "O(n)", "Yes", "No", "No"),
        ("Quick Sort", "O(n log n)", "O(log n)", "No", "No", "Yes"),
        ("Heap Sort", "O(n log n)", "O(1)", "No", "No", "Yes"),
        ("Shell Sort", "O(n^1.25)", "O(1)", "No", "Yes", "Yes"),
        ("Tim Sort", "O(n log n)", "O(n)", "Yes", "Yes", "No"),
        ("Tree Sort", "O(n log n)", "O(n)", "Yes", "No", "No"),
        ("Bucket Sort", "O(n + k)", "O(n + k)", "Yes", "No", "No"),
        ("Radix Sort", "O(d × n)", "O(n + k)", "Yes", "No", "No"),
        ("Counting Sort", "O(n + k)", "O(k)", "Yes", "No", "No"),
        ("Cube Sort", "O(n log n)", "O(n)", "No", "No", "No"),
    ];
    
    println!("{:<15} {:<12} {:<12} {:<8} {:<10} {:<10}", 
        "Algorithm", "Time", "Space", "Stable", "Adaptive", "In-Place");
    println!("{}", "-".repeat(80));
    
    for (name, time, space, stable, adaptive, in_place) in algorithms {
        println!("{:<15} {:<12} {:<12} {:<8} {:<10} {:<10}", 
            name, time, space, stable, adaptive, in_place);
    }
    
    println!("\n📝 Legend:");
    println!("  • Stable: Maintains relative order of equal elements");
    println!("  • Adaptive: Performs better on partially sorted data");  
    println!("  • In-Place: Uses O(1) extra memory");
    println!("  • n = array size, k = range of input, d = number of digits");
}

fn get_user_input(prompt: &str) -> Result<String> {
    use std::io::{self, Write};
    
    print!("{}", prompt);
    io::stdout().flush().map_err(|e| Error::Io(e))?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| Error::Io(e))?;
    
    Ok(input)
}