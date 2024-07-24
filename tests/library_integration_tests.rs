use loop_lib::args::LoopOptions;

#[test]
fn test_loop_options_creation() {
    let options = LoopOptions {
        command: vec!["echo".to_string(), "test".to_string()],
        cwd: Some("/home/user".to_string()),
        include: Some(vec!["dir1".to_string(), "dir2".to_string()]),
        exclude: Some(vec!["dir3".to_string()]),
        include_only: None,
        exclude_only: None,
        include_pattern: Some("*.rs".to_string()),
        exclude_pattern: Some("*.tmp".to_string()),
        init: false,
    };

    assert_eq!(options.command, vec!["echo", "test"]);
    assert_eq!(options.cwd, Some("/home/user".to_string()));
    assert_eq!(
        options.include,
        Some(vec!["dir1".to_string(), "dir2".to_string()])
    );
    assert_eq!(options.exclude, Some(vec!["dir3".to_string()]));
    assert_eq!(options.include_pattern, Some("*.rs".to_string()));
    assert_eq!(options.exclude_pattern, Some("*.tmp".to_string()));
    assert!(!options.init);
}

#[test]
fn test_loop_options_mixed() {
    let options = LoopOptions {
        command: vec!["npm".to_string(), "run".to_string(), "test".to_string()],
        cwd: Some(".".to_string()),
        include: Some(vec!["src".to_string()]),
        exclude: None,
        include_only: None,
        exclude_only: Some(vec!["node_modules".to_string()]),
        include_pattern: Some("*.js".to_string()),
        exclude_pattern: None,
        init: true,
    };

    assert_eq!(options.command, vec!["npm", "run", "test"]);
    assert_eq!(options.cwd, Some(".".to_string()));
    assert_eq!(options.include, Some(vec!["src".to_string()]));
    assert!(options.exclude.is_none());
    assert!(options.include_only.is_none());
    assert_eq!(options.exclude_only, Some(vec!["node_modules".to_string()]));
    assert_eq!(options.include_pattern, Some("*.js".to_string()));
    assert!(options.exclude_pattern.is_none());
    assert!(options.init);
}

#[test]
fn test_loop_options_empty_command() {
    let options = LoopOptions {
        command: vec![],
        cwd: None,
        include: None,
        exclude: None,
        include_only: None,
        exclude_only: None,
        include_pattern: None,
        exclude_pattern: None,
        init: false,
    };

    assert!(options.command.is_empty());
    assert!(options.cwd.is_none());
    assert!(options.include.is_none());
    assert!(options.exclude.is_none());
    assert!(options.include_only.is_none());
    assert!(options.exclude_only.is_none());
    assert!(options.include_pattern.is_none());
    assert!(options.exclude_pattern.is_none());
    assert!(!options.init);
}

#[test]
fn test_loop_options_only_patterns() {
    let options = LoopOptions {
        command: vec!["grep".to_string(), "-r".to_string(), "TODO".to_string()],
        cwd: None,
        include: None,
        exclude: None,
        include_only: None,
        exclude_only: None,
        include_pattern: Some("*.{rs,toml}".to_string()),
        exclude_pattern: Some("**/target/**".to_string()),
        init: false,
    };

    assert_eq!(options.command, vec!["grep", "-r", "TODO"]);
    assert!(options.cwd.is_none());
    assert!(options.include.is_none());
    assert!(options.exclude.is_none());
    assert!(options.include_only.is_none());
    assert!(options.exclude_only.is_none());
    assert_eq!(options.include_pattern, Some("*.{rs,toml}".to_string()));
    assert_eq!(options.exclude_pattern, Some("**/target/**".to_string()));
    assert!(!options.init);
}
