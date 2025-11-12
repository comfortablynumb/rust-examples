#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

// =============================================================================
// 1. FN, FNMUT, AND FNONCE TRAITS
// =============================================================================
// Rust has three closure traits that define how closures capture and use
// variables from their environment:
// - Fn: borrows values immutably
// - FnMut: borrows values mutably
// - FnOnce: takes ownership of values (can only be called once)

fn closure_traits_example() {
    println!("\n--- Closure Traits: Fn, FnMut, FnOnce ---");

    // Fn: immutable borrow - can be called multiple times
    let x = 5;
    let add_x = |y| x + y; // Captures x by immutable reference
    println!("add_x(10) = {}", add_x(10));
    println!("add_x(20) = {}", add_x(20)); // Can call multiple times
    println!("x is still accessible: {}", x);

    // FnMut: mutable borrow - can be called multiple times but requires mut
    let mut count = 0;
    let mut increment = || {
        count += 1; // Captures count by mutable reference
        count
    };
    println!("increment() = {}", increment());
    println!("increment() = {}", increment());
    println!("Final count: {}", count);

    // FnOnce: takes ownership - can only be called once
    let s = String::from("hello");
    let consume = || {
        let _moved = s; // Takes ownership of s
        println!("Consumed string");
    };
    consume(); // Can only call once
    // consume(); // ERROR: cannot call twice
    // println!("{}", s); // ERROR: s has been moved
}

// Functions that accept closures with different traits
fn call_fn<F: Fn()>(f: F) {
    f();
    f(); // Can call multiple times
}

fn call_fn_mut<F: FnMut()>(mut f: F) {
    f();
    f(); // Can call multiple times
}

fn call_fn_once<F: FnOnce()>(f: F) {
    f(); // Can only call once
}

fn closure_trait_bounds_example() {
    println!("\n--- Closure Trait Bounds ---");

    let x = 42;
    call_fn(|| println!("Fn: x = {}", x));

    let mut y = 0;
    call_fn_mut(|| {
        y += 1;
        println!("FnMut: y = {}", y);
    });

    let z = String::from("owned");
    call_fn_once(|| println!("FnOnce: z = {}", z));
}

// =============================================================================
// 2. CLOSURE CAPTURING: BY REFERENCE
// =============================================================================
// By default, closures capture variables by reference (either shared or mutable).

fn capture_by_reference_example() {
    println!("\n--- Capturing by Reference ---");

    let mut numbers = vec![1, 2, 3, 4, 5];
    let len = numbers.len();

    // Captures numbers by immutable reference
    let print_numbers = || {
        println!("Numbers (immutable): {:?}", numbers);
        println!("Length: {}", len);
    };

    print_numbers();
    print_numbers(); // Can call multiple times

    // After closure, we can still read numbers
    println!("Original numbers: {:?}", numbers);

    // Captures numbers by mutable reference
    let mut double_numbers = || {
        for num in &mut numbers {
            *num *= 2;
        }
    };

    double_numbers();
    println!("Doubled numbers: {:?}", numbers);
}

// =============================================================================
// 3. CLOSURE CAPTURING: BY MUTABLE REFERENCE
// =============================================================================
// When a closure mutates captured variables, it captures by mutable reference.

fn capture_by_mutable_reference_example() {
    println!("\n--- Capturing by Mutable Reference ---");

    let mut counter = 0;
    let mut accumulator = 0;

    {
        let mut update = || {
            counter += 1;
            accumulator += counter;
            println!("Counter: {}, Accumulator: {}", counter, accumulator);
        };

        update();
        update();
        update();
    } // Mutable borrow ends here

    println!(
        "Final counter: {}, Final accumulator: {}",
        counter, accumulator
    );
}

// =============================================================================
// 4. CLOSURE CAPTURING: BY VALUE (MOVE KEYWORD)
// =============================================================================
// The 'move' keyword forces a closure to take ownership of captured variables.

fn capture_by_value_example() {
    println!("\n--- Capturing by Value (move) ---");

    let x = vec![1, 2, 3];
    let y = 42;

    // Without move: captures by reference
    let closure1 = || println!("By reference: {:?}", x);
    closure1();
    println!("x is still available: {:?}", x);

    // With move: captures by value (takes ownership)
    let closure2 = move || {
        println!("By value: {:?}", x);
        println!("y: {}", y);
    };
    closure2();
    // println!("{:?}", x); // ERROR: x has been moved

    // move is essential for closures that outlive their environment
    let create_closure = || {
        let z = String::from("inner");
        // Must use move to transfer ownership
        move || println!("Closure owns: {}", z)
    };

    let closure3 = create_closure();
    closure3();
}

