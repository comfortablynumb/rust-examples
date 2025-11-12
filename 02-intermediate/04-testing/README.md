# Testing

This example demonstrates comprehensive testing in Rust, showcasing the powerful testing ecosystem and best practices.

## Overview

Rust has first-class support for testing built into the language and toolchain. The `cargo test` command compiles and runs your tests, and the test framework provides everything you need for unit tests, integration tests, documentation tests, benchmarks, property-based testing, and mocking.

Testing in Rust is designed to be:
- **Easy to write** - simple attribute macros to mark tests
- **Fast to run** - tests run in parallel by default
- **Integrated** - built into Cargo and the language
- **Comprehensive** - supports multiple testing paradigms
- **Safe** - type system helps prevent common testing errors

## Concepts Covered

### 1. Unit Tests
- Tests placed in the same file as the code
- Uses `#[cfg(test)]` module attribute
- Tests private functions and implementation details
- Run with `cargo test`
- Fast and focused on individual units

### 2. Integration Tests
- Tests in the `tests/` directory
- Each file compiled as separate crate
- Test public API from external perspective
- Ensure components work together
- Run with `cargo test` or `cargo test --test <name>`

### 3. Documentation Tests
- Code examples in `///` doc comments
- Verified to compile and run
- Keeps documentation accurate
- Serves as examples and tests
- Run with `cargo test --doc`

### 4. Test Attributes
- `#[test]` - marks a function as a test
- `#[should_panic]` - expects function to panic
- `#[should_panic(expected = "text")]` - expects specific panic message
- `#[ignore]` - skips test by default
- `#[ignore = "reason"]` - skips with explanation

### 5. Assertions
- `assert!()` - boolean condition
- `assert_eq!()` - equality comparison
- `assert_ne!()` - inequality comparison
- Custom messages with format arguments
- Panics on failure with helpful output

### 6. Test Organization
- Nested test modules for grouping
- Test fixtures for setup/teardown
- Helper functions for common assertions
- Table-driven/parameterized tests
- Edge case testing in separate modules

### 7. Property-Based Testing
- Using `proptest` crate
- Generate random test inputs
- Verify properties hold for all inputs
- Finds edge cases automatically
- Great for testing mathematical properties

### 8. Mocking
- Using `mockall` crate
- Mock traits and interfaces
- Verify function calls and arguments
- Simulate dependencies
- Test error conditions

### 9. Benchmarking
- Using `criterion` crate
- Statistical analysis of performance
- Compare different implementations
- Detect performance regressions
- HTML reports with graphs

## Running the Example

### Run the main program
```bash
cargo run
```

### Run all tests
```bash
cargo test
```

### Run tests with output
```bash
cargo test -- --nocapture
```

### Run specific test
```bash
cargo test test_calculator_add
```

### Run tests matching pattern
```bash
cargo test calculator
```

### Run ignored tests
```bash
cargo test -- --ignored
```

### Run tests and ignored tests
```bash
cargo test -- --include-ignored
```

### Run integration tests only
```bash
cargo test --test integration_test
```

### Run doc tests only
```bash
cargo test --doc
```

### Run benchmarks
```bash
cargo bench
```

### Run specific benchmark
```bash
cargo bench calculator
```

### Run tests with multiple threads
```bash
cargo test -- --test-threads=4
```

### Run tests sequentially
```bash
cargo test -- --test-threads=1
```

## Test Organization

This example demonstrates several organizational patterns:

### File Structure
```
06-testing/
├── src/
│   └── main.rs          # Unit tests with #[cfg(test)]
├── tests/
│   └── integration_test.rs  # Integration tests
├── benches/
│   └── benchmarks.rs    # Criterion benchmarks
└── Cargo.toml
```

### Test Modules in Source Files
```rust
// Production code
pub struct Calculator { }

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        // Test code
    }
}
```

