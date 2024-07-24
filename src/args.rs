use clap::{Arg, ArgAction, Command};

/// Represents the command-line options for the loop command.
///
/// This struct holds all the possible options that can be passed to the loop command,
/// including the command to execute, directories to include or exclude, and patterns
/// for filtering directories.
#[derive(Debug, Clone)]
pub struct LoopOptions {
    pub command: Vec<String>,
    pub cwd: Option<String>,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub include_only: Option<Vec<String>>,
    pub exclude_only: Option<Vec<String>>,
    pub include_pattern: Option<String>,
    pub exclude_pattern: Option<String>,
    pub init: bool,
}

/// Parses command-line arguments and returns a LoopOptions struct.
///
/// This function uses the clap library to define and parse command-line arguments,
/// converting them into a LoopOptions struct for easy use in the rest of the program.
pub fn parse_args() -> LoopOptions {
    let matches = Command::new("loop")
        .about("Loop through directories and execute a command")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Matt")
        .arg(
            Arg::new("command")
                .required_unless_present("init")
                .num_args(1..)
                .help("The command to execute in each directory"),
        )
        .arg(
            Arg::new("cwd")
                .short('C')
                .long("cwd")
                .help("The current working directory"),
        )
        .arg(
            Arg::new("include")
                .short('i')
                .long("include")
                .num_args(1..)
                .help("Additional directories to include"),
        )
        .arg(
            Arg::new("exclude")
                .short('e')
                .long("exclude")
                .num_args(1..)
                .help("Directories to exclude"),
        )
        .arg(
            Arg::new("include_only")
                .long("include-only")
                .num_args(1..)
                .help("Only include these directories"),
        )
        .arg(
            Arg::new("exclude_only")
                .long("exclude-only")
                .num_args(1..)
                .help("Exclude all directories except these"),
        )
        .arg(
            Arg::new("include_pattern")
                .long("include-pattern")
                .help("A pattern to include directories"),
        )
        .arg(
            Arg::new("exclude_pattern")
                .long("exclude-pattern")
                .help("A pattern to exclude directories"),
        )
        .arg(
            Arg::new("init")
                .long("init")
                .action(ArgAction::SetTrue)
                .help("Initialize the loop configuration"),
        )
        .get_matches();

    LoopOptions {
        command: matches
            .get_many::<String>("command")
            .map(|v| v.cloned().collect())
            .unwrap_or_default(),
        cwd: matches.get_one::<String>("cwd").cloned(),
        include: matches
            .get_many::<String>("include")
            .map(|v| v.cloned().collect()),
        exclude: matches
            .get_many::<String>("exclude")
            .map(|v| v.cloned().collect()),
        include_only: matches
            .get_many::<String>("include_only")
            .map(|v| v.cloned().collect()),
        exclude_only: matches
            .get_many::<String>("exclude_only")
            .map(|v| v.cloned().collect()),
        include_pattern: matches.get_one::<String>("include_pattern").cloned(),
        exclude_pattern: matches.get_one::<String>("exclude_pattern").cloned(),
        init: matches.get_flag("init"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing() {
        let args = parse_args_from(&["loop", "gst", "--include", "~/bootstrap"]);
        assert_eq!(args.command, vec!["gst"]);
        assert_eq!(args.include, Some(vec!["~/bootstrap".to_string()]));
        assert!(args.exclude.is_none());
        assert!(!args.init);
    }

    #[test]
    fn test_init_flag() {
        let args = parse_args_from(&["loop", "--init"]);
        assert!(args.init);
        assert!(args.command.is_empty());
    }

    // Helper function for testing
    fn parse_args_from(args: &[&str]) -> LoopOptions {
        let matches = Command::new("loop")
            .arg(
                Arg::new("command")
                    .required_unless_present("init")
                    .num_args(1..),
            )
            .arg(Arg::new("cwd").short('C').long("cwd"))
            .arg(Arg::new("include").short('i').long("include").num_args(1..))
            .arg(Arg::new("exclude").short('e').long("exclude").num_args(1..))
            .arg(Arg::new("include_only").long("include-only").num_args(1..))
            .arg(Arg::new("exclude_only").long("exclude-only").num_args(1..))
            .arg(Arg::new("include_pattern").long("include-pattern"))
            .arg(Arg::new("exclude_pattern").long("exclude-pattern"))
            .arg(Arg::new("init").long("init").action(ArgAction::SetTrue))
            .try_get_matches_from(args)
            .unwrap();

        LoopOptions {
            command: matches
                .get_many::<String>("command")
                .map(|v| v.cloned().collect())
                .unwrap_or_default(),
            cwd: matches.get_one::<String>("cwd").cloned(),
            include: matches
                .get_many::<String>("include")
                .map(|v| v.cloned().collect()),
            exclude: matches
                .get_many::<String>("exclude")
                .map(|v| v.cloned().collect()),
            include_only: matches
                .get_many::<String>("include_only")
                .map(|v| v.cloned().collect()),
            exclude_only: matches
                .get_many::<String>("exclude_only")
                .map(|v| v.cloned().collect()),
            include_pattern: matches.get_one::<String>("include_pattern").cloned(),
            exclude_pattern: matches.get_one::<String>("exclude_pattern").cloned(),
            init: matches.get_flag("init"),
        }
    }
}
