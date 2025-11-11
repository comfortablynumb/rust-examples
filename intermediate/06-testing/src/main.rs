#![allow(dead_code)]
#![allow(unused_variables)]

//! # Comprehensive Testing in Rust
//!
//! This module demonstrates various testing techniques in Rust including:
//! - Unit tests
//! - Integration tests
//! - Documentation tests
//! - Property-based testing
//! - Mocking
//! - Benchmarking
//!
//! ## Example: Calculator
//!
//! ```
//! use testing::Calculator;
//!
//! let calc = Calculator::new();
//! assert_eq!(calc.add(2, 3), 5);
//! assert_eq!(calc.subtract(10, 4), 6);
//! ```

use std::collections::HashMap;

// ============================================================================
// SECTION 1: BASIC STRUCTURES FOR TESTING
// ============================================================================

/// A simple calculator for demonstrating unit tests.
///
/// # Examples
///
/// Basic arithmetic operations:
///
/// ```
/// use testing::Calculator;
///
/// let calc = Calculator::new();
/// assert_eq!(calc.multiply(4, 5), 20);
/// ```
///
/// Division with error handling:
///
/// ```
/// use testing::Calculator;
///
/// let calc = Calculator::new();
/// assert_eq!(calc.divide(10, 2), Ok(5));
/// assert_eq!(calc.divide(10, 0), Err("Division by zero"));
/// ```
#[derive(Debug, Clone)]
pub struct Calculator {
    history: Vec<String>,
}

impl Calculator {
    /// Creates a new Calculator instance
    pub fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }

    /// Adds two numbers
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    /// Subtracts b from a
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    /// Multiplies two numbers
    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }

    /// Divides a by b, returns error if b is zero
    pub fn divide(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            Err("Division by zero")
        } else {
            Ok(a / b)
        }
    }

    /// Private helper function to format operation
    fn format_operation(&self, op: &str, a: i32, b: i32, result: i32) -> String {
        format!("{} {} {} = {}", a, op, b, result)
    }

    /// Computes factorial (demonstrates recursive testing)
    pub fn factorial(&self, n: u32) -> u64 {
        match n {
            0 | 1 => 1,
            _ => n as u64 * self.factorial(n - 1),
        }
    }

    /// Gets calculation history
    pub fn history(&self) -> &[String] {
        &self.history
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SECTION 2: USER SERVICE (FOR TESTING VALIDATION AND ERRORS)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub age: u8,
}