// =============================================================================
// 5. RETURNING CLOSURES
// =============================================================================
// Closures can be returned from functions using trait objects or impl Trait.

// Return closure using impl Trait (preferred in modern Rust)
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// Return closure using Box<dyn Fn>
fn make_multiplier(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x * y)
}

// Return mutable closure
fn make_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        count += 1;
        count
    }
}

fn returning_closures_example() {
    println!("\n--- Returning Closures ---");

    let add_5 = make_adder(5);
    println!("add_5(10) = {}", add_5(10));
    println!("add_5(20) = {}", add_5(20));

    let multiply_3 = make_multiplier(3);
    println!("multiply_3(10) = {}", multiply_3(10));
    println!("multiply_3(20) = {}", multiply_3(20));

    let mut counter = make_counter(0);
    println!("counter() = {}", counter());
    println!("counter() = {}", counter());
    println!("counter() = {}", counter());
}

// =============================================================================
// 6. CLOSURES AS FUNCTION PARAMETERS
// =============================================================================
// Closures can be passed as parameters using generic bounds or trait objects.

// Using generic type parameter (monomorphization)
fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

// Using trait object (dynamic dispatch)
fn apply_to_vec(f: &dyn Fn(i32) -> i32, vec: &[i32]) -> Vec<i32> {
    vec.iter().map(|&x| f(x)).collect()
}

// Accepting FnMut closure
fn repeat_n_times<F>(mut f: F, n: usize)
where
    F: FnMut(),
{
    for _ in 0..n {
        f();
    }
}

// Accepting FnOnce closure
fn execute_once<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    f()
}

fn closures_as_parameters_example() {
    println!("\n--- Closures as Function Parameters ---");

    let result = apply_twice(|x| x + 1, 5);
    println!("apply_twice(|x| x + 1, 5) = {}", result);

    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = apply_to_vec(&|x| x * 2, &numbers);
    println!("Doubled: {:?}", doubled);

    let mut counter = 0;
    repeat_n_times(
        || {
            counter += 1;
            println!("Iteration {}", counter);
        },
        3,
    );

    let result = execute_once(|| {
        println!("Executing once");
        42
    });
    println!("Result: {}", result);
}

// =============================================================================
// 7. CLOSURE COERCION
// =============================================================================
// Closures that don't capture variables can be coerced to function pointers.

fn closure_coercion_example() {
    println!("\n--- Closure Coercion ---");

    // Closure with no captures can coerce to function pointer
    let add = |x: i32, y: i32| x + y;
    let fn_ptr: fn(i32, i32) -> i32 = add; // Coercion
    println!("fn_ptr(3, 4) = {}", fn_ptr(3, 4));

    // This works with standard library functions
    let numbers = [1, 2, 3];
    let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();
    println!("Strings: {:?}", strings);

    // Function that accepts both closures and function pointers
    fn call_with_args(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
        f(a, b)
    }

    println!("call_with_args(add, 5, 7) = {}", call_with_args(add, 5, 7));
}

// =============================================================================
// 8. ITERATORS WITH CLOSURES
// =============================================================================
// Iterators heavily use closures for data transformation and filtering.

fn iterators_with_closures_example() {
    println!("\n--- Iterators with Closures ---");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map: transform each element
    let squares: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Squares: {:?}", squares);

    // filter: keep elements that satisfy a predicate
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("Evens: {:?}", evens);

    // fold: reduce to a single value
    #[allow(clippy::unnecessary_fold)]
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);

    // Chain multiple operations
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x > 3)
        .map(|&x| x * 2)
        .take(4)
        .collect();
    println!("Chained operations: {:?}", result);

    // find: find first element matching predicate
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("First even: {:?}", first_even);

    // any and all: check if any/all elements satisfy predicate
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("Has even: {}, All positive: {}", has_even, all_positive);
}

// =============================================================================
// 9. HIGHER-ORDER FUNCTIONS
// =============================================================================
// Functions that take functions/closures as arguments or return them.

fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn partial_apply<F>(f: F, x: i32) -> impl Fn(i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    move |y| f(x, y)
}

fn higher_order_functions_example() {
    println!("\n--- Higher-Order Functions ---");

    // Function composition
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    let add_one_then_double = compose(add_one, double);
    println!("add_one_then_double(5) = {}", add_one_then_double(5));

    // Partial application
    let add = |x: i32, y: i32| x + y;
    let add_10 = partial_apply(add, 10);
    println!("add_10(5) = {}", add_10(5));
    println!("add_10(15) = {}", add_10(15));
}

// =============================================================================
// 10. CLOSURES WITH COMPLEX STATE
// =============================================================================
// Closures can capture and maintain complex state.