### Nested Test Modules
```rust
#[cfg(test)]
mod tests {
    mod calculator_tests {
        #[test]
        fn test_add() { }
    }

    mod edge_cases {
        #[test]
        fn test_overflow() { }
    }
}
```

### Test Fixtures
```rust
struct TestFixture {
    service: UserService,
    users: Vec<u64>,
}

impl TestFixture {
    fn setup() -> Self {
        // Initialize test data
    }
}

#[test]
fn test_with_fixture() {
    let fixture = TestFixture::setup();
    // Use fixture in test
}
```

## Test Attributes

### Basic Test
```rust
#[test]
fn test_something() {
    assert_eq!(2 + 2, 4);
}
```

### Expected Panic
```rust
#[test]
#[should_panic]
fn test_panic() {
    panic!("This should panic");
}
```

### Expected Panic Message
```rust
#[test]
#[should_panic(expected = "division by zero")]
fn test_divide_by_zero() {
    let _ = 10 / 0;
}
```

### Ignored Tests
```rust
#[test]
#[ignore]
fn expensive_test() {
    // Skipped unless --ignored flag used
}

#[test]
#[ignore = "requires database"]
fn database_test() {
    // Skipped with reason
}
```

### Return Result from Tests
```rust
#[test]
fn test_with_result() -> Result<(), String> {
    let value = some_operation()?;
    assert_eq!(value, expected);
    Ok(())
}
```

## Assertions

### Basic Assertions
```rust
// Boolean condition
assert!(value > 0);
assert!(list.is_empty());

// Equality
assert_eq!(actual, expected);
assert_eq!(calc.add(2, 3), 5);

// Inequality
assert_ne!(value, 0);
assert_ne!(result, error_value);
```

### Custom Messages
```rust
assert!(
    value > 0,
    "Value should be positive, got {}",
    value
);

assert_eq!(
    actual,
    expected,
    "Failed: expected {} but got {}",
    expected,
    actual
);
```

### Custom Assertions
```rust
fn assert_almost_equal(a: f64, b: f64, epsilon: f64) {
    assert!(
        (a - b).abs() < epsilon,
        "Values not almost equal: {} vs {} (ε: {})",
        a, b, epsilon
    );
}

#[test]
fn test_floating_point() {
    assert_almost_equal(0.1 + 0.2, 0.3, 0.0001);
}
```

## Common Testing Patterns

### Parameterized Tests
```rust
#[test]
fn test_multiple_cases() {
    let test_cases = vec![
        (2, 3, 5),
        (0, 0, 0),
        (-1, 1, 0),
        (100, 200, 300),
    ];

    for (a, b, expected) in test_cases {
        assert_eq!(
            add(a, b),
            expected,
            "Failed: {} + {} should equal {}",
            a, b, expected
        );
    }
}
```

### Testing Errors
```rust
#[test]
fn test_error_conditions() {
    // Test that error is returned
    let result = divide(10, 0);
    assert!(result.is_err());

    // Test specific error
    assert_eq!(result.unwrap_err(), "Division by zero");
}
```

### Testing Options
```rust
#[test]
fn test_option_values() {
    let result = find_user(1);

    // Test Some
    assert!(result.is_some());
    let user = result.unwrap();
    assert_eq!(user.name, "Alice");

    // Test None
    let result = find_user(999);
    assert!(result.is_none());
}
```

### Testing Collections
```rust
#[test]
fn test_collections() {
    let vec = vec![1, 2, 3, 4, 5];

    assert_eq!(vec.len(), 5);
    assert!(vec.contains(&3));
    assert_eq!(vec[0], 1);
    assert_eq!(vec.first(), Some(&1));
    assert_eq!(vec.last(), Some(&5));
}
```

### Testing State Changes
```rust
#[test]
fn test_state_mutation() {
    let mut account = BankAccount::new();
    assert_eq!(account.balance(), 0.0);

    account.deposit(100.0).unwrap();
    assert_eq!(account.balance(), 100.0);

    account.withdraw(30.0).unwrap();
    assert_eq!(account.balance(), 70.0);
}
```

