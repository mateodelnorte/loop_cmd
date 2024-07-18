pub mod args;
pub mod config;
pub mod executor;

use std::path::PathBuf;
use walkdir::WalkDir;

pub fn run(args: args::Args) -> i32 {
    if args.init {
        config::create_looprc();
        return exitcode::OK;
    }

    let working_dir = args
        .cwd
        .clone()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    std::env::set_current_dir(working_dir).unwrap();

    let config = config::read_looprc();

    let mut first_error_code: Option<i32> = None;

    // Process child directories
    for entry in WalkDir::new(".")
        .min_depth(1)
        .max_depth(1)
        .sort_by_file_name()
    {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() {
            let dir_path = entry.path();
            if executor::should_process_directory(dir_path, &args, &config) {
                let exit_code = executor::execute_command_in_directory(dir_path, &args.command);
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
                let exit_code = executor::execute_command_in_directory(&dir_path, &args.command);
                if exit_code != 0 && first_error_code.is_none() {
                    first_error_code = Some(exit_code);
                }
            } else {
                eprintln!("Warning: {} is not a directory", dir);
            }
        }
    }

    match first_error_code {
        Some(code) => code,
        None => exitcode::OK,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_run() {
        let temp_dir = tempdir().unwrap();
        fs::create_dir(temp_dir.path().join("test_dir")).unwrap();

        let args = args::Args {
            command: vec!["echo".to_string(), "test".to_string()],
            cwd: Some(temp_dir.path().to_string_lossy().into_owned()),
            include: None,
            exclude: None,
            include_only: None,
            exclude_only: None,
            include_pattern: None,
            exclude_pattern: None,
            init: false,
        };
        let exit_code = run(args);
        assert_eq!(exit_code, exitcode::OK);
    }
}
