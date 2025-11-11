//! Integration Tests
//!
//! Integration tests are placed in the `tests/` directory and test the public API
//! of the crate as an external user would use it. Each file in `tests/` is compiled
//! as a separate crate.
//!
//! Run with: cargo test --test integration_test

use testing::*;

// ============================================================================
// INTEGRATION TEST 1: CALCULATOR END-TO-END
// ============================================================================

#[test]
fn integration_calculator_full_workflow() {
    let calc = Calculator::new();

    // Test a series of operations
    let result1 = calc.add(10, 5);
    assert_eq!(result1, 15);

    let result2 = calc.multiply(result1, 2);
    assert_eq!(result2, 30);

    let result3 = calc.divide(result2, 3).unwrap();
    assert_eq!(result3, 10);

    let result4 = calc.subtract(result3, 5);
    assert_eq!(result4, 5);

    let factorial = calc.factorial(result4 as u32);
    assert_eq!(factorial, 120);
}

#[test]
fn integration_calculator_error_handling() {
    let calc = Calculator::new();

    // Test error conditions
    let result = calc.divide(10, 0);
    assert!(result.is_err());
    assert_eq!(result, Err("Division by zero"));

    // Ensure other operations still work after an error
    assert_eq!(calc.add(5, 5), 10);
}

// ============================================================================
// INTEGRATION TEST 2: USER SERVICE WORKFLOW
// ============================================================================

#[test]
fn integration_user_service_complete_workflow() {
    let mut service = UserService::new();

    // Create multiple users
    let id1 = service
        .create_user("alice".to_string(), "alice@example.com".to_string(), 25)
        .unwrap();
    let id2 = service
        .create_user("bob".to_string(), "bob@example.com".to_string(), 30)
        .unwrap();
    let id3 = service
        .create_user("charlie".to_string(), "charlie@example.com".to_string(), 17)
        .unwrap();

    // Verify creation
    assert_eq!(service.count(), 3);

    // Test retrieval
    let alice = service.get_user(id1).unwrap();
    assert_eq!(alice.username, "alice");
    assert!(alice.is_adult());

    let charlie = service.get_user(id3).unwrap();
    assert_eq!(charlie.username, "charlie");
    assert!(!charlie.is_adult());

    // Test search by username
    let bob = service.find_by_username("bob").unwrap();
    assert_eq!(bob.email, "bob@example.com");
    assert_eq!(bob.age, 30);

    // Test deletion
    assert!(service.delete_user(id2));
    assert_eq!(service.count(), 2);
    assert!(service.get_user(id2).is_none());

    // Verify non-existent user
    assert!(service.find_by_username("bob").is_none());
}

#[test]
fn integration_user_service_validation() {
    let mut service = UserService::new();

    // Test various validation scenarios
    let result = service.create_user("".to_string(), "test@test.com".to_string(), 20);
    assert!(result.is_err());

    let result = service.create_user("test".to_string(), "invalid".to_string(), 20);
    assert!(result.is_err());

    let result = service.create_user("kid".to_string(), "kid@test.com".to_string(), 10);
    assert!(result.is_err());

    // Service should still be empty after failed creations
    assert_eq!(service.count(), 0);

    // Valid user should work
    let result = service.create_user("valid".to_string(), "valid@test.com".to_string(), 20);
    assert!(result.is_ok());
    assert_eq!(service.count(), 1);
}

// ============================================================================
// INTEGRATION TEST 3: BANK ACCOUNT WORKFLOW
// ============================================================================

#[test]
fn integration_bank_account_transactions() {
    let mut account = BankAccount::new();
    assert_eq!(account.balance(), 0.0);
    assert_eq!(account.transaction_count(), 0);

    // Deposit sequence
    account.deposit(1000.0).unwrap();
    assert_eq!(account.balance(), 1000.0);
    assert_eq!(account.transaction_count(), 1);

    account.deposit(500.0).unwrap();
    assert_eq!(account.balance(), 1500.0);
    assert_eq!(account.transaction_count(), 2);

    // Withdrawal sequence
    account.withdraw(300.0).unwrap();
    assert_eq!(account.balance(), 1200.0);
    assert_eq!(account.transaction_count(), 3);

    account.withdraw(200.0).unwrap();
    assert_eq!(account.balance(), 1000.0);
    assert_eq!(account.transaction_count(), 4);

    // Test insufficient funds
    let result = account.withdraw(2000.0);
    assert!(result.is_err());
    assert_eq!(account.balance(), 1000.0); // Balance unchanged
    assert_eq!(account.transaction_count(), 4); // Transaction count unchanged
}

#[test]
fn integration_bank_account_edge_cases() {
    let mut account = BankAccount::new();

    // Test invalid operations
    assert!(account.deposit(0.0).is_err());
    assert!(account.deposit(-100.0).is_err());
    assert!(account.withdraw(0.0).is_err());
    assert!(account.withdraw(-50.0).is_err());

    // Balance should remain zero
    assert_eq!(account.balance(), 0.0);
    assert_eq!(account.transaction_count(), 0);

    // Test withdrawal from empty account
    assert!(account.withdraw(10.0).is_err());
}