## Documentation Tests

Documentation tests are written in `///` doc comments and verified by `cargo test`.

### Basic Doc Test
```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use mylib::add;
///
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Multiple Examples
```rust
/// Divides two numbers.
///
/// # Examples
///
/// Basic division:
///
/// ```
/// use mylib::divide;
///
/// assert_eq!(divide(10, 2), Ok(5));
/// ```
///
/// Division by zero returns error:
///
/// ```
/// use mylib::divide;
///
/// assert_eq!(divide(10, 0), Err("Division by zero"));
/// ```
pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}
```

### Doc Tests That Should Panic
```rust
/// This function panics with zero input.
///
/// ```should_panic
/// use mylib::must_be_positive;
///
/// must_be_positive(0); // This panics
/// ```
pub fn must_be_positive(n: i32) {
    assert!(n > 0);
}
```

### Ignoring Doc Tests
```rust
/// Example that requires setup:
///
/// ```ignore
/// let db = Database::connect("localhost");
/// db.query("SELECT * FROM users");
/// ```
pub fn query_database() { }
```

## Property-Based Testing with Proptest

Property-based testing generates random inputs and verifies properties hold for all of them.

### Basic Property Test
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_add_commutative(a in -1000i32..1000, b in -1000i32..1000) {
        prop_assert_eq!(add(a, b), add(b, a));
    }
}
```

### Testing Properties
```rust
proptest! {
    // Associativity: (a + b) + c == a + (b + c)
    #[test]
    fn test_add_associative(a in -100i32..100, b in -100i32..100, c in -100i32..100) {
        let left = add(add(a, b), c);
        let right = add(a, add(b, c));
        prop_assert_eq!(left, right);
    }

    // Involution: reverse(reverse(s)) == s
    #[test]
    fn test_reverse_involution(s in ".*") {
        let reversed_twice = reverse_string(&reverse_string(&s));
        prop_assert_eq!(s, reversed_twice);
    }

    // Roundtrip: convert forth and back yields original
    #[test]
    fn test_temperature_roundtrip(celsius in -100.0f64..100.0) {
        let fahrenheit = celsius_to_fahrenheit(celsius);
        let back = fahrenheit_to_celsius(fahrenheit);
        prop_assert!((celsius - back).abs() < 0.0001);
    }
}
```

### Custom Strategies
```rust
proptest! {
    #[test]
    fn test_with_valid_email(
        username in "[a-z]{3,10}",
        domain in "[a-z]{3,10}",
    ) {
        let email = format!("{}@{}.com", username, domain);
        prop_assert!(validate_email(&email));
    }
}
```

## Mocking with Mockall

Mockall allows you to create mock implementations of traits for testing.

### Basic Mock
```rust
use mockall::*;

#[automock]
pub trait Database {
    fn save_user(&mut self, user: &User) -> Result<(), String>;
    fn find_user(&self, id: u64) -> Option<User>;
}

#[test]
fn test_with_mock() {
    let mut mock_db = MockDatabase::new();

    // Set expectations
    mock_db
        .expect_save_user()
        .times(1)
        .returning(|_| Ok(()));

    // Use mock
    let user = User::new(1, "alice".into(), "alice@test.com".into(), 25).unwrap();
    let result = mock_db.save_user(&user);

    assert!(result.is_ok());
}
```

### Matching Arguments
```rust
use mockall::predicate::*;

#[test]
fn test_mock_with_args() {
    let mut mock = MockDatabase::new();

    mock
        .expect_find_user()
        .with(eq(1))  // Expect specific argument
        .times(1)
        .return_const(Some(expected_user));

    let result = mock.find_user(1);
    assert!(result.is_some());
}
```

