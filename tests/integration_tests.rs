use std::process::Command;
use std::env;
use std::fs;
use std::path::PathBuf;

fn get_binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_c2rust"))
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
    let temp_dir = std::env::temp_dir().join(format!("c2rust_test_{}", std::process::id()));
    let bin_dir = temp_dir.join("bin");
    fs::create_dir_all(&bin_dir).expect("Failed to create temporary C2RUST_HOME directory");
    
    let output = Command::new(get_binary_path())
        .arg("init")
        .env("C2RUST_HOME", &temp_dir)
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    let expected_tool = format!("Tool 'c2rust-init{}' not found", env::consts::EXE_SUFFIX);
    assert!(stderr.contains(&expected_tool));
    
    // Clean up
    if let Err(e) = fs::remove_dir_all(&temp_dir) {
        eprintln!("Failed to remove temporary directory {}: {}", temp_dir.display(), e);
    }
}

#[test]
fn test_build_argument_forwarding() {
    // Create a temporary directory as C2RUST_HOME with a mock build tool
    let temp_dir = std::env::temp_dir().join(format!("c2rust_test_build_{}", std::process::id()));
    let bin_dir = temp_dir.join("bin");
    fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    
    // Create a mock c2rust-build script that echoes its arguments
    #[cfg(unix)]
    {
        let mock_script = "#!/bin/sh\necho \"Args: $@\"\n";
        let script_path = bin_dir.join("c2rust-build");
        fs::write(&script_path, mock_script).expect("Failed to write mock script");
        // Make it executable
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).unwrap();
        
        let output = Command::new(get_binary_path())
            .args(&["build", "--feature", "test_feature", "--no-interactive", "--", "make", "all"])
            .env("C2RUST_HOME", &temp_dir)
            .output()
            .expect("Failed to execute command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("--feature"));
        assert!(stdout.contains("test_feature"));
        assert!(stdout.contains("--no-interactive"));
        assert!(stdout.contains("make"));
        assert!(stdout.contains("all"));
    }
    
    // Clean up
    if let Err(e) = fs::remove_dir_all(&temp_dir) {
        eprintln!("Failed to remove temporary directory {}: {}", temp_dir.display(), e);
    }
}

#[test]
fn test_translate_argument_forwarding() {
    // Create a temporary directory as C2RUST_HOME with a mock translate tool
    let temp_dir = std::env::temp_dir().join(format!("c2rust_test_translate_{}", std::process::id()));
    let bin_dir = temp_dir.join("bin");
    fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    
    #[cfg(unix)]
    {
        let mock_script = "#!/bin/sh\necho \"Args: $@\"\n";
        let script_path = bin_dir.join("c2rust-translate");
        fs::write(&script_path, mock_script).expect("Failed to write mock script");
        
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).unwrap();
        
        let output = Command::new(get_binary_path())
            .args(&["translate", "--feature", "custom", "--allow-all", "--max-fix-attempts", "20", "--show-full-output"])
            .env("C2RUST_HOME", &temp_dir)
            .output()
            .expect("Failed to execute command");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("--feature"));
        assert!(stdout.contains("custom"));
        assert!(stdout.contains("--allow-all"));
        assert!(stdout.contains("--max-fix-attempts"));
        assert!(stdout.contains("20"));
        assert!(stdout.contains("--show-full-output"));
    }
    
    // Clean up
    if let Err(e) = fs::remove_dir_all(&temp_dir) {
        eprintln!("Failed to remove temporary directory {}: {}", temp_dir.display(), e);
    }
}

#[test]
fn test_build_requires_separator() {
    // Test that build command correctly handles the -- separator
    let temp_dir = std::env::temp_dir().join(format!("c2rust_test_separator_{}", std::process::id()));
    let bin_dir = temp_dir.join("bin");
    fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    
    #[cfg(unix)]
    {
        let mock_script = "#!/bin/sh\necho \"Args: $@\"\n";
        let script_path = bin_dir.join("c2rust-build");
        fs::write(&script_path, mock_script).expect("Failed to write mock script");
        
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).unwrap();
        
        // Test with explicit -- separator (preferred usage)
        let output = Command::new(get_binary_path())
            .args(&["build", "--", "make", "all"])
            .env("C2RUST_HOME", &temp_dir)
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should forward with -- separator to underlying tool
        assert!(stdout.contains("--"));
        assert!(stdout.contains("make"));
        assert!(stdout.contains("all"));
    }
    
    // Clean up
    if let Err(e) = fs::remove_dir_all(&temp_dir) {
        eprintln!("Failed to remove temporary directory {}: {}", temp_dir.display(), e);
    }
}
