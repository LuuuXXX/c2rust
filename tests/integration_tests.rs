use std::process::Command;
use std::env;
use std::fs;
use std::path::PathBuf;

fn get_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("c2rust");
    path
}

#[test]
fn test_help_command() {
    let output = Command::new(get_binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("C to Rust translation and build tool"));
    assert!(stdout.contains("init"));
    assert!(stdout.contains("build"));
    assert!(stdout.contains("test"));
    assert!(stdout.contains("clean"));
    assert!(stdout.contains("translate"));
}

#[test]
fn test_init_without_c2rust_home() {
    // Ensure C2RUST_HOME is not set
    let output = Command::new(get_binary_path())
        .arg("init")
        .env_remove("C2RUST_HOME")
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("C2RUST_HOME environment variable is not set"));
    assert!(stderr.contains("Please set C2RUST_HOME"));
}

#[test]
fn test_build_help() {
    let output = Command::new(get_binary_path())
        .args(&["build", "--help"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("执行构建命令"));
    assert!(stdout.contains("--feature"));
    assert!(stdout.contains("--no-interactive"));
}

#[test]
fn test_test_help() {
    let output = Command::new(get_binary_path())
        .args(&["test", "--help"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("执行测试命令"));
    assert!(stdout.contains("--feature"));
}

#[test]
fn test_clean_help() {
    let output = Command::new(get_binary_path())
        .args(&["clean", "--help"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("执行清理命令"));
    assert!(stdout.contains("--feature"));
}

#[test]
fn test_translate_help() {
    let output = Command::new(get_binary_path())
        .args(&["translate", "--help"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("翻译 C 代码为 Rust"));
    assert!(stdout.contains("--feature"));
    assert!(stdout.contains("--allow-all"));
    assert!(stdout.contains("--max-fix-attempts"));
    assert!(stdout.contains("--show-full-output"));
}

#[test]
fn test_tool_not_found() {
    // Create a temporary directory as C2RUST_HOME
    let temp_dir = std::env::temp_dir().join("c2rust_test");
    let _ = fs::create_dir_all(&temp_dir);
    
    let output = Command::new(get_binary_path())
        .arg("init")
        .env("C2RUST_HOME", &temp_dir)
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Tool 'c2rust-init' not found"));
    
    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}
