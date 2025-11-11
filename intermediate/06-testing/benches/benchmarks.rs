//! Benchmarks for the testing crate
//!
//! Run benchmarks with: cargo bench
//!
//! This file demonstrates how to use Criterion for benchmarking Rust code.
//! Criterion provides statistical analysis and detailed reports.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use testing::*;

// ============================================================================
// BENCHMARK 1: CALCULATOR OPERATIONS
// ============================================================================

fn benchmark_calculator_add(c: &mut Criterion) {
    let calc = Calculator::new();

    c.bench_function("calculator_add", |b| {
        b.iter(|| black_box(calc.add(black_box(100), black_box(200))))
    });
}

fn benchmark_calculator_multiply(c: &mut Criterion) {
    let calc = Calculator::new();

    c.bench_function("calculator_multiply", |b| {
        b.iter(|| black_box(calc.multiply(black_box(123), black_box(456))))
    });
}

fn benchmark_calculator_divide(c: &mut Criterion) {
    let calc = Calculator::new();

    c.bench_function("calculator_divide", |b| {
        b.iter(|| black_box(calc.divide(black_box(1000), black_box(7))))
    });
}

fn benchmark_calculator_factorial(c: &mut Criterion) {
    let calc = Calculator::new();

    let mut group = c.benchmark_group("factorial");

    for n in [5, 10, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| black_box(calc.factorial(black_box(n))));
        });
    }

    group.finish();
}

// ============================================================================
// BENCHMARK 2: USER SERVICE OPERATIONS
// ============================================================================

