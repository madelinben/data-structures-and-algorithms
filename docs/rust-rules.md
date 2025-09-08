# Rust Programming Rules & Best Practices

This document outlines the coding standards and best practices for this Rust project focused on sorting algorithms, search algorithms, pathfinding algorithms, tree traversal algorithms, and algorithm visualisation implementation.

## Project Structure & Organisation

### Module Organisation
- Use `mod.rs` files for module declarations
- Keep modules focused on single responsibilities
- Organize related functionality into logical modules
- Use clear, descriptive module names

### Directory Structure
```
src/
├── main.rs              # Simple entry point - delegates to AppController
├── prelude.rs           # Common imports and types
├── error.rs             # Enhanced error handling with specific error types
├── models/              # MVC Models - Data structures and configuration
│   ├── mod.rs           # Module exports
│   ├── config.rs        # Configuration structs (SearchConfig, SortConfig, etc.)
│   └── menu_choice.rs   # Menu choice enums and parsing logic
├── views/               # MVC Views - Console interface and user interaction
│   ├── mod.rs           # Module exports
│   ├── console.rs       # Core console output and input handling
│   ├── menu_display.rs  # Menu presentation logic
│   └── input_handler.rs # User input validation and parsing
├── controllers/         # MVC Controllers - Business logic coordination
│   ├── mod.rs           # Module exports
│   ├── app_controller.rs # Main application controller
│   ├── search_controller.rs # Search algorithm coordination
│   ├── sort_controller.rs # Sort algorithm coordination
│   ├── pathfinder_controller.rs # Pathfinder algorithm coordination
│   └── tree_traversal_controller.rs # Tree traversal algorithm coordination
├── gui/                 # GUI visualisation functionality
│   ├── mod.rs           # Module exports
│   ├── sorting.rs       # Core sorting visualisation logic
│   ├── visualisation.rs # GUI algorithm runners
│   └── renderer.rs      # Frame and GIF rendering
├── search/              # Search algorithm implementations
│   ├── mod.rs           # Search coordinator and benchmarking
│   └── *.rs             # Individual search algorithms
├── sort/                # Sort algorithm implementations
│   ├── mod.rs           # Sort coordinator and benchmarking
│   └── *.rs             # Individual sort algorithms
├── pathfinder/          # Pathfinding algorithm implementations
│   ├── mod.rs           # Pathfinder coordinator and benchmarking
│   └── *.rs             # Individual pathfinding algorithms
├── tree_traversal/      # Tree traversal algorithm implementations
│   ├── mod.rs           # Tree traversal coordinator and benchmarking
│   └── *.rs             # Individual tree traversal algorithms
└── utils/               # Utility functions
    └── mod.rs
```

### Algorithm Module Structure
Algorithm implementations are organised in dedicated modules:
- `sort/` - Contains 13 sorting algorithms (bubble, insertion, selection, merge, quick, heap, shell, tim, tree, bucket, radix, counting, cube)
- `search/` - Contains 6 search algorithms (linear, binary, hash, interpolation, jump, exponential)
- `pathfinder/` - Contains 5 pathfinding algorithms (A*, Dijkstra, breadth-first, depth-first, greedy best-first)
- `tree_traversal/` - Contains 4 tree traversal algorithms (preorder, inorder, postorder, levelorder)
- Each algorithm module includes performance tracking and proper error handling

## MVC Architecture

### Model-View-Controller Pattern
This project follows a clean MVC architecture for better organisation and maintainability:

#### Models (`src/models/`)
- **Purpose**: Data structures, configuration, and business entities
- **Contains**: Config structs, enums for choices, data transfer objects
- **Rules**: Pure data structures, minimal logic, serializable when needed

#### Views (`src/views/`)
- **Purpose**: User interface and presentation logic
- **Contains**: Console output formatting, menu displays, input handlers
- **Rules**: No business logic, only presentation and user interaction

#### Controllers (`src/controllers/`)
- **Purpose**: Business logic coordination and application flow
- **Contains**: Application controller, feature controllers, CLI handling
- **Rules**: Orchestrates models and views, contains business logic

### MVC Benefits
- **Separation of Concerns**: Clear boundaries between data, presentation, and logic
- **Testability**: Each layer can be tested independently
- **Maintainability**: Changes in one layer don't cascade to others
- **Extensibility**: Easy to add new features or modify existing ones

