# Module System

This example demonstrates Rust's module system comprehensively, including module organization, visibility rules, file structure, and best practices.

## Concepts Covered

### Module Basics
- Module declaration with `mod`
- Module files and directory structure
- `mod.rs` vs named files
- Nested modules
- Inline modules

### Visibility and Privacy
- `pub` - Public visibility
- Private by default
- `pub(crate)` - Crate-visible
- `pub(super)` - Parent-visible
- `pub(in path)` - Path-scoped visibility

### Module Paths
- Absolute paths (`crate::`)
- Relative paths (`self::`, `super::`)
- `use` statements
- Path aliases
- Glob imports

### Module Organization
- Module hierarchy
- Re-exports (`pub use`)
- Module splitting strategies
- Separation of concerns

### Advanced Topics
- Conditional compilation (`#[cfg]`)
- Module-level attributes
- Module privacy patterns
- Testing with modules

## Project Structure

```
src/
├── main.rs           # Binary entry point, declares modules
├── utils.rs          # Simple single-file module
├── frontend/
│   ├── mod.rs        # Frontend module definition
│   ├── ui.rs         # UI components submodule
│   └── widgets.rs    # Widgets submodule
└── backend/
    ├── mod.rs        # Backend module definition
    ├── database.rs   # Database submodule
    └── api.rs        # API submodule
```

## Running the Example

```bash
cargo run
```

Run tests:
```bash
cargo test
```

## Module Declaration

### Simple Module (Single File)

```rust
// In main.rs
mod utils;  // Looks for utils.rs

// In utils.rs
pub fn helper() {
    println!("Helper function");
}
```

### Module Directory with mod.rs

```rust
// In main.rs
mod frontend;  // Looks for frontend/mod.rs or frontend.rs

// In frontend/mod.rs
pub mod ui;      // Looks for frontend/ui.rs
pub mod widgets; // Looks for frontend/widgets.rs

// In frontend/ui.rs
pub struct Button { }
```

## Visibility Rules

| Visibility | Accessible From | Use Case |
|------------|----------------|----------|
| (private) | Same module only | Internal implementation |
| `pub` | Everywhere | Public API |
| `pub(crate)` | Same crate | Internal crate API |
| `pub(super)` | Parent module | Parent-child sharing |
| `pub(in path)` | Specific path | Fine-grained control |

### Examples

```rust
// Private by default - only accessible in same module
fn private_function() { }

// Public - accessible everywhere
pub fn public_function() { }

// Crate-visible - accessible anywhere in this crate
pub(crate) fn crate_function() { }

// Parent-visible - accessible in parent module
pub(super) fn parent_function() { }

// Path-scoped - accessible in specific module path
pub(in crate::backend) fn backend_only() { }
```

## Module Paths

### Absolute Paths

```rust
// Start from crate root
crate::utils::print_message("Hello");
crate::frontend::ui::Button::new("Click");
```

### Relative Paths

```rust
// self - current module
self::helper_function();

// super - parent module
super::parent_function();

// Multiple levels
super::super::root_function();
```

### Use Statements

```rust
// Bring into scope
use crate::frontend::ui::Button;
let btn = Button::new("Click");

// Multiple items
use crate::frontend::{ui, widgets};

// With alias
use crate::backend::database as db;
db::connect();

// Glob import (use sparingly)
use crate::utils::*;
```

## Re-exports

Re-exports allow you to expose items from submodules at a higher level:

```rust
// In frontend/mod.rs
pub mod widgets;

// Re-export for convenience
pub use widgets::Label;
pub use widgets::TextBox;

// In main.rs
use crate::frontend::Label;  // Instead of frontend::widgets::Label
```

### Benefits of Re-exports

1. **Simplified API** - Hide internal structure
2. **Flexibility** - Reorganize without breaking users
3. **Convenience** - Shorter paths for common items

## Common Patterns

### Pattern 1: Feature Modules

```rust
// Organize by feature
src/
├── main.rs
├── auth/
│   ├── mod.rs
│   ├── login.rs
│   └── logout.rs
├── users/
│   ├── mod.rs
│   ├── create.rs
│   └── update.rs
```

### Pattern 2: Layer Modules

```rust
// Organize by architectural layer
src/
├── main.rs
├── models/
├── views/
├── controllers/
└── services/
```

### Pattern 3: Flat Structure

```rust
// Simple projects
src/
├── main.rs
├── config.rs
├── utils.rs
└── types.rs
```

### Pattern 4: lib.rs + main.rs

```rust
// Library + binary
src/
├── lib.rs        # Library root
├── main.rs       # Binary using library
└── modules/
    └── ...
```

## Best Practices

### 1. Keep Modules Focused