### Returning Different Values
```rust
#[test]
fn test_mock_sequence() {
    let mut mock = MockEmailService::new();

    mock
        .expect_send_email()
        .times(2)
        .returning(|_, _, _| Ok(()))
        .times(1)
        .returning(|_, _, _| Err("Network error".into()));

    // First two calls succeed
    assert!(mock.send_email("user@test.com", "Subject", "Body").is_ok());
    assert!(mock.send_email("user@test.com", "Subject", "Body").is_ok());

    // Third call fails
    assert!(mock.send_email("user@test.com", "Subject", "Body").is_err());
}
```

### Verifying Call Count
```rust
#[test]
fn test_call_count() {
    let mut mock = MockEmailService::new();

    mock
        .expect_send_email()
        .times(3)  // Exactly 3 times
        .returning(|_, _, _| Ok(()));

    for i in 0..3 {
        mock.send_email(&format!("user{}@test.com", i), "Hi", "Body").unwrap();
    }

    // Mock automatically verifies call count on drop
}
```

## Benchmarking with Criterion

Criterion provides statistical benchmarking with regression detection.

### Basic Benchmark
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_add(c: &mut Criterion) {
    c.bench_function("add", |b| {
        b.iter(|| {
            black_box(add(black_box(100), black_box(200)))
        })
    });
}

criterion_group!(benches, benchmark_add);
criterion_main!(benches);
```

### Benchmark Groups
```rust
fn benchmark_factorial(c: &mut Criterion) {
    let mut group = c.benchmark_group("factorial");

    for n in [5, 10, 15, 20].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(n),
            n,
            |b, &n| {
                b.iter(|| black_box(factorial(black_box(n))));
            }
        );
    }

    group.finish();
}
```

### Benchmarking with Setup
```rust
fn benchmark_with_setup(c: &mut Criterion) {
    c.bench_function("user_lookup", |b| {
        b.iter_batched(
            || {
                // Setup code (not measured)
                let mut service = UserService::new();
                service.create_user("alice".into(), "alice@test.com".into(), 25).unwrap();
                service
            },
            |service| {
                // Measured code
                black_box(service.find_by_username("alice"))
            },
            criterion::BatchSize::SmallInput,
        )
    });
}
```

### Comparing Implementations
```rust
fn benchmark_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_reverse");

    group.bench_function("chars_rev", |b| {
        b.iter(|| reverse_with_chars(black_box("hello world")));
    });

    group.bench_function("bytes_rev", |b| {
        b.iter(|| reverse_with_bytes(black_box("hello world")));
    });

    group.finish();
}
```

## Best Practices

### 1. Test Organization
- Keep unit tests close to the code they test
- Use integration tests for end-to-end scenarios
- Group related tests in modules
- Use descriptive test names

### 2. Test Coverage
- Test happy paths and edge cases
- Test error conditions
- Test boundary values
- Test state transitions
- Don't aim for 100% coverage - focus on critical paths

### 3. Test Independence
- Each test should be independent
- Don't rely on test execution order
- Clean up resources in each test
- Use fixtures for common setup

### 4. Assertions
- Use specific assertions (assert_eq! vs assert!)
- Include helpful error messages
- Test one thing per test
- Make failures easy to diagnose

### 5. Test Performance
- Keep unit tests fast
- Use `#[ignore]` for slow tests
- Run expensive tests separately
- Parallelize when possible

### 6. Documentation
- Write doc tests for public APIs
- Keep examples simple and focused
- Test all code in documentation
- Show common use cases

### 7. Property-Based Testing
- Use for testing mathematical properties
- Verify invariants
- Find edge cases automatically
- Complement example-based tests

### 8. Mocking
- Mock external dependencies
- Don't mock everything
- Verify behavior, not implementation
- Keep mocks simple

## When to Use Each Test Type

### Unit Tests
- Testing individual functions
- Testing private implementation details
- Fast, focused tests
- Most tests should be unit tests

### Integration Tests
- Testing public API
- End-to-end workflows
- Component interaction
- User-facing functionality

### Documentation Tests
- Public API examples
- Tutorial-style documentation
- Ensuring docs stay current
- Simple usage demonstrations

