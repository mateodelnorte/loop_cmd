# loop / loop_cmd

`loop` is a simple, but powerful command-line tool that allows you to execute commands in multiple directories simultaneously. It's designed to simplify batch operations across multiple projects or subdirectories, making it an essential tool for developers managing complex project structures.

## Features

- Execute commands in child directories
- Include or exclude specific directories
- Use patterns to filter directories
- Initialize configuration file

## Installation

You can install `loop` using the following command:

```bash
curl -fsSL https://raw.githubusercontent.com/mateodelnorte/loop_cmd/main/install.sh | sh
```

This will download and install `loop` so that it can be used as a command from your PATH.

## Usage

### Basic Usage

Execute a command in all immediate subdirectories:

```bash
loop "git status"
```

### Include Specific Directories

Execute a command only in specified directories:

```bash
loop "npm install" --include dir1 dir2
```

### Exclude Directories

Execute a command in all subdirectories except specified ones:

```bash
loop "cargo build" --exclude target node_modules
```

### Use Patterns

Include directories matching a pattern:

```bash
loop "yarn test" --include-pattern "app-"
```

Exclude directories matching a pattern:

```bash
loop "make clean" --exclude-pattern "-old"
```

### Initialize Configuration

Create a `.looprc` configuration file in your current directory:

```bash
loop --init
```

The above will create a `.looprc` file in your current directory similar to the following: 

```json
{
  "ignore": [
    ".git"
  ]
}
```

This file can be used to set default options for the `loop` command. The above file, for instance, will automatically apply `--ignore .git .vagrant .vscode target` to all commands run using `loop` within the directory containing the file.

## Configuration

You can create a `.looprc` file in your current directory to set default options. Use `loop --init` to create a template configuration file.

## Examples

1. Update all Git repositories:

```bash
loop "git pull origin main"
```

1. Run tests in all JavaScript projects:

```bash
loop "npm test" --include-pattern "*-js"
```

1. Clean build artifacts in C++ projects:

```bash
loop "make clean" --exclude build node_modules
```

1. Update dependencies in Python projects:

```bash
loop "pip install -r requirements.txt --upgrade" --include-pattern "py-*"
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[MIT License](LICENSE)
