use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;
use std::process::{Command, exit};

/// Returns the platform-specific dynamic library file extension.
/// - Linux: .so
/// - macOS: .dylib
/// - Windows: .dll
fn get_lib_extension() -> &'static str {
    if cfg!(target_os = "macos") {
        ".dylib"
    } else if cfg!(target_os = "windows") {
        ".dll"
    } else {
        ".so"
    }
}

/// Constructs a platform-specific library filename.
/// Examples:
/// - Linux: libhook -> libhook.so
/// - macOS: libhook -> libhook.dylib
/// - Windows: libhook -> libhook.dll
fn get_lib_filename(base_name: &str) -> String {
    format!("{}{}", base_name, get_lib_extension())
}


#[derive(Parser)]
#[command(name = "c2rust-xw")]
#[command(about = "C to Rust translation and build tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 初始化 c2rust 项目
    Init,
    
    /// 执行构建命令
    Build {
        #[arg(long)]
        feature: Option<String>,
        
        #[arg(long)]
        no_interactive: bool,
        
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
        build_cmd: Vec<String>,
    },
    
    /// 执行测试命令
    Test {
        #[arg(long)]
        feature: Option<String>,
        
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
        test_cmd: Vec<String>,
    },
    
    /// 执行清理命令
    Clean {
        #[arg(long)]
        feature: Option<String>,
        
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
        clean_cmd: Vec<String>,
    },
    
    /// 翻译 C 代码为 Rust
    Translate {
        #[arg(long)]
        feature: Option<String>,
        
        #[arg(long)]
        allow_all: bool,
        
        #[arg(long)]
        max_fix_attempts: Option<usize>,
        
        #[arg(long)]
        show_full_output: bool,
    },
    
    /// 合并翻译后的 Rust 模块
    Merge {
        #[arg(long)]
        feature: Option<String>,
        
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },
}

fn get_c2rust_home() -> Result<PathBuf, String> {
    match env::var("C2RUST_HOME") {
        Ok(home) => {
            let path = PathBuf::from(home);
            if !path.is_dir() {
                return Err(format!(
                    "Error: C2RUST_HOME is not a directory: {}\n\
Please set C2RUST_HOME to the root directory of your c2rust installation.\n\
Example: export C2RUST_HOME=/path/to/c2rust",
                    path.display()
                ));
            }
            let bin_dir = path.join("bin");
            if !bin_dir.is_dir() {
                return Err(format!(
                    "Error: C2RUST_HOME/bin is not a directory: {}\n\
Please ensure the bin directory exists in your c2rust installation.",
                    bin_dir.display()
                ));
            }
            Ok(path)
        }
        Err(_) => Err(
            "Error: C2RUST_HOME environment variable is not set.\n\
Please set C2RUST_HOME to the root directory of your c2rust installation.\n\
Example: export C2RUST_HOME=/path/to/c2rust".to_string()
        ),
    }
}

fn get_tool_path(tool_name: &str) -> Result<PathBuf, String> {
    let c2rust_home = get_c2rust_home()?;
    let tool_path = c2rust_home
        .join("bin")
        .join(format!("c2rust-{}{}", tool_name, std::env::consts::EXE_SUFFIX));
    
    if !tool_path.exists() {
        return Err(format!(
            "Error: Tool '{}' not found at path: {}\n\
Please ensure the tool is installed in $C2RUST_HOME/bin/",
            format!("c2rust-{}{}", tool_name, std::env::consts::EXE_SUFFIX),
            tool_path.display()
        ));
    }
    
    if !tool_path.is_file() {
        return Err(format!(
            "Error: Tool path exists but is not a file: {}\n\
Please ensure c2rust-{}{} is an executable file in $C2RUST_HOME/bin/",
            tool_path.display(),
            tool_name,
            std::env::consts::EXE_SUFFIX
        ));
    }
    
    Ok(tool_path)
}

/// Runs a tool without any custom environment variables.
/// This is a convenience wrapper around run_tool_with_env.
///
/// # Arguments
/// * `tool_name` - The name of the tool to run (e.g., "init", "build")
/// * `args` - Command line arguments to pass to the tool
///
/// # Returns
/// The exit code of the tool process
fn run_tool(tool_name: &str, args: &[String]) -> i32 {
    run_tool_with_env(tool_name, args, &[])
}

/// Runs a tool with custom environment variables.
/// The tool is executed as a subprocess with the specified arguments and environment variables.
///
/// # Arguments
/// * `tool_name` - The name of the tool to run (e.g., "build", "translate")
/// * `args` - Command line arguments to pass to the tool
/// * `env_vars` - Slice of (key, value) pairs for environment variables to set
///
/// # Returns
/// The exit code of the tool process. On Unix, signals are mapped to 128+signal number.
fn run_tool_with_env(tool_name: &str, args: &[String], env_vars: &[(&str, PathBuf)]) -> i32 {
    let tool_path = match get_tool_path(tool_name) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };
    
    let mut cmd = Command::new(tool_path);
    cmd.args(args);
    
    // Set environment variables
    for (key, value) in env_vars {
        cmd.env(key, value);
    }
    
    let status = cmd.status();
    
    match status {
        Ok(exit_status) => {
            #[cfg(unix)]
            {
                use std::os::unix::process::ExitStatusExt;
                // Return exit code if available, otherwise map Unix signal to 128+signal
                if let Some(code) = exit_status.code() {
                    code
                } else if let Some(signal) = exit_status.signal() {
                    128 + signal
                } else {
                    1
                }
            }
            #[cfg(not(unix))]
            {
                exit_status.code().unwrap_or(1)
            }
        }
        Err(e) => {
            eprintln!("Error executing tool: {}", e);
            1
        }
    }
}

