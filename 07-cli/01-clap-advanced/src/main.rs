//! Advanced Clap CLI Example
//!
//! This example demonstrates advanced usage of the clap crate for building
//! sophisticated command-line applications with:
//! - Subcommands with their own arguments
//! - Argument groups and conflicts
//! - Custom validators
//! - Configuration file loading (TOML and JSON)
//! - Environment variable integration
//! - Realistic CLI tool structure

use anyhow::{Context, Result};
use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};
use directories::ProjectDirs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// A sophisticated task management CLI tool
///
/// This tool demonstrates advanced clap features including subcommands,
/// argument validation, config file loading, and environment variables.
#[derive(Parser, Debug)]
#[command(name = "taskmaster")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Path to config file (overrides default location)
    #[arg(short, long, global = true, env = "TASKMASTER_CONFIG")]
    config: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long, global = true, env = "TASKMASTER_VERBOSE")]
    verbose: bool,

    /// Output format
    #[arg(short = 'f', long, global = true, value_enum, default_value = "text")]
    format: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    /// Plain text output
    Text,
    /// JSON formatted output
    Json,
    /// Pretty-printed JSON
    JsonPretty,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add a new task
    Add(AddArgs),

    /// List tasks with various filters
    List(ListArgs),

    /// Update an existing task
    Update(UpdateArgs),

    /// Delete a task
    Delete(DeleteArgs),

    /// Search tasks
    Search(SearchArgs),

    /// Manage configuration
    Config(ConfigArgs),

    /// Export/Import tasks
    Transfer(TransferArgs),
}

/// Arguments for adding a new task
#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("priority_group")
        .args(&["priority", "urgent"])
        .required(false)
))]
struct AddArgs {
    /// Task description
    #[arg(value_parser = validate_task_description)]
    description: String,

    /// Task priority (1-5, where 5 is highest)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=5))]
    priority: Option<u8>,

    /// Mark task as urgent (sets priority to 5)
    #[arg(short, long, conflicts_with = "priority")]
    urgent: bool,

    /// Tags for the task (comma-separated)
    #[arg(short, long, value_delimiter = ',')]
    tags: Vec<String>,

    /// Due date in YYYY-MM-DD format
    #[arg(short, long, value_parser = validate_date)]
    due: Option<String>,

    /// Assigned to (must be valid email)
    #[arg(short, long, value_parser = validate_email)]
    assignee: Option<String>,
}

/// Arguments for listing tasks
#[derive(Debug, Args)]
struct ListArgs {
    /// Filter by status
    #[arg(short, long, value_enum)]
    status: Option<TaskStatus>,

    /// Filter by priority (minimum)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=5))]
    priority: Option<u8>,

    /// Filter by tag
    #[arg(short, long)]
    tag: Option<String>,

    /// Show only tasks assigned to this person
    #[arg(short, long)]
    assignee: Option<String>,

    /// Sort by field
    #[arg(long, value_enum, default_value = "created")]
    sort_by: SortField,

    /// Reverse sort order
    #[arg(short, long)]
    reverse: bool,

    /// Limit number of results
    #[arg(short = 'n', long)]
    limit: Option<usize>,
}

#[derive(Debug, Clone, ValueEnum)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Blocked,
}

#[derive(Debug, Clone, ValueEnum)]
enum SortField {
    Created,
    Priority,
    Due,
    Status,
}

/// Arguments for updating a task
#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("update_fields")
        .args(&["description", "priority", "status", "add_tags", "remove_tags", "assignee"])
        .required(true)
        .multiple(true)
))]
struct UpdateArgs {
    /// Task ID to update
    id: String,

    /// New description
    #[arg(short, long)]
    description: Option<String>,

    /// New priority
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=5))]
    priority: Option<u8>,

    /// New status
    #[arg(short, long, value_enum)]
    status: Option<TaskStatus>,

    /// Tags to add (comma-separated)
    #[arg(long, value_delimiter = ',')]
    add_tags: Vec<String>,

    /// Tags to remove (comma-separated)
    #[arg(long, value_delimiter = ',')]
    remove_tags: Vec<String>,

    /// Update assignee
    #[arg(short, long)]
    assignee: Option<String>,
}

/// Arguments for deleting a task
#[derive(Debug, Args)]
struct DeleteArgs {
    /// Task ID(s) to delete
    #[arg(required = true)]
    ids: Vec<String>,

    /// Skip confirmation prompt
    #[arg(short = 'y', long)]
    yes: bool,
}

/// Arguments for searching tasks
#[derive(Debug, Args)]
struct SearchArgs {
    /// Search query (searches in description, tags, and assignee)
    query: String,

