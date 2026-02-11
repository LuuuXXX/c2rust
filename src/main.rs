use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;
use std::process::{Command, exit};

#[derive(Parser)]
#[command(name = "c2rust")]
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
        
        #[arg(last = true, allow_hyphen_values = true, required = true)]
        build_cmd: Vec<String>,
    },
    
    /// 执行测试命令
    Test {
        #[arg(long)]
        feature: Option<String>,
        
        #[arg(last = true, allow_hyphen_values = true, required = true)]
        test_cmd: Vec<String>,
    },
    
    /// 执行清理命令
    Clean {
        #[arg(long)]
        feature: Option<String>,
        
        #[arg(last = true, allow_hyphen_values = true, required = true)]
        clean_cmd: Vec<String>,
    },
    
    /// 翻译 C 代码为 Rust
    Translate {
        #[arg(long, default_value = "default")]
        feature: String,
        
        #[arg(long)]
        allow_all: bool,
        
        #[arg(long, default_value = "10")]
        max_fix_attempts: usize,
        
        #[arg(long)]
        show_full_output: bool,
    },
}

fn get_c2rust_home() -> Result<PathBuf, String> {
    match env::var("C2RUST_HOME") {
        Ok(home) => Ok(PathBuf::from(home)),
        Err(_) => Err(
            "Error: C2RUST_HOME environment variable is not set.\n\
             Please set C2RUST_HOME to the root directory of your c2rust installation.\n\
             Example: export C2RUST_HOME=/path/to/c2rust".to_string()
        ),
    }
}

fn get_tool_path(tool_name: &str) -> Result<PathBuf, String> {
    let c2rust_home = get_c2rust_home()?;
    let tool_path = c2rust_home.join("bin").join(format!("c2rust-{}", tool_name));
    
    if !tool_path.exists() {
        return Err(format!(
            "Error: Tool '{}' not found at path: {}\n\
             Please ensure the tool is installed in $C2RUST_HOME/bin/",
            format!("c2rust-{}", tool_name),
            tool_path.display()
        ));
    }
    
    Ok(tool_path)
}

fn run_tool(tool_name: &str, args: &[String]) -> i32 {
    let tool_path = match get_tool_path(tool_name) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };
    
    let status = Command::new(tool_path)
        .args(args)
        .status();
    
    match status {
        Ok(exit_status) => exit_status.code().unwrap_or(1),
        Err(e) => {
            eprintln!("Error executing tool: {}", e);
            1
        }
    }
}

fn main() {
    let cli = Cli::parse();
    
    let exit_code = match cli.command {
        Commands::Init => {
            run_tool("init", &[])
        }
        
        Commands::Build { feature, no_interactive, build_cmd } => {
            let mut args = Vec::new();
            
            if let Some(f) = feature {
                args.push("--feature".to_string());
                args.push(f);
            }
            
            if no_interactive {
                args.push("--no_interactive".to_string());
            }
            
            args.push("--".to_string());
            args.extend(build_cmd);
            
            run_tool("build", &args)
        }
        
        Commands::Test { feature, test_cmd } => {
            let mut args = Vec::new();
            
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
            
            args.push("--feature".to_string());
            args.push(feature);
            
            if allow_all {
                args.push("--allow_all".to_string());
            }
            
            args.push("--max_fix_attempts".to_string());
            args.push(max_fix_attempts.to_string());
            
            if show_full_output {
                args.push("--show_full_output".to_string());
            }
            
            run_tool("translate", &args)
        }
    };
    
    exit(exit_code);
}
