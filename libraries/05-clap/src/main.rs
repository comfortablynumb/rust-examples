#![allow(dead_code)]
#![allow(unused_variables)]

//! Comprehensive Clap CLI Example
//!
//! This example demonstrates a Git-like CLI tool with advanced clap features:
//! - Derive API with #[derive(Parser)]
//! - Subcommands and nested subcommands
//! - Optional and required arguments
//! - Short and long flags
//! - Default values
//! - Value validation with possible_values and value_parser
//! - Argument groups
//! - Environment variable fallback
//! - Custom help text
//!
//! Run with: cargo run -- --help
//! Example commands:
//!   cargo run -- --config /path/to/config add task "Buy groceries" --priority high
//!   cargo run -- list --filter active --format json
//!   cargo run -- show 42 --verbose
//!   cargo run -- remote add origin https://github.com/user/repo
//!   cargo run -- config set user.name "John Doe"

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

// ============================================================================
// Main CLI Structure
// ============================================================================

/// TaskFlow - A powerful task and project management CLI tool
///
/// This is the main CLI entry point demonstrating clap's derive API.
/// It showcases global arguments that apply to all subcommands.
#[derive(Parser, Debug)]
#[command(
    name = "taskflow",
    version = "1.0.0",
    author = "Your Name <your.email@example.com>",
    about = "A Git-like task management CLI",
    long_about = "TaskFlow is a comprehensive task management tool with Git-like interface.\n\
                  Manage tasks, projects, and remotes with ease.",
    // Custom help template
    help_template = "{before-help}{name} {version}\n{author-with-newline}\
                     {about-with-newline}\n{usage-heading} {usage}\n\n\
                     {all-args}{after-help}"
)]
struct Cli {
    /// Global verbosity flag - can be used multiple times for increased verbosity
    /// Examples: -v (info), -vv (debug), -vvv (trace)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Global quiet flag - suppresses all output except errors
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    quiet: bool,

    /// Configuration file path
    /// Falls back to TASKFLOW_CONFIG environment variable
    #[arg(
        short,
        long,
        global = true,
        env = "TASKFLOW_CONFIG",
        value_name = "FILE",
        help = "Path to configuration file"
    )]
    config: Option<PathBuf>,

    /// Output format for results
    #[arg(
        long,
        global = true,
        value_enum,
        default_value = "text",
        help = "Output format"
    )]
    format: OutputFormat,

    /// Enable colored output
    #[arg(long, global = true, default_value = "true", value_parser = clap::value_parser!(bool))]
    color: bool,

    /// Working directory
    #[arg(long, global = true, env = "TASKFLOW_DIR", value_name = "DIR")]
    work_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

// ============================================================================
// Output Format Enum - Demonstrates ValueEnum
// ============================================================================

/// Output format options using ValueEnum
/// This trait allows clap to automatically handle string-to-enum conversion
/// and provides shell completion support
#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    /// Plain text output (default)
    Text,
    /// JSON formatted output
    Json,
    /// YAML formatted output
    Yaml,
    /// Compact one-line output
    Compact,
}

// ============================================================================
// Priority Enum - Demonstrates possible values
// ============================================================================

/// Task priority levels
#[derive(Debug, Clone, Copy, ValueEnum)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

// ============================================================================
// Status Enum
// ============================================================================

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Status {
    Active,
    Completed,
    Pending,
    Archived,
}

