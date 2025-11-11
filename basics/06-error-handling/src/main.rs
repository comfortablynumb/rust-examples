// Error Handling in Rust
//
// Rust doesn't have exceptions. Instead, it uses the Result<T, E> type for
// recoverable errors and panic! for unrecoverable errors. This makes error
// handling explicit and forces you to handle potential failures.

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() {
    println!("=== Rust Error Handling Examples ===\n");

    // Example 1: panic! for unrecoverable errors
    println!("1. Panic (Unrecoverable Errors):");
    panic_examples();
    println!();

    // Example 2: Result<T, E> for recoverable errors
    println!("2. Result Type:");
    result_examples();
    println!();

    // Example 3: The ? operator
    println!("3. The ? Operator:");
    question_mark_operator();
    println!();

    // Example 4: Custom error types
    println!("4. Custom Error Types:");
    custom_errors();
    println!();

    // Example 5: Error propagation
    println!("5. Error Propagation:");
    error_propagation();
    println!();

    // Example 6: Multiple error types
    println!("6. Multiple Error Types:");
    multiple_error_types();
    println!();

    // Example 7: Option<T> for optional values
    println!("7. Option Type:");
    option_examples();
    println!();

    // Example 8: Combining Option and Result
    println!("8. Combining Option and Result:");
    combining_option_result();
    println!();
}

// Example 1: panic! - Unrecoverable errors
fn panic_examples() {
    // Uncomment to see panic in action (program will crash)
    // panic!("This is a panic!");

    // Common causes of panic:
    // 1. Explicit panic!
    // 2. Accessing out of bounds
    // 3. unwrap() on None or Err

    println!("  Panic is for unrecoverable errors");
    println!("  Use Result for recoverable errors instead");

    // Safe alternative to panicking
    let v = vec![1, 2, 3];
    match v.get(10) {
        Some(val) => println!("  Value: {}", val),
        None => println!("  Index out of bounds (handled safely)"),
    }
}

// Example 2: Result<T, E> basics
fn result_examples() {
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    // Handling Result with match
    match divide(10.0, 2.0) {
        Ok(result) => println!("  10 / 2 = {}", result),
        Err(e) => println!("  Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("  Result: {}", result),
        Err(e) => println!("  Error: {}", e),
    }

    // unwrap() - panics if Err
    let result = divide(20.0, 4.0).unwrap();
    println!("  Unwrapped result: {}", result);

    // expect() - like unwrap but with custom message
    let result = divide(15.0, 3.0).expect("Division failed");
    println!("  Expected result: {}", result);

    // unwrap_or() - provides default value
    let result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("  With default: {}", result);

    // unwrap_or_else() - compute default with closure
    let result = divide(10.0, 0.0).unwrap_or_else(|e| {
        println!("  Error occurred: {}, using default", e);
        -1.0
    });
    println!("  Computed default: {}", result);
}

// Example 3: The ? operator for error propagation
fn question_mark_operator() {
    fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
        // ? unwraps Ok or returns Err early
        let num = s.parse::<i32>()?;
        Ok(num * 2)
    }

    match parse_and_double("21") {
        Ok(result) => println!("  Parsed and doubled: {}", result),
        Err(e) => println!("  Parse error: {}", e),
    }

    match parse_and_double("not a number") {
        Ok(result) => println!("  Result: {}", result),
        Err(e) => println!("  Parse error: {}", e),
    }

    // Chaining with ?
    fn complex_operation(s: &str) -> Result<i32, ParseIntError> {
        let num1 = s.parse::<i32>()?;
        let num2 = "10".parse::<i32>()?;
        Ok(num1 + num2)
    }

    match complex_operation("5") {
        Ok(result) => println!("  Complex result: {}", result),
        Err(e) => println!("  Error: {}", e),
    }
}

// Example 4: Custom error types
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => {
                write!(f, "Cannot take square root of negative number")
            }
            MathError::Overflow => write!(f, "Arithmetic overflow occurred"),
        }
    }
}

impl std::error::Error for MathError {}

