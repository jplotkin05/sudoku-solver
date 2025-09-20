# Sudoku Solver 🧩

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Build](https://github.com/jplotkin05/sudoku-solver/actions/workflows/rust.yml/badge.svg)](https://github.com/jplotkin05/sudoku-solver/actions)

A Sudoku solver implemented with Rust.

---

## 📖 Table of Contents

- [Overview](#-overview)
- [Features](#-features)
- [Motivation](#-motivation)
- [Installation](#%EF%B8%8F-installation)
- [Usage](#-usage)
  - [Building](#building)
  - [Running](#running)
- [Examples](#-examples)
- [Testing](#-testing)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [Acknowledgements](#-acknowledgements)
- [License](#-license)

---

## 🔍 Overview

This repository provides a command-line Sudoku solver algorithm written in Rust.  
It takes a partially filled Sudoku board and attempts to solve it, yielding a completed board if the puzzle is valid and solvable.

---

## ✨ Features

- Pure Rust implementation  
- Efficient backtracking / search-based solver  
- Clean and minimal dependencies  
- Unit-tested logic  
- Simple CLI for running puzzles  

---

## 🎯 Motivation

The project was built to:

- Explore algorithmic problem solving in Rust  
- Practice ownership, borrowing, and efficient data structures  
- Provide a simple tool for solving Sudoku puzzles  

---

## ⚙️ Installation

Make sure you have Rust installed (via [rustup](https://rustup.rs/)).

Clone the repo:

```bash
git clone https://github.com/jplotkin05/sudoku-solver.git
cd sudoku-solver
```

---

## 🚀 Usage

### Building

Build the project in release mode for best performance:

```bash
cargo build --release
```

### Running

Run the solver with a Sudoku input file:

```bash
cargo run -- puzzles/example.txt
```

- Input must be a text file with the Sudoku grid written row by row.  
- Use `.` or `0` to represent blank cells.  
- The solver will print the solved board (or report if unsolvable).  

---

## 🖼 Examples

### Example Input (`puzzles/example.txt`):

```
5 3 . . 7 . . . .
6 . . 1 9 5 . . .
. 9 8 . . . . 6 .
8 . . . 6 . . . 3
4 . . 8 . 3 . . 1
7 . . . 2 . . . 6
. 6 . . . . 2 8 .
. . . 4 1 9 . . 5
. . . . 8 . . 7 9
```

### Example Run:

```bash
cargo run -- puzzles/example.txt
```

### Example Output:

```
5 3 4 6 7 8 9 1 2
6 7 2 1 9 5 3 4 8
1 9 8 3 4 2 5 6 7
8 5 9 7 6 1 4 2 3
4 2 6 8 5 3 7 9 1
7 1 3 9 2 4 8 5 6
9 6 1 5 3 7 2 8 4
2 8 7 4 1 9 6 3 5
3 4 5 2 8 6 1 7 9
```

---

## ✅ Testing

Run the unit tests:

```bash
cargo test
```

---

## 🛤 Roadmap

- [ ] Add difficulty classification (easy, medium, hard)  
- [ ] Benchmark performance on large sets of puzzles  
- [ ] Support for multiple solution strategies  
- [ ] Option to generate Sudoku puzzles  

---

## 🤝 Contributing

Contributions are welcome!  

1. Fork the repo  
2. Create your feature branch (`git checkout -b feature/my-feature`)  
3. Commit your changes (`git commit -m 'Add some feature'`)  
4. Push to the branch (`git push origin feature/my-feature`)  
5. Open a pull request  

---

## 🙏 Acknowledgements

- [Rust](https://www.rust-lang.org/) for making systems programming fun and safe  
- Classic Sudoku puzzle inspiration  

---

## 📄 License

This project is licensed under the MIT License.  
See the [LICENSE](LICENSE) file for details.
