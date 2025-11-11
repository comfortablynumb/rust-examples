#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// Module System in Rust
//
// Demonstrates the module system including:
// - Module declaration and organization
// - Public vs private visibility
// - use statements and paths
// - Re-exports
// - Module file structure

// Declare modules
// This tells Rust to look for frontend/mod.rs or frontend.rs
mod frontend;
mod backend;
mod utils;

// Use statements bring items into scope
use frontend::ui::Button;
use backend::database::User;

// Can also use glob imports (use sparingly)
// use utils::*;

fn main() {
    println!("=== Rust Module System ===\n");

    // Example 1: Using modules with full paths
    println!("1. Module Paths:");
    utils::print_message("Hello from utils!");

    // Using absolute path from crate root
    crate::utils::print_separator();
    println!();

    // Example 2: Frontend modules
    println!("2. Frontend Modules:");
    let button = Button::new("Click me");
    button.render();

    let window = frontend::ui::Window::new("My Window", 800, 600);
    window.show();

    // Using widgets through re-export
    let label = frontend::Label::new("Text");  // Re-exported in frontend/mod.rs
    label.display();
    println!();

    // Example 3: Backend modules
    println!("3. Backend Modules:");
    let user = User::new("alice", "alice@example.com");
    println!("  User: {}", user.username());

    backend::database::connect();
    backend::database::disconnect();
    println!();

    // Example 4: Nested module access
    println!("4. API Endpoints:");
    backend::api::get_users();
    backend::api::create_user();
    println!();

    // Example 5: Visibility examples
    println!("5. Visibility:");
    visibility_examples();
    println!();

    // Example 6: Module aliasing
    println!("6. Module Aliasing:");
    use backend::database as db;
    db::query();
    println!();

    // Example 7: Inline modules
    println!("7. Inline Modules:");
    inline_module_examples();
    println!();

    // Example 8: Conditional compilation
    println!("8. Conditional Modules:");
    conditional_modules();
    println!();

    println!("Module system examples complete!");
}

// Example: Visibility modifiers
fn visibility_examples() {
    // Public items can be accessed from anywhere
    mod example {
        pub fn public_function() {
            println!("  Public function - accessible everywhere");
        }

        fn private_function() {
            println!("  Private function - only in this module");
        }

        pub(crate) fn crate_visible() {
            println!("  Crate-visible - accessible in this crate only");
        }

        pub(super) fn parent_visible() {
            println!("  Parent-visible - accessible in parent module");
        }

        pub fn call_all() {
            public_function();
            private_function();
            crate_visible();
            parent_visible();
        }
    }

    example::public_function();
    example::crate_visible();
    example::parent_visible();
    // example::private_function();  // Error! Private
}

// Example: Inline modules (defined in the same file)
fn inline_module_examples() {
    mod math {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }

        // Private helper function
        fn validate(x: i32) -> bool {
            x > 0
        }
    }

    println!("  2 + 3 = {}", math::add(2, 3));
    println!("  2 * 3 = {}", math::multiply(2, 3));
}

// Example: Conditional compilation
fn conditional_modules() {
    #[cfg(target_os = "linux")]
    mod linux_specific {
        pub fn platform_function() {
            println!("  Running on Linux");
        }
    }

    #[cfg(target_os = "windows")]
    mod windows_specific {
        pub fn platform_function() {
            println!("  Running on Windows");
        }
    }

    #[cfg(target_os = "macos")]
    mod macos_specific {
        pub fn platform_function() {
            println!("  Running on macOS");
        }
    }

    #[cfg(target_os = "linux")]
    linux_specific::platform_function();

    #[cfg(target_os = "windows")]
    windows_specific::platform_function();

    #[cfg(target_os = "macos")]
    macos_specific::platform_function();
}

// Example: Super and self keywords
mod outer {
    pub fn outer_function() {
        println!("Outer function");
    }

    pub mod inner {
        pub fn inner_function() {
            // self refers to current module
            self::helper();

            // super refers to parent module
            super::outer_function();

            // Call from crate root
            crate::utils::print_message("From inner module");
        }

        fn helper() {
            println!("Helper in inner module");
        }
    }
}

// Example: Re-exports in main
pub use backend::database::User as DbUser;
pub use frontend::ui::Button as UiButton;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_access() {
        let button = Button::new("Test");
        assert_eq!(button.label(), "Test");
    }

    #[test]
    fn test_user_creation() {
        let user = User::new("test", "test@example.com");
        assert_eq!(user.username(), "test");
    }
}
