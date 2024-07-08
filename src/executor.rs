use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{self, Write};
use regex::Regex;
use crate::args::Args;
use crate::config::LoopConfig;

pub fn should_process_directory(dir_path: &Path, args: &Args, config: &LoopConfig) -> bool {
    let dir_name = dir_path.file_name().unwrap_or_default().to_str().unwrap();

    if let Some(ref include_only) = args.include_only {
        return include_only.contains(&dir_name.to_string()) || include_only.contains(&".".to_string());
    }

    if let Some(ref exclude_only) = args.exclude_only {
        return !exclude_only.contains(&dir_name.to_string());
    }

    if let Some(ref include) = args.include {
        if include.contains(&dir_name.to_string()) {
            return true;
        }
    }

    if let Some(ref exclude) = args.exclude {
        if exclude.contains(&dir_name.to_string()) {
            return false;
        }
    }

    if let Some(ref include_pattern) = args.include_pattern {
        let re = Regex::new(include_pattern).unwrap();
        if !re.is_match(dir_name) {
            return false;
        }
    }

    if let Some(ref exclude_pattern) = args.exclude_pattern {
        let re = Regex::new(exclude_pattern).unwrap();
        if re.is_match(dir_name) {
            return false;
        }
    }

    if config.ignore.contains(&dir_name.to_string()) {
        return false;
    }

    true
}

pub fn execute_command_in_directory(dir: &Path, command: &[String]) -> i32 {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let command_str = command.join(" ");
    
    let script = if shell.ends_with("zsh") {
        format!(
            r#"
            source ~/.zshrc 2>/dev/null
            eval "{}"
            "#,
            command_str.replace('"', r#"\""#)
        )
    } else if shell.ends_with("fish") {
        format!(
            r#"
            source ~/.config/fish/config.fish 2>/dev/null
            {}
            "#,
            command_str
        )
    } else {
        // Assume bash-like shell
        format!(
            r#"
            if [ -f ~/.bashrc ]; then . ~/.bashrc; fi
            {}
            "#,
            command_str
        )
    };

    println!();

    let status = Command::new(&shell)
        .arg("-c")
        .arg(&script)
        .env("HOME", std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string()))
        .current_dir(dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command");

    let exit_code = status.code().unwrap_or(-1);
    let dir_name = dir.file_name().unwrap_or_default().to_str().unwrap();

    if status.success() {
        println!("\x1b[32m{} ✓\x1b[0m", dir_name);
    } else {
        println!("\x1b[31m{} ✗: exited code {}\x1b[0m", dir_name, exit_code);
    }

    io::stdout().flush().unwrap();

    exit_code
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LoopConfig;
    use std::path::PathBuf;

    #[test]
    fn test_should_process_directory() {
        let args = Args {
            command: vec!["test".to_string()],
            cwd: None,
            include: Some(vec!["include_dir".to_string()]),
            exclude: Some(vec!["exclude_dir".to_string()]),
            include_only: None,
            exclude_only: None,
            include_pattern: None,
            exclude_pattern: None,
            init: false,
        };

        let config = LoopConfig {
            ignore: vec![".git".to_string()],
        };

        assert!(should_process_directory(&PathBuf::from("include_dir"), &args, &config));
        assert!(!should_process_directory(&PathBuf::from("exclude_dir"), &args, &config));
        assert!(!should_process_directory(&PathBuf::from(".git"), &args, &config));
        assert!(should_process_directory(&PathBuf::from("normal_dir"), &args, &config));
    }

    #[test]
    fn test_should_process_directory_with_patterns() {
        let args = Args {
            command: vec!["test".to_string()],
            cwd: None,
            include: None,
            exclude: None,
            include_only: None,
            exclude_only: None,
            include_pattern: Some("src.*".to_string()),
            exclude_pattern: Some("test.*".to_string()),
            init: false,
        };

        let config = LoopConfig {
            ignore: vec!["node_modules".to_string(), "target".to_string()],
        };

        assert!(should_process_directory(&PathBuf::from("src"), &args, &config));
        assert!(!should_process_directory(&PathBuf::from("test"), &args, &config));
        assert!(!should_process_directory(&PathBuf::from("node_modules"), &args, &config));
        assert!(!should_process_directory(&PathBuf::from("target"), &args, &config));
    }

    #[test]
    fn test_execute_command_in_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path();
        
        let exit_code = execute_command_in_directory(dir_path, &["echo".to_string(), "test".to_string()]);
        assert_eq!(exit_code, 0);

        let exit_code = execute_command_in_directory(dir_path, &["false".to_string()]);
        assert_ne!(exit_code, 0);
    }
}