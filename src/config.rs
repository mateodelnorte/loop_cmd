use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoopConfig {
    pub ignore: Vec<String>,
}

pub fn create_looprc() {
    use std::env;

    let config = LoopConfig {
        ignore: vec![
            ".git".to_string(),
            ".vagrant".to_string(),
            ".vscode".to_string(),
        ],
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