// ============================================================================
// Main Command Enum - Demonstrates Subcommands
// ============================================================================

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task
    ///
    /// Creates a new task with the specified description and optional metadata.
    /// Tasks can have priorities, tags, and due dates.
    #[command(visible_alias = "a")]
    Add {
        /// Task description (required)
        #[arg(value_name = "DESCRIPTION", help = "Task description")]
        description: String,

        /// Task priority
        #[arg(short, long, value_enum, default_value = "medium")]
        priority: Priority,

        /// Task tags (can be specified multiple times)
        /// Example: --tag work --tag urgent
        #[arg(short, long = "tag", value_name = "TAG")]
        tags: Vec<String>,

        /// Due date in ISO format (YYYY-MM-DD)
        #[arg(short, long, value_name = "DATE", value_parser = validate_date)]
        due: Option<String>,

        /// Assign task to user
        #[arg(short, long, env = "TASKFLOW_USER")]
        assignee: Option<String>,

        /// Make task recurring (in days)
        #[arg(short, long, value_name = "DAYS", value_parser = validate_repeat_days)]
        repeat: Option<u32>,
    },

    /// List tasks with filtering options
    ///
    /// Display tasks matching the specified criteria.
    /// Supports various filters and sorting options.
    #[command(visible_alias = "ls")]
    List {
        /// Filter by status
        #[arg(short, long, value_enum)]
        filter: Option<Status>,

        /// Filter by priority
        #[arg(short, long, value_enum)]
        priority: Option<Priority>,

        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,

        /// Filter by assignee
        #[arg(short, long)]
        assignee: Option<String>,

        /// Sort by field
        #[arg(
            long,
            value_name = "FIELD",
            default_value = "created",
            value_parser = ["created", "modified", "priority", "due"]
        )]
        sort: String,

        /// Sort in reverse order
        #[arg(short, long)]
        reverse: bool,

        /// Limit number of results
        #[arg(short, long, value_name = "NUM", value_parser = validate_positive_usize)]
        limit: Option<usize>,

        /// Show archived tasks
        #[arg(long)]
        show_archived: bool,
    },

    /// Show detailed information about a task
    #[command(visible_alias = "info")]
    Show {
        /// Task ID
        #[arg(value_name = "ID", value_parser = validate_positive_u64)]
        task_id: u64,

        /// Show full task history
        #[arg(long)]
        history: bool,

        /// Show related tasks
        #[arg(long)]
        related: bool,
    },

    /// Update an existing task
    ///
    /// Modify task properties. Only specified fields will be updated.
    #[command(visible_alias = "up")]
    Update {
        /// Task ID to update
        #[arg(value_name = "ID", value_parser = validate_positive_u64)]
        task_id: u64,

        /// New task description
        #[arg(short, long)]
        description: Option<String>,

        /// Update priority
        #[arg(short, long, value_enum)]
        priority: Option<Priority>,

        /// Update status
        #[arg(short, long, value_enum)]
        status: Option<Status>,

        /// Add tags (can be specified multiple times)
        #[arg(long = "add-tag")]
        add_tags: Vec<String>,

        /// Remove tags (can be specified multiple times)
        #[arg(long = "remove-tag", conflicts_with = "clear_tags")]
        remove_tags: Vec<String>,

        /// Clear all tags
        #[arg(long)]
        clear_tags: bool,

        /// Update assignee
        #[arg(short, long)]
        assignee: Option<String>,
    },

    /// Delete a task
    ///
    /// Permanently remove a task from the system.
    #[command(visible_alias = "rm")]
    Delete {
        /// Task ID(s) to delete (can specify multiple)
        #[arg(value_name = "ID", required = true, num_args = 1..)]
        task_ids: Vec<u64>,

        /// Force delete without confirmation
        #[arg(short, long)]
        force: bool,

        /// Also delete related tasks
        #[arg(long)]
        cascade: bool,
    },

    /// Manage remote repositories
    ///
    /// Configure remote sync targets for task synchronization.
    #[command(subcommand)]
    Remote(RemoteCommands),

    /// Configuration management
    ///
    /// View and modify configuration settings.
    #[command(subcommand)]
    Config(ConfigCommands),

    /// Project management commands
    ///
    /// Organize tasks into projects.
    #[command(subcommand)]
    Project(ProjectCommands),

    /// Search tasks and projects
    ///
    /// Full-text search across tasks and metadata.
    Search {
        /// Search query
        #[arg(value_name = "QUERY")]
        query: String,

        /// Case-sensitive search
        #[arg(short = 'C', long)]
        case_sensitive: bool,

        /// Use regular expressions
        #[arg(short, long)]
        regex: bool,

        /// Search in task descriptions only
        #[arg(long, conflicts_with_all = ["tags", "comments"])]
        descriptions: bool,

        /// Search in tags only
        #[arg(long, conflicts_with_all = ["descriptions", "comments"])]
        tags: bool,

        /// Search in comments only
        #[arg(long, conflicts_with_all = ["descriptions", "tags"])]
        comments: bool,

        /// Maximum number of results
        #[arg(short, long, default_value = "50")]
        max_results: usize,
    },

    /// Export tasks to various formats
    Export {
        /// Output file path
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,

        /// Export format
        #[arg(short, long, value_enum, default_value = "json")]
        format: ExportFormat,

        /// Include archived tasks
        #[arg(long)]
        include_archived: bool,

        /// Filter by project
        #[arg(short, long)]
        project: Option<String>,
    },

    /// Import tasks from file
    Import {
        /// Input file path
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Input format (auto-detected if not specified)
        #[arg(short, long, value_enum)]
        format: Option<ExportFormat>,

        /// Skip validation
        #[arg(long)]
        skip_validation: bool,

        /// Dry run (don't actually import)
        #[arg(long)]
        dry_run: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell type
        #[arg(value_enum)]
        shell: Shell,
    },
}

