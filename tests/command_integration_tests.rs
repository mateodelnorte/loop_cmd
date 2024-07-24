use assert_cmd::Command;
use once_cell::sync::Lazy;
use predicates::prelude::*;
use std::fs;
use std::sync::Mutex;
use tempfile::tempdir;

// Create a global mutex
static CURRENT_DIR_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[test]
fn test_init_creates_looprc() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("loop").unwrap();
    cmd.current_dir(&temp_dir).arg("--init").assert().success();

    assert!(temp_dir.path().join(".looprc").exists());
}

#[test]
fn test_execute_in_child_directories() {
    let temp_dir = tempdir().unwrap();
    fs::create_dir(temp_dir.path().join("dir1")).unwrap();
    fs::create_dir(temp_dir.path().join("dir2")).unwrap();

    let mut cmd = Command::cargo_bin("loop").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("echo")
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("dir1"))
        .stdout(predicate::str::contains("dir2"));
}

#[test]
fn test_include_directory() {
    let temp_dir = tempdir().unwrap();
    fs::create_dir(temp_dir.path().join("include_dir")).unwrap();

    let mut cmd = Command::cargo_bin("loop").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("echo")
        .arg("test")
        .arg("--include")
        .arg(temp_dir.path().join("include_dir").to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("include_dir"));
}

#[test]
fn test_exclude_directory() {
    let temp_dir = tempdir().unwrap();
    fs::create_dir(temp_dir.path().join("include_dir")).unwrap();
    fs::create_dir(temp_dir.path().join("exclude_dir")).unwrap();

    let mut cmd = Command::cargo_bin("loop").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("echo")
        .arg("test")
        .arg("--exclude")
        .arg("exclude_dir")
        .assert()
        .success()
        .stdout(predicate::str::contains("include_dir"))
        .stdout(predicate::str::contains("exclude_dir").not());
}

#[cfg(test)]
mod tests {
    use super::*;
    use loop_lib::config::{create_looprc, read_looprc, LoopConfig};

    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_create_looprc() {
        let _lock = CURRENT_DIR_MUTEX.lock().unwrap();
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        create_looprc();

        let content = std::fs::read_to_string(".looprc").unwrap();
        println!("Created .looprc content:\n{}", content);

        let config: LoopConfig = serde_json::from_str(&content).unwrap();

        assert!(!config.ignore.is_empty());
        assert!(
            config.ignore.contains(&".git".to_string()),
            "'.git' not found in ignore list"
        );
    }

    #[test]
    fn test_read_looprc() {
        let _lock = CURRENT_DIR_MUTEX.lock().unwrap();
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let test_config = LoopConfig {
            ignore: vec!["test_dir".to_string()],
        };
        let json = serde_json::to_string_pretty(&test_config).unwrap();
        fs::write(".looprc", json).unwrap();

        let read_config = read_looprc();
        assert_eq!(read_config.ignore, vec!["test_dir".to_string()]);
    }

    #[test]
    fn test_read_looprc_missing_file() {
        let _lock = CURRENT_DIR_MUTEX.lock().unwrap();
        let temp_dir = tempdir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let config = read_looprc();
        assert!(config.ignore.is_empty());
    }
}