impl User {
    pub fn new(id: u64, username: String, email: String, age: u8) -> Result<Self, String> {
        if username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        if age < 13 {
            return Err("User must be at least 13 years old".to_string());
        }
        Ok(User {
            id,
            username,
            email,
            age,
        })
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

pub struct UserService {
    users: HashMap<u64, User>,
    next_id: u64,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_user(&mut self, username: String, email: String, age: u8) -> Result<u64, String> {
        let user = User::new(self.next_id, username, email, age)?;
        let id = user.id;
        self.users.insert(id, user);
        self.next_id += 1;
        Ok(id)
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn delete_user(&mut self, id: u64) -> bool {
        self.users.remove(&id).is_some()
    }

    pub fn count(&self) -> usize {
        self.users.len()
    }

    pub fn find_by_username(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }
}

impl Default for UserService {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SECTION 3: FILE PROCESSOR (FOR TESTING WITH TRAITS)
// ============================================================================

pub trait DataProcessor {
    fn process(&self, data: &str) -> String;
    fn validate(&self, data: &str) -> bool;
}

pub struct FileProcessor {
    prefix: String,
}

impl FileProcessor {
    pub fn new(prefix: String) -> Self {
        FileProcessor { prefix }
    }
}

impl DataProcessor for FileProcessor {
    fn process(&self, data: &str) -> String {
        format!("{}: {}", self.prefix, data.to_uppercase())
    }

    fn validate(&self, data: &str) -> bool {
        !data.is_empty() && data.len() <= 1000
    }
}

// ============================================================================
// SECTION 4: TEMPERATURE CONVERTER (FOR DOC TESTS)
// ============================================================================

/// Converts temperature from Celsius to Fahrenheit.
///
/// # Examples
///
/// ```
/// use testing::celsius_to_fahrenheit;
///
/// let fahrenheit = celsius_to_fahrenheit(0.0);
/// assert_eq!(fahrenheit, 32.0);
///
/// let fahrenheit = celsius_to_fahrenheit(100.0);
/// assert_eq!(fahrenheit, 212.0);
/// ```
///
/// Negative temperatures:
///
/// ```
/// use testing::celsius_to_fahrenheit;
///
/// let fahrenheit = celsius_to_fahrenheit(-40.0);
/// assert_eq!(fahrenheit, -40.0);
/// ```
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Converts temperature from Fahrenheit to Celsius.
///
/// # Examples
///
/// ```
/// use testing::fahrenheit_to_celsius;
///
/// let celsius = fahrenheit_to_celsius(32.0);
/// assert_eq!(celsius, 0.0);
///
/// let celsius = fahrenheit_to_celsius(212.0);
/// assert_eq!(celsius, 100.0);
/// ```
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// ============================================================================
// SECTION 5: STRING UTILITIES (FOR TESTING STRINGS)
// ============================================================================

/// Reverses a string.
///
/// # Examples
///
/// ```
/// use testing::reverse_string;
///
/// assert_eq!(reverse_string("hello"), "olleh");
/// assert_eq!(reverse_string(""), "");
/// ```
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Checks if a string is a palindrome.
///
/// # Examples
///
/// ```
/// use testing::is_palindrome;
///
/// assert!(is_palindrome("racecar"));
/// assert!(is_palindrome(""));
/// assert!(!is_palindrome("hello"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    s == s.chars().rev().collect::<String>()
}

// ============================================================================
// SECTION 6: VECTOR UTILITIES (FOR TESTING WITH GENERICS)
// ============================================================================

pub fn find_max<T: Ord + Copy>(slice: &[T]) -> Option<T> {
    if slice.is_empty() {
        None
    } else {
        Some(*slice.iter().max().unwrap())
    }
}

pub fn find_min<T: Ord + Copy>(slice: &[T]) -> Option<T> {
    if slice.is_empty() {
        None
    } else {
        Some(*slice.iter().min().unwrap())
    }
}

// ============================================================================
// SECTION 7: BANKING EXAMPLE (FOR TESTING WITH STATE)
// ============================================================================

#[derive(Debug, Clone)]
pub struct BankAccount {
    balance: f64,
    transactions: Vec<f64>,
}

impl BankAccount {
    pub fn new() -> Self {
        BankAccount {
            balance: 0.0,
            transactions: Vec::new(),
        }
    }

    pub fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive".to_string());
        }
        self.balance += amount;
        self.transactions.push(amount);
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        self.transactions.push(-amount);
        Ok(())
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }
}

impl Default for BankAccount {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

fn main() {
    println!("=== Rust Testing Examples ===\n");

    // Calculator demo
    let calc = Calculator::new();
    println!("Calculator:");
    println!("  2 + 3 = {}", calc.add(2, 3));
    println!("  10 - 4 = {}", calc.subtract(10, 4));
    println!("  4 * 5 = {}", calc.multiply(4, 5));
    println!("  10 / 2 = {:?}", calc.divide(10, 2));
    println!("  5! = {}", calc.factorial(5));

    // User service demo
    println!("\nUser Service:");
    let mut user_service = UserService::new();
    match user_service.create_user("alice".to_string(), "alice@example.com".to_string(), 25) {
        Ok(id) => {
            println!("  Created user with ID: {}", id);
            if let Some(user) = user_service.get_user(id) {
                println!("  User: {:?}", user);
                println!("  Is adult: {}", user.is_adult());
            }
        }
        Err(e) => println!("  Error: {}", e),
    }

    // File processor demo
    println!("\nFile Processor:");
    let processor = FileProcessor::new("PROCESSED".to_string());
    let result = processor.process("hello world");
    println!("  Result: {}", result);
    println!("  Valid: {}", processor.validate("hello"));

    // Temperature conversion demo
    println!("\nTemperature Conversion:");
    println!("  0°C = {}°F", celsius_to_fahrenheit(0.0));
    println!("  100°C = {}°F", celsius_to_fahrenheit(100.0));
    println!("  32°F = {}°C", fahrenheit_to_celsius(32.0));

    // String utilities demo
    println!("\nString Utilities:");
    println!("  Reverse 'hello': {}", reverse_string("hello"));
    println!("  Is 'racecar' palindrome: {}", is_palindrome("racecar"));
    println!("  Is 'hello' palindrome: {}", is_palindrome("hello"));

    // Vector utilities demo
    println!("\nVector Utilities:");
    let numbers = vec![5, 2, 8, 1, 9];
    println!("  Max of {:?}: {:?}", numbers, find_max(&numbers));
    println!("  Min of {:?}: {:?}", numbers, find_min(&numbers));

    // Bank account demo
    println!("\nBank Account:");
    let mut account = BankAccount::new();
    account.deposit(100.0).unwrap();
    println!("  After deposit of $100: ${}", account.balance());
    account.withdraw(30.0).unwrap();
    println!("  After withdrawal of $30: ${}", account.balance());
    println!("  Total transactions: {}", account.transaction_count());

    println!("\n=== Run 'cargo test' to execute all tests ===");
}

// ============================================================================
// TESTS MODULE
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // TEST 1-5: BASIC UNIT TESTS WITH ASSERTIONS
    // ========================================================================

