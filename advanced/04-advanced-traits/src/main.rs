#![allow(dead_code)]
#![allow(unused_variables)]

// Advanced Traits in Rust
//
// Demonstrates advanced trait concepts including:
// - Associated types vs generic type parameters
// - Trait objects and dynamic dispatch
// - Supertraits
// - Default generic type parameters
// - Fully qualified syntax
// - Newtype pattern for orphan rule
// - Operator overloading
// - Returning trait objects

use std::fmt;
use std::ops::Add;

// ============================================================================
// 1. ASSOCIATED TYPES VS GENERICS
// ============================================================================

// Associated types: One impl per type
trait Iterator {
    type Item; // Associated type

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32; // Concrete type for this impl

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// Generic type parameters: Multiple impls possible
trait Add2<RHS = Self> {
    type Output;

    fn add2(self, rhs: RHS) -> Self::Output;
}

struct Millimeters(u32);
struct Meters(u32);

// Can have multiple impls with different RHS types
impl Add2<Meters> for Millimeters {
    type Output = Millimeters;

    fn add2(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

// ============================================================================
// 2. SUPERTRAITS
// ============================================================================

// Supertrait: requires implementors to also implement Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// Must implement Display first
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Now can implement OutlinePrint
impl OutlinePrint for Point {}

// ============================================================================
// 3. NEWTYPE PATTERN TO BYPASS ORPHAN RULE
// ============================================================================

// Can't implement external trait on external type directly
// But can wrap it in a newtype

struct Wrapper(Vec<String>);

// Now we can implement Display for our wrapper
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// ============================================================================
// 4. TRAIT OBJECTS AND DYNAMIC DISPATCH
// ============================================================================

trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }
}

// Using trait objects for heterogeneous collections
struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn new() -> Self {
        Screen {
            components: Vec::new(),
        }
    }

    fn add(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    fn render(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

// ============================================================================
// 5. RETURNING TRAIT OBJECTS
// ============================================================================

fn returns_drawable(choice: bool) -> Box<dyn Draw> {
    if choice {
        Box::new(Circle { radius: 5.0 })
    } else {
        Box::new(Rectangle {
            width: 10.0,
            height: 20.0,
        })
    }
}

// ============================================================================
// 6. FULLY QUALIFIED SYNTAX
// ============================================================================

trait Pilot {
    fn fly(&self);
    fn name() -> String;
}

trait Wizard {
    fn fly(&self);
    fn name() -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }

    fn name() -> String {
        "Pilot".to_string()
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }

    fn name() -> String {
        "Wizard".to_string()
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }

    fn name() -> String {
        "Human".to_string()
    }
}

// ============================================================================
// 7. OPERATOR OVERLOADING
// ============================================================================

#[derive(Debug, Copy, Clone, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
}

// Implement Add trait for operator overloading
impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{} + {}i", self.real, self.imag)
        } else {
            write!(f, "{} - {}i", self.real, -self.imag)
        }
    }
}

// ============================================================================
// 8. DEFAULT TYPE PARAMETERS
// ============================================================================

// Default type parameter: RHS = Self
trait Multiply<RHS = Self> {
    type Output;

    fn multiply(self, rhs: RHS) -> Self::Output;
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Vector2D {
    x: f64,
    y: f64,
}

// Scalar multiplication (using default parameter)
impl Multiply<f64> for Vector2D {
    type Output = Vector2D;

    fn multiply(self, scalar: f64) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// ============================================================================
// 9. TRAIT BOUNDS IN STRUCTS
// ============================================================================

struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }
}

// Conditional methods based on trait bounds
impl<T: fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("The largest member is first = {}", self.first);
        } else {
            println!("The largest member is second = {}", self.second);
        }
    }
}

// ============================================================================
// 10. EXTENSION TRAITS
// ============================================================================

// Add methods to existing types through extension traits
trait StringExt {
    fn is_palindrome(&self) -> bool;
    fn word_count(&self) -> usize;
}

impl StringExt for str {
    fn is_palindrome(&self) -> bool {
        let cleaned: String = self.chars().filter(|c| c.is_alphanumeric()).collect();
        let cleaned = cleaned.to_lowercase();
        cleaned == cleaned.chars().rev().collect::<String>()
    }

    fn word_count(&self) -> usize {
        self.split_whitespace().count()
    }
}

// ============================================================================
// 11. BLANKET IMPLEMENTATIONS
// ============================================================================

trait MyToString {
    fn my_to_string(&self) -> String;
}