fn complex_state_example() {
    println!("\n--- Closures with Complex State ---");

    // Closure maintaining a cache
    let mut cache: HashMap<i32, i32> = HashMap::new();
    let mut memoized_square = |x: i32| {
        *cache.entry(x).or_insert_with(|| {
            println!("Computing square of {}", x);
            x * x
        })
    };

    println!("First call: {}", memoized_square(5));
    println!("Second call: {}", memoized_square(5)); // Cached
    println!("Third call: {}", memoized_square(10));

    // Closure with multiple captured variables
    let mut total = 0;
    let mut count = 0;
    let mut running_average = |x: i32| {
        total += x;
        count += 1;
        total as f64 / count as f64
    };

    println!("Average after adding 10: {}", running_average(10));
    println!("Average after adding 20: {}", running_average(20));
    println!("Average after adding 30: {}", running_average(30));
}

// =============================================================================
// 11. CLOSURE RECURSION
// =============================================================================
// Closures can be recursive using explicit type annotations.

fn closure_recursion_example() {
    println!("\n--- Closure Recursion ---");

    // Recursive closure requires explicit type and Fn trait
    fn make_factorial() -> impl Fn(u32) -> u32 {
        fn factorial_impl(n: u32) -> u32 {
            if n <= 1 { 1 } else { n * factorial_impl(n - 1) }
        }
        factorial_impl
    }

    let factorial = make_factorial();
    println!("factorial(5) = {}", factorial(5));
    println!("factorial(10) = {}", factorial(10));

    // Using Y combinator pattern for more complex recursion
    let fib = |n: u32| {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let temp = a;
            a = b;
            b += temp;
        }
        a
    };

    println!("fib(10) = {}", fib(10));
}

// =============================================================================
// 12. CLOSURES IN DATA STRUCTURES
// =============================================================================
// Storing closures in structs and vectors.

struct EventHandler<F>
where
    F: Fn(&str),
{
    handler: F,
}

impl<F> EventHandler<F>
where
    F: Fn(&str),
{
    fn new(handler: F) -> Self {
        EventHandler { handler }
    }

    fn handle(&self, event: &str) {
        (self.handler)(event);
    }
}

// Vec of trait objects
fn closures_in_data_structures_example() {
    println!("\n--- Closures in Data Structures ---");

    // Closure in a struct
    let handler = EventHandler::new(|event| {
        println!("Handling event: {}", event);
    });
    handler.handle("user_login");
    handler.handle("user_logout");

    // Vec of closures using trait objects
    let operations: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x * x),
    ];

    let input = 5;
    for (i, op) in operations.iter().enumerate() {
        println!("Operation {}: f({}) = {}", i, input, op(input));
    }
}

// =============================================================================
// 13. LAZY EVALUATION WITH CLOSURES
// =============================================================================
// Closures enable lazy evaluation patterns.

struct Lazy<F, T>
where
    F: FnOnce() -> T,
{
    init: Option<F>,
    value: Option<T>,
}

impl<F, T> Lazy<F, T>
where
    F: FnOnce() -> T,
{
    fn new(init: F) -> Self {
        Lazy {
            init: Some(init),
            value: None,
        }
    }

    fn get(&mut self) -> &T {
        if self.value.is_none() {
            let init = self.init.take().unwrap();
            self.value = Some(init());
        }
        self.value.as_ref().unwrap()
    }
}

fn lazy_evaluation_example() {
    println!("\n--- Lazy Evaluation with Closures ---");

    let mut expensive = Lazy::new(|| {
        println!("Computing expensive value...");
        42
    });

    println!("Lazy value created but not computed yet");
    println!("First access: {}", expensive.get());
    println!("Second access: {}", expensive.get()); // No recomputation
}

// =============================================================================
// 14. CLOSURE SIZE AND PERFORMANCE
// =============================================================================
// Understanding closure memory layout and performance.

fn closure_size_example() {
    println!("\n--- Closure Size and Performance ---");

    // Closure with no captures (zero-sized)
    let no_capture = || 42;
    println!(
        "Size of closure with no captures: {} bytes",
        std::mem::size_of_val(&no_capture)
    );

    // Closure capturing a small value
    let x = 42;
    let capture_i32 = || x;
    println!(
        "Size of closure capturing i32: {} bytes",
        std::mem::size_of_val(&capture_i32)
    );

    // Closure capturing multiple values
    let a = 10;
    let b = 20;
    let c = 30;
    let capture_multiple = || a + b + c;
    println!(
        "Size of closure capturing 3 i32s: {} bytes",
        std::mem::size_of_val(&capture_multiple)
    );

    // Closure capturing a Vec (captures by reference)
    #[allow(clippy::useless_vec)]
    let vec = vec![1, 2, 3, 4, 5];
    let capture_vec = || vec.len();
    println!(
        "Size of closure capturing Vec by ref: {} bytes",
        std::mem::size_of_val(&capture_vec)
    );

    // Closure moving a Vec (captures by value)
    #[allow(clippy::useless_vec)]
    let vec2 = vec![1, 2, 3, 4, 5];
    let move_vec = move || vec2.len();
    println!(
        "Size of closure capturing Vec by move: {} bytes",
        std::mem::size_of_val(&move_vec)
    );
}

