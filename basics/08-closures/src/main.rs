// Closures in Rust
//
// Closures are anonymous functions that can capture their environment.
// They're extremely useful for callbacks, iterator adapters, and functional
// programming patterns. Rust has three closure traits: Fn, FnMut, and FnOnce.

fn main() {
    println!("=== Rust Closures Examples ===\n");

    // Example 1: Basic closure syntax
    println!("1. Basic Closure Syntax:");
    basic_closures();
    println!();

    // Example 2: Type inference and annotations
    println!("2. Type Inference:");
    type_inference();
    println!();

    // Example 3: Capturing environment
    println!("3. Capturing Environment:");
    capturing_environment();
    println!();

    // Example 4: Move closures
    println!("4. Move Closures:");
    move_closures();
    println!();

    // Example 5: Function traits (Fn, FnMut, FnOnce)
    println!("5. Function Traits:");
    function_traits();
    println!();

    // Example 6: Closures as parameters
    println!("6. Closures as Parameters:");
    closures_as_parameters();
    println!();

    // Example 7: Returning closures
    println!("7. Returning Closures:");
    returning_closures();
    println!();

    // Example 8: Closures with iterators
    println!("8. Closures with Iterators:");
    closures_with_iterators();
    println!();
}

// Example 1: Basic closure syntax
fn basic_closures() {
    // Simple closure
    let add_one = |x| x + 1;
    println!("  5 + 1 = {}", add_one(5));

    // Multiple parameters
    let add = |x, y| x + y;
    println!("  3 + 4 = {}", add(3, 4));

    // Multiple statements
    let multiply_and_print = |x, y| {
        let result = x * y;
        println!("    {} * {} = {}", x, y, result);
        result
    };
    let _ = multiply_and_print(6, 7);

    // No parameters
    let greet = || println!("  Hello from closure!");
    greet();
}

// Example 2: Type inference and annotations
fn type_inference() {
    // Rust infers types from usage
    let add = |x, y| x + y;
    println!("  Inferred: {}", add(5, 10));

    // Explicit type annotations
    let add_explicit = |x: i32, y: i32| -> i32 { x + y };
    println!("  Explicit: {}", add_explicit(5, 10));

    // Once a closure is used, types are fixed
    let example = |x| x;
    let _s = example(String::from("hello"));
    // let _n = example(5); // Error! Type is now fixed to String

    // Closure vs function
    fn function(x: i32) -> i32 {
        x + 1
    }
    let closure = |x: i32| -> i32 { x + 1 };

    println!("  Function: {}", function(5));
    println!("  Closure: {}", closure(5));
}

// Example 3: Capturing environment
fn capturing_environment() {
    let x = 4;

    // Immutable borrow
    let equal_to_x = |z| z == x;
    println!("  5 == 4: {}", equal_to_x(5));
    println!("  4 == 4: {}", equal_to_x(4));

    // Can still use x (immutable borrow)
    println!("  x = {}", x);

    // Mutable borrow
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("    Count: {}", count);
    };

    increment();
    increment();
    increment();

    // Can use count after closure is done
    println!("  Final count: {}", count);

    // Multiple captures
    let x = 1;
    let y = 2;
    let sum = |z| x + y + z;
    println!("  1 + 2 + 3 = {}", sum(3));
}

// Example 4: Move closures
fn move_closures() {
    // move keyword transfers ownership
    let list = vec![1, 2, 3];
    println!("  Before move: {:?}", list);

    let contains_move = move |x| list.contains(&x);

    println!("  Contains 2: {}", contains_move(2));
    // println!("{:?}", list); // Error! list was moved

    // Useful for threads
    let data = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        println!("    Thread data: {:?}", data);
    });
    handle.join().unwrap();
    // data is no longer available here

    // Move with Copy types
    let x = 5; // i32 implements Copy
    let closure = move || println!("    x = {}", x);
    closure();
    println!("  x still available: {}", x); // x was copied, not moved
}

