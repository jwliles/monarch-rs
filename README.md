# MONARCH - Advanced Git Management Suite

A powerful platform for managing multiple Git repositories concurrently with a unified interface, intelligent hook management, and advanced visualization.

## Features

- **Multi-Repository Management**: Concurrent operations across multiple repositories
- **Status-at-a-Glance**: Visual monitoring of repository status with customizable indicators
- **Command Sequencing**: Chain Git commands into visual pipelines
- **Intelligent Error Handling**: Contextual suggestions for resolving Git errors
- **Visual Hook Builder**: Create and manage Git hooks with a user-friendly interface
- **Cross-Repository Analysis**: Track patterns and activity across your repositories

## Installation

### Prerequisites

- Rust 2024 edition and toolchain
- Git command-line tool

### Build from source

```bash
git clone https://github.com/yourusername/monarch-rs.git
cd monarch-rs
cargo build --release
```

The compiled binary will be available at `target/release/monarch`.

## Usage

```bash
monarch [OPTIONS] <OPERATION> <SOURCE_DIR> [TARGET_DIR]
```

### Operations

- **Clone**: Clone repositories from source directory to target directory
- **Pull**: Pull latest changes for all repositories in source directory
- **Push**: Push local changes for all repositories in source directory
- **Status**: List all repositories and their status
- **Run**: Run arbitrary git command on all repositories
- **Hook**: Manage Git hooks across repositories

### Options

- `--verbose`: Enable verbose logging
- `--reverse`: Reverse the direction of operations (e.g., clone from target to source)
- `--filter`: Filter repositories by name pattern

### Examples

Clone all repositories from one directory to another:

```bash
monarch clone ~/projects ~/backup
```

With verbose output:

```bash
monarch --verbose clone ~/projects ~/backup
```

List all repositories and their status:

```bash
monarch status ~/projects
```

Run a git command on all repositories:

```bash
monarch run ~/projects -- branch -a
```

### Hook Management

List hooks for all repositories:

```bash
monarch hook list ~/projects
```

Create a new hook across all repositories:

```bash
monarch hook create ~/projects --hook-type pre-commit --script-path ~/myhooks/pre-commit.sh
```

Install a sample hook across all repositories:

```bash
monarch hook install ~/projects --hook-type pre-commit
```

Remove a hook from all repositories:

```bash
monarch hook remove ~/projects --hook-type pre-commit
```

## Development

### Project Structure

- `src/`
  - `main.rs`: Entry point
  - `args.rs`: Command-line argument parsing
  - `git/`: Git operations and repository management
  - `operations/`: Implementation of supported operations
    - `hook.rs`: Git hook management operations
  - `utils/`: Utility functions

### Running Tests

```bash
cargo test
```

## Requirements

- Rust (Minimum supported version: 1.75.0)
- GNU/Linux or other free operating system
- Git command-line tool
- Standard system libraries

**Note**: Monarch is developed exclusively for free operating systems. It is not officially tested or supported on proprietary platforms.

## License

This project is licensed under the terms found in the LICENSE file.