    #[test]
    fn test_calculator_add() {
        let calc = Calculator::new();
        assert_eq!(calc.add(2, 3), 5);
        assert_eq!(calc.add(-1, 1), 0);
        assert_eq!(calc.add(0, 0), 0);
    }

    #[test]
    fn test_calculator_subtract() {
        let calc = Calculator::new();
        assert_eq!(calc.subtract(10, 4), 6);
        assert_eq!(calc.subtract(5, 5), 0);
        assert_eq!(calc.subtract(0, 10), -10);
    }

    #[test]
    fn test_calculator_multiply() {
        let calc = Calculator::new();
        assert_eq!(calc.multiply(4, 5), 20);
        assert_eq!(calc.multiply(0, 100), 0);
        assert_eq!(calc.multiply(-2, 3), -6);
    }

    #[test]
    fn test_calculator_divide_success() {
        let calc = Calculator::new();
        assert_eq!(calc.divide(10, 2), Ok(5));
        assert_eq!(calc.divide(9, 3), Ok(3));
        assert_eq!(calc.divide(-10, 2), Ok(-5));
    }

    #[test]
    fn test_calculator_divide_by_zero() {
        let calc = Calculator::new();
        assert_eq!(calc.divide(10, 0), Err("Division by zero"));
        assert!(calc.divide(0, 0).is_err());
    }

    // ========================================================================
    // TEST 6-8: TESTING WITH ASSERT!, ASSERT_NE!
    // ========================================================================

    #[test]
    fn test_assertions_various() {
        let calc = Calculator::new();
        let result = calc.add(2, 2);

        assert!(result > 0, "Result should be positive");
        assert!(result == 4, "Result should equal 4");
        assert_ne!(result, 5, "Result should not equal 5");
    }

    #[test]
    fn test_user_validation() {
        let user = User::new(1, "alice".to_string(), "alice@test.com".to_string(), 25);
        assert!(user.is_ok(), "User creation should succeed");

        let user = user.unwrap();
        assert_eq!(user.username, "alice");
        assert!(user.email.contains('@'));
        assert!(user.is_adult());
    }

