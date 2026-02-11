use std::process::Command;
use std::env;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn get_binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_c2rust"))
}

/// Returns the platform-specific dynamic library file extension for tests
fn get_lib_extension() -> &'static str {
    if cfg!(target_os = "macos") {
        ".dylib"
    } else if cfg!(target_os = "windows") {
        ".dll"
    } else {
        ".so"
    }
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
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&bin_dir).expect("Failed to create temporary C2RUST_HOME directory");
    
    let output = Command::new(get_binary_path())
        .arg("init")
        .env("C2RUST_HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    let expected_tool = format!("Tool 'c2rust-init{}' not found", env::consts::EXE_SUFFIX);
    assert!(stderr.contains(&expected_tool));
    
    // temp_dir is automatically cleaned up when it goes out of scope
}

#[test]
#[cfg(unix)]
fn test_build_argument_forwarding() {
    // Create a temporary directory as C2RUST_HOME with a mock build tool
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    
    // Create a mock c2rust-build script that echoes its arguments
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
        .env("C2RUST_HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--feature"));
    assert!(stdout.contains("test_feature"));
    assert!(stdout.contains("--no-interactive"));
    assert!(stdout.contains("make"));
    assert!(stdout.contains("all"));
    
    // temp_dir is automatically cleaned up when it goes out of scope
}

#[test]
#[cfg(unix)]
fn test_translate_argument_forwarding() {
    // Create a temporary directory as C2RUST_HOME with a mock translate tool
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    
    let mock_script = "#!/bin/sh\necho \"Args: $@\"\n";
    let script_path = bin_dir.join("c2rust-translate");
    fs::write(&script_path, mock_script).expect("Failed to write mock script");
    
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms).unwrap();
    
    let output = Command::new(get_binary_path())
        .args(&["translate", "--feature", "custom", "--allow-all", "--max-fix-attempts", "20", "--show-full-output"])
        .env("C2RUST_HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--feature"));
    assert!(stdout.contains("custom"));
    assert!(stdout.contains("--allow-all"));
    assert!(stdout.contains("--max-fix-attempts"));
    assert!(stdout.contains("20"));
    assert!(stdout.contains("--show-full-output"));
    
    // temp_dir is automatically cleaned up when it goes out of scope
}

