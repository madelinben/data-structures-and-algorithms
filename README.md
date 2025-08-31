# Data Structures and Algorithms in Rust

Rust implementation of data structure principles and algorithms, with performance benchmarking and educational visualisation.

## Usage

```bash
# Interactive menu (default)
cargo run

# Run specific sorting algorithm
cargo run -- sort <algorithm> --size <n>

# Run sorting benchmark comparison
cargo run -- sort benchmark --size <n>

# Run specific search algorithm  
cargo run -- search <algorithm> --target <word>

# Run search benchmark comparison
cargo run -- search benchmark --target <word>

# Generate algorithm visualizations (requires --features gui)
cargo run --features gui -- visualize <algorithm> --size <n>
cargo run --features gui -- visualize all --size <n>
```

**Available sorting algorithms:**
`bubble`, `insertion`, `selection`, `merge`, `quick`, `heap`, `shell`, `tim`, `tree`, `bucket`, `radix`, `counting`, `cube`

**Available search algorithms:**
`linear`, `binary`, `hash`, `interpolation`, `jump`, `exponential`

## Algorithm Complexity

### Sorting Algorithms Complexity
| Algorithm | Best Case | Average Case | Worst Case | Space | Stable | In-Place |
|-----------|-----------|--------------|------------|-------|--------|----------|
| Bubble Sort | O(n) | O(n²) | O(n²) | O(1) | ✓ | ✓ |
| Insertion Sort | O(n) | O(n²) | O(n²) | O(1) | ✓ | ✓ |
| Selection Sort | O(n²) | O(n²) | O(n²) | O(1) | ✗ | ✓ |
| Merge Sort | O(n log n) | O(n log n) | O(n log n) | O(n) | ✓ | ✗ |
| Quick Sort | O(n log n) | O(n log n) | O(n²) | O(log n) | ✗ | ✓ |
| Heap Sort | O(n log n) | O(n log n) | O(n log n) | O(1) | ✗ | ✓ |
| Shell Sort | O(n log n) | O(n^1.25) | O(n²) | O(1) | ✗ | ✓ |
| Tim Sort | O(n) | O(n log n) | O(n log n) | O(n) | ✓ | ✗ |
| Tree Sort | O(n log n) | O(n log n) | O(n²) | O(n) | ✓ | ✗ |
| Bucket Sort | O(n + k) | O(n + k) | O(n²) | O(n + k) | ✓ | ✗ |
| Radix Sort | O(d × n) | O(d × n) | O(d × n) | O(n + k) | ✓ | ✗ |
| Counting Sort | O(n + k) | O(n + k) | O(n + k) | O(k) | ✓ | ✗ |

### Search Algorithms Complexity
| Algorithm | Best Case | Average Case | Worst Case | Space | Prerequisite |
|-----------|-----------|--------------|------------|-------|--------------|
| Linear Search | O(1) | O(n) | O(n) | O(1) | None |
| Binary Search | O(1) | O(log n) | O(log n) | O(1) | Sorted data |
| Hash Search | O(1) | O(1) | O(n) | O(n) | Hash table |
| Interpolation Search | O(1) | O(log log n) | O(n) | O(1) | Uniform distribution |
| Jump Search | O(1) | O(√n) | O(√n) | O(1) | Sorted data |
| Exponential Search | O(1) | O(log n) | O(log n) | O(1) | Sorted data |