```rust
// ✓ Good - single responsibility
mod authentication { }
mod database { }
mod api { }

// ✗ Bad - mixed responsibilities
mod stuff { }
mod helpers { }
```

### 2. Use Visibility Appropriately

```rust
// ✓ Good - minimal public API
pub struct User {
    name: String,        // Private
    email: String,       // Private
}

impl User {
    pub fn new(name: &str, email: &str) -> Self { }  // Public
    pub fn name(&self) -> &str { }                    // Public
    fn validate_email(&self) -> bool { }              // Private
}

// ✗ Bad - everything public
pub struct User {
    pub name: String,
    pub email: String,
    pub internal_state: State,
}
```

### 3. Organize Re-exports

```rust
// ✓ Good - clearly grouped
pub use self::models::{User, Post, Comment};
pub use self::error::{Error, Result};

// ✗ Bad - scattered
pub use self::models::User;
pub use self::error::Error;
pub use self::models::Post;
```

### 4. Avoid Circular Dependencies

```rust
// ✗ Bad - circular dependency
mod a {
    use super::b::B;
}

mod b {
    use super::a::A;  // Error!
}

// ✓ Good - extract common interface
mod interface {
    pub trait Common { }
}

mod a {
    use super::interface::Common;
}

mod b {
    use super::interface::Common;
}
```

### 5. Use Crate Visibility for Internal APIs

```rust
// Internal implementation detail
pub(crate) fn internal_helper() { }

// Public API
pub fn public_api() {
    internal_helper();  // OK in same crate
}
```

## Module Testing

### Testing in Same File

```rust
// In module file
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;  // Import parent module items

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

### Testing in tests/ Directory

```rust
// tests/integration_test.rs
use my_crate::frontend::ui::Button;

#[test]
fn test_button_creation() {
    let btn = Button::new("Click");
    assert_eq!(btn.label(), "Click");
}
```

## Conditional Compilation

### Platform-specific Modules

```rust
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;
```

### Feature-based Modules

```rust
#[cfg(feature = "database")]
mod database;

#[cfg(feature = "networking")]
mod networking;
```

## Common Mistakes

### 1. Not Making Module Public

```rust
// ✗ Bad
mod utils;  // Private!

// ✓ Good
pub mod utils;  // Public
```

### 2. Forgetting to Declare Submodules

```rust
// In frontend/mod.rs

// ✗ Bad - forgot to declare
// frontend/ui.rs exists but not declared

// ✓ Good
pub mod ui;  // Declared
```

### 3. Using Wrong Path

```rust
// ✗ Bad - relative without self/super
use utils::helper;  // Won't work!

// ✓ Good
use crate::utils::helper;  // Absolute from crate root
use super::utils::helper;  // Relative from parent
```

### 4. Overusing pub

```rust
// ✗ Bad - exposing internals
pub struct Internal {
    pub internal_field: i32,
    pub private_state: State,
}

// ✓ Good - hiding implementation
pub struct Public {
    field: i32,  // Private
}

impl Public {
    pub fn field(&self) -> i32 {  // Public accessor
        self.field
    }
}
```

## Migration Strategies

### From Flat to Modular

```rust
// Before
main.rs (1000 lines)

// After
main.rs
├── models.rs
├── views.rs
├── controllers.rs
└── utils.rs
```

### From mod.rs to Split Files

```rust
// Before
frontend/
└── mod.rs (500 lines)

// After
frontend/
├── mod.rs (re-exports only)
├── ui.rs
├── widgets.rs
└── layout.rs
```

## Performance Considerations

### Compilation Units

- Each module is a separate compilation unit
- More modules = better incremental compilation
- Balance granularity with maintainability

### Module Loading

- Modules are loaded at compile time
- No runtime overhead for module organization
- Choose structure for clarity, not performance

## Resources

- [The Rust Book - Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust by Example - Modules](https://doc.rust-lang.org/rust-by-example/mod.html)
- [The Rust Reference - Modules](https://doc.rust-lang.org/reference/items/modules.html)
- [Rust API Guidelines - Organization](https://rust-lang.github.io/api-guidelines/organization.html)

## Real-World Examples

Look at these crates for module organization examples:

- **tokio** - Large async runtime with clear module structure
- **serde** - Well-organized serialization framework
- **rocket** - Web framework with clean module separation
- **diesel** - Database ORM with excellent organization

## Summary

The module system is fundamental to Rust projects:

1. **Organize logically** - Group related functionality
2. **Control visibility** - Hide implementation details
3. **Use re-exports** - Simplify public API
4. **Think in layers** - Separate concerns
5. **Test thoroughly** - Use module structure for testing
6. **Document clearly** - Explain module purpose

Good module organization makes code:
- Easier to understand
- Simpler to maintain
- Better to test
- More reusable
