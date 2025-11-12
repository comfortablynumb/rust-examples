# Macros

This example demonstrates macros in Rust, focusing on declarative macros (`macro_rules!`). Macros are a form of metaprogramming that allow you to write code that writes code.

## Concepts Covered

### Declarative Macros
- `macro_rules!` syntax
- Pattern matching in macros
- Multiple patterns
- Macro hygiene
- Fragment specifiers

### Macro Patterns
- Simple macros without arguments
- Macros with expressions
- Macros with repetition
- Recursive macros
- Variadic macros

### Common Use Cases
- Code generation
- Domain-specific languages (DSLs)
- Reducing boilerplate
- Conditional compilation
- Custom assertions and logging
- Trait implementations

### Standard Library Macros
- `vec!`, `println!`, `format!`
- `assert!`, `assert_eq!`, `assert_ne!`
- `dbg!`, `matches!`
- `todo!`, `unimplemented!`, `unreachable!`
- `panic!`

## Running the Example

```bash
cargo run
```

Run tests:
```bash
cargo test
```

## Key Takeaways

1. **Macros run at compile time** - They expand before compilation
2. **Hygienic** - Variables in macros don't leak to calling scope
3. **Pattern matching** - Macros can have multiple patterns
4. **Repetition** - Use `$( )*` for variable number of arguments
5. **Type safety** - Macro errors caught at compile time
6. **Performance** - No runtime overhead

## Macro Fragment Specifiers

| Specifier | Description | Example |
|-----------|-------------|---------|
| `expr` | Expression | `1 + 2`, `foo()` |
| `ident` | Identifier | `x`, `my_var` |
| `ty` | Type | `i32`, `Vec<T>` |
| `pat` | Pattern | `Some(x)`, `0..=10` |
| `stmt` | Statement | `let x = 5;` |
| `block` | Block | `{ ... }` |
| `item` | Item | `fn foo() {}`, `struct Bar` |
| `meta` | Meta item | `cfg(target_os = "linux")` |
| `tt` | Token tree | Any single token |
| `path` | Path | `std::vec::Vec` |
| `literal` | Literal | `42`, `"hello"` |

## Repetition Operators

| Operator | Meaning | Example |
|----------|---------|---------|
| `*` | Zero or more | `$( $x:expr ),*` |
| `+` | One or more | `$( $x:expr ),+` |
| `?` | Zero or one | `$( $x:expr )?` |

## Common Patterns

### Simple macro
```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

say_hello!();
```

### Macro with parameter
```rust
macro_rules! print_value {
    ($val:expr) => {
        println!("Value: {}", $val);
    };
}

print_value!(42);
```

### Multiple patterns
```rust
macro_rules! create_value {
    () => { 0 };
    ($val:expr) => { $val };
    ($a:expr, $b:expr) => { $a + $b };
}

let x = create_value!();        // 0
let y = create_value!(5);       // 5
let z = create_value!(3, 4);    // 7
```

### Repetition
```rust
macro_rules! vec_macro {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

let v = vec_macro![1, 2, 3, 4, 5];
```

### HashMap creation
```rust
macro_rules! hash_map {
    ( $( $key:expr => $val:expr ),* ) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

let map = hash_map! {
    "one" => 1,
    "two" => 2
};
```

### Conditional compilation
```rust
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

debug_print!("Only in debug builds");
```

### Timing macro
```rust
macro_rules! time_it {
    ($name:expr, $code:block) => {
        {
            let start = std::time::Instant::now();
            let result = $code;
            println!("{} took: {:?}", $name, start.elapsed());
            result
        }
    };
}

let result = time_it!("Operation", {
    // expensive computation
    42
});
```

### Custom assertions
```rust
macro_rules! assert_in_range {
    ($val:expr, $min:expr, $max:expr) => {
        assert!(
            $val >= $min && $val <= $max,
            "Value {} not in [{}, {}]",
            $val, $min, $max
        );
    };
}

assert_in_range!(50, 1, 100);
```