fn custom_errors() {
    fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }

    fn safe_sqrt(x: f64) -> Result<f64, MathError> {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("  Division: {}", result),
        Err(e) => println!("  Error: {}", e),
    }

    match safe_sqrt(-4.0) {
        Ok(result) => println!("  Square root: {}", result),
        Err(e) => println!("  Error: {}", e),
    }
}

// Example 5: Error propagation with ?
fn error_propagation() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut file = File::open("username.txt")?;
        let mut username = String::new();
        file.read_to_string(&mut username)?;
        Ok(username)
    }

    // Shorter version using method chaining
    fn read_username_short() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("username.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    match read_username_from_file() {
        Ok(name) => println!("  Username: {}", name),
        Err(e) => println!(
            "  Could not read username: {} (expected - file doesn't exist)",
            e.kind()
        ),
    }
}

// Example 6: Handling multiple error types
fn multiple_error_types() {
    // Using a common error type
    #[derive(Debug)]
    enum AppError {
        Io(io::Error),
        Parse(ParseIntError),
    }

    impl std::fmt::Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                AppError::Io(e) => write!(f, "IO error: {}", e),
                AppError::Parse(e) => write!(f, "Parse error: {}", e),
            }
        }
    }

    impl From<io::Error> for AppError {
        fn from(error: io::Error) -> Self {
            AppError::Io(error)
        }
    }

    impl From<ParseIntError> for AppError {
        fn from(error: ParseIntError) -> Self {
            AppError::Parse(error)
        }
    }

    fn process_data(data: &str) -> Result<i32, AppError> {
        let num = data.parse::<i32>()?; // Automatically converted to AppError
        Ok(num * 2)
    }

    match process_data("42") {
        Ok(result) => println!("  Processed data: {}", result),
        Err(e) => println!("  Error: {}", e),
    }

    match process_data("not a number") {
        Ok(result) => println!("  Processed data: {}", result),
        Err(e) => println!("  Error: {}", e),
    }
}

// Example 7: Option<T> for optional values
fn option_examples() {
    fn find_word(text: &str, word: &str) -> Option<usize> {
        text.find(word)
    }

    let text = "Hello, Rust!";

    // Handling Option with match
    match find_word(text, "Rust") {
        Some(pos) => println!("  Found 'Rust' at position {}", pos),
        None => println!("  Word not found"),
    }

    // if let for simple cases
    if let Some(pos) = find_word(text, "Hello") {
        println!("  Found 'Hello' at position {}", pos);
    }

    // unwrap_or
    let pos = find_word(text, "Python").unwrap_or(0);
    println!("  Position with default: {}", pos);

    // map - transform the value inside Option
    let doubled = Some(5).map(|x| x * 2);
    println!("  Doubled: {:?}", doubled);

    // and_then - chain operations that return Option
    let result = Some(5)
        .and_then(|x| Some(x * 2))
        .and_then(|x| if x > 5 { Some(x) } else { None });
    println!("  Chained: {:?}", result);

    // or - provide alternative Option
    let result = None.or(Some(42));
    println!("  With alternative: {:?}", result);
}

// Example 8: Combining Option and Result
fn combining_option_result() {
    fn get_user_age(user_id: u32) -> Option<u32> {
        // Simulate database lookup
        if user_id == 1 {
            Some(25)
        } else {
            None
        }
    }

    fn validate_age(age: u32) -> Result<u32, String> {
        if age >= 18 {
            Ok(age)
        } else {
            Err(String::from("User is under 18"))
        }
    }

    fn check_user_access(user_id: u32) -> Result<String, String> {
        let age = get_user_age(user_id).ok_or("User not found")?;
        validate_age(age)?;
        Ok(String::from("Access granted"))
    }

    match check_user_access(1) {
        Ok(msg) => println!("  User 1: {}", msg),
        Err(e) => println!("  User 1: {}", e),
    }

    match check_user_access(2) {
        Ok(msg) => println!("  User 2: {}", msg),
        Err(e) => println!("  User 2: {}", e),
    }

    // Converting between Option and Result
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("No value");
    println!("  Option to Result: {:?}", res);

    let res: Result<i32, String> = Ok(100);
    let opt: Option<i32> = res.ok();
    println!("  Result to Option: {:?}", opt);
}
