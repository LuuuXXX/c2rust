# c2rust

A unified command-line tool for C to Rust translation and build management.

## Overview

This tool integrates six c2rust subcommands into a single CLI interface:

- **init** - Initialize a c2rust project structure
- **build** - Execute C project build commands
- **test** - Execute C project test commands
- **clean** - Clean build artifacts
- **translate** - Translate C code to Rust
- **merge** - Merge translated Rust modules (executed after translate)

## Installation

### Prerequisites

1. Install Rust and Cargo (https://rustup.rs/)
2. Install the c2rust subtools in your `$C2RUST_HOME/bin/` directory:
   - c2rust-init
   - c2rust-build
   - c2rust-test
   - c2rust-clean
   - c2rust-translate
   - c2rust-merge

### Building from Source

```bash
git clone https://github.com/LuuuXXX/c2rust.git
cd c2rust
cargo build --release
```

The binary will be available at `target/release/c2rust-xw` (or `c2rust-xw.exe` on Windows).

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
c2rust-xw <COMMAND> [OPTIONS]
```

### Commands

#### 1. Initialize Project

Initialize a `.c2rust` directory structure:

```bash
c2rust-xw init
```

#### 2. Build

Execute build commands for your C project:

```bash
c2rust-xw build -- make
c2rust-xw build --feature <name> -- make all
c2rust-xw build --no-interactive -- cmake --build .
```

**Options:**
- `--feature <name>` - Specify a feature name
- `--no-interactive` - Run in non-interactive mode
- `-- <build_cmd>` - Build command to execute (required)

#### 3. Test

Execute test commands for your C project:

```bash
c2rust-xw test -- make test
c2rust-xw test --feature <name> -- ctest
```

**Options:**
- `--feature <name>` - Specify a feature name
- `-- <test_cmd>` - Test command to execute (required)

#### 4. Clean

Execute clean commands to remove build artifacts:

```bash
c2rust-xw clean -- make clean
c2rust-xw clean --feature <name> -- rm -rf build/
```

**Options:**
- `--feature <name>` - Specify a feature name
- `-- <clean_cmd>` - Clean command to execute (required)

#### 5. Translate

Translate C code to Rust:

```bash
c2rust-xw translate
c2rust-xw translate --feature <name>
c2rust-xw translate --allow-all
c2rust-xw translate --max-fix-attempts 20
c2rust-xw translate --show-full-output
```

**Options:**
- `--feature <name>` - Specify a feature name (optional)
- `--allow-all` - Allow all unsafe operations
- `--max-fix-attempts <n>` - Maximum number of fix attempts (optional, uses translator's default if omitted)
- `--show-full-output` - Show full output during translation

#### 6. Merge

Merge translated Rust modules (executed after translate):

```bash
c2rust-xw merge
c2rust-xw merge --feature <name>
c2rust-xw merge <additional_args>
```

**Options:**
- `--feature <name>` - Specify a feature name (optional)

## Examples

### Basic Workflow

```bash
# 1. Initialize a c2rust project
c2rust-xw init

# 2. Build your C project
c2rust-xw build -- make

# 3. Run tests
c2rust-xw test -- make test

# 4. Translate to Rust
c2rust-xw translate

# 5. Merge translated modules
c2rust-xw merge

# 6. Clean up
c2rust-xw clean -- make clean
```

### Advanced Usage

```bash
# Build with a specific feature
c2rust-xw build --feature experimental -- make all

# Translate with custom settings
c2rust-xw translate --feature advanced --max-fix-attempts 20 --allow-all

# Merge with a specific feature
c2rust-xw merge --feature advanced

# Non-interactive build
c2rust-xw build --no-interactive -- cmake --build .
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

Note: On Windows, tool names include the `.exe` extension (e.g., `c2rust-init.exe`).

Ensure that the required subtools are installed in `$C2RUST_HOME/bin/`.

## Development

### Running Tests

```bash
cargo test
```

### Building for Development

```bash
cargo build
./target/debug/c2rust-xw --help
```

### Building for Release

```bash
cargo build --release
./target/release/c2rust-xw --help
```

## Related Projects

- [c2rust-init](https://github.com/LuuuXXX/c2rust-init) - Initialize c2rust project structure
- [c2rust-build](https://github.com/LuuuXXX/c2rust-build) - Build C projects
- [c2rust-test](https://github.com/LuuuXXX/c2rust-test) - Test C projects
- [c2rust-clean](https://github.com/LuuuXXX/c2rust-clean) - Clean build artifacts
- [c2rust-translate](https://github.com/LuuuXXX/c2rust-translate) - Translate C to Rust
- [c2rust-merge](https://github.com/LuuuXXX/c2rust-merge) - Merge translated Rust modules

## License

This project is part of the c2rust ecosystem.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.