## Coding Standards

### Naming Conventions
- Use `snake_case` for functions, variables, and modules
- Use `PascalCase` for types, structs, enums
- Use `SCREAMING_SNAKE_CASE` for constants
- Use descriptive names that clearly indicate purpose

### Language & Spelling Standards
- **Use English (GB) spelling** throughout the codebase for consistency with Rust ecosystem
- Apply GB spelling to variable names, function names, file names, comments, and documentation
- Common examples:
  - `visualisation` not `visualization`
  - `analyse` not `analyze` 
  - `colour` not `color`
  - `optimisation` not `optimization`
  - `initialise` not `initialize`
  - `realise` not `realize`
  - `organise` not `organize`
  - `centralise` not `centralize`

### Error Handling
- Use `Result<T, E>` for fallible operations
- Define custom error types using `thiserror`
- Propagate errors using `?` operator when appropriate
- Handle errors at appropriate boundaries

#### Specific Error Types
- `Error::Input(String)` - User input errors (empty input, invalid format)
- `Error::Validation(String)` - Data validation errors (out of range, invalid values)
- `Error::NotFound(String)` - Resource not found errors (missing files, unknown algorithms)
- `Error::Generic(String)` - General application errors
- `Error::Io(std::io::Error)` - File I/O errors (transparent)
- `Error::ParseInt(std::num::ParseIntError)` - Integer parsing errors (transparent)

#### Error Helper Methods
```rust
Error::input("Target word cannot be empty")
Error::validation("Array size must be greater than 0")
Error::not_found("File not found: data.txt")
Error::generic("Unexpected algorithm failure")
```

### Performance Considerations
- Use appropriate data structures for the task
- Prefer `Vec<T>` over `LinkedList<T>` for most use cases
- Use `HashMap` for O(1) lookups
- Use `BTreeMap` when ordered iteration is needed
- Consider memory allocation patterns
- Use `&str` for string slices, `String` for owned strings

### Memory Management
- Prefer borrowing (`&T`) over ownership when possible
- Use `Cow<str>` for conditionally borrowed strings
- Be mindful of unnecessary clones
- Use `Box<T>` for heap allocation when stack won't suffice

## Data Structures Guidelines

### For Algorithm Implementation
- Use `Vec<i32>` for sorting algorithm inputs for consistency
- Use `Vec<String>` for search algorithm inputs
- Use `Grid` with `Vec<Vec<CellType>>` for pathfinding algorithm inputs
- Use `HashMap<String, usize>` for hash-based search implementations
- Use `VecDeque<SortStep>` for visualisation step recording
- Use `Vec<Position>` for pathfinding results and path representation
- Consider using `BTreeMap` when deterministic ordering is needed for reproducible results

### Memory Efficiency
- Use `&[T]` slices instead of `Vec<T>` in algorithm parameters when ownership isn't needed
- Use appropriate collection sizes with `with_capacity()` when size is known
- Minimize allocations in algorithm hot paths for better performance
- Use `Box<T>` for tree-based algorithms (like tree sort) to manage heap allocations

## Algorithm Implementation

### Sorting Algorithm Principles
- Implement proper performance tracking (comparisons, swaps, memory allocations)
- Use the `PerformanceCounter` struct for consistent metrics
- Handle edge cases (empty arrays, single elements, duplicates)
- Maintain algorithm stability properties where applicable
- Implement in-place sorting when possible for memory efficiency

### Search Algorithm Principles
- Return both search result and performance metrics (comparisons, duration)
- Handle sorted vs unsorted data requirements appropriately
- Implement proper bounds checking and edge case handling
- Use appropriate data structures for each algorithm type (arrays, hash maps)

### Pathfinding Algorithm Principles
- Use `Grid` structure with `Position` coordinates for consistent spatial representation
- Return both path result and performance metrics (nodes explored, comparisons, duration)
- Handle edge cases (no path exists, start equals end, invalid positions)
- Use `PerformanceCounter` for consistent metrics tracking (nodes explored, frontier size, comparisons)
- Implement proper neighbor validation (bounds checking, obstacle avoidance)
- Support different heuristics (Manhattan distance, Euclidean distance)