    /// Use case-sensitive search
    #[arg(short = 'C', long)]
    case_sensitive: bool,

    /// Use regex pattern matching
    #[arg(short, long)]
    regex: bool,

    /// Search only in specific field
    #[arg(short = 'F', long, value_enum)]
    field: Option<SearchField>,
}

#[derive(Debug, Clone, ValueEnum)]
enum SearchField {
    Description,
    Tags,
    Assignee,
}

/// Arguments for config management
#[derive(Debug, Args)]
struct ConfigArgs {
    #[command(subcommand)]
    action: ConfigAction,
}

#[derive(Debug, Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show,

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },

    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },

    /// Reset configuration to defaults
    Reset {
        /// Skip confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },
}

/// Arguments for export/import operations
#[derive(Debug, Args)]
struct TransferArgs {
    #[command(subcommand)]
    operation: TransferOperation,
}

#[derive(Debug, Subcommand)]
enum TransferOperation {
    /// Export tasks to a file
    Export {
        /// Output file path
        output: PathBuf,

        /// Export format
        #[arg(short, long, value_enum, default_value = "json")]
        format: ExportFormat,

        /// Include completed tasks
        #[arg(long)]
        include_completed: bool,
    },

    /// Import tasks from a file
    Import {
        /// Input file path
        input: PathBuf,

        /// Merge with existing tasks (default is to replace)
        #[arg(short, long)]
        merge: bool,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum ExportFormat {
    Json,
    Csv,
    Toml,
}

/// Application configuration
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    /// Default output format
    default_format: String,

    /// Default sort field
    default_sort: String,

    /// User email for task assignment
    user_email: Option<String>,

    /// Custom data directory
    data_dir: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_format: "text".to_string(),
            default_sort: "created".to_string(),
            user_email: None,
            data_dir: None,
        }
    }
}

/// Custom validator for task descriptions
fn validate_task_description(s: &str) -> Result<String, String> {
    if s.is_empty() {
        return Err("Task description cannot be empty".to_string());
    }

    if s.len() < 3 {
        return Err("Task description must be at least 3 characters".to_string());
    }

    if s.len() > 500 {
        return Err("Task description must be at most 500 characters".to_string());
    }

    Ok(s.to_string())
}

/// Custom validator for date format (YYYY-MM-DD)
fn validate_date(s: &str) -> Result<String, String> {
    let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    if !date_regex.is_match(s) {
        return Err("Date must be in YYYY-MM-DD format".to_string());
    }

    // Parse to check validity
    let parts: Vec<&str> = s.split('-').collect();
    let year: u32 = parts[0].parse().map_err(|_| "Invalid year")?;
    let month: u32 = parts[1].parse().map_err(|_| "Invalid month")?;
    let day: u32 = parts[2].parse().map_err(|_| "Invalid day")?;

    if year < 2000 || year > 2100 {
        return Err("Year must be between 2000 and 2100".to_string());
    }

    if month < 1 || month > 12 {
        return Err("Month must be between 1 and 12".to_string());
    }

    if day < 1 || day > 31 {
        return Err("Day must be between 1 and 31".to_string());
    }

    Ok(s.to_string())
}

/// Custom validator for email addresses
fn validate_email(s: &str) -> Result<String, String> {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    if !email_regex.is_match(s) {
        return Err("Invalid email address format".to_string());
    }

    Ok(s.to_string())
}

/// Load configuration from file
fn load_config(config_path: Option<PathBuf>) -> Result<Config> {
    let path = if let Some(p) = config_path {
        p
    } else {
        get_default_config_path()?
    };

    if !path.exists() {
        return Ok(Config::default());
    }

    let contents = fs::read_to_string(&path).context("Failed to read config file")?;

    // Try TOML first, then JSON
    if path.extension().and_then(|s| s.to_str()) == Some("toml") {
        toml::from_str(&contents).context("Failed to parse TOML config")
    } else {
        serde_json::from_str(&contents).context("Failed to parse JSON config")
    }
}

/// Get the default config file path
fn get_default_config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "example", "taskmaster")
        .context("Failed to determine config directory")?;

    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir).context("Failed to create config directory")?;

    Ok(config_dir.join("config.toml"))
}

