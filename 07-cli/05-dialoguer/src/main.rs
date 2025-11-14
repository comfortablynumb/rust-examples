//! Dialoguer Interactive Prompts Example
//!
//! This example demonstrates comprehensive usage of dialoguer for
//! creating interactive command-line prompts:
//! - Text input with validation
//! - Password input
//! - Confirmation prompts
//! - Single selection menus
//! - Multi-selection menus
//! - Custom validation and theming

use dialoguer::{
    theme::{ColorfulTheme, SimpleTheme},
    Confirm, Editor, Input, MultiSelect, Password, Select,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen

        println!("╔═══════════════════════════════════════════════════════════╗");
        println!("║         Dialoguer Interactive Prompts Demo               ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
        println!();
        println!("Choose a demo:");
        println!();
        println!("  1. Text Input");
        println!("  2. Password Input");
        println!("  3. Confirmation Prompts");
        println!("  4. Selection Menu");
        println!("  5. Multi-Selection");
        println!("  6. Validation");
        println!("  7. Themed Prompts");
        println!("  8. Editor Input");
        println!("  9. Complete Form Example");
        println!("  0. Exit");
        println!();

        let choice: String = Input::new()
            .with_prompt("Enter your choice")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.chars().all(|c| c.is_ascii_digit())
                    && input.len() == 1
                    && input.chars().next().unwrap() <= '9'
                {
                    Ok(())
                } else {
                    Err("Please enter a single digit (0-9)")
                }
            })
            .interact_text()?;

        match choice.as_str() {
            "1" => text_input_demo()?,
            "2" => password_demo()?,
            "3" => confirmation_demo()?,
            "4" => selection_demo()?,
            "5" => multi_selection_demo()?,
            "6" => validation_demo()?,
            "7" => themed_prompts_demo()?,
            "8" => editor_demo()?,
            "9" => complete_form_demo()?,
            "0" => {
                println!("\nGoodbye!");
                break;
            }
            _ => unreachable!(),
        }

        println!("\nPress Enter to continue...");
        Input::<String>::new().allow_empty(true).interact_text()?;
    }

    Ok(())
}

/// Demo 1: Text Input
fn text_input_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n═══ Text Input Demo ═══\n");

    // Basic text input
    let name: String = Input::new()
        .with_prompt("What's your name?")
        .interact_text()?;

    println!("Hello, {}!", name);

    // Input with default value
    let language: String = Input::new()
        .with_prompt("Favorite programming language")
        .default("Rust".to_string())
        .interact_text()?;

    println!("You chose: {}", language);

    // Optional input (allow empty)
    let nickname: String = Input::new()
        .with_prompt("Nickname (optional)")
        .allow_empty(true)
        .interact_text()?;

    if !nickname.is_empty() {
        println!("Nice nickname: {}", nickname);
    }

    // Numeric input
    let age: u32 = Input::new()
        .with_prompt("What's your age?")
        .interact_text()?;

    println!("You are {} years old", age);

    // Input with initial text
    let email: String = Input::new()
        .with_prompt("Email address")
        .with_initial_text("user@")
        .interact_text()?;

    println!("Email: {}", email);

    Ok(())
}

/// Demo 2: Password Input
fn password_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n═══ Password Input Demo ═══\n");

    // Simple password
    let password: String = Password::new().with_prompt("Enter password").interact()?;

    println!("Password length: {}", password.len());

    // Password with confirmation
    let new_password: String = Password::new()
        .with_prompt("Create new password")
        .with_confirmation("Confirm password", "Passwords don't match")
        .interact()?;

    println!(
        "✓ Password set successfully (length: {})",
        new_password.len()
    );

    // Password with minimum length
    let secure_password: String = Password::new()
        .with_prompt("Enter secure password (min 8 chars)")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() >= 8 {
                Ok(())
            } else {
                Err("Password must be at least 8 characters")
            }
        })
        .interact()?;

    println!("✓ Secure password accepted");

    Ok(())
}

