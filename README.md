# VIM 4 C++

A modern, terminal-based code editor specifically designed for C++ programming, combining the power of VIM-like controls with intelligent code suggestions and snippets.

## Features

### 🎨 Modern Interface
- Clean, minimalist terminal UI with syntax highlighting
- Real-time clock display
- Mode indicator (Normal/Insert/Command)
- Customizable color scheme optimized for coding

### ⌨️ VIM-Style Controls
- Modal editing with Normal, Insert, and Command modes
- Familiar VIM navigation (h,j,k,l)
- Command mode operations (:w, :q, :wq)
- Efficient cursor movement and text manipulation

### 🚀 Smart Editing Features
- Auto-completion for brackets and parentheses
- Intelligent code suggestions for:
  - C++ keywords
  - Custom snippets
  - Context-aware completions
- Tab-trigger completion system

### 📝 Code Snippets
Built-in snippets for common C++ patterns:

#### Fast I/O Templates
- `fio1`: Basic fast I/O setup
- `fio2`: Buffered input optimization
- `fio3`: Complete FastIO class implementation

#### Algorithms
- Sorting:
  - `bubblesort`: Bubble Sort implementation
  - `mergesort`: Merge Sort implementation
  - `quicksort`: Quick Sort implementation
  - `heapsort`: Heap Sort implementation
- Searching:
  - `binarysearch`: Binary Search implementation
  - `lowerbound`: Lower Bound implementation

#### Data Structures
- `dsu`: Disjoint Set Union implementation
- `segtree`: Segment Tree implementation

#### Graph Algorithms
- `dfs`: Depth First Search
- `bfs`: Breadth First Search
- `dijkstra`: Dijkstra's Shortest Path

### 📁 File Operations
- Open files directly from command line
- Save files with :w command
- Automatic file backup (coming soon)

## Installation

### Clone the repository
```
git clone https://github.com/your-username/vim4cpp.git
```

### Build the project
```
cd vim4cpp
cargo build --release
```

### Run the project
```
cargo run --release
```

## Usage

### Basic Commands
- `i`: Enter Insert mode
- `Esc`: Return to Normal mode
- `:w`: Save file
- `:q`: Quit
- `:wq` or `:x`: Save and quit

### Navigation
- `h`: Move left
- `j`: Move down
- `k`: Move up
- `l`: Move right

### Code Completion
- Type at least 2 characters to trigger suggestions
- Use `Tab` to cycle through suggestions
- Press `Enter` to accept suggestion

## Future Scope

### Planned Features
1. **Enhanced Language Support**
   - Extended C++ syntax highlighting
   - Support for other languages (Python, Rust, etc.)
   - Language-specific snippets

2. **IDE-like Features**
   - Integrated terminal
   - Build system integration
   - Error highlighting
   - Jump to definition

3. **Customization**
   - User-defined snippets
   - Configurable keybindings
   - Theme customization
   - Plugin system

4. **Advanced Features**
   - Split window support
   - Integrated Git support
   - Code folding
   - Multiple cursors

5. **Performance Optimizations**
   - Improved file handling for large files
   - Async operations
   - Memory optimization

### Community Goals
- Build a plugin ecosystem
- Create a snippet marketplace
- Develop language server protocol support
- Implement remote development capabilities

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.