    #[test]
    fn test_user_validation_failures() {
        // Empty username
        let result = User::new(1, "".to_string(), "test@test.com".to_string(), 20);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Username cannot be empty");

        // Invalid email
        let result = User::new(1, "bob".to_string(), "invalid-email".to_string(), 20);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid email format");

        // Under age
        let result = User::new(1, "kid".to_string(), "kid@test.com".to_string(), 10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User must be at least 13 years old");
    }

    // ========================================================================
    // TEST 9-11: TESTING PANICS WITH #[should_panic]
    // ========================================================================

    #[test]
    #[should_panic]
    fn test_panic_basic() {
        panic!("This test should panic");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic_with_message() {
        let v = vec![1, 2, 3];
        let _ = v[99]; // This will panic with "index out of bounds"
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_panic_on_assertion() {
        assert!(false, "assertion failed: this should panic");
    }

    // ========================================================================
    // TEST 12-14: IGNORED TESTS
    // ========================================================================

    #[test]
    #[ignore]
    fn expensive_test() {
        // This test is ignored by default, run with: cargo test -- --ignored
        let calc = Calculator::new();
        let result = calc.factorial(20);
        assert!(result > 0);
    }

    #[test]
    #[ignore = "requires network connection"]
    fn test_network_feature() {
        // This test would require network access
        assert!(true);
    }

    #[test]
    #[ignore = "slow computation"]
    fn test_heavy_computation() {
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        assert_eq!(sum, 499500);
    }

    // ========================================================================
    // TEST 15-17: TESTING PRIVATE FUNCTIONS
    // ========================================================================

    #[test]
    fn test_private_function_format_operation() {
        let calc = Calculator::new();
        // Can access private functions within the same module
        let result = calc.format_operation("+", 2, 3, 5);
        assert_eq!(result, "2 + 3 = 5");
    }

    #[test]
    fn test_calculator_factorial() {
        let calc = Calculator::new();
        assert_eq!(calc.factorial(0), 1);
        assert_eq!(calc.factorial(1), 1);
        assert_eq!(calc.factorial(5), 120);
        assert_eq!(calc.factorial(10), 3628800);
    }

    #[test]
    fn test_default_implementations() {
        let calc = Calculator::default();
        assert_eq!(calc.history().len(), 0);

        let service = UserService::default();
        assert_eq!(service.count(), 0);

        let account = BankAccount::default();
        assert_eq!(account.balance(), 0.0);
    }

    // ========================================================================
    // TEST 18-22: TESTING WITH SETUP AND TEARDOWN (FIXTURES)
    // ========================================================================

    struct TestFixture {
        user_service: UserService,
        test_users: Vec<u64>,
    }

    impl TestFixture {
        fn setup() -> Self {
            let mut user_service = UserService::new();
            let mut test_users = Vec::new();

            // Create some test users
            let id1 = user_service
                .create_user("alice".to_string(), "alice@test.com".to_string(), 25)
                .unwrap();
            let id2 = user_service
                .create_user("bob".to_string(), "bob@test.com".to_string(), 30)
                .unwrap();
            let id3 = user_service
                .create_user("charlie".to_string(), "charlie@test.com".to_string(), 15)
                .unwrap();

            test_users.push(id1);
            test_users.push(id2);
            test_users.push(id3);

            TestFixture {
                user_service,
                test_users,
            }
        }
    }

    #[test]
    fn test_user_service_with_fixture() {
        let fixture = TestFixture::setup();
        assert_eq!(fixture.user_service.count(), 3);
        assert_eq!(fixture.test_users.len(), 3);
    }

    #[test]
    fn test_user_service_get_user() {
        let fixture = TestFixture::setup();
        let user = fixture.user_service.get_user(fixture.test_users[0]);
        assert!(user.is_some());
        assert_eq!(user.unwrap().username, "alice");
    }

    #[test]
    fn test_user_service_delete_user() {
        let mut fixture = TestFixture::setup();
        let initial_count = fixture.user_service.count();

        let deleted = fixture.user_service.delete_user(fixture.test_users[0]);
        assert!(deleted);
        assert_eq!(fixture.user_service.count(), initial_count - 1);

        let user = fixture.user_service.get_user(fixture.test_users[0]);
        assert!(user.is_none());
    }

    #[test]
    fn test_user_service_find_by_username() {
        let fixture = TestFixture::setup();
        let user = fixture.user_service.find_by_username("bob");
        assert!(user.is_some());
        assert_eq!(user.unwrap().email, "bob@test.com");

        let user = fixture.user_service.find_by_username("nonexistent");
        assert!(user.is_none());
    }

    #[test]
    fn test_user_is_adult() {
        let fixture = TestFixture::setup();
        let alice = fixture.user_service.get_user(fixture.test_users[0]).unwrap();
        let charlie = fixture.user_service.get_user(fixture.test_users[2]).unwrap();

        assert!(alice.is_adult());
        assert!(!charlie.is_adult());
    }

    // ========================================================================
    // TEST 23-25: TESTING FILE PROCESSOR (TRAIT TESTING)
    // ========================================================================

    #[test]
    fn test_file_processor_process() {
        let processor = FileProcessor::new("PREFIX".to_string());
        let result = processor.process("hello");
        assert_eq!(result, "PREFIX: HELLO");
    }

    #[test]
    fn test_file_processor_validate() {
        let processor = FileProcessor::new("TEST".to_string());
        assert!(processor.validate("valid data"));
        assert!(!processor.validate(""));

        let long_string = "a".repeat(1001);
        assert!(!processor.validate(&long_string));
    }

    #[test]
    fn test_data_processor_trait() {
        let processor: Box<dyn DataProcessor> = Box::new(FileProcessor::new("BOX".to_string()));
        let result = processor.process("test");
        assert!(result.contains("BOX"));
        assert!(result.contains("TEST"));
    }

    // ========================================================================
    // TEST 26-28: TEMPERATURE CONVERSION TESTS
    // ========================================================================

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
    }

    #[test]
    fn test_temperature_conversion_roundtrip() {
        let celsius = 25.0;
        let fahrenheit = celsius_to_fahrenheit(celsius);
        let back_to_celsius = fahrenheit_to_celsius(fahrenheit);
        assert!((celsius - back_to_celsius).abs() < 0.0001);
    }

    // ========================================================================
    // TEST 29-31: STRING UTILITY TESTS
    // ========================================================================

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string(""), "");
        assert_eq!(reverse_string("a"), "a");
        assert_eq!(reverse_string("Rust"), "tsuR");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn test_palindrome_case_insensitive() {
        assert!(is_palindrome("RaceCar"));
        assert!(is_palindrome("No lemon, no melon"));
    }

    // ========================================================================
    // TEST 32-34: VECTOR UTILITY TESTS (GENERICS)
    // ========================================================================

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[1, 5, 3, 2]), Some(5));
        assert_eq!(find_max(&[-1, -5, -3]), Some(-1));
        assert_eq!(find_max(&[42]), Some(42));
        assert_eq!(find_max::<i32>(&[]), None);
    }

    #[test]
    fn test_find_min() {
        assert_eq!(find_min(&[1, 5, 3, 2]), Some(1));
        assert_eq!(find_min(&[-1, -5, -3]), Some(-5));
        assert_eq!(find_min(&[42]), Some(42));
        assert_eq!(find_min::<i32>(&[]), None);
    }

    #[test]
    fn test_find_with_chars() {
        assert_eq!(find_max(&['a', 'z', 'm']), Some('z'));
        assert_eq!(find_min(&['a', 'z', 'm']), Some('a'));
    }

    // ========================================================================
    // TEST 35-37: BANK ACCOUNT TESTS (STATEFUL TESTING)
    // ========================================================================

    #[test]
    fn test_bank_account_deposit() {
        let mut account = BankAccount::new();
        assert_eq!(account.balance(), 0.0);

        account.deposit(100.0).unwrap();
        assert_eq!(account.balance(), 100.0);

        account.deposit(50.0).unwrap();
        assert_eq!(account.balance(), 150.0);
        assert_eq!(account.transaction_count(), 2);
    }

    #[test]
    fn test_bank_account_withdraw() {
        let mut account = BankAccount::new();
        account.deposit(100.0).unwrap();

        account.withdraw(30.0).unwrap();
        assert_eq!(account.balance(), 70.0);
        assert_eq!(account.transaction_count(), 2);
    }

    #[test]
    fn test_bank_account_insufficient_funds() {
        let mut account = BankAccount::new();
        account.deposit(50.0).unwrap();

        let result = account.withdraw(100.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient funds");
        assert_eq!(account.balance(), 50.0);
    }

    // ========================================================================
    // TEST 38-40: PARAMETERIZED TESTS (TABLE-DRIVEN TESTS)
    // ========================================================================

    #[test]
    fn test_calculator_add_parameterized() {
        let calc = Calculator::new();
        let test_cases = vec![
            (2, 3, 5),
            (0, 0, 0),
            (-1, 1, 0),
            (100, 200, 300),
            (-5, -5, -10),
        ];

        for (a, b, expected) in test_cases {
            assert_eq!(
                calc.add(a, b),
                expected,
                "Failed: {} + {} should equal {}",
                a, b, expected
            );
        }
    }

    #[test]
    fn test_calculator_multiply_parameterized() {
        let calc = Calculator::new();
        let test_cases = vec![
            (2, 3, 6),
            (0, 100, 0),
            (-2, 3, -6),
            (4, 5, 20),
            (-1, -1, 1),
        ];

        for (a, b, expected) in test_cases {
            assert_eq!(
                calc.multiply(a, b),
                expected,
                "Failed: {} * {} should equal {}",
                a, b, expected
            );
        }
    }

    #[test]
    fn test_palindrome_parameterized() {
        let test_cases = vec![
            ("racecar", true),
            ("hello", false),
            ("A man a plan a canal Panama", true),
            ("", true),
            ("a", true),
            ("ab", false),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                is_palindrome(input),
                expected,
                "Failed: is_palindrome({:?}) should be {}",
                input,
                expected
            );
        }
    }

    // ========================================================================
    // TEST 41-43: CUSTOM ASSERTIONS AND HELPER FUNCTIONS
    // ========================================================================

    fn assert_almost_equal(a: f64, b: f64, epsilon: f64) {
        assert!(
            (a - b).abs() < epsilon,
            "Values are not almost equal: {} vs {} (epsilon: {})",
            a, b, epsilon
        );
    }

    #[test]
    fn test_custom_assertion_temperature() {
        let celsius = 37.0; // Body temperature
        let fahrenheit = celsius_to_fahrenheit(celsius);
        assert_almost_equal(fahrenheit, 98.6, 0.1);
    }

    #[test]
    fn test_custom_assertion_division() {
        let calc = Calculator::new();
        let result = calc.divide(10, 3).unwrap() as f64;
        assert_almost_equal(result, 3.0, 1.0);
    }

    #[test]
    fn test_result_unwrapping() {
        let calc = Calculator::new();
        let result = calc.divide(10, 2);

        // Test Result methods
        assert!(result.is_ok());
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 5);
    }

    // ========================================================================
    // NESTED TEST MODULE (ORGANIZATION)
    // ========================================================================

    mod calculator_edge_cases {
        use super::*;

        #[test]
        fn test_large_numbers() {
            let calc = Calculator::new();
            assert_eq!(calc.add(i32::MAX - 1, 1), i32::MAX);
            assert_eq!(calc.subtract(i32::MIN + 1, 1), i32::MIN);
        }

        #[test]
        fn test_zero_operations() {
            let calc = Calculator::new();
            assert_eq!(calc.multiply(0, 1000000), 0);
            assert_eq!(calc.multiply(1000000, 0), 0);
        }

        #[test]
        fn test_negative_operations() {
            let calc = Calculator::new();
            assert_eq!(calc.multiply(-1, -1), 1);
            assert_eq!(calc.multiply(-5, -5), 25);
        }
    }

    mod bank_account_edge_cases {
        use super::*;

        #[test]
        fn test_negative_deposit() {
            let mut account = BankAccount::new();
            let result = account.deposit(-50.0);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Deposit amount must be positive");
        }

        #[test]
        fn test_negative_withdrawal() {
            let mut account = BankAccount::new();
            account.deposit(100.0).unwrap();
            let result = account.withdraw(-50.0);
            assert!(result.is_err());
        }

        #[test]
        fn test_zero_operations() {
            let mut account = BankAccount::new();
            assert!(account.deposit(0.0).is_err());
            assert!(account.withdraw(0.0).is_err());
        }
    }
}

