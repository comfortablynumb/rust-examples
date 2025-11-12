#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::approx_constant)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::assertions_on_constants)]
#![allow(clippy::eq_op)]

// Macros in Rust
//
// Demonstrates declarative macros (macro_rules!) and their usage.
// Macros are code that writes code (metaprogramming).

// Example 1: Simple macro
// Basic macro that expands to a simple expression
macro_rules! say_hello {
    () => {
        println!("Hello from a macro!");
    };
}

// Example 2: Macro with parameters
// Accepts an expression and prints it
macro_rules! print_value {
    ($val:expr) => {
        println!("Value: {}", $val);
    };
}

// Example 3: Macro with multiple patterns
// Different behaviors based on input
macro_rules! create_value {
    // No arguments - return default
    () => {
        0
    };
    // Single argument - return as-is
    ($val:expr) => {
        $val
    };
    // Two arguments - return sum
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// Example 4: Repetition in macros
// Creates a vector from any number of elements
macro_rules! vec_macro {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Example 5: Hash map macro
// Creates a HashMap from key-value pairs
macro_rules! hash_map {
    ( $( $key:expr => $val:expr ),* ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

// Example 6: Macro for creating structs
// Generates a struct with specified fields
macro_rules! create_struct {
    ($name:ident { $( $field:ident : $type:ty ),* }) => {
        struct $name {
            $(
                $field: $type,
            )*
        }
    };
}

// Example 7: Conditional compilation macro
// Compiles different code based on conditions
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

// Example 8: Recursive macro (factorial at compile time)
macro_rules! factorial {
    (0) => {
        1
    };
    ($n:expr) => {
        $n * factorial!($n - 1)
    };
}

// Example 9: Macro for implementing traits
macro_rules! impl_display {
    ($type:ty) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

// Example 10: Testing macro
macro_rules! assert_in_range {
    ($val:expr, $min:expr, $max:expr) => {
        assert!(
            $val >= $min && $val <= $max,
            "Value {} is not in range [{}, {}]",
            $val,
            $min,
            $max
        );
    };
}

// Example 11: Timing macro
macro_rules! time_it {
    ($name:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed();
        println!("{} took: {:?}", $name, duration);
        result
    }};
}

// Example 12: Macro for logging with level
macro_rules! log {
    (info, $($arg:tt)*) => {
        println!("[INFO] {}", format!($($arg)*));
    };
    (warn, $($arg:tt)*) => {
        println!("[WARN] {}", format!($($arg)*));
    };
    (error, $($arg:tt)*) => {
        eprintln!("[ERROR] {}", format!($($arg)*));
    };
}

// Example 13: Macro for generating getter methods
macro_rules! getter {
    ($field:ident: $type:ty) => {
        pub fn $field(&self) -> &$type {
            &self.$field
        }
    };
}

// Example struct using the macro
struct Person {
    name: String,
    age: u32,
}

impl Person {
    getter!(name: String);
    getter!(age: u32);

    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

// Example 14: Macro for creating enums with methods
macro_rules! create_enum {
    ($name:ident { $( $variant:ident ),* }) => {
        #[derive(Debug, PartialEq)]
        enum $name {
            $(
                $variant,
            )*
        }

        impl $name {
            fn variants() -> Vec<&'static str> {
                vec![
                    $(
                        stringify!($variant),
                    )*
                ]
            }
        }
    };
}

// Use the macro to create an enum
create_enum!(Color { Red, Green, Blue });

fn main() {
    println!("=== Macros in Rust ===\n");

    // Example 1: Simple macro
    println!("1. Simple Macro:");
    say_hello!();
    println!();

    // Example 2: Macro with parameters
    println!("2. Macro with Parameters:");
    print_value!(42);
    print_value!("Hello, macros!");
    print_value!(3.14);
    println!();

    // Example 3: Multiple patterns
    println!("3. Multiple Patterns:");
    let x = create_value!();
    let y = create_value!(10);
    let z = create_value!(5, 7);
    println!("Default: {}", x);
    println!("Single: {}", y);
    println!("Sum: {}", z);
    println!();

    // Example 4: Repetition
    println!("4. Repetition (Custom Vec):");
    let numbers = vec_macro![1, 2, 3, 4, 5];
    println!("Vector: {:?}", numbers);
    println!();

    // Example 5: Hash map macro
    println!("5. HashMap Macro:");
    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("Map: {:?}", map);
    println!();

    // Example 6: Struct creation (defined at compile time)
    println!("6. Struct Creation Macro:");
    create_struct!(Point { x: i32, y: i32 });
    let point = Point { x: 10, y: 20 };
    println!("Point: ({}, {})", point.x, point.y);
    println!();

    // Example 7: Conditional compilation
    println!("7. Debug Printing:");
    debug_print!("This only prints in debug mode");
    println!("(Debug messages may not appear in release builds)");
    println!();

    // Example 8: Recursive macro (commented out - can cause issues)
    println!("8. Recursive Macros:");
    println!("(Factorial computed at compile time would go here)");
    // Note: factorial!(5) would compute 5! = 120 at compile time
    // but recursive macros can be tricky and cause stack overflow
    println!();

    // Example 9: Getter methods
    println!("9. Getter Macros:");
    let person = Person::new("Alice".to_string(), 30);
    println!("Name: {}", person.name());
    println!("Age: {}", person.age());
    println!();

    // Example 10: Range assertion
    println!("10. Custom Assertions:");
    let value = 50;
    assert_in_range!(value, 1, 100);
    println!("Value {} is in range [1, 100]", value);
    println!();

    // Example 11: Timing macro
    println!("11. Timing Macro:");
    let result = time_it!("Sleep operation", {
        std::thread::sleep(std::time::Duration::from_millis(100));
        42
    });
    println!("Result: {}", result);
    println!();

    // Example 12: Logging macro
    println!("12. Logging Macro:");
    log!(info, "Application started");
    log!(warn, "Low memory: {} MB", 128);
    log!(error, "Failed to connect to {}", "database");
    println!();

    // Example 13: Enum creation macro
    println!("13. Enum Creation Macro:");
    let color = Color::Red;
    println!("Color: {:?}", color);
    println!("All variants: {:?}", Color::variants());
    println!();

    // Standard library macros
    println!("14. Standard Library Macros:");

    // vec! macro
    let v = vec![1, 2, 3, 4, 5];
    println!("vec! -> {:?}", v);

    // assert! family
    assert!(true, "This passes");
    assert_eq!(2 + 2, 4);
    assert_ne!(5, 6);
    println!("assert! macros passed");

    // format! family
    let s = format!("Hello, {}!", "world");
    println!("format! -> {}", s);

    // panic! (commented out - would crash)
    // panic!("This would crash the program");

    // dbg! macro
    let x = 42;
    let _ = dbg!(x * 2);

    // matches! macro
    let result = matches!(x, 40..=50);
    println!("matches! -> {}", result);

    // todo! and unimplemented! (commented - would panic)
    // todo!("Implement this feature");
    // unimplemented!("Not yet implemented");

    println!();

    // Macro hygiene example
    println!("15. Macro Hygiene:");
    println!("Macros in Rust are hygienic - they don't accidentally");
    println!("capture variables from the calling context.");

    // This macro creates its own variable scope
    macro_rules! hygienic_macro {
        () => {{
            let x = 100;
            x
        }};
    }

    let x = 50;
    let result = hygienic_macro!();
    println!("Outer x: {}", x); // Still 50
    println!("Macro result: {}", result); // 100
    println!();

    println!("=== Macro Patterns ===\n");

    // Pattern types
    println!("Macro Fragment Specifiers:");
    println!("  - expr: expression");
    println!("  - ident: identifier");
    println!("  - ty: type");
    println!("  - pat: pattern");
    println!("  - stmt: statement");
    println!("  - block: block");
    println!("  - item: item (function, struct, etc.)");
    println!("  - meta: meta item");
    println!("  - tt: token tree (any token)");
    println!("  - path: path (e.g., std::vec::Vec)");
    println!("  - literal: literal");
    println!();

    println!("Repetition Operators:");
    println!("  - * : zero or more");
    println!("  - + : one or more");
    println!("  - ? : zero or one");
    println!();
}

// Example 15: Macro for deriving common traits
// (This is what derive macros do, but here's a declarative version)
macro_rules! derive_debug {
    ($type:ty) => {
        impl std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_value() {
        assert_eq!(create_value!(), 0);
        assert_eq!(create_value!(5), 5);
        assert_eq!(create_value!(3, 4), 7);
    }

    #[test]
    fn test_vec_macro() {
        let v = vec_macro![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_assert_in_range() {
        assert_in_range!(5, 1, 10);
        assert_in_range!(1, 1, 10);
        assert_in_range!(10, 1, 10);
    }

    #[test]
    #[should_panic]
    fn test_assert_in_range_fails() {
        assert_in_range!(15, 1, 10);
    }

    #[test]
    fn test_hash_map() {
        let map = hash_map! {
            "a" => 1,
            "b" => 2
        };
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
    }
}