// Blanket implementation: for any type that implements Display
impl<T: fmt::Display> MyToString for T {
    fn my_to_string(&self) -> String {
        format!("{}", self)
    }
}

// ============================================================================
// 12. MARKER TRAITS
// ============================================================================

// Marker trait (no methods)
trait Certified {}

struct SafeProduct {
    name: String,
}

impl Certified for SafeProduct {}

fn require_certified<T: Certified>(_product: &T) {
    println!("Product is certified!");
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

fn main() {
    println!("=== Advanced Traits in Rust ===\n");

    // Example 1: Associated types
    println!("1. Associated Types:");
    let mut counter = Counter { count: 0 };
    while let Some(n) = counter.next() {
        print!("{} ", n);
    }
    println!("\n");

    // Example 2: Supertraits
    println!("2. Supertraits:");
    let point = Point { x: 3, y: 5 };
    point.outline_print();
    println!();

    // Example 3: Newtype pattern
    println!("3. Newtype Pattern:");
    let w = Wrapper(vec![
        String::from("hello"),
        String::from("world"),
        String::from("rust"),
    ]);
    println!("Wrapped vector: {}\n", w);

    // Example 4: Trait objects
    println!("4. Trait Objects:");
    let mut screen = Screen::new();
    screen.add(Box::new(Circle { radius: 5.0 }));
    screen.add(Box::new(Rectangle {
        width: 10.0,
        height: 20.0,
    }));
    screen.add(Box::new(Circle { radius: 3.0 }));
    screen.render();
    println!();

    // Example 5: Returning trait objects
    println!("5. Returning Trait Objects:");
    let shape1 = returns_drawable(true);
    let shape2 = returns_drawable(false);
    shape1.draw();
    shape2.draw();
    println!();

    // Example 6: Fully qualified syntax
    println!("6. Fully Qualified Syntax:");
    let person = Human;
    person.fly(); // Calls Human::fly by default
    Pilot::fly(&person); // Explicitly call Pilot's fly
    Wizard::fly(&person); // Explicitly call Wizard's fly
    println!("Human name: {}", Human::name());
    println!("Pilot name: {}", <Human as Pilot>::name());
    println!("Wizard name: {}", <Human as Wizard>::name());
    println!();

    // Example 7: Operator overloading
    println!("7. Operator Overloading:");
    let c1 = Complex::new(3.0, 4.0);
    let c2 = Complex::new(1.0, 2.0);
    let c3 = c1 + c2;
    println!("{} + {} = {}\n", c1, c2, c3);

    // Example 8: Default type parameters
    println!("8. Default Type Parameters:");
    let v = Vector2D { x: 3.0, y: 4.0 };
    let scaled = v.multiply(2.0);
    println!("Vector {:?} * 2.0 = {:?}\n", v, scaled);

    // Example 9: Conditional methods
    println!("9. Conditional Methods:");
    let pair = Pair::new(10, 20);
    pair.cmp_display();
    println!();

    // Example 10: Extension traits
    println!("10. Extension Traits:");
    let text = "A man a plan a canal Panama";
    println!("Is '{}' a palindrome? {}", text, text.is_palindrome());
    println!("Word count: {}\n", text.word_count());

    // Example 11: Blanket implementations
    println!("11. Blanket Implementations:");
    let number = 42;
    let text = "hello";
    println!("Number as string: {}", number.my_to_string());
    println!("Text as string: {}\n", text.my_to_string());

    // Example 12: Marker traits
    println!("12. Marker Traits:");
    let product = SafeProduct {
        name: "Premium Coffee".to_string(),
    };
    require_certified(&product);

    println!("\nAdvanced traits examples complete!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_addition() {
        let c1 = Complex::new(3.0, 4.0);
        let c2 = Complex::new(1.0, 2.0);
        let result = c1 + c2;
        assert_eq!(result, Complex::new(4.0, 6.0));
    }

    #[test]
    fn test_palindrome() {
        assert_eq!("racecar".is_palindrome(), true);
        assert_eq!("hello".is_palindrome(), false);
        assert_eq!("A man a plan a canal Panama".is_palindrome(), true);
    }

    #[test]
    fn test_word_count() {
        assert_eq!("hello world".word_count(), 2);
        assert_eq!("one two three four".word_count(), 4);
    }

    #[test]
    fn test_vector_multiply() {
        let v = Vector2D { x: 3.0, y: 4.0 };
        let result = v.multiply(2.0);
        assert_eq!(result, Vector2D { x: 6.0, y: 8.0 });
    }
}
