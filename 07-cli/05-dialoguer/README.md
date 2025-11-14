# Dialoguer Interactive Prompts

Comprehensive examples demonstrating dialoguer for creating beautiful, interactive command-line prompts and forms with validation, theming, and user-friendly interfaces.

## Features Demonstrated

### 1. **Text Input**
- Basic text input
- Input with default values
- Optional input (allow empty)
- Numeric input with type safety
- Initial text pre-population

### 2. **Password Input**
- Masked password entry
- Password confirmation
- Custom validation (minimum length, complexity)
- Secure input handling

### 3. **Confirmation Prompts**
- Yes/No questions
- Default values
- Explicit confirmation for dangerous operations
- Setup wizards with multiple confirmations

### 4. **Selection Menus**
- Single selection from list
- Default selection
- Arrow key navigation
- Custom formatting

### 5. **Multi-Selection**
- Multiple item selection with checkboxes
- Space to toggle, Enter to confirm
- Pre-selected defaults
- Batch operations

### 6. **Validation**
- Email validation
- Age range validation
- Username pattern validation
- URL format validation
- Custom validation functions

### 7. **Themed Prompts**
- Colorful theme with icons
- Simple monochrome theme
- Custom theme support
- Consistent styling

### 8. **Editor Integration**
- Multi-line text input
- Opens system default editor
- Character and line counting

### 9. **Complete Forms**
- Multi-step data collection
- Combined validation
- Summary display
- Final confirmation

## Usage

```bash
# Run the interactive demo
cargo run

# Select from the menu:
# 1. Text Input
# 2. Password Input
# 3. Confirmation Prompts
# 4. Selection Menu
# 5. Multi-Selection
# 6. Validation
# 7. Themed Prompts
# 8. Editor Input
# 9. Complete Form Example
# 0. Exit
```

## Code Examples

### Basic Text Input

```rust
use dialoguer::Input;

let name: String = Input::new()
    .with_prompt("What's your name?")
    .interact_text()?;

println!("Hello, {}!", name);
```

### Input with Default Value

```rust
let language: String = Input::new()
    .with_prompt("Favorite programming language")
    .default("Rust".to_string())
    .interact_text()?;
```

### Optional Input

```rust
let nickname: String = Input::new()
    .with_prompt("Nickname (optional)")
    .allow_empty(true)
    .interact_text()?;
```

### Password Input

```rust
use dialoguer::Password;

let password: String = Password::new()
    .with_prompt("Enter password")
    .interact()?;
```

### Password with Confirmation

```rust
let password: String = Password::new()
    .with_prompt("Create new password")
    .with_confirmation("Confirm password", "Passwords don't match")
    .interact()?;
```

### Confirmation Prompt

```rust
use dialoguer::Confirm;

let proceed = Confirm::new()
    .with_prompt("Do you want to continue?")
    .interact()?;

if proceed {
    println!("Continuing...");
}
```

### Selection Menu

```rust
use dialoguer::Select;

let options = vec!["Option 1", "Option 2", "Option 3"];

let selection = Select::new()
    .with_prompt("Choose an option")
    .items(&options)
    .interact()?;

println!("Selected: {}", options[selection]);
```

### Multi-Selection

```rust
use dialoguer::MultiSelect;

let features = vec!["Feature A", "Feature B", "Feature C"];

let selections = MultiSelect::new()
    .with_prompt("Select features (space to toggle)")
    .items(&features)
    .interact()?;

for idx in selections {
    println!("Enabled: {}", features[idx]);
}
```

### Input Validation

```rust
let email: String = Input::new()
    .with_prompt("Email address")
    .validate_with(|input: &String| -> Result<(), &str> {
        if input.contains('@') && input.contains('.') {
            Ok(())
        } else {
            Err("Invalid email address")
        }
    })
    .interact_text()?;
```

### Themed Prompts

```rust
use dialoguer::theme::ColorfulTheme;

let name: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Your name")
    .interact_text()?;
```

### Editor Input

```rust
use dialoguer::Editor;

let content = Editor::new()
    .edit("Enter your text here...")?
    .unwrap_or_default();
```

## Validation Patterns

### Email Validation

```rust
.validate_with(|input: &String| -> Result<(), &str> {
    if input.contains('@') && input.contains('.') {
        Ok(())
    } else {
        Err("Invalid email address")
    }
})
```