// ============================================================================
// PROPERTY-BASED TESTING WITH PROPTEST
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_add_commutative(a in -1000i32..1000i32, b in -1000i32..1000i32) {
            let calc = Calculator::new();
            prop_assert_eq!(calc.add(a, b), calc.add(b, a));
        }

        #[test]
        fn test_add_associative(a in -100i32..100i32, b in -100i32..100i32, c in -100i32..100i32) {
            let calc = Calculator::new();
            let left = calc.add(calc.add(a, b), c);
            let right = calc.add(a, calc.add(b, c));
            prop_assert_eq!(left, right);
        }

        #[test]
        fn test_multiply_commutative(a in -1000i32..1000i32, b in -1000i32..1000i32) {
            let calc = Calculator::new();
            prop_assert_eq!(calc.multiply(a, b), calc.multiply(b, a));
        }

        #[test]
        fn test_reverse_string_involution(s in ".*") {
            let reversed_once = reverse_string(&s);
            let reversed_twice = reverse_string(&reversed_once);
            prop_assert_eq!(s, reversed_twice);
        }

        #[test]
        fn test_temperature_conversion_roundtrip(celsius in -100.0f64..100.0f64) {
            let fahrenheit = celsius_to_fahrenheit(celsius);
            let back = fahrenheit_to_celsius(fahrenheit);
            prop_assert!((celsius - back).abs() < 0.0001);
        }

        #[test]
        fn test_string_length_preserved(s in ".*") {
            let reversed = reverse_string(&s);
            prop_assert_eq!(s.len(), reversed.len());
        }

        #[test]
        fn test_divide_then_multiply(a in 1i32..1000i32, b in 1i32..100i32) {
            let calc = Calculator::new();
            let divided = calc.divide(a * b, b).unwrap();
            prop_assert_eq!(divided, a);
        }
    }
}

