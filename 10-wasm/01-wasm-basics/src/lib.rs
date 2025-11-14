use wasm_bindgen::prelude::*;

/// Add two numbers together
/// This function is exported to JavaScript and can be called from JS
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Greet a person by name
/// Demonstrates string handling across the Rust/JS boundary
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to WebAssembly with Rust.", name)
}

/// Calculate factorial recursively
/// Shows more complex computation in WASM
#[wasm_bindgen]
pub fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// Fibonacci sequence
/// Demonstrates performance of WASM for computational tasks
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Check if a number is prime
/// Example of boolean return type
#[wasm_bindgen]
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Reverse a string
/// Demonstrates string manipulation in WASM
#[wasm_bindgen]
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Count vowels in a string
#[wasm_bindgen]
pub fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_greet() {
        assert_eq!(
            greet("World"),
            "Hello, World! Welcome to WebAssembly with Rust."
        );
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(7));
        assert!(!is_prime(8));
    }
}