### Range Validation

```rust
.validate_with(|input: &u32| -> Result<(), &str> {
    if *input >= 18 && *input <= 120 {
        Ok(())
    } else {
        Err("Age must be between 18 and 120")
    }
})
```

### Pattern Validation

```rust
.validate_with(|input: &String| -> Result<(), &str> {
    if input.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Ok(())
    } else {
        Err("Only letters, numbers, and underscores allowed")
    }
})
```

### Length Validation

```rust
.validate_with(|input: &String| -> Result<(), &str> {
    if input.len() >= 3 && input.len() <= 20 {
        Ok(())
    } else {
        Err("Must be 3-20 characters")
    }
})
```

## Prompt Types

### Input

```rust
Input::new()
    .with_prompt("Question")
    .default("default value".to_string())
    .allow_empty(true)
    .with_initial_text("initial")
    .validate_with(|input| { /* ... */ })
    .interact_text()?
```

### Password

```rust
Password::new()
    .with_prompt("Password")
    .with_confirmation("Confirm", "Mismatch error")
    .validate_with(|input| { /* ... */ })
    .interact()?
```

### Confirm

```rust
Confirm::new()
    .with_prompt("Question?")
    .default(true)
    .wait_for_newline(false)
    .interact()?
```

### Select

```rust
Select::new()
    .with_prompt("Choose")
    .items(&items)
    .default(0)
    .interact()?
```

### MultiSelect

```rust
MultiSelect::new()
    .with_prompt("Choose multiple")
    .items(&items)
    .defaults(&[0, 1, 2])
    .interact()?
```

### Editor

```rust
Editor::new()
    .edit("initial content")?
```

## Themes

### Colorful Theme

The default colorful theme with icons and colors:

```rust
use dialoguer::theme::ColorfulTheme;

Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Name")
    .interact_text()?;
```

### Simple Theme

Monochrome theme without colors:

```rust
use dialoguer::theme::SimpleTheme;

Input::with_theme(&SimpleTheme)
    .with_prompt("Name")
    .interact_text()?;
```

### Custom Theme

Create your own theme:

```rust
use dialoguer::theme::Theme;
use console::Style;

struct MyTheme;

impl Theme for MyTheme {
    // Implement theme methods...
}
```

## Best Practices

### 1. Always Validate User Input

```rust
// Don't trust user input
let age: u32 = Input::new()
    .with_prompt("Age")
    .validate_with(|input: &u32| {
        if *input >= 0 && *input <= 150 {
            Ok(())
        } else {
            Err("Invalid age")
        }
    })
    .interact_text()?;
```

### 2. Provide Clear Prompts

```rust
// Good
Input::new()
    .with_prompt("Email address (required)")
    .interact_text()?;

// Better
Input::new()
    .with_prompt("Email address")
    .validate_with(validate_email)
    .interact_text()?;
```

### 3. Use Appropriate Defaults

```rust
Confirm::new()
    .with_prompt("Delete all files?")
    .default(false)  // Safe default for dangerous operation
    .interact()?;
```

### 4. Group Related Prompts

```rust
fn collect_user_info() -> Result<UserInfo> {
    println!("User Registration\n");

    let name = Input::new()
        .with_prompt("Full name")
        .interact_text()?;

    let email = Input::new()
        .with_prompt("Email")
        .validate_with(validate_email)
        .interact_text()?;

    Ok(UserInfo { name, email })
}
```

### 5. Provide Feedback

```rust
let password = Password::new()
    .with_prompt("Create password")
    .with_confirmation("Confirm", "Passwords don't match")
    .interact()?;

println!("✓ Password created successfully");
```

## Common Patterns

### Configuration Wizard

```rust
fn config_wizard() -> Result<Config> {
    println!("Configuration Wizard\n");

    let host = Input::new()
        .with_prompt("Database host")
        .default("localhost".to_string())
        .interact_text()?;

    let port: u16 = Input::new()
        .with_prompt("Database port")
        .default(5432)
        .interact_text()?;

    let ssl = Confirm::new()
        .with_prompt("Use SSL?")
        .default(true)
        .interact()?;

    Ok(Config { host, port, ssl })
}
```

### Multi-Step Form

