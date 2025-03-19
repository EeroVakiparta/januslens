# JanusLens: Git Visualization Tool

A lightweight, high-performance Git visualization tool built with Tauri and Rust, offering GitKraken-like functionality with superior performance for large repositories.

## 🔍 Project Overview

JanusLens (named after the Roman god Janus who simultaneously looks to the past and future) provides intuitive visualization of code history and branches. This project aims to be a viable alternative to GitKraken, focusing on core features while providing exceptional performance.

## ⚡ Key Features (MVP)

- **Repository Visualization**: Interactive, performance-optimized Git graph
- **Advanced Diff Viewer**: Side-by-side comparison with syntax highlighting
- **Commit Management**: Intuitive staging and commit interface
- **Branch/Merge Handling**: Visual branch and merge operations
- **Large Repository Support**: Optimized for performance with enormous codebases

## 🛠️ Technology Stack

- **Backend**: Rust + git2 library for high-performance Git operations
- **Framework**: Tauri (Rust-based alternative to Electron with significantly lower memory footprint)
- **Frontend**: Svelte with TypeScript for a reactive UI
- **Visualization**: D3.js for branch graphs

## 💪 Technical Advantages

- **Memory Efficient**: ~10-30x less memory usage than Electron apps
- **Performance**: Rust backend handles heavy Git operations outside the UI thread
- **Smart Loading**: Pagination, virtualization, and chunked processing for large repos
- **Native Integration**: Direct Git operations via Rust's git2 library

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (16+)
- [npm](https://www.npmjs.com/) (8+)

### Installation

```bash
# Clone the repository
git clone https://github.com/YOUR-USERNAME/januslens.git
cd januslens

# Install dependencies
npm install

# Run the development version
npm run tauri dev
```

### Building for Production

```bash
# Create a production build
npm run tauri build
```

## 📖 Documentation

- [Setup Guide](docs/SETUP.md) - Complete instructions for setting up the development environment
- [Developer Guide](docs/DEVELOPER.md) - Detailed development instructions
- [Architecture Overview](docs/ARCHITECTURE.md) - Overview of the application architecture
- [Contributing Guidelines](CONTRIBUTING.md) - How to contribute to the project
- [Project Plan](docs/PROJECT_PLAN.md) - Project roadmap and development phases
- [Changelog](CHANGELOG.md) - Record of all notable changes to the project

## 💻 Target Platforms

- macOS
- Windows
- Linux (experimental)

## 📝 License

TBD

---

Built with ❤️ to solve the need for a lightweight, performant Git visualization tool. 