/// Gets the full path to a library file in C2RUST_HOME/lib.
/// If the file doesn't exist, returns an error with a user-friendly message.
/// Note: The error message starts with "Warning:" because missing libraries
/// are treated as non-fatal - the tool will still execute but without the
/// environment variable set.
///
/// # Arguments
/// * `lib_name` - The name of the library file (e.g., "libhook.so")
///
/// # Returns
/// * `Ok(PathBuf)` - The full path to the library if it exists
/// * `Err(String)` - A warning message if the library doesn't exist
fn get_lib_path(lib_name: &str) -> Result<PathBuf, String> {
    let c2rust_home = get_c2rust_home()?;
    let lib_path = c2rust_home.join("lib").join(lib_name);
    
    if !lib_path.exists() {
        return Err(format!(
            "Warning: Library '{}' not found at path: {}\n\
Please ensure the library is installed in $C2RUST_HOME/lib/",
            lib_name,
            lib_path.display()
        ));
    }
    
    if !lib_path.is_file() {
        return Err(format!(
            "Warning: Path '{}' for library '{}' exists but is not a file.\n\
Please ensure that '{}' is a regular file in $C2RUST_HOME/lib/",
            lib_path.display(),
            lib_name,
            lib_name
        ));
    }
    
    Ok(lib_path)
}

/// Gets the full path to a directory in C2RUST_HOME/python.
/// If the directory doesn't exist, returns an error with a user-friendly message.
/// Note: The error message starts with "Warning:" because missing directories
/// are treated as non-fatal - the tool will still execute but without the
/// environment variable set.
///
/// # Arguments
/// * `dir_name` - The name of the directory (e.g., "translate_and_fix")
///
/// # Returns
/// * `Ok(PathBuf)` - The full path to the directory if it exists
/// * `Err(String)` - A warning message if the directory doesn't exist
fn get_python_dir(dir_name: &str) -> Result<PathBuf, String> {
    let c2rust_home = get_c2rust_home()?;
    let python_dir = c2rust_home.join("python").join(dir_name);
    
    if !python_dir.exists() {
        return Err(format!(
            "Warning: Python directory '{}' not found at path: {}\n\
Please ensure the directory exists in $C2RUST_HOME/python/",
            dir_name,
            python_dir.display()
        ));
    }
    
    if !python_dir.is_dir() {
        return Err(format!(
            "Warning: Path '{}' exists but is not a directory.\n\
Expected a directory named '{}' under $C2RUST_HOME/python/",
            python_dir.display(),
            dir_name
        ));
    }
    
    Ok(python_dir)
}

fn main() {
    let cli = Cli::parse();
    
    let exit_code = match cli.command {
        Commands::Init => {
            run_tool("init", &["init".to_string()])
        }
        
        Commands::Build { feature, no_interactive, build_cmd } => {
            let mut args = Vec::new();
            args.push("build".to_string());
            
            if let Some(f) = feature {
                args.push("--feature".to_string());
                args.push(f);
            }
            
            if no_interactive {
                args.push("--no-interactive".to_string());
            }
            
            args.push("--".to_string());
            args.extend(build_cmd);
            
            // Set C2RUST_HOOK_LIB environment variable, unless the user has already provided one
            let mut env_vars = Vec::new();
            if env::var("C2RUST_HOOK_LIB").is_err() {
                match get_lib_path(&get_lib_filename("libhook")) {
                    Ok(lib_path) => {
                        env_vars.push(("C2RUST_HOOK_LIB", lib_path));
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            
            run_tool_with_env("build", &args, &env_vars)
        }
        
        Commands::Test { feature, test_cmd } => {
            let mut args = Vec::new();
            args.push("test".to_string());
            
            if let Some(f) = feature {
                args.push("--feature".to_string());
                args.push(f);
            }
            
            args.push("--".to_string());
            args.extend(test_cmd);
            
            run_tool("test", &args)
        }
        
        Commands::Clean { feature, clean_cmd } => {
            let mut args = Vec::new();
            args.push("clean".to_string());
            
            if let Some(f) = feature {
                args.push("--feature".to_string());
                args.push(f);
            }
            
            args.push("--".to_string());
            args.extend(clean_cmd);
            
            run_tool("clean", &args)
        }
        
        Commands::Translate { feature, allow_all, max_fix_attempts, show_full_output } => {
            let mut args = Vec::new();
            args.push("translate".to_string());
            
            if let Some(f) = feature {
                args.push("--feature".to_string());
                args.push(f);
            }
            
            if allow_all {
                args.push("--allow-all".to_string());
            }
            
            if let Some(attempts) = max_fix_attempts {
                args.push("--max-fix-attempts".to_string());
                args.push(attempts.to_string());
            }
            
            if show_full_output {
                args.push("--show-full-output".to_string());
            }
            
            // Set environment variables for translate command, unless the user has already provided them
            let mut env_vars = Vec::new();
            if env::var("C2RUST_TRANSLATE_DIR").is_err() {
                match get_python_dir("translate_and_fix") {
                    Ok(dir_path) => {
                        env_vars.push(("C2RUST_TRANSLATE_DIR", dir_path));
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            if env::var("C2RUST_HYBRID_BUILD_LIB").is_err() {
                match get_lib_path(&get_lib_filename("libc2rust-hybrid-build")) {
                    Ok(lib_path) => {
                        env_vars.push(("C2RUST_HYBRID_BUILD_LIB", lib_path));
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            
            run_tool_with_env("translate", &args, &env_vars)
        }
        
        Commands::Merge { feature, extra_args } => {
            let mut args = Vec::new();
            args.push("merge".to_string());
            
            if let Some(f) = feature {
                args.push("--feature".to_string());
                args.push(f);
            }
            
            args.extend(extra_args);
            
            run_tool("merge", &args)
        }
    };
    
    exit(exit_code);
}
