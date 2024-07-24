pub mod args;
pub mod config;
pub mod executor;

use crate::args::LoopOptions;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Runs the main loop command based on the provided options.
///
/// This function is the entry point for the loop command's functionality.
/// It handles initialization if requested, or executes the loop command
/// based on the provided options.
pub fn run(args: LoopOptions) -> i32 {
    if args.init {
        config::create_looprc();
        return exitcode::OK;
    }

    let options = LoopOptions {
        command: args.command,
        cwd: args.cwd,
        include: args.include,
        exclude: args.exclude,
        include_only: args.include_only,
        exclude_only: args.exclude_only,
        include_pattern: args.include_pattern,
        exclude_pattern: args.exclude_pattern,
        init: args.init,
    };

    execute_loop(options)
}

/// Executes the loop command with the given options.
///
/// This function processes directories based on the provided options,
/// executing the specified command in each relevant directory.
pub fn execute_loop(options: LoopOptions) -> i32 {
    let working_dir = options
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
            if executor::should_process_directory(dir_path, &options, &config) {
                let exit_code = executor::execute_command_in_directory(dir_path, &options.command);
                if exit_code != 0 && first_error_code.is_none() {
                    first_error_code = Some(exit_code);
                }
            }
        }
    }

    // Process included directories
    if let Some(ref include_dirs) = options.include {
        for dir in include_dirs {
            let dir_path = PathBuf::from(dir);
            if dir_path.is_dir() {
                let exit_code = executor::execute_command_in_directory(&dir_path, &options.command);
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

        let args = LoopOptions {
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