// ============================================================================
// Remote Subcommands - Demonstrates Nested Subcommands
// ============================================================================

#[derive(Subcommand, Debug)]
enum RemoteCommands {
    /// Add a new remote
    Add {
        /// Remote name
        #[arg(value_name = "NAME")]
        name: String,

        /// Remote URL
        #[arg(value_name = "URL")]
        url: String,

        /// Authentication token
        #[arg(short, long, env = "TASKFLOW_TOKEN")]
        token: Option<String>,

        /// Set as default remote
        #[arg(long)]
        default: bool,
    },

    /// Remove a remote
    Remove {
        /// Remote name
        #[arg(value_name = "NAME")]
        name: String,

        /// Force removal
        #[arg(short, long)]
        force: bool,
    },

    /// List configured remotes
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Synchronize with remote
    Sync {
        /// Remote name (uses default if not specified)
        #[arg(value_name = "NAME")]
        remote: Option<String>,

        /// Push local changes only
        #[arg(long, conflicts_with = "pull")]
        push: bool,

        /// Pull remote changes only
        #[arg(long, conflicts_with = "push")]
        pull: bool,

        /// Force sync (overwrite conflicts)
        #[arg(short, long)]
        force: bool,
    },
}

// ============================================================================
// Config Subcommands
// ============================================================================

#[derive(Subcommand, Debug)]
enum ConfigCommands {
    /// Get configuration value
    Get {
        /// Configuration key
        #[arg(value_name = "KEY")]
        key: String,
    },

    /// Set configuration value
    Set {
        /// Configuration key
        #[arg(value_name = "KEY")]
        key: String,

        /// Configuration value
        #[arg(value_name = "VALUE")]
        value: String,

        /// Set globally (default: local)
        #[arg(short, long)]
        global: bool,
    },

    /// Unset configuration value
    Unset {
        /// Configuration key
        #[arg(value_name = "KEY")]
        key: String,

        /// Unset globally
        #[arg(short, long)]
        global: bool,
    },

    /// List all configuration values
    List {
        /// Show global configuration
        #[arg(short, long)]
        global: bool,

        /// Show local configuration
        #[arg(short, long)]
        local: bool,

        /// Show configuration with origins
        #[arg(long)]
        show_origin: bool,
    },
}

// ============================================================================
// Project Subcommands
// ============================================================================

#[derive(Subcommand, Debug)]
enum ProjectCommands {
    /// Create a new project
    Create {
        /// Project name
        #[arg(value_name = "NAME")]
        name: String,

        /// Project description
        #[arg(short, long)]
        description: Option<String>,

        /// Initialize with template
        #[arg(short, long, value_enum)]
        template: Option<ProjectTemplate>,
    },

    /// List all projects
    List {
        /// Show archived projects
        #[arg(long)]
        archived: bool,
    },

    /// Show project details
    Show {
        /// Project name or ID
        #[arg(value_name = "PROJECT")]
        project: String,

        /// Show task statistics
        #[arg(long)]
        stats: bool,
    },

    /// Archive a project
    Archive {
        /// Project name or ID
        #[arg(value_name = "PROJECT")]
        project: String,
    },

    /// Delete a project
    Delete {
        /// Project name or ID
        #[arg(value_name = "PROJECT")]
        project: String,

        /// Force delete without confirmation
        #[arg(short, long)]
        force: bool,

        /// Delete all associated tasks
        #[arg(long)]
        delete_tasks: bool,
    },
}

// ============================================================================
// Additional Enums
// ============================================================================

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ExportFormat {
    Json,
    Yaml,
    Csv,
    Markdown,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[allow(clippy::enum_variant_names)]
enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ProjectTemplate {
    Basic,
    Agile,
    Kanban,
    Personal,
}

// ============================================================================
// Custom Validators
// ============================================================================

/// Validates date format (YYYY-MM-DD)
fn validate_date(s: &str) -> Result<String, String> {
    // Simple validation for ISO date format
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return Err(String::from("Date must be in format YYYY-MM-DD"));
    }

    let year = parts[0]
        .parse::<u32>()
        .map_err(|_| String::from("Invalid year"))?;
    let month = parts[1]
        .parse::<u32>()
        .map_err(|_| String::from("Invalid month"))?;
    let day = parts[2]
        .parse::<u32>()
        .map_err(|_| String::from("Invalid day"))?;

    if !(2000..=2100).contains(&year) {
        return Err(String::from("Year must be between 2000 and 2100"));
    }
    if !(1..=12).contains(&month) {
        return Err(String::from("Month must be between 1 and 12"));
    }
    if !(1..=31).contains(&day) {
        return Err(String::from("Day must be between 1 and 31"));
    }

    Ok(s.to_string())
}