// ============================================================================
// INTEGRATION TEST 4: DATA PROCESSOR
// ============================================================================

#[test]
fn integration_file_processor_workflow() {
    let processor = FileProcessor::new("LOG".to_string());

    // Test processing
    let result = processor.process("important message");
    assert_eq!(result, "LOG: IMPORTANT MESSAGE");

    let result = processor.process("error occurred");
    assert_eq!(result, "LOG: ERROR OCCURRED");

    // Test validation
    assert!(processor.validate("valid data"));
    assert!(processor.validate(&"x".repeat(1000)));
    assert!(!processor.validate(""));
    assert!(!processor.validate(&"x".repeat(1001)));
}

#[test]
fn integration_data_processor_trait() {
    let processor: Box<dyn DataProcessor> = Box::new(FileProcessor::new("TEST".to_string()));

    // Test through trait interface
    let data = "sample data";
    assert!(processor.validate(data));

    let processed = processor.process(data);
    assert!(processed.contains("TEST"));
    assert!(processed.contains("SAMPLE DATA"));
}

// ============================================================================
// INTEGRATION TEST 5: TEMPERATURE CONVERSIONS
// ============================================================================

#[test]
fn integration_temperature_conversions() {
    // Test common temperature points
    let test_cases = vec![
        (0.0, 32.0),      // Freezing point of water
        (100.0, 212.0),   // Boiling point of water
        (-40.0, -40.0),   // Same in both scales
        (37.0, 98.6),     // Human body temperature (approximate)
        (20.0, 68.0),     // Room temperature
    ];

    for (celsius, fahrenheit) in test_cases {
        let converted_f = celsius_to_fahrenheit(celsius);
        assert!(
            (converted_f - fahrenheit).abs() < 0.1,
            "{}°C should be {}°F, got {}°F",
            celsius,
            fahrenheit,
            converted_f
        );

        let converted_c = fahrenheit_to_celsius(fahrenheit);
        assert!(
            (converted_c - celsius).abs() < 0.1,
            "{}°F should be {}°C, got {}°C",
            fahrenheit,
            celsius,
            converted_c
        );
    }
}

#[test]
fn integration_temperature_roundtrip() {
    let temperatures: Vec<f64> = vec![-40.0, -20.0, 0.0, 25.0, 37.0, 100.0];

    for temp in temperatures {
        let fahrenheit = celsius_to_fahrenheit(temp);
        let back_to_celsius = fahrenheit_to_celsius(fahrenheit);
        assert!(
            (temp - back_to_celsius).abs() < 0.0001,
            "Roundtrip failed for {}°C",
            temp
        );
    }
}

// ============================================================================
// INTEGRATION TEST 6: STRING UTILITIES
// ============================================================================

#[test]
fn integration_string_utilities() {
    // Test reverse
    let test_strings = vec![
        ("hello", "olleh"),
        ("Rust", "tsuR"),
        ("A", "A"),
        ("", ""),
        ("12345", "54321"),
    ];

    for (input, expected) in test_strings {
        assert_eq!(reverse_string(input), expected);
    }

    // Test palindrome
    let palindromes = vec![
        "racecar",
        "A man a plan a canal Panama",
        "Was it a car or a cat I saw",
        "Madam",
        "",
    ];

    for word in palindromes {
        assert!(
            is_palindrome(word),
            "{:?} should be a palindrome",
            word
        );
    }

    let non_palindromes = vec!["hello", "world", "Rust", "testing"];

    for word in non_palindromes {
        assert!(
            !is_palindrome(word),
            "{:?} should not be a palindrome",
            word
        );
    }
}

// ============================================================================
// INTEGRATION TEST 7: VECTOR UTILITIES
// ============================================================================

#[test]
fn integration_vector_utilities() {
    // Test with integers
    let numbers = vec![5, 2, 8, 1, 9, 3];
    assert_eq!(find_max(&numbers), Some(9));
    assert_eq!(find_min(&numbers), Some(1));

    // Test with negative numbers
    let negatives = vec![-5, -2, -8, -1, -9];
    assert_eq!(find_max(&negatives), Some(-1));
    assert_eq!(find_min(&negatives), Some(-9));

    // Test with mixed numbers
    let mixed = vec![-10, -5, 0, 5, 10];
    assert_eq!(find_max(&mixed), Some(10));
    assert_eq!(find_min(&mixed), Some(-10));

    // Test with single element
    assert_eq!(find_max(&[42]), Some(42));
    assert_eq!(find_min(&[42]), Some(42));

    // Test with empty slice
    assert_eq!(find_max::<i32>(&[]), None);
    assert_eq!(find_min::<i32>(&[]), None);

    // Test with characters
    let chars = vec!['z', 'a', 'm', 'b'];
    assert_eq!(find_max(&chars), Some('z'));
    assert_eq!(find_min(&chars), Some('a'));
}