// Example 5: Function traits
fn function_traits() {
    // Fn - can be called multiple times, immutable borrow
    let x = 5;
    let fn_closure = || x + 1;
    println!("  Fn: {} {}", fn_closure(), fn_closure());

    // FnMut - can be called multiple times, mutable borrow
    let mut count = 0;
    let mut fn_mut_closure = || {
        count += 1;
        count
    };
    println!("  FnMut: {} {}", fn_mut_closure(), fn_mut_closure());

    // FnOnce - can be called once, consumes captured variables
    let s = String::from("hello");
    let fn_once_closure = || {
        println!("    FnOnce: {}", s);
        s // s is moved out
    };
    let _result = fn_once_closure();
    // fn_once_closure(); // Error! Can only call once

    // All closures implement FnOnce
    // Closures that don't move captured vars implement FnMut
    // Closures that don't mutate captured vars implement Fn
}

// Example 6: Closures as parameters
fn closures_as_parameters() {
    // Generic with Fn trait bound
    fn apply<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }

    let double = |x| x * 2;
    let square = |x| x * x;

    println!("  Apply double: {}", apply(double, 5));
    println!("  Apply square: {}", apply(square, 5));

    // FnMut example
    fn apply_mut<F>(mut f: F, x: i32, times: i32)
    where
        F: FnMut(i32),
    {
        for _ in 0..times {
            f(x);
        }
    }

    let mut sum = 0;
    apply_mut(
        |x| {
            sum += x;
            println!("    Sum now: {}", sum);
        },
        5,
        3,
    );

    // FnOnce example
    fn consume<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    let s = String::from("consumed");
    consume(|| println!("  Consuming: {}", s));
}

// Example 7: Returning closures
fn returning_closures() {
    // Return impl Fn
    fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }

    let add_5 = make_adder(5);
    println!("  10 + 5 = {}", add_5(10));
    println!("  20 + 5 = {}", add_5(20));

    // Return Box<dyn Fn> for dynamic dispatch
    fn make_multiplier(x: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |y| x * y)
    }

    let times_3 = make_multiplier(3);
    println!("  5 * 3 = {}", times_3(5));

    // Factory pattern
    fn make_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
        match op {
            "add" => Box::new(|x, y| x + y),
            "sub" => Box::new(|x, y| x - y),
            "mul" => Box::new(|x, y| x * y),
            "div" => Box::new(|x, y| x / y),
            _ => Box::new(|_, _| 0),
        }
    }

    let add_op = make_operation("add");
    let mul_op = make_operation("mul");
    println!("  Add: {}", add_op(10, 5));
    println!("  Multiply: {}", mul_op(10, 5));
}

// Example 8: Closures with iterators
fn closures_with_iterators() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map with closure
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("  Doubled: {:?}", doubled);

    // filter with closure
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("  Evens: {:?}", evens);

    // Capturing environment in iterator closure
    let threshold = 5;
    let above_threshold: Vec<&i32> = numbers.iter().filter(|x| **x > threshold).collect();
    println!("  Above {}: {:?}", threshold, above_threshold);

    // fold with closure
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("  Sum: {}", sum);

    // Complex chain
    let result: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .filter(|x| *x > 20)
        .collect();
    println!("  Even squares > 20: {:?}", result);

    // Custom operation
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let people = vec![
        Person {
            name: "Alice".to_string(),
            age: 25,
        },
        Person {
            name: "Bob".to_string(),
            age: 30,
        },
        Person {
            name: "Charlie".to_string(),
            age: 35,
        },
    ];

    let names: Vec<String> = people
        .iter()
        .filter(|p| p.age > 28)
        .map(|p| p.name.clone())
        .collect();
    println!("  People over 28: {:?}", names);

    // sort_by with closure
    let mut words = vec!["banana", "apple", "cherry", "date"];
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("  Sorted by length: {:?}", words);
}