### Visualization Integration
- Use `GuiPerformanceCounter` for recording visualisation steps
- Implement context ranges for recursive algorithms (purple highlighting)
- Record comparisons (red highlighting) and swaps (green highlighting) appropriately
- Ensure consistent colour scheme across all algorithm visualisations



## Documentation Standards

### Code Documentation
- Document all public APIs with `///`
- Include examples in documentation
- Explain algorithmic complexity where relevant
- Document safety requirements for unsafe code

### README and Docs
- Maintain clear project documentation
- Include build and run instructions
- Document algorithm time and space complexities
- Include performance benchmark results and visualisation features

## Dependencies

### Core Dependencies
- `rand` - Random number generation
- `serde` - Serialization for saving/loading models
- `clap` - Command line argument parsing
- `thiserror` - Error handling
- `tokio` - Async runtime
- `prettytable-rs` - Table formatting for benchmarks
- `rayon` - Parallel processing

### GUI Dependencies
- `gif` - GIF generation for algorithm visualisation
- `plotters` - Visualisation and rendering support

### Development Dependencies
- `criterion` - Benchmarking
- `proptest` - Property-based testing
- `tempfile` - Temporary files for testing

## GUI Architecture

### Centralized GUI Module (`src/gui/`)
All GUI and visualisation functionality is centralized in the `gui` module:

#### GUI Components
- **sorting.rs**: Core sorting visualisation logic, step recording, and GIF generation
- **visualisation.rs**: Algorithm-specific GUI runners and entry points  
- **renderer.rs**: Frame rendering utilities and static image generation

#### GUI Design Principles
- **Always Available**: GUI functionality is always available without feature gates
- **Modular**: Separate concerns (step recording, frame rendering, algorithm coordination)
- **Performance**: Efficient frame generation with consistent scaling and memory usage
- **User-friendly**: Clear progress indication, error handling, and colour-coded visualisation
- **GIF-Only**: Only generates animated GIF files to show algorithm operation stages
- **Consistent**: Standardized colour scheme
  - **Sorting**: purple=context, red=comparison, green=swap, blue=default
  - **Pathfinding**: blue=open, black=blocked, purple=context, red=comparison, green=path

### GUI Integration
- Controllers coordinate between CLI/interactive modes and GUI
- Views handle GUI-related user input (algorithm selection, format choice)
- Models define GUI configuration and parameters

## Performance Guidelines

### Optimization Priorities
1. Algorithm efficiency (Big O complexity)
2. Memory usage patterns
3. CPU cache locality
4. Minimizing allocations in hot paths
5. GUI rendering performance (frame rate, memory usage)

### Profiling
- Use `cargo flamegraph` for CPU profiling
- Use `valgrind` or similar for memory analysis
- Benchmark critical paths with `criterion`

## Git and Version Control

### Commit Messages
- Use conventional commit format
- Include scope: `feat(sort): add heap sort implementation` or `fix(gui): correct colour scheme in visualisation`
- Keep commits focused and atomic

### Branching Strategy
- Use feature branches for new functionality
- Keep main branch stable
- Use descriptive branch names

## Build Configuration

### Cargo.toml Best Practices
- Specify exact versions for dependencies  
- Always include GUI dependencies (no feature gates needed)
- Configure release profile for optimisation
- Include metadata fields (description, license, etc.)

### Simple Build Process
```bash
# Development with debug info
cargo run

# Release build for performance benchmarking
cargo run --release

# All GUI functionality available by default - no feature flags needed
```

### Release Configuration
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

This ensures optimal performance for sorting algorithms, search algorithms, pathfinding algorithms, tree traversal algorithms, and visualisation rendering.

## MVC Implementation Guidelines

### Controller Best Practices
- Keep controllers focused on coordination, not implementation
- Use dependency injection for testability
- Handle all error cases gracefully
- Delegate presentation logic to views

### View Best Practices  
- No business logic in views
- Consistent error message formatting
- User input validation at the view layer
- Reusable UI components

### Model Best Practices
- Immutable when possible
- Clear validation rules
- Serializable for configuration persistence
- Type-safe enums for choices and states