// ============================================================================
// MOCKING WITH MOCKALL
// ============================================================================

#[cfg(test)]
mod mock_tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    #[automock]
    pub trait Database {
        fn save_user(&mut self, user: &User) -> Result<(), String>;
        fn find_user(&self, id: u64) -> Option<User>;
        fn delete_user(&mut self, id: u64) -> bool;
    }

    #[automock]
    pub trait EmailService {
        fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
    }

    #[test]
    fn test_database_mock_save() {
        let mut mock_db = MockDatabase::new();
        let user = User::new(1, "test".to_string(), "test@test.com".to_string(), 20).unwrap();

        mock_db
            .expect_save_user()
            .times(1)
            .returning(|_| Ok(()));

        let result = mock_db.save_user(&user);
        assert!(result.is_ok());
    }

    #[test]
    fn test_database_mock_find() {
        let mut mock_db = MockDatabase::new();
        let expected_user = User::new(1, "alice".to_string(), "alice@test.com".to_string(), 25).unwrap();

        mock_db
            .expect_find_user()
            .with(eq(1))
            .times(1)
            .return_const(Some(expected_user.clone()));

        let result = mock_db.find_user(1);
        assert!(result.is_some());
        assert_eq!(result.unwrap().username, "alice");
    }

    #[test]
    fn test_email_service_mock() {
        let mut mock_email = MockEmailService::new();

        mock_email
            .expect_send_email()
            .with(eq("user@test.com"), eq("Welcome"), always())
            .times(1)
            .returning(|_, _, _| Ok(()));

        let result = mock_email.send_email("user@test.com", "Welcome", "Welcome to our service");
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_with_failure() {
        let mut mock_db = MockDatabase::new();

        mock_db
            .expect_save_user()
            .times(1)
            .returning(|_| Err("Database error".to_string()));

        let user = User::new(1, "test".to_string(), "test@test.com".to_string(), 20).unwrap();
        let result = mock_db.save_user(&user);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Database error");
    }

    #[test]
    fn test_mock_call_count() {
        let mut mock_email = MockEmailService::new();

        mock_email
            .expect_send_email()
            .times(3)
            .returning(|_, _, _| Ok(()));

        // Send 3 emails
        for i in 0..3 {
            let result = mock_email.send_email(
                &format!("user{}@test.com", i),
                "Subject",
                "Body"
            );
            assert!(result.is_ok());
        }
    }
}