### Logging macro
```rust
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

log!(info, "Application started");
log!(error, "Failed: {}", error_msg);
```

## Types of Macros

### 1. Declarative Macros (`macro_rules!`)
- Pattern matching based
- Most common type
- This example focuses on these

### 2. Procedural Macros
- Function-like: `my_macro!(...)`
- Derive: `#[derive(MyTrait)]`
- Attribute: `#[my_attribute]`
- Require separate crate with `proc-macro = true`

### 3. Built-in Macros
- Provided by standard library
- `println!`, `vec!`, `format!`, etc.

## Macro Hygiene

Rust macros are **hygienic**, meaning:
- Variables in macros don't accidentally capture outer scope
- Macros create their own scope
- Prevents naming conflicts

```rust
macro_rules! my_macro {
    () => {
        let x = 100;  // This x is separate
        x
    };
}

let x = 50;
let result = my_macro!();
// x is still 50, not affected by macro
```

## Best Practices

1. **Use macros sparingly** - Functions are often better
2. **Document macro behavior** - Explain patterns and usage
3. **Keep macros simple** - Complex macros are hard to debug
4. **Use descriptive names** - Make intent clear
5. **Test macros** - Write unit tests for macro output
6. **Prefer functions** - Use macros only when necessary
7. **Consider proc macros** - For complex code generation

## When to Use Macros

**Good use cases:**
- Reducing repetitive boilerplate
- Creating DSLs
- Conditional compilation
- Code generation at compile time
- Custom syntax

**When to avoid:**
- Simple abstractions (use functions)
- Runtime logic (use functions/methods)
- Type abstraction (use generics)
- Complex logic (hard to debug)

## Debugging Macros

### See macro expansion
```bash
cargo rustc -- -Z unstable-options --pretty=expanded
```

### Use `cargo expand` (requires cargo-expand)
```bash
cargo install cargo-expand
cargo expand
```

### Debug with `dbg!`
```rust
macro_rules! my_macro {
    ($x:expr) => {
        dbg!($x)  // Shows value and location
    };
}
```

## Common Errors

### "no rules expected this token"
- Pattern doesn't match input
- Check fragment specifiers

### "recursion limit reached"
- Infinite macro recursion
- Set limit: `#![recursion_limit = "256"]`

### "macro expansion ignores token"
- Missing separators in repetition
- Check `,` or `;` between repeated items

### "expected expression, found X"
- Wrong fragment specifier
- Use correct type (expr, ty, ident, etc.)

## Advanced Patterns

### TT Muncher
```rust
macro_rules! count {
    () => { 0 };
    ($head:tt $($tail:tt)*) => {
        1 + count!($($tail)*)
    };
}
```

### Internal Rules
```rust
macro_rules! my_macro {
    // Public interface
    ($val:expr) => {
        my_macro!(@internal $val, 0)
    };

    // Internal rule (not meant for direct use)
    (@internal $val:expr, $default:expr) => {
        // implementation
    };
}
```

### Incremental TT Muncher
```rust
macro_rules! parse {
    // Base case
    (@parse [] [$($output:tt)*]) => {
        $($output)*
    };

    // Recursive case
    (@parse [$head:tt $($tail:tt)*] [$($output:tt)*]) => {
        parse!(@parse [$($tail)*] [$($output)* process!($head)])
    };
}
```

## Resources

- [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/)
- [Rust by Example - Macros](https://doc.rust-lang.org/rust-by-example/macros.html)
- [The Rust Reference - Macros](https://doc.rust-lang.org/reference/macros.html)
- [std::macro documentation](https://doc.rust-lang.org/std/macro.index.html)

## Examples in the Wild

### Popular crates using macros
- **serde**: `#[derive(Serialize, Deserialize)]`
- **clap**: Command-line argument parsing
- **lazy_static**: Static initialization
- **log**: Logging facade
- **assert_matches**: Advanced pattern matching assertions
