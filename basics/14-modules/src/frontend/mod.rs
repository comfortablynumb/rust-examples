// Frontend module
//
// This mod.rs file makes frontend a module and declares its submodules
// When you have frontend/mod.rs, you can have other files in the frontend directory

// Declare submodules
pub mod ui;
pub mod widgets;

// Re-export commonly used items
// This allows users to write `frontend::Button` instead of `frontend::ui::Button`
pub use widgets::Label;
pub use widgets::TextBox;

// Module-level function
pub fn init() {
    println!("  Frontend initialized");
}

// Private module-level function
fn internal_setup() {
    println!("Internal frontend setup");
}

// Struct defined directly in mod.rs
pub struct Theme {
    pub name: String,
    pub primary_color: String,
}

impl Theme {
    pub fn new(name: &str, color: &str) -> Self {
        Theme {
            name: name.to_string(),
            primary_color: color.to_string(),
        }
    }

    pub fn default() -> Self {
        Theme {
            name: "Default".to_string(),
            primary_color: "#007bff".to_string(),
        }
    }
}
