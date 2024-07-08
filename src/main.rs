use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs;
use serde::{Serialize, Deserialize};
use regex::Regex;

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

fn main() {
    let args = Args::parse();

    if args.init {
        create_looprc();
        return;
    }

    let working_dir = args.cwd.clone().unwrap_or_else(|| std::env::current_dir().unwrap());
    std::env::set_current_dir(&working_dir).unwrap();

    let config = read_looprc();

    for entry in WalkDir::new(".").min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() {
            let dir_name = entry.file_name().to_str().unwrap();
            
            if should_process_directory(dir_name, &args, &config) {
                execute_command_in_directory(&entry.path(), &args.command);
            }
        }
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

fn should_process_directory(dir_name: &str, args: &Args, config: &LoopConfig) -> bool {
    if let Some(ref include_only) = args.include_only {
        return include_only.contains(&dir_name.to_string());
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

    if let Some(ref ignore) = config.ignore {
        if ignore.contains(&dir_name.to_string()) {
            return false;
        }
    }

    true
}

fn execute_command_in_directory(dir: &std::path::Path, command: &[String]) {
    use std::io::{self, Write};
    use std::process::{Command, Stdio};

    println!("{:?}:\n", dir);
    let command_str = command.join(" ");
    let script = format!("zsh -ic '{}'", command_str);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .env("HOME", std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string()))
        .current_dir(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        print!("\x1b[32m{}\x1b[0m", stdout); // Green color for success
    } else {
        print!("\x1b[31mCode {}. Failed in {:?}: {}\x1b[0m", output.status.code().unwrap_or(-1), dir, stderr); // Red color for error
    }

    io::stdout().flush().unwrap();
    io::stderr().flush().unwrap();
}