// Traits in Rust
//
// Traits define shared behavior across types. They're similar to interfaces
// in other languages but more powerful. Traits enable polymorphism, generic
// programming, and are fundamental to Rust's type system.

use std::fmt;

fn main() {
    println!("=== Rust Traits Examples ===\n");

    // Example 1: Basic trait definition and implementation
    println!("1. Basic Traits:");
    basic_traits();
    println!();

    // Example 2: Default implementations
    println!("2. Default Implementations:");
    default_implementations();
    println!();

    // Example 3: Traits as parameters
    println!("3. Traits as Parameters:");
    traits_as_parameters();
    println!();

    // Example 4: Trait bounds
    println!("4. Trait Bounds:");
    trait_bounds();
    println!();

    // Example 5: Multiple trait bounds
    println!("5. Multiple Trait Bounds:");
    multiple_trait_bounds();
    println!();

    // Example 6: Associated types
    println!("6. Associated Types:");
    associated_types();
    println!();

    // Example 7: Trait objects (dynamic dispatch)
    println!("7. Trait Objects:");
    trait_objects();
    println!();

    // Example 8: Derivable traits
    println!("8. Derivable Traits:");
    derivable_traits();
    println!();

    // Example 9: Operator overloading with traits
    println!("9. Operator Overloading:");
    operator_overloading();
    println!();
}

// Example 1: Basic trait definition
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    location: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn basic_traits() {
    let article = Article {
        headline: String::from("Rust 2.0 Released!"),
        location: String::from("San Francisco, CA"),
        author: String::from("Alice"),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust is awesome!"),
        reply: false,
    };

    println!("  Article: {}", article.summarize());
    println!("  Tweet: {}", tweet.summarize());
}

// Example 2: Default implementations
trait Greet {
    fn name(&self) -> String;

    // Default implementation
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }

    // Can be overridden
    fn formal_greet(&self) -> String {
        format!("Good day, {}.", self.name())
    }
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn name(&self) -> String {
        self.name.clone()
    }

    // Override the default
    fn formal_greet(&self) -> String {
        format!("Greetings, Mr./Ms. {}.", self.name())
    }
}

fn default_implementations() {
    let person = Person {
        name: String::from("Bob"),
    };

    println!("  {}", person.greet()); // Uses default
    println!("  {}", person.formal_greet()); // Uses override
}

// Example 3: Traits as parameters
fn notify(item: &impl Summary) {
    println!("  Breaking news! {}", item.summarize());
}

// Trait bound syntax (equivalent to above)
fn notify_verbose<T: Summary>(item: &T) {
    println!("  Alert! {}", item.summarize());
}

fn traits_as_parameters() {
    let tweet = Tweet {
        username: String::from("tech_news"),
        content: String::from("Rust reaches 1 million users!"),
        reply: false,
    };

    notify(&tweet);
    notify_verbose(&tweet);
}

// Example 4: Trait bounds in generic functions
trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("Integer: {}", self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("String: {}", self)
    }
}

fn print_item<T: Printable>(item: T) {
    println!("  {}", item.format());
}

fn trait_bounds() {
    print_item(42);
    print_item(String::from("Hello, Rust!"));
}

// Example 5: Multiple trait bounds
trait Draw {
    fn draw(&self) -> String;
}

trait Resize {
    fn resize(&self, scale: f32) -> String;
}

struct Circle {
    radius: f32,
}

impl Draw for Circle {
    fn draw(&self) -> String {
        format!("Drawing circle with radius {}", self.radius)
    }
}

impl Resize for Circle {
    fn resize(&self, scale: f32) -> String {
        format!("Resizing circle from {} to {}", self.radius, self.radius * scale)
    }
}

// Multiple bounds with +
fn draw_and_resize<T: Draw + Resize>(shape: &T, scale: f32) {
    println!("  {}", shape.draw());
    println!("  {}", shape.resize(scale));
}

// where clause for complex bounds
fn process<T, U>(t: &T, u: &U)
where
    T: Draw + Resize,
    U: Draw,
{
    println!("  Processing shape 1: {}", t.draw());
    println!("  Processing shape 2: {}", u.draw());
}

fn multiple_trait_bounds() {
    let circle = Circle { radius: 5.0 };
    draw_and_resize(&circle, 2.0);
}

// Example 6: Associated types
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn associated_types() {
    let mut counter = Counter::new();

    println!("  Counting:");
    while let Some(num) = counter.next() {
        println!("    {}", num);
    }
}

// Example 7: Trait objects for dynamic dispatch
trait Animal {
    fn make_sound(&self) -> String;
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        "Meow!".to_string()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

fn trait_objects() {
    // Using trait objects with Box<dyn Trait>
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: String::from("Rex"),
        }),
        Box::new(Cat {
            name: String::from("Whiskers"),
        }),
        Box::new(Dog {
            name: String::from("Buddy"),
        }),
    ];

    println!("  Animals:");
    for animal in animals {
        println!("    {} says: {}", animal.name(), animal.make_sound());
    }

    // Using &dyn Trait
    let dog = Dog {
        name: String::from("Max"),
    };
    let animal_ref: &dyn Animal = &dog;
    println!("  Reference: {} says {}", animal_ref.name(), animal_ref.make_sound());
}

// Example 8: Derivable traits
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

fn derivable_traits() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 2, y: 1 };

    // Debug
    println!("  Debug: {:?}", p1);

    // Clone
    let p1_clone = p1.clone();
    println!("  Cloned: {:?}", p1_clone);

    // PartialEq
    println!("  p1 == p2: {}", p1 == p2);
    println!("  p1 == p3: {}", p1 == p3);

    // PartialOrd
    println!("  p1 < p3: {}", p1 < p3);
}

// Example 9: Operator overloading
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Vector {
    x: f64,
    y: f64,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Custom Display trait
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn operator_overloading() {
    let v1 = Vector { x: 1.0, y: 2.0 };
    let v2 = Vector { x: 3.0, y: 4.0 };

    let v3 = v1 + v2;

    println!("  {} + {} = {}", v1, v2, v3);
}
