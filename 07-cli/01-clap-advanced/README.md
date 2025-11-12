# Advanced Clap CLI Example

A comprehensive example demonstrating advanced usage of the `clap` crate for building sophisticated command-line applications in Rust.

## Features Demonstrated

### 1. **Subcommands**
- Multiple subcommands with their own argument sets
- Nested subcommands (config and transfer operations)
- Command-specific help text and documentation

### 2. **Argument Groups and Conflicts**
- Mutually exclusive arguments (`--priority` vs `--urgent`)
- Required argument groups (at least one field in update command)
- Complex validation logic

### 3. **Custom Validators**
- Task description length validation (3-500 characters)
- Date format validation (YYYY-MM-DD)
- Email address validation using regex

### 4. **Configuration File Loading**
- Support for both TOML and JSON config files
- Automatic config directory creation
- Merging command-line args with config values

### 5. **Environment Variable Integration**
- `TASKMASTER_CONFIG` - Override config file location
- `TASKMASTER_VERBOSE` - Enable verbose output
- Automatic environment variable parsing

### 6. **Value Enums**
- Type-safe enumeration for output formats
- Auto-generated value parsing and validation
- Built-in help text for enum values

### 7. **Multiple Output Formats**
- Plain text output
- JSON output
- Pretty-printed JSON

## Usage Examples

### Basic Commands

```bash
# Add a task with validation
cargo run -- add "Implement user authentication" --priority 5

# Add an urgent task (priority automatically set to 5)
cargo run -- add "Fix critical bug" --urgent

# Add task with tags and due date
cargo run -- add "Write documentation" --tags backend,api --due 2024-12-31

# Add task with assignee (validates email format)
cargo run -- add "Code review" --assignee user@example.com
```

### List and Filter Tasks

```bash
# List all tasks
cargo run -- list

# Filter by status
cargo run -- list --status todo

# Filter by minimum priority
cargo run -- list --priority 4

# Filter by tag
cargo run -- list --tag backend

# Sort and limit results
cargo run -- list --sort-by priority --reverse --limit 10

# Output as JSON
cargo run -- --format json list
```

### Update Tasks

```bash
# Update task description
cargo run -- update task-001 --description "New description"

# Update multiple fields
cargo run -- update task-001 --priority 5 --status in-progress

# Add and remove tags
cargo run -- update task-001 --add-tags urgent,backend --remove-tags frontend
```

### Delete Tasks

```bash
# Delete with confirmation prompt
cargo run -- delete task-001

# Delete multiple tasks without confirmation
cargo run -- delete task-001 task-002 task-003 -y
```

### Search Tasks

```bash
# Simple search
cargo run -- search "documentation"

# Case-sensitive search
cargo run -- search "Documentation" --case-sensitive

# Regex search
cargo run -- search "doc.*ion" --regex

# Search specific field
cargo run -- search "user@example.com" --field assignee
```

### Configuration Management

```bash
# Show current configuration
cargo run -- config show

# Set configuration values
cargo run -- config set user_email "user@example.com"
cargo run -- config set default_format "json"

# Get specific config value
cargo run -- config get user_email

# Reset configuration to defaults
cargo run -- config reset -y
```

### Export/Import

```bash
# Export tasks to JSON
cargo run -- transfer export tasks.json

# Export to CSV with completed tasks
cargo run -- transfer export tasks.csv --format csv --include-completed

# Import tasks (merge mode)
cargo run -- transfer import tasks.json --merge
```

### Global Options

```bash
# Enable verbose output
cargo run -- --verbose list

# Use custom config file
cargo run -- --config /path/to/config.toml list

# Set output format globally
cargo run -- --format json-pretty list
```

### Environment Variables

```bash
# Use environment variables
export TASKMASTER_VERBOSE=true
export TASKMASTER_CONFIG=/custom/config.toml
cargo run -- list
```

## Architecture

### Command Structure

```
taskmaster
├── add        - Add new task
├── list       - List tasks with filters
├── update     - Update existing task
├── delete     - Delete tasks
├── search     - Search tasks
├── config     - Configuration management
│   ├── show   - Show config
│   ├── set    - Set config value
│   ├── get    - Get config value
│   └── reset  - Reset config
└── transfer   - Import/Export
    ├── export - Export tasks
    └── import - Import tasks
```

### Validation Pipeline

1. **Clap parsing** - Type validation, range checks
2. **Custom validators** - Business logic validation
3. **Config loading** - Merge with stored configuration
4. **Command execution** - Process with validated data

## Key Concepts

### Derive Macros

The example uses Clap's derive macros for clean, declarative argument parsing:

```rust
#[derive(Parser)]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}
```

### Argument Groups

Enforce relationships between arguments:

```rust
#[command(group(
    ArgGroup::new("priority_group")
        .args(&["priority", "urgent"])
        .required(false)
))]
```

### Custom Validators

Implement domain-specific validation:

```rust
fn validate_email(s: &str) -> Result<String, String> {
    let regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !regex.is_match(s) {
        return Err("Invalid email format".to_string());
    }
    Ok(s.to_string())
}
```

### Value Enums

Type-safe enum parsing:

```rust
#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Text,
    Json,
    JsonPretty,
}
```

## Testing

Run the included tests:

```bash
# Run unit tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Test specific validator
cargo test test_validate_email
```

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run directly
cargo run -- --help
```

## Dependencies

- **clap** - Command-line argument parsing
- **serde** - Serialization/deserialization
- **serde_json** - JSON support
- **toml** - TOML config files
- **anyhow** - Error handling
- **directories** - Cross-platform config directories
- **regex** - Regular expressions for validation

## Production Considerations

This example demonstrates patterns suitable for production use:

1. **Comprehensive validation** - Multiple layers of input validation
2. **Clear error messages** - User-friendly error reporting
3. **Configuration management** - Flexible config file support
4. **Testing** - Unit tests for validators and logic
5. **Documentation** - Extensive inline comments and help text
6. **Type safety** - Leveraging Rust's type system throughout

## Learning Resources

- [Clap Documentation](https://docs.rs/clap/)
- [Clap Derive Tutorial](https://github.com/clap-rs/clap/blob/master/examples/tutorial_derive/README.md)
- [Command-line Apps in Rust](https://rust-cli.github.io/book/)