### Property-Based Tests
- Mathematical properties
- Invariants and contracts
- Finding edge cases
- Complementing example tests

### Benchmarks
- Performance-critical code
- Comparing implementations
- Detecting regressions
- Optimization validation

## Common Testing Patterns in This Example

1. **Calculator** - Basic unit testing with assertions
2. **UserService** - State management and validation testing
3. **BankAccount** - Testing state mutations and transactions
4. **FileProcessor** - Testing trait implementations
5. **Temperature** - Testing conversions and roundtrips
6. **String utilities** - Testing string operations
7. **Vector utilities** - Testing with generics
8. **Property tests** - Commutativity, associativity, involution
9. **Mocks** - Database and email service mocking
10. **Benchmarks** - Performance testing all components

## Performance Tips

### 1. Parallel Execution
Tests run in parallel by default. Use `--test-threads=1` only when necessary.

### 2. Avoid Slow Tests
```rust
#[test]
#[ignore = "slow integration test"]
fn expensive_operation() {
    // Slow test
}
```

### 3. Use Test Fixtures
Reuse setup code:
```rust
struct TestFixture { /* ... */ }

impl TestFixture {
    fn setup() -> Self { /* ... */ }
}
```

### 4. Benchmark Appropriately
- Use `black_box()` to prevent compiler optimizations
- Measure realistic workloads
- Compare relative performance, not absolute

## Troubleshooting

### Tests Pass Individually But Fail Together
- Tests may be sharing state
- Use `--test-threads=1` to verify
- Make tests truly independent

### Flaky Tests
- Remove non-deterministic behavior
- Don't rely on timing
- Avoid random values without seeds
- Mock external dependencies

### Slow Test Suite
- Identify slow tests with `--show-output`
- Move slow tests to integration tests
- Use `#[ignore]` for expensive tests
- Run subset of tests during development

## Resources and Further Reading

### Official Documentation
- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust By Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- [Cargo Book - Tests](https://doc.rust-lang.org/cargo/guide/tests.html)
- [rustc Book - Tests](https://doc.rust-lang.org/rustc/tests/index.html)

### Testing Crates
- [proptest](https://docs.rs/proptest/) - Property-based testing
- [mockall](https://docs.rs/mockall/) - Mocking framework
- [criterion](https://docs.rs/criterion/) - Statistical benchmarking
- [quickcheck](https://docs.rs/quickcheck/) - Alternative property testing
- [rstest](https://docs.rs/rstest/) - Fixture and parameterized testing
- [test-case](https://docs.rs/test-case/) - Parameterized tests macro

### Advanced Topics
- [Mutation Testing](https://github.com/llogiq/mutagen)
- [Coverage Tools](https://github.com/mozilla/grcov)
- [Fuzzing](https://rust-fuzz.github.io/book/)
- [Test-Driven Development in Rust](https://www.manning.com/books/rust-in-action)

### Testing Patterns
- [Testing Strategies](https://matklad.github.io/2021/05/31/how-to-test.html)
- [Effective Rust Testing](https://www.lpalmieri.com/posts/how-to-write-a-rest-client-in-rust-with-reqwest-and-wiremock/)
- [Property-Based Testing Guide](https://github.com/BurntSushi/quickcheck)

### Best Practices
- [API Guidelines - Tests](https://rust-lang.github.io/api-guidelines/documentation.html)
- [Rust Testing Patterns](https://github.com/rust-unofficial/patterns/blob/master/patterns/behavioural/testing.md)

## Summary

Rust's testing ecosystem provides comprehensive tools for ensuring code quality:

- **Built-in testing** - No external framework needed
- **Multiple test types** - Unit, integration, doc, property, benchmarks
- **Powerful assertions** - Clear failure messages
- **Property-based testing** - Find edge cases automatically
- **Mocking support** - Test in isolation
- **Performance testing** - Statistical benchmarking
- **Developer-friendly** - Fast, parallel, integrated

The combination of Rust's type system and testing tools makes it possible to write highly reliable software with confidence.
