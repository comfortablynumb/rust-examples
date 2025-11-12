// Utility functions module
// This is a simple module in a single file

pub fn print_message(msg: &str) {
    println!("  [Utils] {}", msg);
}

pub fn print_separator() {
    println!("  {}", "=".repeat(50));
}

// Private function - not accessible outside this module
fn internal_helper() {
    println!("This is internal");
}

// Public function that uses private function
pub fn public_wrapper() {
    internal_helper();
}

// Crate-visible - can be used anywhere in this crate, but not by external crates
pub(crate) fn crate_only() {
    println!("  Only visible within this crate");
}

// Module-level constant
pub const VERSION: &str = "1.0.0";

// Module-level static
pub static MAX_CONNECTIONS: u32 = 100;