// =============================================================================
// MAIN FUNCTION
// =============================================================================

fn main() {
    println!("=== Advanced Closures ===");

    closure_traits_example();
    closure_trait_bounds_example();
    capture_by_reference_example();
    capture_by_mutable_reference_example();
    capture_by_value_example();
    returning_closures_example();
    closures_as_parameters_example();
    closure_coercion_example();
    iterators_with_closures_example();
    higher_order_functions_example();
    complex_state_example();
    closure_recursion_example();
    closures_in_data_structures_example();
    lazy_evaluation_example();
    closure_size_example();

    println!("\n=== All examples completed successfully! ===");
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_traits() {
        // Fn trait
        let x = 5;
        let add_x = |y| x + y;
        assert_eq!(add_x(10), 15);
        assert_eq!(add_x(20), 25);

        // FnMut trait
        let mut count = 0;
        let mut increment = || {
            count += 1;
            count
        };
        assert_eq!(increment(), 1);
        assert_eq!(increment(), 2);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_returning_closures() {
        let add_5 = make_adder(5);
        assert_eq!(add_5(10), 15);
        assert_eq!(add_5(20), 25);

        let multiply_3 = make_multiplier(3);
        assert_eq!(multiply_3(10), 30);
        assert_eq!(multiply_3(20), 60);

        let mut counter = make_counter(0);
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_closures_as_parameters() {
        let result = apply_twice(|x| x + 1, 5);
        assert_eq!(result, 7); // 5 + 1 + 1

        let numbers = vec![1, 2, 3, 4, 5];
        let doubled = apply_to_vec(&|x| x * 2, &numbers);
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);

        let result = execute_once(|| 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_iterators_with_closures() {
        let numbers = vec![1, 2, 3, 4, 5];

        let squares: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
        assert_eq!(squares, vec![1, 4, 9, 16, 25]);

        let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
        assert_eq!(evens, vec![2, 4]);

        let sum = numbers.iter().fold(0, |acc, &x| acc + x);
        assert_eq!(sum, 15);

        let has_even = numbers.iter().any(|&x| x % 2 == 0);
        assert!(has_even);

        let all_positive = numbers.iter().all(|&x| x > 0);
        assert!(all_positive);
    }

    #[test]
    fn test_higher_order_functions() {
        let add_one = |x: i32| x + 1;
        let double = |x: i32| x * 2;
        let composed = compose(add_one, double);
        assert_eq!(composed(5), 12); // (5 + 1) * 2

        let add = |x: i32, y: i32| x + y;
        let add_10 = partial_apply(add, 10);
        assert_eq!(add_10(5), 15);
        assert_eq!(add_10(15), 25);
    }

    #[test]
    fn test_lazy_evaluation() {
        use std::cell::Cell;

        let call_count = Cell::new(0);
        let mut lazy = Lazy::new(|| {
            call_count.set(call_count.get() + 1);
            42
        });

        assert_eq!(call_count.get(), 0); // Not called yet
        assert_eq!(*lazy.get(), 42);
        assert_eq!(call_count.get(), 1); // Called once
        assert_eq!(*lazy.get(), 42);
        assert_eq!(call_count.get(), 1); // Still only called once
    }

    #[test]
    fn test_closures_in_data_structures() {
        let handler = EventHandler::new(|event| {
            assert!(event.starts_with("test_"));
        });
        handler.handle("test_event");

        let operations: Vec<Box<dyn Fn(i32) -> i32>> = vec![
            Box::new(|x| x + 1),
            Box::new(|x| x * 2),
            Box::new(|x| x * x),
        ];

        assert_eq!(operations[0](5), 6);
        assert_eq!(operations[1](5), 10);
        assert_eq!(operations[2](5), 25);
    }

    #[test]
    fn test_closure_capturing() {
        // By reference
        let x = vec![1, 2, 3];
        let len_closure = || x.len();
        assert_eq!(len_closure(), 3);
        assert_eq!(x.len(), 3); // x still accessible

        // By value (move)
        let y = vec![1, 2, 3, 4];
        let move_closure = move || y.len();
        assert_eq!(move_closure(), 4);
        // y is no longer accessible here
    }
}