#[test]
#[cfg(unix)]
fn test_build_requires_separator() {
    // Test that build command correctly handles the -- separator
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    
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
        .env("C2RUST_HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should forward with -- separator to underlying tool
    assert!(stdout.contains("--"));
    assert!(stdout.contains("make"));
    assert!(stdout.contains("all"));
    
    // temp_dir is automatically cleaned up when it goes out of scope
}

#[test]
#[cfg(unix)]
fn test_build_env_vars_with_lib() {
    // Create a temporary directory as C2RUST_HOME with lib directory and mock build tool
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    let bin_dir = temp_dir.path().join("bin");
    let lib_dir = temp_dir.path().join("lib");
    fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    fs::create_dir_all(&lib_dir).expect("Failed to create lib directory");
    
    // Create the libhook library file with platform-specific extension
    let lib_name = format!("libhook{}", get_lib_extension());
    let hook_lib_path = lib_dir.join(&lib_name);
    fs::write(&hook_lib_path, "mock library").expect("Failed to create library");
    
    // Create a mock c2rust-build script that prints environment variables
    let mock_script = "#!/bin/sh\necho \"C2RUST_HOOK_LIB=$C2RUST_HOOK_LIB\"\n";
    let script_path = bin_dir.join("c2rust-build");
    fs::write(&script_path, mock_script).expect("Failed to write mock script");
    
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms).unwrap();
    
    let output = Command::new(get_binary_path())
        .args(&["build", "--", "make"])
        .env("C2RUST_HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Check that C2RUST_HOOK_LIB is set correctly
    assert!(stdout.contains("C2RUST_HOOK_LIB="));
    assert!(stdout.contains("libhook"));
    
    // temp_dir is automatically cleaned up when it goes out of scope
}

#[test]
#[cfg(unix)]
fn test_build_env_vars_without_lib() {
    // Create a temporary directory as C2RUST_HOME without lib directory
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    
    // Create a mock c2rust-build script that prints environment variables
    let mock_script = "#!/bin/sh\necho \"C2RUST_HOOK_LIB=$C2RUST_HOOK_LIB\"\n";
    let script_path = bin_dir.join("c2rust-build");
    fs::write(&script_path, mock_script).expect("Failed to write mock script");
    
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms).unwrap();
    
    let output = Command::new(get_binary_path())
        .args(&["build", "--", "make"])
        .env("C2RUST_HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    // Should still succeed even if lib is not found, but print warning
    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Check for warning message
    assert!(stderr.contains("Warning") || stderr.contains("libhook"));
    
    // temp_dir is automatically cleaned up when it goes out of scope
}

#[test]
#[cfg(unix)]
fn test_translate_env_vars_with_dirs() {
    // Create a temporary directory as C2RUST_HOME with python and lib directories
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    let bin_dir = temp_dir.path().join("bin");
    let lib_dir = temp_dir.path().join("lib");
    let python_dir = temp_dir.path().join("python").join("translate_and_fix");
    fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    fs::create_dir_all(&lib_dir).expect("Failed to create lib directory");
    fs::create_dir_all(&python_dir).expect("Failed to create python directory");
    
    // Create the required files with platform-specific extension
    let lib_name = format!("libc2rust-hybrid-build{}", get_lib_extension());
    let hybrid_lib_path = lib_dir.join(&lib_name);
    fs::write(&hybrid_lib_path, "mock library").expect("Failed to create library");
    let python_file_path = python_dir.join("script.py");
    fs::write(&python_file_path, "# mock script").expect("Failed to create script.py");
    
    // Create a mock c2rust-translate script that prints environment variables
    let mock_script = "#!/bin/sh\necho \"C2RUST_TRANSLATE_DIR=$C2RUST_TRANSLATE_DIR\"\necho \"C2RUST_HYBRID_BUILD_LIB=$C2RUST_HYBRID_BUILD_LIB\"\n";
    let script_path = bin_dir.join("c2rust-translate");
    fs::write(&script_path, mock_script).expect("Failed to write mock script");
    
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms).unwrap();
    
    let output = Command::new(get_binary_path())
        .args(&["translate"])
        .env("C2RUST_HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Check that both environment variables are set correctly
    assert!(stdout.contains("C2RUST_TRANSLATE_DIR="));
    assert!(stdout.contains("translate_and_fix"));
    assert!(stdout.contains("C2RUST_HYBRID_BUILD_LIB="));
    assert!(stdout.contains("libc2rust-hybrid-build"));
    
    // temp_dir is automatically cleaned up when it goes out of scope
}

#[test]
#[cfg(unix)]
fn test_translate_env_vars_without_dirs() {
    // Create a temporary directory as C2RUST_HOME without python and lib directories
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    let bin_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");
    
    // Create a mock c2rust-translate script that prints environment variables
    let mock_script = "#!/bin/sh\necho \"C2RUST_TRANSLATE_DIR=$C2RUST_TRANSLATE_DIR\"\necho \"C2RUST_HYBRID_BUILD_LIB=$C2RUST_HYBRID_BUILD_LIB\"\n";
    let script_path = bin_dir.join("c2rust-translate");
    fs::write(&script_path, mock_script).expect("Failed to write mock script");
    
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms).unwrap();
    
    let output = Command::new(get_binary_path())
        .args(&["translate"])
        .env("C2RUST_HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");
    
    // Should still succeed even if directories are not found, but print warnings
    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Check for warning messages
    assert!(stderr.contains("Warning"));
    
    // temp_dir is automatically cleaned up when it goes out of scope
}
