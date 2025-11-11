// Pattern Matching with match in Rust
//
// The match expression is a powerful control flow construct that allows you to
// compare a value against a series of patterns and execute code based on which
// pattern matches. It's exhaustive - you must handle all possible cases.

fn main() {
    println!("=== Rust Pattern Matching Examples ===\n");

    // Example 1: Basic match with integers
    println!("1. Basic Match with Integers:");
    basic_integer_match();
    println!();

    // Example 2: Match with enums
    println!("2. Match with Enums:");
    enum_matching();
    println!();

    // Example 3: Destructuring with match
    println!("3. Destructuring with Match:");
    destructuring_match();
    println!();

    // Example 4: Match guards
    println!("4. Match Guards:");
    match_guards();
    println!();

    // Example 5: Match with Option
    println!("5. Match with Option:");
    option_matching();
    println!();

    // Example 6: Match with Result
    println!("6. Match with Result:");
    result_matching();
    println!();

    // Example 7: Multiple patterns and ranges
    println!("7. Multiple Patterns and Ranges:");
    multiple_patterns();
    println!();

    // Example 8: if let - syntactic sugar
    println!("8. if let - Syntactic Sugar:");
    if_let_examples();
    println!();
}

// Example 1: Basic integer matching
fn basic_integer_match() {
    let number = 7;

    match number {
        1 => println!("  One!"),
        2 | 3 | 5 | 7 | 11 => println!("  {} is a prime number", number),
        13..=19 => println!("  {} is a teen", number),
        _ => println!("  {} is something else", number),
    }

    // match can return values
    let description = match number {
        n if n < 0 => "negative",
        0 => "zero",
        1..=10 => "small positive",
        11..=100 => "medium positive",
        _ => "large positive",
    };
    println!("  {} is {}", number, description);
}

// Example 2: Matching enums
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    // ... etc
}

fn enum_matching() {
    let coin = Coin::Quarter(UsState::Alaska);

    let value = match coin {
        Coin::Penny => {
            println!("  Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("  State quarter from {:?}!", state);
            25
        }
    };

    println!("  Value in cents: {}", value);

    // Matching multiple enum variants
    let coin2 = Coin::Penny;
    match coin2 {
        Coin::Penny | Coin::Nickel => println!("  Small change!"),
        Coin::Dime | Coin::Quarter(_) => println!("  Bigger coin!"),
    }
}

// Example 3: Destructuring with match
fn destructuring_match() {
    // Destructuring tuples
    let point = (3, 5);
    match point {
        (0, 0) => println!("  Origin"),
        (0, y) => println!("  On y-axis at {}", y),
        (x, 0) => println!("  On x-axis at {}", x),
        (x, y) => println!("  At coordinates ({}, {})", x, y),
    }

    // Destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x: 0, y: 0 } => println!("  At origin"),
        Point { x: 0, y } => println!("  On y-axis at y = {}", y),
        Point { x, y: 0 } => println!("  On x-axis at x = {}", x),
        Point { x, y } => println!("  At ({}, {})", x, y),
    }

    // Destructuring nested structures
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    let color = Color::Rgb(122, 17, 40);
    match color {
        Color::Rgb(r, g, b) => {
            println!("  RGB color: red={}, green={}, blue={}", r, g, b)
        }
        Color::Hsv(h, s, v) => {
            println!("  HSV color: hue={}, saturation={}, value={}", h, s, v)
        }
    }
}

// Example 4: Match guards
fn match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("  Less than five: {}", x),
        Some(x) => println!("  Five or more: {}", x),
        None => println!("  No value"),
    }

    // Multiple conditions
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("  Yes"),
        _ => println!("  No (x={}, y={})", x, y),
    }

    // Complex guard conditions
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("  Equal"),
        (x, y) if x + y == 0 => println!("  Pair ({}, {}) sums to zero", x, y),
        (x, _) if x % 2 == 0 => println!("  First is even"),
        _ => println!("  No special relationship"),
    }
}

// Example 5: Matching Option
fn option_matching() {
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    // Match on Option
    match some_number {
        Some(n) => println!("  Got a number: {}", n),
        None => println!("  No number"),
    }

    match no_number {
        Some(n) => println!("  Got a number: {}", n),
        None => println!("  No number found"),
    }

    // Using match to transform Option
    let incremented = match some_number {
        Some(n) => Some(n + 1),
        None => None,
    };
    println!("  Incremented: {:?}", incremented);

    // Nested Option matching
    let nested = Some(Some(5));
    match nested {
        Some(Some(n)) => println!("  Deeply nested value: {}", n),
        Some(None) => println!("  Outer Some, inner None"),
        None => println!("  Outer None"),
    }
}

// Example 6: Matching Result
fn result_matching() {
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");

    match success {
        Ok(value) => println!("  Success! Value: {}", value),
        Err(e) => println!("  Error: {}", e),
    }

    match failure {
        Ok(value) => println!("  Success! Value: {}", value),
        Err(e) => println!("  Failed with error: {}", e),
    }

    // Using match to handle different error types
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
    }

    fn divide(x: f64, y: f64) -> Result<f64, MathError> {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("  Division result: {}", value),
        Err(MathError::DivisionByZero) => println!("  Error: Cannot divide by zero"),
        Err(e) => println!("  Error: {:?}", e),
    }
}

// Example 7: Multiple patterns and ranges
fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("  One or two"),
        3 => println!("  Three"),
        _ => println!("  Something else"),
    }

    // Ranges
    let age = 25;
    match age {
        0 => println!("  Just born"),
        1..=12 => println!("  Child"),
        13..=19 => println!("  Teenager"),
        20..=64 => println!("  Adult ({} years old)", age),
        65.. => println!("  Senior"),
        _ => println!("  Invalid age"),
    }

    // Char ranges
    let letter = 'c';
    match letter {
        'a'..='j' => println!("  Early ASCII letter"),
        'k'..='z' => println!("  Late ASCII letter"),
        _ => println!("  Something else"),
    }
}

// Example 8: if let - syntactic sugar for simple matches
fn if_let_examples() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // if let - when you only care about one pattern
    if let Some(color) = favorite_color {
        println!("  Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("  Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("  Using purple as the background color (age: {})", age);
        } else {
            println!("  Using orange as the background color (age: {})", age);
        }
    } else {
        println!("  Using blue as the background color");
    }

    // while let - loop while pattern matches
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("  Popped: {}", top);
    }
}