/// Demo 3: Confirmation Prompts
fn confirmation_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n═══ Confirmation Prompts Demo ═══\n");

    // Simple yes/no
    let proceed = Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()?;

    if proceed {
        println!("✓ Proceeding...");
    } else {
        println!("✗ Cancelled");
    }

    // Default to yes
    let delete = Confirm::new()
        .with_prompt("Delete all files?")
        .default(false)
        .interact()?;

    if delete {
        println!("⚠ Files would be deleted!");
    } else {
        println!("✓ Safe - no files deleted");
    }

    // Wait for explicit confirmation
    let dangerous_action = Confirm::new()
        .with_prompt("This is a dangerous operation. Are you absolutely sure?")
        .default(false)
        .wait_for_newline(true)
        .interact()?;

    if dangerous_action {
        println!("⚠ Dangerous action confirmed!");
    } else {
        println!("✓ Action cancelled");
    }

    // Multiple confirmations
    println!("\nSetup wizard:");

    let install = Confirm::new()
        .with_prompt("Install dependencies?")
        .default(true)
        .interact()?;

    let configure = Confirm::new()
        .with_prompt("Configure settings?")
        .default(true)
        .interact()?;

    let start = Confirm::new()
        .with_prompt("Start service?")
        .default(true)
        .interact()?;

    println!("\nSetup plan:");
    println!("  Install: {}", if install { "✓" } else { "✗" });
    println!("  Configure: {}", if configure { "✓" } else { "✗" });
    println!("  Start: {}", if start { "✓" } else { "✗" });

    Ok(())
}

/// Demo 4: Selection Menu
fn selection_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n═══ Selection Menu Demo ═══\n");

    // Simple selection
    let colors = vec!["Red", "Green", "Blue", "Yellow", "Magenta", "Cyan"];

    let selection = Select::new()
        .with_prompt("Choose your favorite color")
        .items(&colors)
        .interact()?;

    println!("You selected: {}", colors[selection]);

    // Selection with default
    let sizes = vec!["Small", "Medium", "Large", "Extra Large"];

    let size_selection = Select::new()
        .with_prompt("Select size")
        .items(&sizes)
        .default(1) // Default to "Medium"
        .interact()?;

    println!("Size: {}", sizes[size_selection]);

    // Selection with categories
    println!("\n--- Programming Languages ---");

    let languages = vec![
        "Rust",
        "Python",
        "JavaScript",
        "Go",
        "C++",
        "Java",
        "TypeScript",
        "Ruby",
    ];

    let lang_selection = Select::new()
        .with_prompt("Select your primary language")
        .items(&languages)
        .interact()?;

    println!("Selected: {}", languages[lang_selection]);

    // Action menu
    println!("\n--- Actions ---");

    let actions = vec![
        "Create new file",
        "Open existing file",
        "Save current file",
        "Save as...",
        "Close file",
        "Exit",
    ];

    let action_selection = Select::new()
        .with_prompt("What would you like to do?")
        .items(&actions)
        .interact()?;

    println!("Action: {}", actions[action_selection]);

    Ok(())
}

/// Demo 5: Multi-Selection
fn multi_selection_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n═══ Multi-Selection Demo ═══\n");

    // Multiple selections
    let features = vec![
        "User authentication",
        "Database integration",
        "REST API",
        "GraphQL API",
        "Real-time updates",
        "Email notifications",
        "File uploads",
        "Search functionality",
    ];

    let selections = MultiSelect::new()
        .with_prompt("Select features to enable (use space to select, enter to confirm)")
        .items(&features)
        .interact()?;

    if selections.is_empty() {
        println!("No features selected");
    } else {
        println!("\nSelected features:");
        for selection in selections {
            println!("  ✓ {}", features[selection]);
        }
    }

    // With defaults pre-selected
    let plugins = vec![
        "Syntax highlighting",
        "Auto-completion",
        "Linting",
        "Formatting",
        "Git integration",
        "Terminal",
    ];

    // Pre-select first three (true for selected, false for not selected)
    let defaults = vec![true, true, true, false, false, false, false, false];

    let plugin_selections = MultiSelect::new()
        .with_prompt("Configure plugins")
        .items(&plugins)
        .defaults(&defaults)
        .interact()?;

    println!("\nEnabled plugins:");
    for selection in plugin_selections {
        println!("  ✓ {}", plugins[selection]);
    }

    Ok(())
}

/// Demo 6: Validation
fn validation_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n═══ Validation Demo ═══\n");

    // Email validation
    let email: String = Input::new()
        .with_prompt("Email address")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains('@') && input.contains('.') {
                Ok(())
            } else {
                Err("Please enter a valid email address")
            }
        })
        .interact_text()?;

    println!("✓ Email accepted: {}", email);

    // Age validation
    let age: u32 = Input::new()
        .with_prompt("Age")
        .validate_with(|input: &u32| -> Result<(), &str> {
            if *input >= 18 && *input <= 120 {
                Ok(())
            } else if *input < 18 {
                Err("You must be at least 18 years old")
            } else {
                Err("Please enter a valid age")
            }
        })
        .interact_text()?;

    println!("✓ Age accepted: {}", age);

    // Username validation
    let username: String = Input::new()
        .with_prompt("Username (alphanumeric, 3-20 chars)")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() < 3 {
                Err("Username must be at least 3 characters")
            } else if input.len() > 20 {
                Err("Username must be at most 20 characters")
            } else if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
                Err("Username can only contain letters, numbers, and underscores")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    println!("✓ Username accepted: {}", username);

    // URL validation
    let url: String = Input::new()
        .with_prompt("Website URL")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.starts_with("http://") || input.starts_with("https://") {
                Ok(())
            } else {
                Err("URL must start with http:// or https://")
            }
        })
        .interact_text()?;

    println!("✓ URL accepted: {}", url);

    Ok(())
}

