// Iterators in Rust
//
// Iterators are a powerful abstraction that allow you to process sequences of
// elements. They're lazy (don't compute until consumed) and zero-cost (compile
// to the same code as hand-written loops).

fn main() {
    println!("=== Rust Iterators Examples ===\n");

    // Example 1: Creating iterators
    println!("1. Creating Iterators:");
    creating_iterators();
    println!();

    // Example 2: Iterator adapters
    println!("2. Iterator Adapters:");
    iterator_adapters();
    println!();

    // Example 3: Iterator consumers
    println!("3. Iterator Consumers:");
    iterator_consumers();
    println!();

    // Example 4: Chaining iterators
    println!("4. Chaining Iterators:");
    chaining_iterators();
    println!();

    // Example 5: Custom iterators
    println!("5. Custom Iterators:");
    custom_iterators();
    println!();

    // Example 6: Common patterns
    println!("6. Common Patterns:");
    common_patterns();
    println!();

    // Example 7: Performance considerations
    println!("7. Performance (zero-cost abstraction):");
    performance_examples();
    println!();
}

// Example 1: Creating iterators
fn creating_iterators() {
    // iter() - borrows each element
    let v = vec![1, 2, 3];
    let iter = v.iter();
    for val in iter {
        print!("  {} ", val);
    }
    println!("(iter - borrowed)");

    // into_iter() - takes ownership
    let v = vec![1, 2, 3];
    let iter = v.into_iter();
    for val in iter {
        print!("  {} ", val);
    }
    println!("(into_iter - owned)");
    // v is no longer valid here

    // iter_mut() - mutable borrow
    let mut v = vec![1, 2, 3];
    for val in v.iter_mut() {
        *val *= 2;
    }
    println!("  After iter_mut: {:?}", v);

    // Range iterators
    for i in 0..5 {
        print!("  {} ", i);
    }
    println!("(range 0..5)");

    for i in (0..5).rev() {
        print!("  {} ", i);
    }
    println!("(reversed)");
}

// Example 2: Iterator adapters (transformations - lazy)
fn iterator_adapters() {
    let v = vec![1, 2, 3, 4, 5];

    // map - transform each element
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("  Doubled: {:?}", doubled);

    // filter - keep elements matching predicate
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("  Evens: {:?}", evens);

    // take - first n elements
    let first_three: Vec<&i32> = v.iter().take(3).collect();
    println!("  First three: {:?}", first_three);

    // skip - skip first n elements
    let skip_two: Vec<&i32> = v.iter().skip(2).collect();
    println!("  Skip two: {:?}", skip_two);

    // enumerate - add index
    for (i, val) in v.iter().enumerate() {
        println!("    Index {}: {}", i, val);
    }

    // zip - combine two iterators
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    let pairs: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("  Zipped: {:?}", pairs);

    // rev - reverse iterator
    let reversed: Vec<&i32> = v.iter().rev().collect();
    println!("  Reversed: {:?}", reversed);
}

// Example 3: Iterator consumers (trigger computation)
fn iterator_consumers() {
    let v = vec![1, 2, 3, 4, 5];

    // collect - convert to collection
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("  Collected: {:?}", doubled);

    // sum - add all elements
    let sum: i32 = v.iter().sum();
    println!("  Sum: {}", sum);

    // product - multiply all elements
    let product: i32 = v.iter().product();
    println!("  Product: {}", product);

    // count - number of elements
    let count = v.iter().count();
    println!("  Count: {}", count);

    // max, min
    let max = v.iter().max();
    let min = v.iter().min();
    println!("  Max: {:?}, Min: {:?}", max, min);

    // find - first matching element
    let first_even = v.iter().find(|x| *x % 2 == 0);
    println!("  First even: {:?}", first_even);

    // any, all
    let has_even = v.iter().any(|x| x % 2 == 0);
    let all_positive = v.iter().all(|x| *x > 0);
    println!("  Has even: {}, All positive: {}", has_even, all_positive);

    // fold - accumulate with custom logic
    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("  Fold sum: {}", sum);

    // reduce - like fold but returns Option
    let max = v.iter().reduce(|a, b| if a > b { a } else { b });
    println!("  Reduce max: {:?}", max);

    // for_each - perform action on each element
    print!("  For each: ");
    v.iter().for_each(|x| print!("{} ", x));
    println!();
}