```rust
fn registration_form() -> Result<User> {
    // Step 1: Basic info
    println!("Step 1: Basic Information\n");
    let name = Input::new()
        .with_prompt("Name")
        .interact_text()?;

    // Step 2: Credentials
    println!("\nStep 2: Credentials\n");
    let password = Password::new()
        .with_prompt("Password")
        .with_confirmation("Confirm", "Mismatch")
        .interact()?;

    // Step 3: Preferences
    println!("\nStep 3: Preferences\n");
    let newsletter = Confirm::new()
        .with_prompt("Subscribe?")
        .interact()?;

    Ok(User { name, password, newsletter })
}
```

### Conditional Prompts

```rust
let install_type = Select::new()
    .with_prompt("Installation type")
    .items(&["Quick", "Custom"])
    .interact()?;

let config = if install_type == 1 {
    // Custom installation - ask more questions
    collect_custom_config()?
} else {
    // Quick installation - use defaults
    Config::default()
};
```

### Error Handling

```rust
loop {
    match Password::new()
        .with_prompt("Password")
        .interact() {
        Ok(pass) => break pass,
        Err(e) => {
            println!("Error: {}. Please try again.", e);
            continue;
        }
    }
}
```

## Integration Examples

### CLI Application Setup

```rust
fn setup_cli_app() -> Result<()> {
    println!("Welcome to MyApp Setup\n");

    let config_path = Input::new()
        .with_prompt("Config file location")
        .default(get_default_config_path())
        .interact_text()?;

    let log_level = Select::new()
        .with_prompt("Log level")
        .items(&["Error", "Warn", "Info", "Debug"])
        .default(2)
        .interact()?;

    let features = MultiSelect::new()
        .with_prompt("Enable features")
        .items(&["API", "Web UI", "Database", "Cache"])
        .interact()?;

    save_config(Config {
        config_path,
        log_level,
        features,
    })?;

    println!("\n✓ Setup complete!");
    Ok(())
}
```

### Data Migration Tool

```rust
fn migration_wizard() -> Result<()> {
    println!("Database Migration Wizard\n");

    let source_db = Input::new()
        .with_prompt("Source database URL")
        .interact_text()?;

    let target_db = Input::new()
        .with_prompt("Target database URL")
        .interact_text()?;

    let tables = MultiSelect::new()
        .with_prompt("Select tables to migrate")
        .items(&get_tables(&source_db)?)
        .interact()?;

    let dry_run = Confirm::new()
        .with_prompt("Perform dry run first?")
        .default(true)
        .interact()?;

    if dry_run {
        println!("Performing dry run...");
        // Dry run logic
    }

    let confirm = Confirm::new()
        .with_prompt("Proceed with migration?")
        .default(false)
        .interact()?;

    if confirm {
        migrate(source_db, target_db, tables)?;
    }

    Ok(())
}
```

## Dependencies

- **dialoguer** - Interactive CLI prompts
- **console** - Terminal styling and colors

## Terminal Compatibility

Dialoguer works across platforms:
- **Linux/Unix** - Full support
- **macOS** - Full support
- **Windows** - Full support with Windows Terminal or ConEmu

Some features require:
- Interactive terminal (TTY)
- Terminal with cursor positioning support
- Editor integration requires $EDITOR or $VISUAL environment variable

## Troubleshooting

### Prompts Not Showing
Ensure running in an interactive terminal:
```rust
if !console::Term::stdout().is_term() {
    eprintln!("Not running in an interactive terminal");
    std::process::exit(1);
}
```

### Editor Not Opening
Set your editor environment variable:
```bash
export EDITOR=nano
# or
export VISUAL=vim
```

### Color Issues
Dialoguer auto-detects terminal capabilities. If colors don't work, use SimpleTheme.

## Learning Resources

- [Dialoguer Documentation](https://docs.rs/dialoguer/)
- [Dialoguer GitHub](https://github.com/console-rs/dialoguer)
- [Console Documentation](https://docs.rs/console/)

## Production Considerations

1. **Non-Interactive Mode** - Provide CLI flags as alternatives
2. **Error Handling** - Handle Ctrl+C and I/O errors gracefully
3. **Accessibility** - Support screen readers where possible
4. **Testing** - Mock prompts for automated testing
5. **Documentation** - Document all prompts and expected inputs
6. **User Experience** - Keep prompts concise and clear
7. **Validation** - Always validate critical inputs
8. **Defaults** - Provide sensible defaults for common cases