// ============================================================================
// INTEGRATION TEST 8: COMPLEX SCENARIO
// ============================================================================

#[test]
fn integration_complex_scenario() {
    // Simulate a real-world scenario combining multiple components

    // 1. Create a user service and add users
    let mut user_service = UserService::new();
    let user_id = user_service
        .create_user("john".to_string(), "john@bank.com".to_string(), 30)
        .unwrap();

    let user = user_service.get_user(user_id).unwrap();
    assert!(user.is_adult());

    // 2. Create a bank account for the user
    let mut account = BankAccount::new();
    account.deposit(5000.0).unwrap();

    // 3. Process some transactions
    let calc = Calculator::new();
    let salary = 3000;
    let bonus = calc.multiply(salary, 2) / 10; // 20% bonus

    account.deposit(salary as f64).unwrap();
    account.deposit(bonus as f64).unwrap();

    // 4. Calculate expenses
    let rent = 1500;
    let utilities = 200;
    let total_expenses = calc.add(rent, utilities);

    account.withdraw(total_expenses as f64).unwrap();

    // 5. Verify final state
    let expected_balance = 5000.0 + 3000.0 + 600.0 - 1700.0;
    assert_eq!(account.balance(), expected_balance);
    assert_eq!(account.transaction_count(), 4);

    // 6. Process some data
    let processor = FileProcessor::new("TRANSACTION".to_string());
    let log_entry = format!("User {} balance: {}", user.username, account.balance());
    let processed = processor.process(&log_entry);
    assert!(processed.contains("TRANSACTION"));
    assert!(processor.validate(&log_entry));
}

// ============================================================================
// INTEGRATION TEST 9: ERROR PROPAGATION
// ============================================================================

#[test]
fn integration_error_handling_chain() {
    fn process_user_transaction(
        service: &mut UserService,
        username: &str,
        email: &str,
        age: u8,
        deposit_amount: f64,
    ) -> Result<(u64, f64), String> {
        // Create user (can fail)
        let user_id = service.create_user(username.to_string(), email.to_string(), age)?;

        // Create account and deposit (can fail)
        let mut account = BankAccount::new();
        account.deposit(deposit_amount)?;

        Ok((user_id, account.balance()))
    }

    let mut service = UserService::new();

    // Test successful case
    let result = process_user_transaction(
        &mut service,
        "alice",
        "alice@test.com",
        25,
        1000.0,
    );
    assert!(result.is_ok());
    let (user_id, balance) = result.unwrap();
    assert_eq!(balance, 1000.0);
    assert!(service.get_user(user_id).is_some());

    // Test failure in user creation
    let result = process_user_transaction(
        &mut service,
        "",
        "test@test.com",
        25,
        1000.0,
    );
    assert!(result.is_err());

    // Test failure in deposit
    let result = process_user_transaction(
        &mut service,
        "bob",
        "bob@test.com",
        25,
        -100.0,
    );
    assert!(result.is_err());
}

// ============================================================================
// INTEGRATION TEST 10: PERFORMANCE AND STRESS
// ============================================================================

#[test]
fn integration_stress_test_user_service() {
    let mut service = UserService::new();
    let user_count = 100;

    // Create many users
    for i in 0..user_count {
        let result = service.create_user(
            format!("user{}", i),
            format!("user{}@test.com", i),
            20 + (i as u8 % 50),
        );
        assert!(result.is_ok());
    }

    assert_eq!(service.count(), user_count);

    // Verify all users exist
    for i in 1..=user_count {
        let user = service.get_user(i as u64);
        assert!(user.is_some());
    }

    // Delete half the users
    for i in (1..=user_count).step_by(2) {
        assert!(service.delete_user(i as u64));
    }

    assert_eq!(service.count(), user_count / 2);
}

#[test]
fn integration_stress_test_bank_account() {
    let mut account = BankAccount::new();
    let transaction_count = 1000;

    // Initial deposit
    account.deposit(100000.0).unwrap();

    // Perform many small transactions
    for i in 1..=transaction_count {
        if i % 2 == 0 {
            let _ = account.deposit(10.0);
        } else {
            let _ = account.withdraw(5.0);
        }
    }

    // We did 1 initial + 1000 transactions
    assert_eq!(account.transaction_count(), transaction_count + 1);

    // Balance should be: 100000 + (500 * 10) - (500 * 5) = 102500
    assert_eq!(account.balance(), 102500.0);
}

#[test]
fn integration_calculator_large_factorial() {
    let calc = Calculator::new();

    // Test factorials up to 20
    let expected_factorials = vec![
        (0, 1),
        (1, 1),
        (5, 120),
        (10, 3628800),
        (15, 1307674368000),
    ];

    for (n, expected) in expected_factorials {
        assert_eq!(
            calc.factorial(n),
            expected,
            "{}! should equal {}",
            n,
            expected
        );
    }
}
