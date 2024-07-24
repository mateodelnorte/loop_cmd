use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

/// Represents the configuration for the loop command.
///
/// This struct holds the configuration options that can be set in the .looprc file,
/// such as directories to ignore.
#[derive(Serialize, Deserialize, Debug)]
pub struct LoopConfig {
    pub ignore: Vec<String>,
}

/// Creates a .looprc file in the current directory with the default configuration.
///
/// This function creates a .looprc file in the current directory with the default configuration.
/// The default configuration is a list of directories to ignore, such as .git.
pub fn create_looprc() {
    use std::env;

    let config = LoopConfig {
        ignore: vec![".git".to_string()],
    };
    let json = serde_json::to_string_pretty(&config).unwrap();
    let file_path = ".looprc";
    fs::write(file_path, &json).unwrap();
    let full_path = env::current_dir().unwrap().join(file_path);
    println!(
        "Created .looprc file at {} with content: {}",
        full_path.display(),
        json
    );
}

/// Reads the .looprc configuration file from the current directory.
///
/// This function attempts to read and parse the .looprc file, returning
/// a LoopConfig struct with the parsed configuration. If the file doesn't
/// exist or can't be parsed, it returns a default configuration.
pub fn read_looprc() -> LoopConfig {
    match fs::read_to_string(".looprc") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|err| {
            eprintln!("Failed to parse .looprc: {}", err);
            LoopConfig { ignore: vec![] }
        }),
        Err(_err) => {
            // Fail silently and continue
            LoopConfig { ignore: vec![] }
        }
    }
}
