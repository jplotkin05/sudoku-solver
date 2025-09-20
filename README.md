# Sudoku Solver 🧩

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org/)
[![Build](https://github.com/jplotkin05/sudoku-solver/actions/workflows/rust.yml/badge.svg)](https://github.com/jplotkin05/sudoku-solver/actions)

A Sudoku solver implemented in Rust.

---

## 📖 Table of Contents

- [Overview](#overview)  
- [Features](#features)  
- [Motivation](#motivation)  
- [Installation](#installation)  
- [Usage](#usage)  
  - [Building](#building)  
  - [Running](#running)  
- [Examples](#examples)  
- [Testing](#testing)  
- [Roadmap](#roadmap)  
- [Contributing](#contributing)  
- [Acknowledgements](#acknowledgements)  
- [License](#license)  

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