/// Demo 7: Themed Prompts
fn themed_prompts_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n═══ Themed Prompts Demo ═══\n");

    // Colorful theme
    println!("--- Colorful Theme ---");

    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your name")
        .interact_text()?;

    let proceed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Continue with colorful theme?")
        .interact()?;

    if proceed {
        let colors = vec!["Red", "Green", "Blue"];
        let _selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick a color")
            .items(&colors)
            .interact()?;
    }

    // Simple theme
    println!("\n--- Simple Theme ---");

    let _email: String = Input::with_theme(&SimpleTheme)
        .with_prompt("Email")
        .interact_text()?;

    Ok(())
}

/// Demo 8: Editor Input
fn editor_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n═══ Editor Input Demo ═══\n");

    println!("This will open your default editor for multi-line input.");

    let proceed = Confirm::new().with_prompt("Open editor?").interact()?;

    if proceed {
        let content = Editor::new()
            .edit("Enter your text here...")?
            .unwrap_or_default();

        println!("\nYou entered:");
        println!("─────────────────────");
        println!("{}", content);
        println!("─────────────────────");
        println!("Character count: {}", content.len());
        println!("Line count: {}", content.lines().count());
    }

    Ok(())
}

/// Demo 9: Complete Form Example
fn complete_form_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n═══ Complete Form Example ═══\n");
    println!("Let's create a user profile!\n");

    // Basic info
    let full_name: String = Input::new()
        .with_prompt("Full name")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("Name cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    let email: String = Input::new()
        .with_prompt("Email")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains('@') && input.contains('.') {
                Ok(())
            } else {
                Err("Invalid email address")
            }
        })
        .interact_text()?;

    let age: u32 = Input::new()
        .with_prompt("Age")
        .validate_with(|input: &u32| -> Result<(), &str> {
            if *input >= 13 && *input <= 120 {
                Ok(())
            } else {
                Err("Age must be between 13 and 120")
            }
        })
        .interact_text()?;

    // Password
    let password: String = Password::new()
        .with_prompt("Create password (min 8 chars)")
        .with_confirmation("Confirm password", "Passwords don't match")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() >= 8 {
                Ok(())
            } else {
                Err("Password must be at least 8 characters")
            }
        })
        .interact()?;

    // Preferences
    let role = Select::new()
        .with_prompt("Select your role")
        .items(&["Developer", "Designer", "Manager", "Student", "Other"])
        .interact()?;

    let role_name = match role {
        0 => "Developer",
        1 => "Designer",
        2 => "Manager",
        3 => "Student",
        4 => "Other",
        _ => unreachable!(),
    };

    // Interests
    let interests = vec![
        "Web Development",
        "Mobile Development",
        "Data Science",
        "Machine Learning",
        "DevOps",
        "Security",
        "UI/UX Design",
        "Game Development",
    ];

    let selected_interests = MultiSelect::new()
        .with_prompt("Select your interests (space to select, enter to confirm)")
        .items(&interests)
        .interact()?;

    // Newsletter
    let newsletter = Confirm::new()
        .with_prompt("Subscribe to newsletter?")
        .default(true)
        .interact()?;

    // Terms
    let accept_terms = Confirm::new()
        .with_prompt("I accept the terms and conditions")
        .default(false)
        .wait_for_newline(true)
        .interact()?;

    if !accept_terms {
        println!("\n✗ Registration cancelled - terms not accepted");
        return Ok(());
    }

    // Display summary
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║                    Profile Summary                        ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("Name:           {}", full_name);
    println!("Email:          {}", email);
    println!("Age:            {}", age);
    println!("Password:       {}", "*".repeat(password.len()));
    println!("Role:           {}", role_name);
    println!();

    if selected_interests.is_empty() {
        println!("Interests:      None selected");
    } else {
        println!("Interests:");
        for i in selected_interests {
            println!("                - {}", interests[i]);
        }
    }

    println!();
    println!("Newsletter:     {}", if newsletter { "Yes" } else { "No" });
    println!("Terms Accepted: ✓");

    println!();
    let confirm = Confirm::new()
        .with_prompt("Create this profile?")
        .interact()?;

    if confirm {
        println!("\n✓ Profile created successfully!");
    } else {
        println!("\n✗ Profile creation cancelled");
    }

    Ok(())
}
