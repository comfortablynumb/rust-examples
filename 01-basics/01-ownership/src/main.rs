#![allow(dead_code)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::manual_map)]
#![allow(clippy::unnecessary_literal_unwrap)]
#![allow(clippy::bind_instead_of_map)]
#![allow(clippy::unnecessary_fold)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::let_and_return)]
#![allow(unused_variables)]
#![allow(clippy::iter_count)]

// Ownership, Borrowing, and Lifetimes in Rust
//
// Rust's ownership system is what makes it unique. It enforces memory safety
// without garbage collection through three main rules:
// 1. Each value has a single owner
// 2. When the owner goes out of scope, the value is dropped
// 3. Ownership can be transferred (moved) or temporarily borrowed

fn main() {
    println!("=== Rust Ownership Examples ===\n");

    // Example 1: Basic Ownership and Move Semantics
    println!("1. Basic Ownership:");
    basic_ownership();
    println!();

    // Example 2: Borrowing with References
    println!("2. Borrowing with References:");
    borrowing_examples();
    println!();

    // Example 3: Mutable References
    println!("3. Mutable References:");
    mutable_references();
    println!();

    // Example 4: Lifetimes
    println!("4. Lifetimes:");
    lifetime_examples();
    println!();

    // Example 5: String vs &str
    println!("5. String vs &str:");
    string_types();
    println!();
}

// Example 1: Basic ownership rules
fn basic_ownership() {
    // When a value is assigned to a variable, that variable becomes the owner
    let s1 = String::from("hello");
    println!("  s1 = {}", s1);

    // Move: ownership is transferred from s1 to s2
    let s2 = s1;
    println!("  s2 = {} (ownership moved from s1)", s2);
    // println!("  s1 = {}", s1); // This would cause a compile error!

    // For types that implement Copy (like integers), values are copied instead
    let x = 5;
    let y = x; // x is copied, not moved
    println!("  x = {}, y = {} (integers implement Copy)", x, y);

    // Passing to a function also moves ownership
    let s3 = String::from("world");
    take_ownership(s3);
    // println!("  s3 = {}", s3); // Would error - s3 is no longer valid

    // Functions can return ownership
    let s4 = gives_ownership();
    println!("  s4 = {} (ownership returned from function)", s4);
}

fn take_ownership(s: String) {
    println!("  take_ownership got: {}", s);
    // s goes out of scope and is dropped here
}

fn gives_ownership() -> String {
    String::from("yours")
}

// Example 2: Borrowing with references
fn borrowing_examples() {
    let s1 = String::from("hello");

    // Immutable borrow: we can have multiple immutable references
    let len = calculate_length(&s1);
    println!("  The length of '{}' is {}.", s1, len);

    // Multiple immutable borrows are allowed
    let r1 = &s1;
    let r2 = &s1;
    println!("  r1: {}, r2: {}", r1, r2);

    // We can still use s1 after borrowing
    println!("  s1 is still valid: {}", s1);
}

fn calculate_length(s: &str) -> usize {
    s.len()
    // s goes out of scope but doesn't drop the data because it doesn't own it
}

// Example 3: Mutable references
fn mutable_references() {
    let mut s = String::from("hello");
    println!("  Before: {}", s);

    // Mutable borrow - can only have ONE mutable reference at a time
    change(&mut s);
    println!("  After: {}", s);

    // Can't have immutable and mutable references at the same time
    let r1 = &s; // immutable borrow
    let r2 = &s; // immutable borrow
    println!("  Immutable borrows: {} and {}", r1, r2);
    // r1 and r2 go out of scope here

    // Now we can create a mutable borrow
    let r3 = &mut s;
    r3.push('!');
    println!("  Mutable borrow: {}", r3);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// Example 4: Lifetimes
fn lifetime_examples() {
    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("  The longest string is: {}", result);
    }
    // result can't be used here if it referenced string2
    // because string2 has been dropped

    // Lifetime in structs
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = Excerpt {
        part: first_sentence,
    };
    println!("  Excerpt: {}", excerpt.part);
}

// Lifetime annotation: 'a means both parameters and return value live for the same lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Structs can hold references, but they need lifetime annotations
struct Excerpt<'a> {
    part: &'a str,
}

// Example 5: Understanding String vs &str
fn string_types() {
    // String: owned, heap-allocated, growable
    let mut owned = String::from("hello");
    owned.push_str(" world");
    println!("  String (owned): {}", owned);

    // &str: string slice, reference to string data, immutable
    let slice: &str = &owned[0..5];
    println!("  &str (borrowed slice): {}", slice);

    // String literals are &str
    let literal: &str = "hello";
    println!("  String literal: {}", literal);

    // Function that accepts both String and &str using deref coercion
    print_string(&owned);
    print_string(literal);

    // Ownership transfer vs borrowing
    let s1 = String::from("transfer");
    take_string(s1);
    // s1 is no longer valid here

    let s2 = String::from("borrow");
    borrow_string(&s2);
    println!("  s2 is still valid: {}", s2); // s2 is still valid
}

fn print_string(s: &str) {
    println!("  print_string: {}", s);
}

fn take_string(s: String) {
    println!("  take_string (ownership transferred): {}", s);
}

fn borrow_string(s: &str) {
    println!("  borrow_string (borrowed): {}", s);
}
