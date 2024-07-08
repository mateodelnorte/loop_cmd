use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs;
use serde::{Serialize, Deserialize};
use regex::Regex;
use std::process::{Command, Stdio};
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(name = "command")]
    command: Vec<String>,

    #[arg(long, value_name = "DIR")]
    cwd: Option<PathBuf>,

    #[arg(long, value_name = "DIRS", use_value_delimiter = true)]
    include: Option<Vec<String>>,

    #[arg(long, value_name = "DIRS", use_value_delimiter = true)]
    include_only: Option<Vec<String>>,

    #[arg(long, value_name = "PATTERN")]
    include_pattern: Option<String>,

    #[arg(long, value_name = "DIRS", use_value_delimiter = true)]
    exclude: Option<Vec<String>>,

    #[arg(long, value_name = "DIRS", use_value_delimiter = true)]
    exclude_only: Option<Vec<String>>,

    #[arg(long, value_name = "PATTERN")]
    exclude_pattern: Option<String>,

    #[arg(long)]
    init: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoopConfig {
    ignore: Option<Vec<String>>,
}

fn main() -> std::process::ExitCode {
    let args = Args::parse();

    if args.init {
        create_looprc();
        return std::process::ExitCode::SUCCESS;
    }

    let working_dir = args.cwd.clone().unwrap_or_else(|| std::env::current_dir().unwrap());
    std::env::set_current_dir(&working_dir).unwrap();

    let config = read_looprc();

    let mut first_error_code: Option<i32> = None;

    // Process child directories
    for entry in WalkDir::new(".").min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() {
            let dir_path = entry.path();
            if should_process_directory(dir_path, &args, &config) {
                let exit_code = execute_command_in_directory(dir_path, &args.command);
                if exit_code != 0 && first_error_code.is_none() {
                    first_error_code = Some(exit_code);
                }
            }
        }
    }

    // Process included directories
    if let Some(ref include_dirs) = args.include {
        for dir in include_dirs {
            let dir_path = PathBuf::from(dir);
            if dir_path.is_dir() {
                let exit_code = execute_command_in_directory(&dir_path, &args.command);
                if exit_code != 0 && first_error_code.is_none() {
                    first_error_code = Some(exit_code);
                }
            } else {
                eprintln!("Warning: {} is not a directory", dir);
            }
        }
    }

    match first_error_code {
        Some(code) => std::process::ExitCode::from(code as u8),
        None => std::process::ExitCode::SUCCESS,
    }
}

fn create_looprc() {
    let config = LoopConfig {
        ignore: Some(vec![
            ".git".to_string(),
            ".vagrant".to_string(),
            ".vscode".to_string(),
        ]),
    };
    let json = serde_json::to_string_pretty(&config).unwrap();
    fs::write(".looprc", json).unwrap();
    println!("Created .looprc file");
}

fn read_looprc() -> LoopConfig {
    match fs::read_to_string(".looprc") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| LoopConfig { ignore: None }),
        Err(_) => LoopConfig { ignore: None },
    }
}

fn should_process_directory(dir_path: &std::path::Path, args: &Args, config: &LoopConfig) -> bool {
    // Explicitly exclude the current directory
    if dir_path == std::path::Path::new(".") {
        return false;
    }

    let dir_name = dir_path.file_name().unwrap_or_default().to_str().unwrap();

    if let Some(ref include_only) = args.include_only {
        return include_only.contains(&dir_name.to_string()) || include_only.contains(&".".to_string());
    }

    if let Some(ref exclude_only) = args.exclude_only {
        return !exclude_only.contains(&dir_name.to_string());
    }

    if let Some(ref include) = args.include {
        if include.contains(&dir_name.to_string()) || (dir_path == std::path::Path::new(".") && include.contains(&".".to_string())) {
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

    if let Some(ref ignore) = config.ignore {
        if ignore.contains(&dir_name.to_string()) {
            return false;
        }
    }

    true // Default to processing the directory if no exclusion criteria are met
}

fn execute_command_in_directory(dir: &std::path::Path, command: &[String]) -> i32 {
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
    let dir_name = if dir == std::path::Path::new(".") {
        "."
    } else {
        dir.file_name().unwrap_or_default().to_str().unwrap()
    };

    if status.success() {
        println!("\x1b[32m{} ✓\x1b[0m", dir_name);
    } else {
        println!("\x1b[31m{} ✗: exited code {}\x1b[0m", dir_name, exit_code);
    }

    io::stdout().flush().unwrap();

    exit_code
}