/// Save configuration to file
fn save_config(config: &Config, config_path: Option<PathBuf>) -> Result<()> {
    let path = if let Some(p) = config_path {
        p
    } else {
        get_default_config_path()?
    };

    let contents = toml::to_string_pretty(config).context("Failed to serialize config")?;

    fs::write(&path, contents).context("Failed to write config file")?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config = load_config(cli.config.clone())?;

    if cli.verbose {
        eprintln!("Debug: Running command: {:?}", cli.command);
        eprintln!("Debug: Output format: {:?}", cli.format);
        eprintln!("Debug: Config: {:?}", config);
    }

    // Execute the appropriate command
    match cli.command {
        Commands::Add(args) => handle_add(args, &config, cli.verbose)?,
        Commands::List(args) => handle_list(args, &config, &cli.format)?,
        Commands::Update(args) => handle_update(args, &config, cli.verbose)?,
        Commands::Delete(args) => handle_delete(args, cli.verbose)?,
        Commands::Search(args) => handle_search(args, &cli.format)?,
        Commands::Config(args) => handle_config(args, cli.config)?,
        Commands::Transfer(args) => handle_transfer(args)?,
    }

    Ok(())
}

fn handle_add(args: AddArgs, config: &Config, verbose: bool) -> Result<()> {
    let priority = if args.urgent {
        5
    } else {
        args.priority.unwrap_or(3)
    };

    if verbose {
        eprintln!("Adding task with priority: {}", priority);
        eprintln!("Tags: {:?}", args.tags);
    }

    println!("Task added successfully!");
    println!("  Description: {}", args.description);
    println!("  Priority: {}", priority);

    if !args.tags.is_empty() {
        println!("  Tags: {}", args.tags.join(", "));
    }

    if let Some(due) = args.due {
        println!("  Due: {}", due);
    }

    if let Some(assignee) = args.assignee {
        println!("  Assignee: {}", assignee);
    } else if let Some(email) = &config.user_email {
        println!("  Assignee: {} (from config)", email);
    }

    Ok(())
}

fn handle_list(args: ListArgs, _config: &Config, format: &OutputFormat) -> Result<()> {
    // Simulate listing tasks
    println!("Listing tasks...");

    if let Some(status) = args.status {
        println!("  Filter: Status = {:?}", status);
    }

    if let Some(priority) = args.priority {
        println!("  Filter: Priority >= {}", priority);
    }

    if let Some(tag) = args.tag {
        println!("  Filter: Tag = {}", tag);
    }

    if let Some(assignee) = args.assignee {
        println!("  Filter: Assignee = {}", assignee);
    }

    println!("  Sort by: {:?} (reverse: {})", args.sort_by, args.reverse);

    if let Some(limit) = args.limit {
        println!("  Limit: {}", limit);
    }

    // Example tasks
    let tasks = vec![
        ("task-001", "Implement user authentication", 5, "todo"),
        ("task-002", "Write documentation", 3, "in-progress"),
        ("task-003", "Fix login bug", 4, "todo"),
    ];

    match format {
        OutputFormat::Text => {
            println!("\nTasks:");
            for (id, desc, priority, status) in tasks {
                println!(
                    "  [{}] {} (Priority: {}, Status: {})",
                    id, desc, priority, status
                );
            }
        }
        OutputFormat::Json | OutputFormat::JsonPretty => {
            let json_tasks: Vec<_> = tasks
                .iter()
                .map(|(id, desc, priority, status)| {
                    serde_json::json!({
                        "id": id,
                        "description": desc,
                        "priority": priority,
                        "status": status
                    })
                })
                .collect();

            if matches!(format, OutputFormat::JsonPretty) {
                println!("{}", serde_json::to_string_pretty(&json_tasks)?);
            } else {
                println!("{}", serde_json::to_string(&json_tasks)?);
            }
        }
    }

    Ok(())
}

fn handle_update(args: UpdateArgs, _config: &Config, verbose: bool) -> Result<()> {
    if verbose {
        eprintln!("Updating task: {}", args.id);
    }

    println!("Task {} updated:", args.id);

    if let Some(desc) = args.description {
        println!("  Description: {}", desc);
    }

    if let Some(priority) = args.priority {
        println!("  Priority: {}", priority);
    }

    if let Some(status) = args.status {
        println!("  Status: {:?}", status);
    }

    if !args.add_tags.is_empty() {
        println!("  Added tags: {}", args.add_tags.join(", "));
    }

    if !args.remove_tags.is_empty() {
        println!("  Removed tags: {}", args.remove_tags.join(", "));
    }

    if let Some(assignee) = args.assignee {
        println!("  Assignee: {}", assignee);
    }

    Ok(())
}

fn handle_delete(args: DeleteArgs, verbose: bool) -> Result<()> {
    if !args.yes {
        println!(
            "Would delete {} task(s). Use -y to confirm.",
            args.ids.len()
        );
        return Ok(());
    }

    if verbose {
        eprintln!("Deleting tasks: {:?}", args.ids);
    }

    for id in args.ids {
        println!("Deleted task: {}", id);
    }

    Ok(())
}

