# Setting Up JanusLens Development Environment

This guide provides detailed steps to set up a fresh development environment for the JanusLens project. Follow these instructions carefully to ensure a smooth setup process.

## Prerequisites

Before you begin, make sure you have the following tools installed on your system:

### Required Tools

1. **Git** - For version control
   - [Download Git](https://git-scm.com/downloads)
   - Verify installation: `git --version`

2. **Rust and Cargo** - For the backend
   - Install using [rustup](https://rustup.rs/)
   - Verify installation: `rustc --version` and `cargo --version`

3. **Node.js and npm** - For the frontend
   - [Download Node.js](https://nodejs.org/) (version 16.x or higher)
   - Verify installation: `node --version` and `npm --version`

4. **Tauri CLI** - For building the application
   - Install using Cargo: `cargo install tauri-cli`
   - Verify installation: `cargo tauri --version`

### Recommended Tools

- **Visual Studio Code** - Recommended IDE
  - [Download VS Code](https://code.visualstudio.com/)
  - Recommended extensions:
    - Rust Analyzer
    - Svelte for VS Code
    - Tauri
    - ESLint
    - Prettier

## Step-by-Step Setup Guide

### 1. Clone the Repository

```bash
# Clone the repository
git clone https://github.com/YOUR-USERNAME/januslens.git

# Navigate to the project directory
cd januslens
```

### 2. Install Dependencies

```bash
# Install npm dependencies
npm install
```

### 3. Build Icons (if necessary)

If you encounter issues with icons during development:

```bash
# Navigate to the Tauri directory
cd src-tauri

# Verify icons are present
ls -la icons/
```

Make sure all the required icon files are present as specified in `tauri.conf.json`.

### 4. Run the Development Server

```bash
# Start the development server
npm run tauri dev
```

This will:
1. Start the Vite development server for the frontend
2. Build the Rust backend
3. Launch the application in development mode

### 5. Troubleshooting Common Setup Issues

#### Rust Compilation Errors

If you encounter errors related to missing Rust dependencies:

```bash
# Navigate to the Tauri directory
cd src-tauri

# Update Rust dependencies
cargo update
```

#### Missing Icons

If you encounter issues with icons:

```bash
# Compare your icon files with the configuration
cat tauri.conf.json | grep "icon"
```

Ensure that the icon paths specified in `tauri.conf.json` match the actual paths in your project.

#### Frontend Build Issues

If you encounter issues with the frontend build:

```bash
# Check for TypeScript errors
npx tsc --noEmit
```

#### Tauri CLI Issues

If you encounter issues with the Tauri CLI:

```bash
# Update Tauri CLI
cargo install tauri-cli --force
```

## Verifying Your Setup

To verify that your setup is working correctly:

1. The application should start without errors when running `npm run tauri dev`
2. You should see the JanusLens UI in the application window
3. The developer tools console should be free of errors

## Next Steps

Once your environment is set up:

1. Read the [Developer Guide](DEVELOPER.md) for information on the project structure and development workflow
2. Check the [Contributing Guidelines](../CONTRIBUTING.md) if you plan to contribute to the project
3. Review the [Project Plan](PROJECT_PLAN.md) to understand the roadmap

## Getting Help

If you encounter any issues during setup:

1. Check the [Common Issues & Troubleshooting](DEVELOPER.md#common-issues--troubleshooting) section
2. Open an issue on GitHub with detailed information about your problem
3. Reach out to the community for assistance

---

Happy coding with JanusLens! 