/// Validates positive u64 values (greater than 0)
fn validate_positive_u64(s: &str) -> Result<u64, String> {
    let value = s
        .parse::<u64>()
        .map_err(|_| String::from("Must be a valid positive number"))?;

    if value == 0 {
        return Err(String::from("Value must be greater than 0"));
    }

    Ok(value)
}

/// Validates positive usize values (greater than 0)
fn validate_positive_usize(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<usize>()
        .map_err(|_| String::from("Must be a valid positive number"))?;

    if value == 0 {
        return Err(String::from("Value must be greater than 0"));
    }

    Ok(value)
}

/// Validates repeat days (1-365)
fn validate_repeat_days(s: &str) -> Result<u32, String> {
    let value = s
        .parse::<u32>()
        .map_err(|_| String::from("Must be a valid number"))?;

    if !(1..=365).contains(&value) {
        return Err(String::from("Repeat days must be between 1 and 365"));
    }

    Ok(value)
}

// ============================================================================
// Main Function - Command Handler
// ============================================================================

fn main() {
    // Parse command-line arguments using the derive API
    let cli = Cli::parse();

    // Display global configuration
    println!("=== TaskFlow CLI ===");
    println!("Verbosity level: {}", cli.verbose);
    println!("Quiet mode: {}", cli.quiet);
    println!("Output format: {:?}", cli.format);
    println!("Colored output: {}", cli.color);

    if let Some(config) = &cli.config {
        println!("Config file: {}", config.display());
    }

    if let Some(work_dir) = &cli.work_dir {
        println!("Working directory: {}", work_dir.display());
    }

    println!();

    // Handle commands
    match &cli.command {
        Commands::Add {
            description,
            priority,
            tags,
            due,
            assignee,
            repeat,
        } => {
            println!("Adding new task:");
            println!("  Description: {}", description);
            println!("  Priority: {:?}", priority);
            if !tags.is_empty() {
                println!("  Tags: {}", tags.join(", "));
            }
            if let Some(due_date) = due {
                println!("  Due date: {}", due_date);
            }
            if let Some(user) = assignee {
                println!("  Assignee: {}", user);
            }
            if let Some(days) = repeat {
                println!("  Repeats every {} days", days);
            }
        }

        Commands::List {
            filter,
            priority,
            tag,
            assignee,
            sort,
            reverse,
            limit,
            show_archived,
        } => {
            println!("Listing tasks:");
            if let Some(status) = filter {
                println!("  Filter by status: {:?}", status);
            }
            if let Some(pri) = priority {
                println!("  Filter by priority: {:?}", pri);
            }
            if let Some(tag_name) = tag {
                println!("  Filter by tag: {}", tag_name);
            }
            if let Some(user) = assignee {
                println!("  Filter by assignee: {}", user);
            }
            println!("  Sort by: {}", sort);
            println!("  Reverse order: {}", reverse);
            if let Some(max) = limit {
                println!("  Limit: {}", max);
            }
            println!("  Show archived: {}", show_archived);
        }

        Commands::Show {
            task_id,
            history,
            related,
        } => {
            println!("Showing task #{}:", task_id);
            println!("  Show history: {}", history);
            println!("  Show related: {}", related);
        }

        Commands::Update {
            task_id,
            description,
            priority,
            status,
            add_tags,
            remove_tags,
            clear_tags,
            assignee,
        } => {
            println!("Updating task #{}:", task_id);
            if let Some(desc) = description {
                println!("  New description: {}", desc);
            }
            if let Some(pri) = priority {
                println!("  New priority: {:?}", pri);
            }
            if let Some(stat) = status {
                println!("  New status: {:?}", stat);
            }
            if !add_tags.is_empty() {
                println!("  Adding tags: {}", add_tags.join(", "));
            }
            if !remove_tags.is_empty() {
                println!("  Removing tags: {}", remove_tags.join(", "));
            }
            if *clear_tags {
                println!("  Clearing all tags");
            }
            if let Some(user) = assignee {
                println!("  New assignee: {}", user);
            }
        }

        Commands::Delete {
            task_ids,
            force,
            cascade,
        } => {
            println!("Deleting tasks: {:?}", task_ids);
            println!("  Force: {}", force);
            println!("  Cascade: {}", cascade);
        }

        Commands::Remote(remote_cmd) => match remote_cmd {
            RemoteCommands::Add {
                name,
                url,
                token,
                default,
            } => {
                println!("Adding remote:");
                println!("  Name: {}", name);
                println!("  URL: {}", url);
                if let Some(t) = token {
                    println!("  Token: {}...", &t[..t.len().min(8)]);
                }
                println!("  Set as default: {}", default);
            }
            RemoteCommands::Remove { name, force } => {
                println!("Removing remote: {}", name);
                println!("  Force: {}", force);
            }
            RemoteCommands::List { verbose } => {
                println!("Listing remotes:");
                println!("  Verbose: {}", verbose);
            }
            RemoteCommands::Sync {
                remote,
                push,
                pull,
                force,
            } => {
                println!("Synchronizing with remote:");
                if let Some(r) = remote {
                    println!("  Remote: {}", r);
                } else {
                    println!("  Remote: default");
                }
                println!("  Push only: {}", push);
                println!("  Pull only: {}", pull);
                println!("  Force: {}", force);
            }
        },

        Commands::Config(config_cmd) => match config_cmd {
            ConfigCommands::Get { key } => {
                println!("Getting config value: {}", key);
            }
            ConfigCommands::Set { key, value, global } => {
                println!("Setting config:");
                println!("  Key: {}", key);
                println!("  Value: {}", value);
                println!("  Global: {}", global);
            }
            ConfigCommands::Unset { key, global } => {
                println!("Unsetting config:");
                println!("  Key: {}", key);
                println!("  Global: {}", global);
            }
            ConfigCommands::List {
                global,
                local,
                show_origin,
            } => {
                println!("Listing configuration:");
                println!("  Global: {}", global);
                println!("  Local: {}", local);
                println!("  Show origin: {}", show_origin);
            }
        },

        Commands::Project(project_cmd) => match project_cmd {
            ProjectCommands::Create {
                name,
                description,
                template,
            } => {
                println!("Creating project:");
                println!("  Name: {}", name);
                if let Some(desc) = description {
                    println!("  Description: {}", desc);
                }
                if let Some(tpl) = template {
                    println!("  Template: {:?}", tpl);
                }
            }
            ProjectCommands::List { archived } => {
                println!("Listing projects:");
                println!("  Show archived: {}", archived);
            }
            ProjectCommands::Show { project, stats } => {
                println!("Showing project: {}", project);
                println!("  Show stats: {}", stats);
            }
            ProjectCommands::Archive { project } => {
                println!("Archiving project: {}", project);
            }
            ProjectCommands::Delete {
                project,
                force,
                delete_tasks,
            } => {
                println!("Deleting project: {}", project);
                println!("  Force: {}", force);
                println!("  Delete tasks: {}", delete_tasks);
            }
        },

        Commands::Search {
            query,
            case_sensitive,
            regex,
            descriptions,
            tags,
            comments,
            max_results,
        } => {
            println!("Searching for: {}", query);
            println!("  Case sensitive: {}", case_sensitive);
            println!("  Use regex: {}", regex);
            println!("  Search descriptions: {}", descriptions);
            println!("  Search tags: {}", tags);
            println!("  Search comments: {}", comments);
            println!("  Max results: {}", max_results);
        }

        Commands::Export {
            output,
            format,
            include_archived,
            project,
        } => {
            println!("Exporting tasks:");
            println!("  Output file: {}", output.display());
            println!("  Format: {:?}", format);
            println!("  Include archived: {}", include_archived);
            if let Some(proj) = project {
                println!("  Project filter: {}", proj);
            }
        }

        Commands::Import {
            input,
            format,
            skip_validation,
            dry_run,
        } => {
            println!("Importing tasks:");
            println!("  Input file: {}", input.display());
            if let Some(fmt) = format {
                println!("  Format: {:?}", fmt);
            } else {
                println!("  Format: auto-detect");
            }
            println!("  Skip validation: {}", skip_validation);
            println!("  Dry run: {}", dry_run);
        }

        Commands::Completions { shell } => {
            println!("Generating shell completions for: {:?}", shell);
            println!("To install, run the appropriate command for your shell:");
            match shell {
                Shell::Bash => {
                    println!("  taskflow completions bash > /etc/bash_completion.d/taskflow")
                }
                Shell::Zsh => println!("  taskflow completions zsh > ~/.zsh/completion/_taskflow"),
                Shell::Fish => println!(
                    "  taskflow completions fish > ~/.config/fish/completions/taskflow.fish"
                ),
                Shell::PowerShell => println!("  taskflow completions powershell > taskflow.ps1"),
            }
        }
    }

    println!("\n=== Command executed successfully ===");
}
