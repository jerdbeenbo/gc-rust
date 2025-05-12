# GC-Rust: A Mark and Sweep Garbage Collector Demo in Rust

## Overview

GC-Rust is an educational project demonstrating a barebones mark-and-sweep garbage collector implemented in Rust. This project was created to explore and contrast Rust's ownership and borrowing model with traditional garbage collection memory management approaches.

## Purpose

This is **not** a production-ready garbage collector. Rather, it's a simplified demonstration that sits on top of Rust and manages a "Virtual Heap":

- Creates a controlled memory environment (a "memory pool" using a Vec of struct Cells)
- Implements basic memory allocation and deallocation
- Tracks references between objects
- Identifies and reclaims unreachable memory through mark-and-sweep collection

The primary goal is educational: to understand the conceptual differences between automatic garbage collection and Rust's compile-time ownership system.

## Limitations

- Works only with i32 primitive values (no complex objects)
- Contains a bounded memory (fixed to 20 positions avaliable on the virtual heap) pool rather than dynamically expanding memory
- Operates as a simulation alongside Rust's own memory management, rather than replacing it
- Reference patterns are mostly artificial/manual, rather than occurring organically through program execution, as this software exisits for demonstration / research purposes only -> not a production ready gc.

## How It Works

The implementation includes:

1. **Memory Cells**: Each cell contains data, reference count, status flags, and references to other cells
2. **Memory Pool**: A fixed-size vector of memory cells
3. **Root Management**: Designating specific cells as "roots" (starting points for garbage collection)
4. **Reference Creation**: Building connections between memory cells
5. **Mark Phase**: Identifying cells that are no longer reachable
6. **Sweep Phase**: Reclaiming unreachable memory cells

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system
- The rand crate for random value generation <- only dependency, which exisits to populate arbitrary `<i32>` data

### Installation

1. Clone the repository:
```
git clone https://github.com/jerdbeenbo/gc-rust.git
cd gc-rust
```

2. Build the project:
```
cargo build --release
```

3. Run the program:
```
cargo run --release
```

## Usage Guide

The program provides an interactive command-line interface. Here are the available commands:

- `--help`: Display a list of available commands
- `--root <pos1> <pos2>`: Designate two cells as roots (entry points for collection)
- `--unroot`: Remove root status from all cells
- `--arb_ref <amount>`: Create arbitrary references from roots to new cells with derived values
- `--link_ref <pos1> <pos2>`: Create reference where pos1 will reference pos2, and pos2 will be referenced by pos1
- `--alloc_at <pos>`: Allocate data at this particular position in memory 
- `--state`: Display the current state of all memory cells
- `--populate`: Fill remaining free cells with data (to demonstrate garbage collection)
- `--gc`: Run the garbage collector (mark and sweep phases)
- `--exit`: End the program

### Simple Example Workflow

1. Start by setting up roots: `--root 0 19` (sets the first and last cells as roots)
2. Create arbitrary references: `--arb_ref 3` (creates 3 cells that reference the roots)
3. Populate remaining cells: `--populate` (fills unused cells with unreferenced data)
4. View the memory state: `--state` (see which cells contain data and references)
5. Run garbage collection: `--gc` (watch as unreferenced cells get reclaimed)
6. View the results: `--state` (see which cells were freed)

## Educational Value

This project helps illustrate:
- How garbage collectors identify and reclaim memory
- The runtime costs associated with garbage collection
- The conceptual differences between GC and Rust's ownership model
- Tradeoffs in memory management approaches
- How reference tracking works in memory management systems

## License

[MIT License]

## Author

Jarred Jenkins (https://github.com/jerdbeenbo)