// Example 4: Chaining iterator adapters
fn chaining_iterators() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Complex chain
    let result: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0) // Keep even numbers
        .map(|x| x * x)           // Square them
        .filter(|x| *x > 10)      // Keep those > 10
        .collect();
    println!("  Even squares > 10: {:?}", result);

    // Another example
    let words = vec!["hello", "world", "rust", "programming"];
    let result: Vec<String> = words
        .iter()
        .filter(|w| w.len() > 4)
        .map(|w| w.to_uppercase())
        .collect();
    println!("  Long words uppercase: {:?}", result);

    // Flat map - flatten nested structures
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flattened: Vec<i32> = nested.iter().flat_map(|v| v.iter()).copied().collect();
    println!("  Flattened: {:?}", flattened);

    // Take while - take elements while condition is true
    let taken: Vec<&i32> = numbers.iter().take_while(|x| **x < 6).collect();
    println!("  Take while < 6: {:?}", taken);

    // Skip while - skip elements while condition is true
    let skipped: Vec<&i32> = numbers.iter().skip_while(|x| **x < 6).collect();
    println!("  Skip while < 6: {:?}", skipped);
}

// Example 5: Custom iterators
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn custom_iterators() {
    let counter = Counter::new(5);

    println!("  Custom counter:");
    for num in counter {
        print!("    {} ", num);
    }
    println!();

    // Can use all iterator methods on custom iterators
    let sum: u32 = Counter::new(10).sum();
    println!("  Sum of 1..=10: {}", sum);

    let evens: Vec<u32> = Counter::new(10).filter(|x| x % 2 == 0).collect();
    println!("  Even numbers: {:?}", evens);

    // Fibonacci iterator
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Fibonacci {
        fn new() -> Fibonacci {
            Fibonacci { curr: 0, next: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            let new_next = self.curr.checked_add(self.next)?;
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        }
    }

    let fibs: Vec<u32> = Fibonacci::new().take(10).collect();
    println!("  First 10 Fibonacci numbers: {:?}", fibs);
}

// Example 6: Common patterns
fn common_patterns() {
    // Map-reduce pattern
    let numbers = vec![1, 2, 3, 4, 5];
    let sum_of_squares: i32 = numbers.iter().map(|x| x * x).sum();
    println!("  Sum of squares: {}", sum_of_squares);

    // Partition - split into two collections
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
        .iter()
        .copied()
        .partition(|x| x % 2 == 0);
    println!("  Evens: {:?}, Odds: {:?}", evens, odds);

    // Group by (using fold)
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let people = vec![
        Person { name: "Alice".to_string(), age: 25 },
        Person { name: "Bob".to_string(), age: 30 },
        Person { name: "Charlie".to_string(), age: 25 },
    ];

    let total_age: u32 = people.iter().map(|p| p.age).sum();
    println!("  Total age: {}", total_age);

    // Find maximum by key
    let oldest = people.iter().max_by_key(|p| p.age);
    println!("  Oldest: {:?}", oldest.map(|p| &p.name));

    // Collect into different collections
    use std::collections::{HashMap, HashSet};

    let numbers = vec![1, 2, 3, 2, 1, 4];
    let unique: HashSet<i32> = numbers.iter().copied().collect();
    println!("  Unique: {:?}", unique);

    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let map: HashMap<_, _> = pairs.iter().copied().collect();
    println!("  Map: {:?}", map);
}

// Example 7: Performance (zero-cost abstraction)
fn performance_examples() {
    let numbers: Vec<i32> = (1..=1000).collect();

    // Iterator version - compiles to same code as manual loop
    let sum: i32 = numbers.iter().filter(|x| *x % 2 == 0).map(|x| x * x).sum();

    println!("  Iterator sum: {}", sum);

    // Manual loop version - same performance
    let mut manual_sum = 0;
    for &num in &numbers {
        if num % 2 == 0 {
            manual_sum += num * num;
        }
    }

    println!("  Manual sum: {}", manual_sum);
    println!("  Both compile to equivalent machine code!");

    // Iterators are lazy - nothing happens until consumed
    let _lazy = numbers.iter().map(|x| {
        // This closure never actually runs!
        println!("Processing {}", x);
        x * 2
    });
    println!("  Created lazy iterator (nothing printed)");

    // Consume it
    let _eager: Vec<i32> = numbers.iter().take(3).map(|x| {
        println!("    Actually processing {}", x);
        x * 2
    }).collect();
}
