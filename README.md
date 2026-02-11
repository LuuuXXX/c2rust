# c2rust

A unified command-line tool for C to Rust translation and build management.

## Overview

This tool integrates five c2rust subcommands into a single CLI interface:

- **init** - Initialize a c2rust project structure
- **build** - Execute C project build commands
- **test** - Execute C project test commands
- **clean** - Clean build artifacts
- **translate** - Translate C code to Rust

## Installation

### Prerequisites

1. Install Rust and Cargo (https://rustup.rs/)
2. Install the c2rust subtools in your `$C2RUST_HOME/bin/` directory:
   - c2rust-init
   - c2rust-build
   - c2rust-test
   - c2rust-clean
   - c2rust-translate

### Building from Source

```bash
git clone https://github.com/LuuuXXX/c2rust.git
cd c2rust
cargo build --release
```

The binary will be available at `target/release/c2rust`.

### Environment Setup

Set the `C2RUST_HOME` environment variable to point to your c2rust installation:

```bash
export C2RUST_HOME=/path/to/c2rust
```

Add this to your `~/.bashrc` or `~/.zshrc` to make it permanent:

```bash
echo 'export C2RUST_HOME=/path/to/c2rust' >> ~/.bashrc
source ~/.bashrc
```

## Usage

### General Syntax

```bash
c2rust <COMMAND> [OPTIONS]
```

### Commands

#### 1. Initialize Project

Initialize a `.c2rust` directory structure:

```bash
c2rust init
```

#### 2. Build

Execute build commands for your C project:

```bash
c2rust build -- make
c2rust build --feature <name> -- make all
c2rust build --no-interactive -- cmake --build .
```

**Options:**
- `--feature <name>` - Specify a feature name
- `--no-interactive` - Run in non-interactive mode
- `-- <build_cmd>` - Build command to execute (required)

#### 3. Test

Execute test commands for your C project:

```bash
c2rust test -- make test
c2rust test --feature <name> -- ctest
```

**Options:**
- `--feature <name>` - Specify a feature name
- `-- <test_cmd>` - Test command to execute (required)

#### 4. Clean

Execute clean commands to remove build artifacts:

```bash
c2rust clean -- make clean
c2rust clean --feature <name> -- rm -rf build/
```

**Options:**
- `--feature <name>` - Specify a feature name
- `-- <clean_cmd>` - Clean command to execute (required)

#### 5. Translate

Translate C code to Rust:

```bash
c2rust translate
c2rust translate --feature <name>
c2rust translate --allow-all
c2rust translate --max-fix-attempts 20
c2rust translate --show-full-output
```

**Options:**
- `--feature <name>` - Specify a feature name (default: "default")
- `--allow-all` - Allow all unsafe operations
- `--max-fix-attempts <n>` - Maximum number of fix attempts (default: 10)
- `--show-full-output` - Show full output during translation

## Examples

### Basic Workflow

```bash
# 1. Initialize a c2rust project
c2rust init

# 2. Build your C project
c2rust build -- make

# 3. Run tests
c2rust test -- make test

# 4. Translate to Rust
c2rust translate

# 5. Clean up
c2rust clean -- make clean
```

### Advanced Usage

```bash
# Build with a specific feature
c2rust build --feature experimental -- make all

# Translate with custom settings
c2rust translate --feature advanced --max-fix-attempts 20 --allow-all

# Non-interactive build
c2rust build --no-interactive -- cmake --build .
```

## Error Handling

### C2RUST_HOME Not Set

If you see this error:

```
Error: C2RUST_HOME environment variable is not set.
Please set C2RUST_HOME to the root directory of your c2rust installation.
Example: export C2RUST_HOME=/path/to/c2rust
```

Set the environment variable:

```bash
export C2RUST_HOME=/path/to/c2rust
```

### Tool Not Found

If you see this error:

```
Error: Tool 'c2rust-<command>' not found at path: /path/to/c2rust/bin/c2rust-<command>
Please ensure the tool is installed in $C2RUST_HOME/bin/
```

Ensure that the required subtools are installed in `$C2RUST_HOME/bin/`.

## Development

### Running Tests

```bash
cargo test
```

### Building for Development

```bash
cargo build
./target/debug/c2rust --help
```

### Building for Release

```bash
cargo build --release
./target/release/c2rust --help
```

## Related Projects

- [c2rust-init](https://github.com/LuuuXXX/c2rust-init) - Initialize c2rust project structure
- [c2rust-build](https://github.com/LuuuXXX/c2rust-build) - Build C projects
- [c2rust-test](https://github.com/LuuuXXX/c2rust-test) - Test C projects
- [c2rust-clean](https://github.com/LuuuXXX/c2rust-clean) - Clean build artifacts
- [c2rust-translate](https://github.com/LuuuXXX/c2rust-translate) - Translate C to Rust

## License

This project is part of the c2rust ecosystem.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.