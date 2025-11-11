// Backend module
//
// Demonstrates module organization for backend functionality

// Declare submodules
pub mod api;
pub mod database;

// Re-export commonly used types
pub use database::{connect, User};

// Module-level initialization
pub fn init() {
    println!("  Backend initialized");
    database::init();
}

// Private helper
fn validate_config() -> bool {
    true
}

// Configuration struct
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn new(host: &str, port: u16) -> Self {
        Config {
            host: host.to_string(),
            port,
        }
    }

    pub fn default() -> Self {
        Config {
            host: "localhost".to_string(),
            port: 8080,
        }
    }
}