### Algorithm Implementation Guidelines
- **Sorting Algorithms**: Implement using consistent `PerformanceCounter` interface
- **Search Algorithms**: Return tuple of (found: bool, comparisons: usize) 
- **Pathfinding Algorithms**: Return tuple of (path: Vec<Position>, counter: PerformanceCounter)
- **Tree Traversal Algorithms**: Return tuple of (path: Vec<NodeValue>, counter: PerformanceCounter)
- **Performance Tracking**: Count comparisons, swaps, memory allocations, nodes explored accurately
- **Edge Cases**: Handle empty inputs, single elements, duplicate values, and unreachable goals properly
- **Visualization**: Use `GuiPerformanceCounter` for step recording in GUI wrapper functions
- **Grid Operations**: Validate positions, handle obstacles, implement proper neighbor checking

### Standardized Menu Structure
All algorithm types follow a consistent menu structure:
1. **Algorithm Information** - Display algorithm complexities, properties, and use cases (learn first!)
2. **Run Complete Benchmark Suite** - Execute all algorithms with performance benchmarking
3. **GUI Visualisation** - Generate animated GIF visualisations showing algorithm operation stages
b. **Back** - Return to main menu

Menu Navigation:
- Use numbers (1-3) for quick selection
- Accept algorithm names directly in visualization menus (e.g., "bubble", "merge", "astar")
- Support "all" or "a" to run all algorithms in visualization menus
- Support "back" or "b" to return to previous menu

### User Input Standards
All algorithm selection inputs throughout the application follow a consistent, simplified structure:

#### Input Format Rules
- **Numbers**: Accept numeric choices (1, 2, 3, etc.) for quick selection
- **Simple Names**: Accept basic algorithm names without variations (e.g., "bubble", "merge", "quick")
- **All Option**: Accept "a", "A", or "all" for running all algorithms
- **Case Insensitive**: All text inputs are converted to lowercase for matching

#### Sorting Algorithm Input
- **Numbers**: 1-13 corresponding to the algorithm list
- **Names**: `bubble`, `insertion`, `selection`, `merge`, `quick`, `heap`, `shell`, `tim`, `tree`, `bucket`, `radix`, `counting`, `cube`
- **All**: `a`, `A`, or `all`
- **Back**: `b`, `B`, or `back` (returns to sort menu)

#### Search Algorithm Input  
- **Numbers**: 1-6 corresponding to the algorithm list
- **Names**: `linear`, `binary`, `hash`, `interpolation`, `exponential`, `jump`
- **All**: `a`, `A`, or `all`
- **Back**: `b`, `B`, or `back` (returns to search menu)

#### Pathfinding Algorithm Input
- **Numbers**: 1-5 corresponding to the algorithm list  
- **Names**: `astar`, `dijkstra`, `bfs`, `dfs`, `greedy`
- **All**: `a`, `A`, or `all`
- **Back**: `b`, `B`, or `back` (returns to pathfinder menu)

#### Tree Traversal Algorithm Input
- **Numbers**: 1-4 corresponding to the algorithm list
- **Names**: `preorder`, `inorder`, `postorder`, `levelorder`
- **All**: `a`, `A`, or `all`
- **Back**: `b`, `B`, or `back` (returns to tree traversal menu)

#### Rejected Input Variations
Do not accept complex variations or shortcuts:
- No hyphenated versions: ~~`bubble-sort`~~, ~~`merge-sort`~~
- No compound versions: ~~`bubblesort`~~, ~~`mergesort`~~  
- No abbreviations: ~~`qsort`~~, ~~`msort`~~, ~~`bst`~~
- No full descriptions: ~~`bubble sort`~~, ~~`quick sort`~~

#### Input Validation
- Use `SortAlgorithm::from_str()`, `SearchAlgorithm::from_str()`, `PathfinderAlgorithm::from_str()` for parsing
- Provide clear error messages with examples of valid input
- Show helpful hints about accepted input formats

#### CLI Integration
- Interactive mode algorithm selection follows the same input rules
- CLI subcommands use structured arguments (no algorithm selection by name in CLI)
- All interactive prompts maintain consistent input validation
- Error messages provide context-specific examples

### GUI Integration Guidelines
- GUI functionality is always available and no feature gates are needed
- Separate rendering logic from core algorithm logic
- Efficient memory usage for visualisation with large datasets
- Always generate animated GIF files to show algorithm operation stages
- Implement consistent colour coding across all algorithm visualisations