fn handle_search(args: SearchArgs, format: &OutputFormat) -> Result<()> {
    println!("Searching for: {}", args.query);
    println!("  Case sensitive: {}", args.case_sensitive);
    println!("  Regex: {}", args.regex);

    if let Some(field) = args.field {
        println!("  Field: {:?}", field);
    }

    // Simulate search results
    let results = vec![
        ("task-002", "Write documentation", 3),
        ("task-005", "Document API endpoints", 3),
    ];

    match format {
        OutputFormat::Text => {
            println!("\nFound {} results:", results.len());
            for (id, desc, priority) in results {
                println!("  [{}] {} (Priority: {})", id, desc, priority);
            }
        }
        OutputFormat::Json | OutputFormat::JsonPretty => {
            let json_results: Vec<_> = results
                .iter()
                .map(|(id, desc, priority)| {
                    serde_json::json!({
                        "id": id,
                        "description": desc,
                        "priority": priority
                    })
                })
                .collect();

            if matches!(format, OutputFormat::JsonPretty) {
                println!("{}", serde_json::to_string_pretty(&json_results)?);
            } else {
                println!("{}", serde_json::to_string(&json_results)?);
            }
        }
    }

    Ok(())
}

fn handle_config(args: ConfigArgs, config_path: Option<PathBuf>) -> Result<()> {
    match args.action {
        ConfigAction::Show => {
            let config = load_config(config_path.clone())?;
            println!("Current configuration:");
            println!("{}", toml::to_string_pretty(&config)?);
        }
        ConfigAction::Set { key, value } => {
            let mut config = load_config(config_path.clone())?;

            match key.as_str() {
                "default_format" => config.default_format = value,
                "default_sort" => config.default_sort = value,
                "user_email" => config.user_email = Some(value),
                "data_dir" => config.data_dir = Some(PathBuf::from(value)),
                _ => anyhow::bail!("Unknown config key: {}", key),
            }

            save_config(&config, config_path)?;
            println!(
                "Configuration updated: {} = {:?}",
                key,
                match key.as_str() {
                    "default_format" => config.default_format,
                    "default_sort" => config.default_sort,
                    "user_email" => config.user_email.unwrap_or_default(),
                    "data_dir" => config
                        .data_dir
                        .map(|p| p.display().to_string())
                        .unwrap_or_default(),
                    _ => String::new(),
                }
            );
        }
        ConfigAction::Get { key } => {
            let config = load_config(config_path)?;

            let value = match key.as_str() {
                "default_format" => config.default_format,
                "default_sort" => config.default_sort,
                "user_email" => config.user_email.unwrap_or_else(|| "(not set)".to_string()),
                "data_dir" => config
                    .data_dir
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "(not set)".to_string()),
                _ => anyhow::bail!("Unknown config key: {}", key),
            };

            println!("{}", value);
        }
        ConfigAction::Reset { yes } => {
            if !yes {
                println!("This will reset configuration to defaults. Use -y to confirm.");
                return Ok(());
            }

            let config = Config::default();
            save_config(&config, config_path)?;
            println!("Configuration reset to defaults");
        }
    }

    Ok(())
}

fn handle_transfer(args: TransferArgs) -> Result<()> {
    match args.operation {
        TransferOperation::Export {
            output,
            format,
            include_completed,
        } => {
            println!("Exporting tasks to: {}", output.display());
            println!("  Format: {:?}", format);
            println!("  Include completed: {}", include_completed);

            // Simulate export
            let data = serde_json::json!({
                "tasks": [
                    {"id": "task-001", "description": "Example task", "priority": 3}
                ]
            });

            fs::write(&output, serde_json::to_string_pretty(&data)?)?;
            println!("Export complete!");
        }
        TransferOperation::Import { input, merge } => {
            println!("Importing tasks from: {}", input.display());
            println!("  Merge mode: {}", merge);

            let contents = fs::read_to_string(&input)?;
            let data: serde_json::Value = serde_json::from_str(&contents)?;

            println!(
                "Imported {} tasks",
                data["tasks"].as_array().map(|a| a.len()).unwrap_or(0)
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_task_description() {
        assert!(validate_task_description("Valid task").is_ok());
        assert!(validate_task_description("").is_err());
        assert!(validate_task_description("ab").is_err());
        assert!(validate_task_description(&"a".repeat(501)).is_err());
    }

    #[test]
    fn test_validate_date() {
        assert!(validate_date("2024-01-15").is_ok());
        assert!(validate_date("2024-1-15").is_err());
        assert!(validate_date("24-01-15").is_err());
        assert!(validate_date("2024-13-01").is_err());
        assert!(validate_date("2024-01-32").is_err());
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("user.name@example.co.uk").is_ok());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("@example.com").is_err());
        assert!(validate_email("user@").is_err());
    }
}