fn benchmark_user_service_create(c: &mut Criterion) {
    c.bench_function("user_service_create", |b| {
        b.iter_batched(
            || UserService::new(),
            |mut service| {
                black_box(service.create_user(
                    black_box("testuser".to_string()),
                    black_box("test@example.com".to_string()),
                    black_box(25),
                ))
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_user_service_lookup(c: &mut Criterion) {
    let mut service = UserService::new();
    let user_id = service
        .create_user("alice".to_string(), "alice@example.com".to_string(), 25)
        .unwrap();

    c.bench_function("user_service_lookup", |b| {
        b.iter(|| black_box(service.get_user(black_box(user_id))))
    });
}

fn benchmark_user_service_find_by_username(c: &mut Criterion) {
    let mut service = UserService::new();

    // Create multiple users
    for i in 0..100 {
        service
            .create_user(
                format!("user{}", i),
                format!("user{}@example.com", i),
                20 + (i as u8 % 50),
            )
            .unwrap();
    }

    c.bench_function("user_service_find_by_username", |b| {
        b.iter(|| black_box(service.find_by_username(black_box("user50"))))
    });
}

// ============================================================================
// BENCHMARK 3: BANK ACCOUNT OPERATIONS
// ============================================================================

fn benchmark_bank_account_deposit(c: &mut Criterion) {
    c.bench_function("bank_account_deposit", |b| {
        b.iter_batched(
            || BankAccount::new(),
            |mut account| black_box(account.deposit(black_box(100.0))),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_bank_account_withdraw(c: &mut Criterion) {
    c.bench_function("bank_account_withdraw", |b| {
        b.iter_batched(
            || {
                let mut account = BankAccount::new();
                account.deposit(1000.0).unwrap();
                account
            },
            |mut account| black_box(account.withdraw(black_box(50.0))),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_bank_account_transactions(c: &mut Criterion) {
    let mut group = c.benchmark_group("bank_transactions");

    for count in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, &count| {
            b.iter_batched(
                || {
                    let mut account = BankAccount::new();
                    account.deposit(100000.0).unwrap();
                    account
                },
                |mut account| {
                    for i in 0..count {
                        if i % 2 == 0 {
                            let _ = account.deposit(10.0);
                        } else {
                            let _ = account.withdraw(5.0);
                        }
                    }
                    black_box(account)
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

// ============================================================================
// BENCHMARK 4: STRING OPERATIONS
// ============================================================================

fn benchmark_reverse_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("reverse_string");

    let long_string = "a".repeat(1000);
    let test_strings = vec![
        ("short", "hello"),
        ("medium", "The quick brown fox jumps over the lazy dog"),
        ("long", long_string.as_str()),
    ];

    for (name, s) in test_strings.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), s, |b, s| {
            b.iter(|| black_box(reverse_string(black_box(s))));
        });
    }

    group.finish();
}

fn benchmark_is_palindrome(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_palindrome");

    let test_cases = vec![
        ("short_true", "racecar"),
        ("short_false", "hello"),
        ("medium_true", "A man a plan a canal Panama"),
        ("medium_false", "The quick brown fox"),
    ];

    for (name, s) in test_cases.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), s, |b, s| {
            b.iter(|| black_box(is_palindrome(black_box(s))));
        });
    }

    group.finish();
}

// ============================================================================
// BENCHMARK 5: TEMPERATURE CONVERSIONS
// ============================================================================

fn benchmark_temperature_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("temperature");

    group.bench_function("celsius_to_fahrenheit", |b| {
        b.iter(|| black_box(celsius_to_fahrenheit(black_box(25.0))))
    });

    group.bench_function("fahrenheit_to_celsius", |b| {
        b.iter(|| black_box(fahrenheit_to_celsius(black_box(77.0))))
    });

    group.bench_function("roundtrip_conversion", |b| {
        b.iter(|| {
            let celsius = black_box(25.0);
            let fahrenheit = celsius_to_fahrenheit(celsius);
            black_box(fahrenheit_to_celsius(fahrenheit))
        })
    });

    group.finish();
}

// ============================================================================
// BENCHMARK 6: VECTOR UTILITIES
// ============================================================================

fn benchmark_find_max_min(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_max_min");

    for size in [10, 100, 1000].iter() {
        let numbers: Vec<i32> = (0..*size).collect();

        group.bench_with_input(BenchmarkId::new("find_max", size), &numbers, |b, nums| {
            b.iter(|| black_box(find_max(black_box(nums))))
        });

        group.bench_with_input(BenchmarkId::new("find_min", size), &numbers, |b, nums| {
            b.iter(|| black_box(find_min(black_box(nums))))
        });
    }

    group.finish();
}

// ============================================================================
// BENCHMARK 7: DATA PROCESSOR
// ============================================================================

fn benchmark_file_processor(c: &mut Criterion) {
    let processor = FileProcessor::new("PREFIX".to_string());

    let mut group = c.benchmark_group("file_processor");

    group.bench_function("process", |b| {
        b.iter(|| black_box(processor.process(black_box("sample data to process"))))
    });

    group.bench_function("validate", |b| {
        b.iter(|| black_box(processor.validate(black_box("data to validate"))))
    });

    let long_data = "x".repeat(500);
    group.bench_function("validate_long", |b| {
        b.iter(|| black_box(processor.validate(black_box(&long_data))))
    });

    group.finish();
}

// ============================================================================
// BENCHMARK 8: COMPLEX WORKFLOW
// ============================================================================

fn benchmark_complex_workflow(c: &mut Criterion) {
    c.bench_function("complex_workflow", |b| {
        b.iter_batched(
            || (UserService::new(), BankAccount::new(), Calculator::new()),
            |(mut service, mut account, calc)| {
                // Create user
                let user_id = service
                    .create_user("john".to_string(), "john@example.com".to_string(), 30)
                    .unwrap();

                // Bank operations
                account.deposit(5000.0).unwrap();

                // Calculator operations
                let salary = 3000;
                let bonus = calc.multiply(salary, 2) / 10;

                account.deposit(salary as f64).unwrap();
                account.deposit(bonus as f64).unwrap();

                let expenses = calc.add(1500, 200);
                account.withdraw(expenses as f64).unwrap();

                // Process result
                let processor = FileProcessor::new("LOG".to_string());
                let user = service.get_user(user_id).unwrap();
                let log = format!("User {} balance: {}", user.username, account.balance());

                black_box(processor.process(&log))
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

// ============================================================================
// BENCHMARK 9: USER CREATION WITH VALIDATION
// ============================================================================

fn benchmark_user_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("user_validation");

    group.bench_function("valid_user", |b| {
        b.iter(|| {
            black_box(User::new(
                black_box(1),
                black_box("alice".to_string()),
                black_box("alice@example.com".to_string()),
                black_box(25),
            ))
        })
    });

    group.bench_function("invalid_user_empty_name", |b| {
        b.iter(|| {
            black_box(User::new(
                black_box(1),
                black_box("".to_string()),
                black_box("test@example.com".to_string()),
                black_box(25),
            ))
        })
    });

    group.bench_function("invalid_user_bad_email", |b| {
        b.iter(|| {
            black_box(User::new(
                black_box(1),
                black_box("test".to_string()),
                black_box("invalid".to_string()),
                black_box(25),
            ))
        })
    });

    group.finish();
}

// ============================================================================
// BENCHMARK 10: BATCH OPERATIONS
// ============================================================================

fn benchmark_batch_calculator_operations(c: &mut Criterion) {
    let calc = Calculator::new();

    c.bench_function("batch_calculator_operations", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 0..100 {
                result = calc.add(result, i);
                result = calc.multiply(result, 2);
                result = calc.subtract(result, 10);
            }
            black_box(result)
        })
    });
}

fn benchmark_batch_string_operations(c: &mut Criterion) {
    let strings = vec![
        "hello",
        "world",
        "rust",
        "testing",
        "benchmark",
        "performance",
        "optimization",
        "code",
        "example",
        "demo",
    ];

    c.bench_function("batch_string_operations", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for s in &strings {
                let reversed = reverse_string(s);
                let is_pal = is_palindrome(s);
                results.push((reversed, is_pal));
            }
            black_box(results)
        })
    });
}

// ============================================================================
// CRITERION CONFIGURATION
// ============================================================================

criterion_group!(
    calculator_benches,
    benchmark_calculator_add,
    benchmark_calculator_multiply,
    benchmark_calculator_divide,
    benchmark_calculator_factorial
);

criterion_group!(
    user_service_benches,
    benchmark_user_service_create,
    benchmark_user_service_lookup,
    benchmark_user_service_find_by_username
);

criterion_group!(
    bank_account_benches,
    benchmark_bank_account_deposit,
    benchmark_bank_account_withdraw,
    benchmark_bank_account_transactions
);

criterion_group!(
    string_benches,
    benchmark_reverse_string,
    benchmark_is_palindrome
);

criterion_group!(temperature_benches, benchmark_temperature_conversions);

criterion_group!(vector_benches, benchmark_find_max_min);

criterion_group!(processor_benches, benchmark_file_processor);

criterion_group!(workflow_benches, benchmark_complex_workflow);

criterion_group!(validation_benches, benchmark_user_validation);

criterion_group!(
    batch_benches,
    benchmark_batch_calculator_operations,
    benchmark_batch_string_operations
);

criterion_main!(
    calculator_benches,
    user_service_benches,
    bank_account_benches,
    string_benches,
    temperature_benches,
    vector_benches,
    processor_benches,
    workflow_benches,
    validation_benches,
    batch_benches,
);
